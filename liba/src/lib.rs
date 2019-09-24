pub fn whoami() {
    println!("I am liba");
}

#[no_mangle]
fn get_message() -> &'static str {
    "lib A"
}
