#![feature(const_for)]

// turn off all warnings
#![allow(dead_code)]
#![allow(unused_imports)]

mod constants;
mod http;
mod jk;
mod param;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let shared_lib = &args[1];

    http::start_server(shared_lib);
    
    jk::play(shared_lib);
}
