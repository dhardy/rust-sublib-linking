pub fn whoami() {
    println!("I am libb");
}

#[no_mangle]
fn get_message() -> &'static str {
    "lib B"
}
