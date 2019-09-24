// External libs get linked when any of:
// (1) extern crate is used
// (2) a symbol from that lib is referenced
//
// "multiple definition" errors do not reliably happen when two crates are
// linked via "extern crate" for unknown reasons.

extern crate liba;
extern crate libb;
// use liba::whoami;

#[allow(improper_ctypes)]
extern "Rust" {
    fn get_message() -> &'static str;
}

fn main() {
//     whoami();
//     libb::whoami();
    
    let msg = unsafe { get_message() };
    println!("Hello, {}", msg);
}
