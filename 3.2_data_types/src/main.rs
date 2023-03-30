fn main() {
    let guess : u32 = "42".parse().expect("Not a number!");
    println!("{guess}");

    let num : f64 = 42.2;
    println!("{}", num);

    let num : u128 = 42;
    println!("{}", num);

    let b : bool = true;
    println!("{}", b);

    // Char
    let c = 'z';
    let z: char = 'ℤ';
    let heart_eyed_cat = '😻';

    // String
    let s = "aaa";
    let ss : &str = "vvvv";

    // Compound - Tuple
    let tup : (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("{} {} {}", x, y, z);
    println!("{} {} {}", tup.0, tup.1, tup.2);

    // Compound - Array
    let a = [1, 2, 3, 4, 5];
    println!("{} {}", a[0], a[1]);

    let b : [u8; 5] = [0, 1, 2, 3, 4];
    println!("{} {}", b[0], b[1]);
}
