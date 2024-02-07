fn main() {
    let _s1 = String::new(); // Creating a new string

    // Turning string literal into string type
    let data = "initial contents"; // Method 1

    let _s2 = data.to_string(); // Method 2
    let _s2 = "initial contents".to_string();

    let _s2 = String::from("initial contents"); // Method 3
                                               
    // Add to string
    let mut s3 = String::from("foo");
    s3.push_str("bar");
    println!("{s3}");

    // Add a string slice to a string variable 
    // but keep the value of the string slice
    let mut s4 = String::from("foo");
    let s5 = "bar";
    s4.push_str(s5);
    println!("s5 is {s5}");

    // Push a single character
    let mut s6 = String::from("lo");
    s6.push('l');

    // Using the '+' operator
    let s7 = String::from("Hello, ");
    let s8 = String::from("world");
    let s9 = s7 + &s8; 
    // '+' uses the add method fn add(self, s: &str) -> String {
    
    // Adding together words and characters and using format!
    let s10 = String::from("tic");
    let s11 = String::from("tac");
    let s12 = String::from("toe");

    //let s13 = s10 + "-" + &s11 + "-" + &s3;
    let s14 = format!("{s10}-{s11}-{s12}");

    // Strings cannot use indexes in rust
    // Instead you have to slice strings
    let hello = "hello";
    let s15 = &hello[0..4];
    let s16 = &hello[0..1];

    // However, you should not slice one character 
    // because rust could panic with different languages
    // that are coded differently
    // The bytes method shows the bytes 
    // making up chars in different languages
    for b in "Зд".bytes() {
        println!("{b}");
    }

    for c in "Зд".chars() {
        println!("{c}");
    }
}
