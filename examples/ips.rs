//! This example shows how to capture ips addresses from the standard in.
//! Run using: echo '123.5.1.2  not 0:0:0:0:0:0:A00:1' | cargo run --example ips
extern crate rust_regex_dsl;

use rust_regex_dsl::create_capture;
use std::io::BufRead;

create_capture!(Email, any {
    group {
        name: ipv4,
        regex("(?:25[0-5]|2[0-4][0-9]|1?[0-9][0-9]{1,2})(?:\\.(?:25[0-5]|2[0-4][0-9]|1?[0-9]{1,2})){3}")
    }, group {
        name: ipv6,
        regex("(?:(?:(?:[0-9a-fA-F]){1,4})\\:){7}(?:[0-9a-fA-F]){1,4}")
    }
});
fn main() {
    let stdin = std::io::stdin();
    for line in stdin.lock().lines() {
        for ip in Email::catch_all(&line.unwrap()) {
            if let Some(ip) = ip.ipv_4() {
                println!("Got IPv4: {}", ip);
            }
            if let Some(ip) = ip.ipv_6() {
                println!("Got IPv6: {}", ip);
            }
        }
        
    }
}