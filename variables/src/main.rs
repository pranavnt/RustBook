// variables and mutability
fn main() {
    // immutable variables by default
    // takes advantage of concurrency and safety offered by rust
    let mut x = 5;
    println!("x is equal to {}", x);
    x = 6;
    println!("x is equal to {}", x);

    //constants
    // convention is to have uppercase with underscores
    // rather than commas, you can use underscores for readability
    const MAX_POINTS: u32 = 100_000;

    // shadowing
    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("x is equal to {}", x);

    // this will not work if mutable
    // let mut spaces...
    // you cant change a mutable variables type
    let spaces = "   ";
    let spaces = spaces.len();
    println!("spaces is equal to {}", spaces);

    // integer types
    // 8-bit	i8	u8
    // 16-bit	i16	u16
    // 32-bit	i32	u32
    // 64-bit	i64	u64
    // 128-bit	i128	u128
    // arch	isize	usize

    // integer literals
    // Number literals	Example
    // Decimal	98_222
    // Hex	0xff
    // Octal	0o77
    // Binary	0b1111_0000
    // Byte (u8 only)	b'A'

    // floats
    let x = 2.0; // f64
    let y: f32 = 3.0; //f32

    let sum = 5 + 10; // addition
    let diff = 95.5 - 4.3; // subtraction
    let product = 4 * 30; // multiplication
    let quotient = 56.7 / 32.2; // division
    let remainder = 43 % 5; // mod

    // bools!

    let t = true;
    let f: bool = false; // w/ type annoation

    // chars!
    // specified by single quotes
    // represents more than ascii
    // 4 bytes
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';

    // tuples!
    // type annotations optional
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    // tuple destructuring
    let (x, y, z) = tup;

    println!("the value of x is {}", x);

    // access through index
    println!("the value of element with index 1 is {}", tup.1);

    // array type
    // only fixed number of elements
    // vector type is more versatile
    let a = [1, 2, 3, 4, 5];
    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    // with type annoations - type and length
    let a: [i32; 5] = [1, 2, 3, 4, 5];

    // access elements in array
    let first = a[0];
    let second = a[1];
    println!("The first element is {}", first);

    // let d = a[6];
    // this will not work
    // can compile but will throw runtime error
    another_function(first, second);

    // statements do not return values
    // this: let x = (let y = 6); doesnt work
    // function definitionss are also statements 

    let x = 5;

    let y = {
      let x = 3;
      x + 1
    };

    println!("The value of y is: {}",y);
}

// parameters require type annoations
fn another_function(x: i32, y: i32) {
    println!("THe value of x is {} and teh value of y is {}", x, y);
}
