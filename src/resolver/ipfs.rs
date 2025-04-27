pub fn make_ipfs_link(content_hash_string: &str) -> String {
    let content_hash_fixed = content_hash_string.trim_start_matches("ipfs://").trim_start_matches("/ipfs/");
    format!("https://{}.ipfs.w3s.link/", content_hash_fixed)
}