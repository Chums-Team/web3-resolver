use std::collections::HashMap;
use std::str::FromStr;
use anyhow::{Context, Result, anyhow};
use async_trait::async_trait;
use log::debug;
use nekoton::abi::FunctionExt;
use nekoton::transport::jrpc::JrpcTransport;
use nekoton::transport::Transport;
use nekoton::transport::models::RawContractState;
use nekoton_abi::unpack_from_cell;
use nekoton_abi::num_traits::ToPrimitive;
use nekoton_transport::jrpc::JrpcClient;
use reqwest::IntoUrl;
use ton_abi::{Token, Contract, TokenValue, ParamType, Param, Uint, contract};
use ton_block::{MsgAddressInt, MsgAddrStd, AccountStuff};
use ton_types::{AccountId, Cell, SliceData};
use crate::resolver::ipfs::make_ipfs_link;
use crate::resolver::{abi, Resolver};
use crate::models::{ResolvedDomainData, AddressTag};

const ROOT_ADDRESS: &str = "a7d0694c025b61e1a4a846f1cf88980a5df8adf737d17ac58e35bf172c9fca29";
pub const EVERSCALE_RPC_ENDPOINT: &str = "https://jrpc.everwallet.net/rpc";

pub struct EvernameResolver {
    jrpc_transport: JrpcTransport,
    root_address: MsgAddressInt,
    root_contract: Contract,
    domain_contract: Contract,
    onchain_site_contract: Contract,
}

impl EvernameResolver {
    pub fn new<U: IntoUrl>(jrpc_endpoint: U) -> Result<Self> {
        let jrpc_endpoint = jrpc_endpoint.into_url()?;
        let jrpc_client = JrpcClient::new(jrpc_endpoint)?;
        let jrpc_transport = JrpcTransport::new(jrpc_client.clone());
        let root_address = MsgAddressInt::AddrStd(MsgAddrStd{
            anycast: None,
            workchain_id: 0,
            address: AccountId::from_string(ROOT_ADDRESS)?,
        });
        let root_contract = Contract::load(abi::ROOT_ABI_JSON)?;
        let domain_contract = Contract::load(abi::DOMAIN_ABI_JSON)?;
        let onchain_site_contract = Contract::load(abi::ONCHAIN_SITE_ABI_JSON)?;
        Ok(Self {
            jrpc_transport,
            root_address,
            root_contract,
            domain_contract,
            onchain_site_contract
        })
    }
    
    pub fn default() -> Result<Self> {
        Self::new(EVERSCALE_RPC_ENDPOINT)
    }
}

#[async_trait]
impl Resolver for EvernameResolver {
    async fn resolve(&self, domain: &str) -> Result<(ResolvedDomainData, AddressTag)> {
        let resolved_address = self.address_contract(domain.to_string()).await?;
        let records = self.get_records(&resolved_address).await?;
        for tag in AddressTag::resolvable() {
            debug!("Resolving address {} with tag {}", domain, tag);
            if let Some(cell_value) = records.get(&tag) {
                let domain_data = match tag {
                    AddressTag::Onchain => {
                        let cell_value = string_cell_value(cell_value)?;
                        ResolvedDomainData::OnchainData(cell_value)
                    },
                    AddressTag::OnchainContract => {
                        let contract_address = address_cell_value(cell_value)?;
                        debug!("Resolving onchain contract {}", contract_address);
                        let (content, content_type) = self.load_content_from_contract(&contract_address).await?;
                        ResolvedDomainData::OnchainContractData((content, content_type))
                    },
                    AddressTag::Ipfs => {
                        let cell_value = string_cell_value(cell_value)?;
                        let ipfs_url = make_ipfs_link(&cell_value);
                        ResolvedDomainData::DomainString(ipfs_url)
                    },
                    _ => {
                        let cell_value = string_cell_value(cell_value)?;
                        ResolvedDomainData::DomainString(cell_value)
                    },
                };
                return Ok((domain_data, tag));
            }
        }
        Err(anyhow!("No address for requested domain {}", domain))
    }
}

impl EvernameResolver {
    async fn get_contract_state(&self, address: &MsgAddressInt) -> Result<AccountStuff> {
        let state = self.jrpc_transport.get_contract_state(address).await?;
        match state {
            RawContractState::NotExists { .. } => Err(anyhow!("No account state")),
            RawContractState::Exists(contract) => Ok(contract.account)
        }
    }

    async fn address_contract(&self, address_url: String) -> Result<MsgAddressInt> {
        let function = self.root_contract.function("resolve")
            .context("Failed to load 'resolve' function from contract DomainRoot")?;
        let state = self.get_contract_state(&self.root_address).await?;

        let clock = nekoton_utils::SimpleClock{};
        let result = function.run_local(
            &clock,
            state,
            &[
                Token::new("answerId", TokenValue::Uint(Uint::new(0, 32))),
                Token::new("path", TokenValue::String(address_url)),
            ]
        )?;

        let token = result.tokens.ok_or_else(|| anyhow!("empty output"))?
            .into_iter()
            .find(|token| token.name == "certificate")
            .ok_or_else(|| anyhow!("no certificate value"))?;

        match token.value {
            TokenValue::Address(address) => address.to_msg_addr_int().context("missed certificate address"),
            _ => Err(anyhow!("wrong certificate value")),
        }
    }

    

