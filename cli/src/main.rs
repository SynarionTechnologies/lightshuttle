
use lightshuttle_cli::run;

fn main() {
    match run(users::get_current_uid()) {
        Ok(_) => println!("LightShuttle CLI is ready."),
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    }

#[cfg(unix)]
fn ensure_not_root() {
    if users::get_current_uid() == 0 {
        eprintln!("Refusing to run as root.");
        std::process::exit(1);
    }
}

#[cfg(not(unix))]
fn ensure_not_root() {}

fn main() {
    ensure_not_root();

    println!("LightShuttle CLI is ready.");
}
