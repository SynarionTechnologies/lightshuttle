use lightshuttle_cli::run;

fn main() {
    match run(users::get_current_uid()) {
        Ok(_) => println!("LightShuttle CLI is ready."),
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    }
}
