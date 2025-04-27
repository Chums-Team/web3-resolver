use crate::models::{AddressTag, ResolvedDomainData};
use crate::resolver::ipfs::make_ipfs_link;
use crate::resolver::Resolver;
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use log::debug;
use reqwest::{Client, IntoUrl};
use std::sync::Arc;
use url::Url;

pub const UD_BASE_URL: &str = "https://api.unstoppabledomains.com";

pub struct UnstoppableDomainsResolver {
    tlds_url: Url,
    profile_url: Url,
    http_client: Arc<Client>,
    tlds: Vec<String>,
}

impl UnstoppableDomainsResolver {
    pub async fn new<U: IntoUrl>(base_url: U) -> Result<Self> {
        let base_url = base_url.into_url()?;
        let tlds_url = base_url.join("/resolve/supported_tlds")?;
        let profile_url = base_url.join("/profile/public/")?;
        let http_client = Arc::new(Client::new());
        let tlds = fetch_tlds(&http_client, tlds_url.clone()).await?;
        debug!("TLDs: {:?}", tlds);
        Ok(Self {
            tlds_url,
            profile_url,
            http_client,
            tlds,
        })
    }
    
    pub async fn default() -> Result<Self> {
        Self::new(UD_BASE_URL).await
    }

    pub fn get_tlds(&self) -> Vec<String> {
        self.tlds.clone()
    }
    
    pub async fn update_tlds(&mut self) -> Result<()> {
        let tlds = fetch_tlds(&self.http_client, self.tlds_url.clone()).await?;
        debug!("TLDs: {:?}", tlds);
        self.tlds = tlds;
        Ok(())
    }
}

#[async_trait]
impl Resolver for UnstoppableDomainsResolver {
    async fn resolve(&self, domain: &str) -> Result<(ResolvedDomainData, AddressTag)> {
        let url = self.profile_url.join(domain)?;
        let response = self.http_client.get(url).send().await?;
        let body = response.bytes().await?;
        let profile: serde_json::Value = serde_json::from_slice(&body)?;
        let ipfs_url = profile.get("records")
            .and_then(|p| p.get("ipfs.html.value"))
            .and_then(|h| h.as_str())
            .map(|cid| make_ipfs_link(cid));
        let web2_url = profile.get("profile")
            .and_then(|p| p.get("web2Url"))
            .and_then(|u| u.as_str())
            .map(|u| u.to_string());
        let result = web2_url
            .or(ipfs_url)
            .ok_or(anyhow!("Profile for domain {} does not contain IPFS hash or Web2Url", domain))?;
        Ok((ResolvedDomainData::DomainString(result), AddressTag::UnstoppableDomain))
    }
}

async fn fetch_tlds(http_client: &Client, tlds_url: Url) -> Result<Vec<String>> {
    let response = http_client.get(tlds_url).send().await?;
    let body = response.bytes().await?;
    let tlds_value: serde_json::Value = serde_json::from_slice(&body)?;
    let meta_objects = tlds_value.get("meta")
        .and_then(|m| m.as_object())
        .cloned()
        .unwrap_or_default();
    let tlds = meta_objects.iter()
        .filter_map(|(key, value)| {
            let naming_service = value.get("namingService").and_then(|ns| ns.as_str());
            return if naming_service.unwrap_or_default() != "DNS" {
                Some(format!(".{}", key))
            } else {
                None
            }
        })
        .collect();
    Ok(tlds)
}
