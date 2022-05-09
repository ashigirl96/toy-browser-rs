#![feature(box_patterns)]
#![feature(box_syntax)]
use lib::Browser;

mod lib;

fn main() {
    Browser::new(String::from("https://example.com")).run()
}
