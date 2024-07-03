pub use rust_regex_dsl::regex_dsl;

fn main() {
    let regex = regex_dsl! {
        times,
    };
    println!("{}", regex);
    let regex = regex_dsl! {
        times(),
    };
    println!("{}", regex);
    let regex = regex_dsl! {
        times{ "test" },
    };
    let regex = regex_dsl! {
        times{ 
            exactly: 10,
            exactly: 12,
            "test" 
        },
    };
    println!("{}", regex);
    let regex = regex_dsl! {
        times{ 
            exactly: 10,
            at_least: 12,
            "test" 
        },
    };
    println!("{}", regex);
    let regex = regex_dsl! {
        times{ 
            exactly: 10,
            at_most: 12,
            "test" 
        },
    };
    println!("{}", regex);
    let regex = regex_dsl! {
        times{ 
            at_most: 10,
            at_most: 12,
            "test" 
        },
    };
    println!("{}", regex);
    let regex = regex_dsl! {
        times{ 
            at_least: 10,
            at_least: 12,
            "test" 
        },
    };
    println!("{}", regex);
    let regex = regex_dsl! {
        times{ 
            at_least: 10,
            at_most: 12,
            "test",
            "test"
        },
    };
    println!("{}", regex);
    let regex = regex_dsl! {
        times{ 
            exactly: 10,
            lazy,
            greedy,
            "test" 
        },
    };
    println!("{}", regex);
    let regex = regex_dsl! {
        times{ 
            exactly: 10,
            greedy,
            lazy,
            "test" 
        },
    };
    println!("{}", regex);
    let regex = regex_dsl! {
        times{ 
            exactly: 10,
            greedy,
            greedy,
            "test" 
        },
    };
    println!("{}", regex);
    let regex = regex_dsl! {
        times{ 
            exactly: 10,
            lazy,
            lazy,
            "test" 
        },
    };
    println!("{}", regex);
    let regex = regex_dsl! {
        times{ 
            exactly: 10,
            lazy,
        },
    };
    println!("{}", regex);
    let regex = regex_dsl! {
        times{ 
            at_most: 10,
            "test",
        },
    };
    println!("{}", regex);
}