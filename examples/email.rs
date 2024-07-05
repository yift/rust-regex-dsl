//! This example shows how to capture email addresses from the std in.
//! Run using: echo 'this@gfre.com  not fdsgf@other.com' | cargo run --example email
extern crate rust_regex_dsl;

use rust_regex_dsl::create_capture;
use std::io::BufRead;

create_capture!(Email, concat {
    group {
        name: user_name,
        repeat {
            any_of {
                word_character,
                '_',
                '-',
                '.'
            }
        }
    },
    '@',
    group {
        name: host,
        concat {
            repeat {
                concat {
                    repeat {
                        any_of {
                            word_character,
                            '_',
                            '-',
                        }
                    },
                    '.'
                }
            },
            times {
                at_least: 2,
                at_most: 5,
                any_of {
                    word_character,
                    '-'
                }
            }
        }
    }
});
fn main() {
    let stdin = std::io::stdin();
    for line in stdin.lock().lines() {
        for email in Email::catch_all(&line.unwrap()) {
            println!("Got email: {} - user: {}, host: {}", email.0, email.user_name().unwrap(), email.host().unwrap());
        }
        
    }
}