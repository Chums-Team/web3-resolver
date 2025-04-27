use std::fmt::{Display, Formatter};
use anyhow::{anyhow, Result};

#[derive(Clone, PartialEq, Hash, Eq)]
pub enum AddressTag {
    Tor,
    Ipfs,
    Web2,
    Onchain,
    OnchainContract,
    NonWeb3,
    UnstoppableDomain,
}

impl AddressTag {
    const TOR_ADDRESS_TAG: u128 = 1001;
    const IPFS_ADDRESS_TAG: u128 = 1002;
    const WEB2_ADDRESS_TAG: u128 = 1003;
    const ONCHAIN_ADDRESS_TAG: u128 = 1004;
    const ONCHAIN_CONTRACT_ADDRESS_TAG: u128 = 1005;

    pub fn tag(&self) -> u128 {
        match self {
            AddressTag::Tor => Self::TOR_ADDRESS_TAG,
            AddressTag::Ipfs => Self::IPFS_ADDRESS_TAG,
            AddressTag::Web2 => Self::WEB2_ADDRESS_TAG,
            AddressTag::Onchain => Self::ONCHAIN_ADDRESS_TAG,
            AddressTag::OnchainContract => Self::ONCHAIN_CONTRACT_ADDRESS_TAG,
            AddressTag::NonWeb3 => 0,
            AddressTag::UnstoppableDomain => 0,
        }
    }

    /// Available address tags for resolving
    /// Order is priority!
    pub fn resolvable() -> Vec<AddressTag> {
        vec![
            AddressTag::Tor,
            AddressTag::Ipfs,
            AddressTag::Web2,
            AddressTag::Onchain,
            AddressTag::OnchainContract
        ]
    }
}

impl TryFrom<u32> for AddressTag {
    type Error = anyhow::Error;

    fn try_from(value: u32) -> Result<Self> {
        match value as u128 {
            AddressTag::TOR_ADDRESS_TAG => Ok(AddressTag::Tor),
            AddressTag::IPFS_ADDRESS_TAG => Ok(AddressTag::Ipfs),
            AddressTag::WEB2_ADDRESS_TAG => Ok(AddressTag::Web2),
            AddressTag::ONCHAIN_ADDRESS_TAG => Ok(AddressTag::Onchain),
            AddressTag::ONCHAIN_CONTRACT_ADDRESS_TAG => Ok(AddressTag::OnchainContract),
            _ => Err(anyhow!("Unknown address tag"))
        }
    }
}

impl Display for AddressTag {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            AddressTag::Tor => write!(f, "tor({})", Self::TOR_ADDRESS_TAG),
            AddressTag::Ipfs => write!(f, "ipfs({})", Self::IPFS_ADDRESS_TAG),
            AddressTag::Web2 => write!(f, "ip({})", Self::WEB2_ADDRESS_TAG),
            AddressTag::Onchain => write!(f, "onchain({})", Self::ONCHAIN_ADDRESS_TAG),
            AddressTag::OnchainContract => write!(f, "onchain-contract({})", Self::ONCHAIN_CONTRACT_ADDRESS_TAG),
            AddressTag::NonWeb3 => write!(f, "non-ever(plain)"),
            AddressTag::UnstoppableDomain => write!(f, "unstoppable-domain"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum ResolvedDomainData {
    DomainString(String),
    OnchainData(String),
    OnchainContractData((String, String)),
}

impl Display for ResolvedDomainData {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ResolvedDomainData::DomainString(s) => write!(f, "DomainString({})", s),
            ResolvedDomainData::OnchainData(s) => write!(f, "OnchainData({}...)", &s.get(0..10).unwrap_or_default()),
            ResolvedDomainData::OnchainContractData((content, content_type)) =>
                write!(f, "OnchainContractData({}..., {})", &content.get(0..10).unwrap_or_default(), content_type),
        }
    }
}