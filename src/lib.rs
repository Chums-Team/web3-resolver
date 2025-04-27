mod resolver;

pub mod models;
pub use resolver::Web3DomainResolver;
pub use resolver::Resolver;
pub use resolver::evername::EvernameResolver;
pub use resolver::ud::UnstoppableDomainsResolver;
pub use resolver::builder::DomainResolverBuilder;
