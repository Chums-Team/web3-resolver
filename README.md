# web3-resolver

`web3-resolver` is a Rust library for resolving Web3 decentralized domains:
* [Evername](https://evername.io/) ([Everscale](https://everscale.network/) naming system, .ever-domains)
* [Unstoppable Domains](https://unstoppabledomains.com/)

## Supported targets (address tags)
* Evername domains:
  - Tor (query key = 1001)
  - IPFS (query key = 1002)
  - Web2-domain address (query key = 1003)
  - Onchain site (content stored directly in the domain NFT, size is *very* limited) (query key = 1004)
  - OnchainContract (content stored in the separate [eversite contract](https://github.com/Chums-Team/everscale-onchain-site-contract), size is limited) (query key = 1005)
* Unstoppable Domains
* Simple web2 domains when non-web3 address is provided (domain ending is not an .ever or Unstoppable Domains TLD, e.g. .com, .net, etc.)

Evername resolving precedence is according to the key order from 1001 to 1005.
So e.g., if you have a domain that has both a Tor and an IPFS record, the resolver will return the Tor address.

## Installation

Add the dependency to your `Cargo.toml`:

```toml
[dependencies]
web3-resolver = { git = "https://github.com/Chums-Team/web3-resolver" }
```

## Usage

### Basic Example

```rust
use web3_resolver::Web3DomainResolver;
use web3_resolver::models::{AddressTag, ResolvedDomainData};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Create domains resolver
    let domain_resolver = Web3DomainResolver::default();
    // Equivalent to:
    let domain_resolver = Web3DomainResolver::builder()
        .use_cache(true)
        .cache_ttl_seconds(5 * 30)
        .with_eversacale_endpoint("https://jrpc.everwallet.net/rpc")
        .with_unstoppable_domain_base_url("https://api.unstoppabledomains.com")
        .build().await?;

    // Resolve a domain
    let (resolved_data, address_tag) = domain_resolver.resolve("maksimzubov.ever").await?;
    
    println!("Resolved domain data: {}, with tag {}", resolved_data, address_tag);
    
    // Handle the resolved data based on the address tag
    match (address_tag, resolved_data) {
        // Handle Tor address
        (AddressTag::Tor, ResolvedDomainData::DomainString(tor_url)) => 
            println!("This is a Tor address: {}", tor_url),
        // Handle IPFS address
        (AddressTag::Ipfs, ResolvedDomainData::DomainString(ipfs_provider_url)) => 
            println!("This is an IPFS address: {}", ipfs_provider_url),
        // Handle web2 domain address
        (AddressTag::Web2, ResolvedDomainData::DomainString(web2_url)) => 
            println!("This is a web2 domain address: {}", web2_url),
        // Handle onchain site address
        (AddressTag::Onchain, ResolvedDomainData::OnchainData(html_content)) => 
            println!("This is an onchain site content: {}", html_content),
        // Handle onchain contract address
        (AddressTag::OnchainContract, ResolvedDomainData::OnchainContractData((contract_content, content_type))) => 
            println!("This is an onchain site contract with content-type: {}, content: {}", content_type, contract_content),
        // Handle simple non-web3 address
        (AddressTag::NonWeb3, ResolvedDomainData::DomainString(web2_url)) => 
            println!("This is a non-web3 address: {}", web2_url),
        // Handle Unstoppable Domain address
        (AddressTag::UnstoppableDomain, ResolvedDomainData::DomainString(content_url)) =>
            println!("This is an Unstoppable Domain web2 or ipfs address: {}", content_url),
        // Bad data
        _ => eprintln!("Error: bad address tag - domain data combination!"),
    }
    Ok(())
}
```
### Builder options

* `use_cache`: Enable or disable caching. Default is `true`.
* `cache_ttl_seconds`: Set the cache time-to-live in seconds. Default is `300` seconds (5 minutes).
* `with_everscale_endpoint`: Set the JRPC-Everscale endpoint URL. Default is `https://jrpc.everwallet.net/rpc`. Be careful! **GraphQL endpoint is not supported!**
* `with_unstoppable_domain_base_url`: Set the Unstoppable Domains base URL. Default is `https://api.unstoppabledomains.com`.

### Using dedicated resolvers
You can also use dedicated resolvers for specific services:

```rust
use web3_resolver::EvernameResolver;
use web3_resolver::UnstoppableDomainsResolver;
use web3_resolver::Resolver;
use web3_resolver::models::{AddressTag, ResolvedDomainData};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Create ever-domains resolver
    let evername_resolver = EvernameResolver::default()?;
    // Equivalent to:
    let evername_resolver = EvernameResolver::new("https://jrpc.everwallet.net/rpc")?;
    // Create unstoppable-domains resolver
    let ud_resolver = UnstoppableDomainsResolver::default().await?;
    // Equivalent to:
    let ud_resolver = UnstoppableDomainsResolver::new("https://api.unstoppabledomains.com").await?;

    // Resolve evername domain
    let (resolved_data, address_tag) = evername_resolver.resolve("tor-proxy.chums.ever").await?;
    println!("tor-proxy.chums.ever resolved to: {}, with tag {}", resolved_data, address_tag);
    // Handle the resolved data based on the address tag
    match (address_tag, resolved_data) {
        // Handle Tor address
        (AddressTag::Tor, ResolvedDomainData::DomainString(tor_url)) =>
            println!("This is a Tor address: {}", tor_url),
        // Bad data
        _ => eprintln!("Error: Unsupported data!"),
    }

    // Resolve unstoppable domain
    let (resolved_data, address_tag) = ud_resolver.resolve("kombutt.eth").await?;
    println!("kombutt.eth resolved to: {}, with tag {}", resolved_data, address_tag);
    // Handle the resolved data based on the address tag
    match (address_tag, resolved_data) {
        // Handle Tor address
        (AddressTag::UnstoppableDomain, ResolvedDomainData::DomainString(ipfs_url)) =>
            println!("This is a Ipfs address: {}", ipfs_url),
        // Bad data
        _ => eprintln!("Error: Unsupported data!"),
    }
    Ok(())
}
```

## Requirements

- Rust version 1.56 or higher
- Async runtime (e.g., tokio)

## Building
To build the project, use the following command:
```shell
cargo build --release
```

For cross-build you have to install `cross` tool. Cross tool requires container engine, i.e. `docker` to be installed.
```shell
cargo install cross
```
Check that you have appropriate toolchains installed (in the list of `installed targets`):
```shell
rustup show
```
If you don't have say `aarch64-linux-android` toolchain, install it:
```shell
rustup target add aarch64-linux-android
```
Then you can build for Android:
```shell
cross build --target aarch64-linux-android --lib --release
```

For building for iOS/ macOS, you need Apple Mac with Xcode command line tools installed. Then you can build for iOS:
```shell
# iOS
cross build --target aarch64-apple-ios --lib --release
# macOS
cross build --target aarch64-apple-darwin --lib --release
```

For building for Windows, you need to install `x86_64-pc-windows-gnu` toolchain:
```shell
rustup target add x86_64-pc-windows-gnu
cross build --target x86_64-pc-windows-gnu --lib --release
```

And for building for Linux, you need to install `x86_64-unknown-linux-gnu` toolchain:
```shell
rustup target add x86_64-unknown-linux-gnu
cross build --target x86_64-unknown-linux-gnu --lib --release
```

After building, you can find the compiled binary in the `target/{toolchain_name}/release` directory.

## License

This project is licensed under the Apache 2.0 License. See the [LICENSE](LICENSE) file for details.
