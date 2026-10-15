#[cfg(target_os = "android")]
fn main() {}

#[cfg(target_arch = "wasm32")]
fn main() {}

#[cfg(target_os = "ios")]
fn main() -> anyhow::Result<()> {
    moss_ember::run_mobile()
}

#[cfg(not(any(target_arch = "wasm32", target_os = "ios", target_os = "android")))]
fn main() -> anyhow::Result<()> {
    // When running as a static-site build (fission site check / build / package / serve),
    // the CLI passes a subcommand as argv[1]. Dispatch to the site builder in that case;
    // otherwise launch the desktop app as usual.
    let args: Vec<String> = std::env::args().collect();
    let is_site_cmd = args.get(1).map(|a| {
        matches!(
            a.as_str(),
            "check" | "build" | "serve" | "package" | "routes"
        )
    }).unwrap_or(false);

    if is_site_cmd {
        fission::site::build_from_cli(moss_ember::site::site_app())
            .map_err(|e| anyhow::anyhow!("{e}"))
    } else {
        moss_ember::run_desktop()
    }
}
