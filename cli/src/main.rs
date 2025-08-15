fn main() {
    if users::get_current_uid() == 0 {
        eprintln!("Refusing to run as root.");
        std::process::exit(1);
    }

    println!("LightShuttle CLI is ready.");
}
