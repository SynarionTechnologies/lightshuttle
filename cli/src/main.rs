#[cfg(unix)]
fn ensure_not_root() {
    if let Err(msg) = lightshuttle_cli::run(users::get_current_uid()) {
        eprintln!("{msg}");
        std::process::exit(1);
    }
}

#[cfg(not(unix))]
fn ensure_not_root() {}

fn main() {
    ensure_not_root();
    println!("LightShuttle CLI is ready.");
}
