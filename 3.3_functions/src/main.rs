fn main() {
    println!("Hello, world!");
    another_function(5, 'u');

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");

    let x = five();
    println!("The value of x is: {x}");

    let x = plus_one(5);
    println!("The value of x is: {x}");
}

fn another_function(x :i32, label :char) {
  println!("{x} {label}");
}

fn five() -> i32 {
    5
}

// ; - statement (not expression)
fn plus_one(x: i32) -> i32 {
  x + 1;
}