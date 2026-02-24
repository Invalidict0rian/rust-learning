use std::io;

const SIXTY: u32 = 12*5;

fn main() {
    let mut x: u32 = 5;
    println!("The Value of x is {}", x);

    x = 6;
    println!("The Value of x is {}", x);

    //this is shadowing the new let value overshadows the old one and since it is the new declaration it is immutable again
    let x = "meow";
    println!("The Value of x is {}", x);

    {
        let x = 3;
        println!("The Value of x is {} in this innner expression", x);
    }



    println!("My consts val is {}", SIXTY);

    //--------------------------------------------------

    //SCALAR TYPES
    //integers
    //floating-point numbers
    //Booleans
    //characters
    let x: i8 = 0b0100_0000;//0b for binary representation and _ is ignored in numbers so you can use it as a visual separator
    let x: u8 = 0xFF;// hex
    let x: u8 = b'A';// byte apparrently can only be used in u8 I don't really understand this yet but I would guess it is the value of the A char byte in ASCII/Unicode
    let x: i16 = 0o00043;// octal
    let x: u16 = 2_048;// decimal using _ separator
    let x: i32 = -120;
    let x: u32 = 255;
    let x: i64 = -200;
    let x: u64 = 2048;
    let x: i128 = -200;
    let x: u128 = 2048;
    // architecture dependant size thats pretty neat
    let x: isize = -200;
    let x: usize = 2048;

    //as a fun note integer overflow does not work in debug mode but does in release

    // two floating point types
    let x: f32 = 2.0001;
    let x: f64 = 2.000001;

    //booleans
    let x: bool = true;

    //chars these are utf-8 in general and that means multiple(4) bytes not your normal 1 byte for ascii
    let x: char = 'x';

    let heart_eyed_cat: char = '😻';

    //-------------------------------------------

    //COMPOUND TYPES

    //tuples
    //Tuples have a fixed length: Once declared, they cannot grow or shrink in size.
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup; // look unpacking/destructuring

    let w = tup.0; //kinda weird but you can use the . to index into a tuple

    println!("The value of y is: {y}");

    //arrays
    //arrays in Rust have a fixed length. and all elements must share a type
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    
    let a: [i32; 5] = [3; 5]; //this creates an array filled with 5 3s
    let first = a[0];// you index in using square brackets

    //------------------------------
    //indexing game with stdin
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");



}
