pub mod compare_str;

use compare_str::{longest, ImportantExcerpt, longest_with_an_announcement};

fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);

    let string3 = String::from("long string is long");
    {
        let string4 = String::from("short string");
        let result = longest(string3.as_str(), string4.as_str());
        println!("The longest string is {}", result);
    }

    let string5 = String::from("example");
    let string6 = String::from("sample");

    let novel = String::from("Cale me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let _i = ImportantExcerpt {
        part: first_sentence,
    };

    let s: &'static str = "I have a static lifetime.";
    let ann: i32 = 125;

    let result = longest_with_an_announcement(string5.as_str(), 
                                              string6.as_str(),
                                              ann);
}
