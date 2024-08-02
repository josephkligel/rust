fn main() {
    let x = Some(5);
    let y = 10;
    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched y = {y}"),
        _ => println!("Default case, x = {x:?}"),
    }

    println!("at the end: x = {x:?}, y = {y}");

    let x = 1;

    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    let x = 5;

    match x {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }

    let x = 'c';

    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }

    struct Point {
        x: i32,
        y: i32,
    }

    let p = Point {x: 0, y: 7};

    let Point {x, y} = p;
    assert_eq!(0, x);
    assert_eq!(7, y);

    match p {
        Point {x, y: 0} => println!("On th x axis at {x}"),
        Point {x: 0, y} => println!("On the y axis at {y}"),
        Point {x, y} => {
            println!("On neither axis ({x}, {y})");
        }
    }

    enum Color {
        Rgb(i32, i32, i32),
        Hsv(i32, i32, i32),
    }

    enum Message {
        Quit,
        Move {x: i32, y: i32},
        Write(String),
        ChangeColor(Color),
    }

    let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));
    
    match msg {
        Message::ChangeColor(Color::Rgb(r,g ,b )) => {
            println!("Change color to red {r}, green {g}, and blue {b}");
        }
        Message::ChangeColor(Color::Hsv(h,s,v)) => {
            println!("Change color to hue {h}, saturation {s}, value {v}");
        }
        _ => (),
    }

    let ((feet, inches), Point {x, y}) = ((3, 10), Point {x: 3, y: -10});
}

