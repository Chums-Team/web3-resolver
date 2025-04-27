use anyhow::{anyhow, Result};
use mini_moka::sync::Cache;
use crate::resolver::evername::{EvernameResolver, EVERSCALE_RPC_ENDPOINT};
use crate::resolver::ud::{UnstoppableDomainsResolver, UD_BASE_URL};
use crate::Web3DomainResolver;

pub struct DomainResolverBuilder {
    eversacale_endpoint: String,
    unstoppable_domain_base_url: String,
    use_cache: bool,
    cache_ttl_seconds: Option<u64>,
}

impl Default for DomainResolverBuilder {
    fn default() -> Self {
        DomainResolverBuilder {
            eversacale_endpoint: EVERSCALE_RPC_ENDPOINT.to_string(),
            unstoppable_domain_base_url: UD_BASE_URL.to_string(),
            use_cache: true,
            cache_ttl_seconds: Some(5 * 60),
        }
    }
}

impl DomainResolverBuilder {
    pub fn no_cache(self) -> Self {
        Self {
            use_cache: false,
            cache_ttl_seconds: None,
            ..self
        }
    }
    
    pub fn with_eversacale_endpoint(self, endpoint: &str) -> Self {
        Self {
            eversacale_endpoint: endpoint.to_string(),
            ..self
        }
    }
    
    pub fn with_unstoppable_domain_base_url(self, base_url: &str) -> Self {
        Self {
            unstoppable_domain_base_url: base_url.to_string(),
            ..self
        }
    }
    
    pub fn use_cache(self, use_cache: bool) -> Self {
        Self {
            use_cache,
            ..self
        }
    }
    
    pub fn cache_ttl_seconds(self, ttl: u64) -> Self {
        Self {
            cache_ttl_seconds: Some(ttl),
            ..self
        }
    }

    pub async fn build(&self) -> Result<Web3DomainResolver> {
        let ud_resolver = UnstoppableDomainsResolver::new(&self.unstoppable_domain_base_url).await?;
        let evername_resolver = EvernameResolver::new(&self.eversacale_endpoint)?;
        let dns_cache = match (self.use_cache, self.cache_ttl_seconds) {
            (true, Some(ttl)) if ttl > 0 => Some(Cache::builder()
                .time_to_live(std::time::Duration::from_secs(ttl))
                .build()),
            (true, ttl_val) => {
                return Err(anyhow!("Cache is on, but TTL is not set or invalid: {:?}", ttl_val));
            }
            (false, _) => None,
        };
        Ok(Web3DomainResolver::new(ud_resolver, evername_resolver, dns_cache))
    }
}