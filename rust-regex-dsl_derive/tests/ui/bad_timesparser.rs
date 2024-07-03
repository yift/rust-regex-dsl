pub use rust_regex_dsl::regex_dsl;

fn main() {
    let regex = regex_dsl! {
        times{ 
            exactly: 1.0,
            lazy,
            "test",
        },
    };
    println!("{}", regex);
    let regex = regex_dsl! {
        times{ 
            exactly: 0,
            lazy,
            "test",
        },
    };
    println!("{}", regex);
    let regex = regex_dsl! {
        times{ 
            exactly: -1,
            lazy,
            "test",
        },
    };
    println!("{}", regex);
    let regex = regex_dsl! {
        times{ 
            exactly: "1",
            lazy,
            "test",
        },
    };
    println!("{}", regex);
    let regex = regex_dsl! {
        times{ 
            exactly: "1",
            lazy2,
            "test",
        },
    };
    println!("{}", regex);
    let regex = regex_dsl! {
        times{ 
            at_least: 10,
            at_most: 4,
            lazy,
            "test",
        },
    };
    println!("{}", regex);
  
}