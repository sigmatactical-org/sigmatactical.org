#![forbid(unsafe_code)]

fn main() -> std::io::Result<()> {
    let addr = sigma_theme::warp::listen_addr_from_env();
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()?
        .block_on(sigma_theme::warp::serve(
            "sigmatactical.org",
            addr,
            sigmatactical_org::routes(),
        ))
}
