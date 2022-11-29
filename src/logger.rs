pub fn init() {
    init_level();
    env_logger::builder()
        .target(env_logger::Target::Stdout)
        .init();
}

#[cfg(debug_assertions)]
fn init_level() {}

#[cfg(not(debug_assertions))]
fn init_level() {
    use std::env;

    env::set_var("RUST_LOG", "error,oneword=info");
}
