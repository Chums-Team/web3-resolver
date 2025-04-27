use crate::models::{AddressTag, ResolvedDomainData};
use crate::resolver::evername::EvernameResolver;
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use log::debug;
use mini_moka::sync::Cache;
use std::time::Duration;
use ud::UnstoppableDomainsResolver;

pub mod ud;
pub mod evername;
pub mod builder;
mod abi;
mod ipfs;


#[async_trait]
pub trait Resolver {
    async fn resolve(&self, domain: &str) -> Result<(ResolvedDomainData, AddressTag)>;
}

pub struct Web3DomainResolver {
    ud_resolver: UnstoppableDomainsResolver,
    evername_resolver: EvernameResolver,
    dns_cache: Option<Cache<String, (ResolvedDomainData, AddressTag)>>,
}

impl Web3DomainResolver {
    pub fn builder() -> builder::DomainResolverBuilder {
        builder::DomainResolverBuilder::default()
    }
    
    pub async fn default() -> Result<Self> {
        let ud_resolver = UnstoppableDomainsResolver::default().await?;
        let evername_resolver = EvernameResolver::default()?;
        let dns_cache = Some(Cache::builder().time_to_live(Duration::from_secs(5 * 60)).build());
        Ok(Self {
            ud_resolver,
            evername_resolver,
            dns_cache
        })
    }
    
    pub(crate) fn new(ud_resolver: UnstoppableDomainsResolver, 
                      evername_resolver: EvernameResolver,
                      dns_cache: Option<Cache<String, (ResolvedDomainData, AddressTag)>>) -> Self {
        Self {
            ud_resolver,
            evername_resolver,
            dns_cache
        }
    }

    pub async fn resolve(&self, domain: &str) -> Result<(ResolvedDomainData, AddressTag)> {
        let domain = domain.to_owned();
        if let Some(cache) = &self.dns_cache {
            if let Some(found) = cache.get(&domain) {
                return Ok(found);
            }
        }
        let (resolved_data, address_tag) = if domain.ends_with(".ever") {
            let (resolved_data, address_tag) = self.evername_resolver.resolve(&domain).await?;
            debug!("Ever host {} resolved into: {} with tag {}", domain, resolved_data, address_tag);
            (resolved_data, address_tag)
        } else if self.ud_resolver.get_tlds().iter().any(|tld| domain.ends_with(tld)) {
            let (resolved_data, address_tag) = self.ud_resolver.resolve(&domain).await
                .map_err(|e| anyhow!("Failed to resolve Unstoppable Domain: {}", e))?;
            debug!("Unstoppable domain host {} resolved into: {} with tag {}", domain, resolved_data, address_tag);
            (resolved_data, address_tag)
        } else {
            (ResolvedDomainData::DomainString(domain.to_owned()), AddressTag::NonWeb3)
        };
        if let Some(cache) = &self.dns_cache {
            // do not cache onchain content
            if address_tag != AddressTag::Onchain && address_tag != AddressTag::OnchainContract {
                cache.insert(domain.clone(), (resolved_data.clone(), address_tag.clone()));
            }
        };
        Ok((resolved_data, address_tag))
    }
}
