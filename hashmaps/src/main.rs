use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    // get returns an Option<&V>
    // Calling copied gets an Option<i32>
    // unwrap_or sets the value to 0 if no entry for the key exists

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    // HashMaps and Ownership
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid 
    // because they are now owned by the map variable
    
    // Updating a value
    scores.insert(String::from("Blue"), 25);
    println!("{:#?}", scores);

    // Adding a key and value only if a key isn't present
    scores.entry(String::from("Yellow")).or_insert(100);
    scores.entry(String::from("Green")).or_insert(100);

    println!("{:#?}", scores);

    // Updating a value based on the old value
    let text = "hello world wonderful world";

    let mut map2 = HashMap::new();

    for word in text.split_whitespace() { 
        // split_whitespace seperates the text by tokens
        // delimeted by whitespaces
        let count = map2.entry(word).or_insert(0);
        // or_insert returns a mutable reference (&mut V) to value
        *count += 1;
    }

    println!("{:?}", map2);

}