    async fn get_records(&self, address: &MsgAddressInt) -> Result<HashMap<AddressTag, Cell>> {
        let function = self.domain_contract.function("getRecords")
            .context("Failed to load 'getRecords' function from contract Domain")?;

        let state = self.get_contract_state(address).await?;

        let clock = nekoton_utils::SimpleClock{};
        let result = function.run_local(
            &clock,
            state,
            &[
                Token::new("answerId", TokenValue::Uint(Uint::new(0, 32)))
            ]
        )?;

        let token = result.tokens.ok_or_else(|| anyhow!("empty output"))?
            .into_iter()
            .find(|token| token.name == "records")
            .ok_or_else(|| anyhow!("No value"))?;

        match token.value {
            TokenValue::Map(ParamType::Uint(32), ParamType::Cell, content) => {
                let mut result = HashMap::new();
                for (key, cell) in content {
                    let key_token = TokenValue::from(key);
                    match (key_token, cell) {
                        (TokenValue::Uint(uint), TokenValue::Cell(cell)) => {
                            let key_u32 = uint.number.to_u32().ok_or_else(|| anyhow!("could not convert map key to uint32: {}", uint.number))?;
                            if let Ok(tag) = AddressTag::try_from(key_u32) {
                                result.insert(tag, cell);
                            }
                        },
                        _ => return Err(anyhow!("bad map value"))
                    }
                }
                Ok(result)
            },
            _ => Err(anyhow!("none value")),
        }
    }

    async fn load_content_from_contract(&self, address: &str) -> Result<(String, String)> {
        let function = self.onchain_site_contract.function("getDetails")
            .context("Failed to load 'getDetails' function from contract Eversite")?;

        let msg_address = MsgAddressInt::from_str(address)?;
        let state = self.get_contract_state(&msg_address).await?;

        let clock = nekoton_utils::SimpleClock{};
        let result = function.run_local(
            &clock,
            state,
            &[]
        )?;
        let tokens = result.tokens.ok_or_else(|| anyhow!("empty output"))?;
        let content = tokens
            .iter()
            .find(|token| token.name == "content")
            .map(|token| &token.value)
            .ok_or_else(|| anyhow!("No content"))?;
        let default_content_type = "text/html; charset=utf-8".to_string();
        let content_type = tokens
            .iter()
            .find(|token| token.name == "contentType")
            .map(|token| &token.value)
            .and_then(|value| match value {
                TokenValue::String(ct) => Some(ct),
                _ => None
            })
            .unwrap_or(&default_content_type);

        match content {
            TokenValue::Map(ParamType::Uint(8), ParamType::Cell, content) => {
                let mut result = String::new();
                for (_, cell) in content {
                    match cell {
                        TokenValue::Cell(cell) => {
                            let partial_params = &[Param::new("value0", ParamType::String)];
                            let data = SliceData::load_cell_ref(cell)?;
                            match unpack_from_cell(
                                partial_params, data, false, contract::ABI_VERSION_2_0
                            )?.get(0).context("malformed cell data")? {
                                Token { name: _, value: TokenValue::String(s) } => {
                                    result.push_str(s);
                                },
                                _ => return Err(anyhow!("malformed cell data"))
                            }
                        },
                        _ => return Err(anyhow!("bad cell value in map"))
                    }
                }
                Ok((result, content_type.to_string()))
            },
            _ => Err(anyhow!("wrong getDetails value")),
        }
    }
}

fn string_cell_value(cell: &Cell) -> Result<String> {
    let partial_params = &[Param::new("value", ParamType::String)];
    let data = SliceData::load_cell_ref(cell)?;
    let tokens = unpack_from_cell(partial_params, data, false, contract::ABI_VERSION_2_0)?;
    let first_token = tokens.get(0).context("malformed cell data")?;
    match first_token {
        Token { name: _, value: TokenValue::String(s) } => Ok(s.clone()),
        _ => Err(anyhow!("malformed cell data"))
    }
}

fn address_cell_value(cell: &Cell) -> Result<String> {
    let partial_params = &[Param::new("value", ParamType::Address)];
    let data = SliceData::load_cell_ref(cell)?;
    let tokens = unpack_from_cell(partial_params, data, false, contract::ABI_VERSION_2_0)?;
    let first_token = tokens.get(0).context("malformed cell data")?;
    match first_token {
        Token { name: _, value: TokenValue::Address(address) } => Ok(address.to_string()),
        _ => Err(anyhow!("malformed cell data"))
    }
}
