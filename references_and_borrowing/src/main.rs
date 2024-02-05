fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

    let mut s2 = s1.clone();

    change(&mut s2);

    println!("{}", s2);

    let mut s3 = s1.clone();

    let r1 = &s3;
    
    let r2 = &s3;

    println!("{}, {}", r1, r2);
     
    let r3 = &mut s3;

    println!("{}", r3);
}

fn calculate_length(s: &String) -> usize{
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

//fn dangle() -> &String {
//    let s = String::from("hello");

//    &s
//}

fn no_dangle() -> String {
    let s = String::from("hello");

    s
}
