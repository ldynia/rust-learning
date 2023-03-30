#![feature(core_intrinsics)]

fn print_type<T>(_: &T) {
    println!("Type: {}", std::intrinsics::type_name::<T>());
}

fn main() {

    let mut x = 5;
    println!("x: {}", x);

    x = 6;
    println!("x: {}", x);

    const THREE_HOURS_IN_SECONDS :u8 = 255;
    println!("THREE_HOURS_IN_SECONDS: {}", THREE_HOURS_IN_SECONDS);

    let y : f32 = 2.2;
    {
      let y = y + 0.2;
      println!("y: {}", y);
    }
    println!("y: {}", y);

    // let mut spaces = "    ";
    // spaces = spaces.len();
    let spaces : u8 = 45;
    println!("spaces: {}", spaces);

    print_type(&spaces);
}