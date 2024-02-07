

fn main() {
    let mut v1: Vec<i32> = Vec::new();

    let mut v2 = vec![1, 2, 3, 4, 5];

    v1.push(5);
    v1.push(6);
    v1.push(7);
    v1.push(8);

    for (mut i, item) in v1.iter_mut().enumerate() {
        i += 1;
        *item += 10;
        println!("v1{i}: {item}");
    }

    println!("{}", v1[0]);

    let _v1 = v1; // Discard mutable

    let third: i32 = v2[2];
    println!("The third element in v2 is {third}");

    let third: Option<&i32> = v2.get(3);
    match third {
        Some(third) => println!("The third element in v2 is {third}"),
        None => println!("There is no third element."),
    }

    let first = v2[0];

    v2.push(6);

    println!("The first element is: {first}");

    let mut v3 = vec![100, 32, 57];
    for i in &mut v3 {
        *i += 50;
        println!("{i}");
    }

    // To hold different variable types, use an enum
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    {
        let v = vec![1, 2, 3, 4];

        // do stuff with v
    } // <- v goes out of scope and is freed here
}
