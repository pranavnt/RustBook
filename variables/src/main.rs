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

    println!("The value of y is: {}", y);

    let x = plus_one(five());
    println!("The value of x is: {}", x);

    let num = 3;

    // if expressions
    // if condition is met, run block of code
    if num < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // if num will not work -- rust does not convert integers/any other type to booleans

    // not equal to
    if num != 0 {
        println!("number isnt 0");
    }

    // else ifs
    if num % 4 == 0 {
        println!("number is divisible by 4")
    } else if num % 3 == 0 {
        println!("number is divisible by 3")
    } else if num % 2 == 0 {
        println!("number is divisible by 2")
    } else {
        println!("number is not divisible by 2, 3, or 4")
    }

    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("number = {}", number);

    let mut counter = 0;

    let result = loop {
        if counter == 10 {
            break counter;
        } else {
            counter += 1;
        }
    };

    println!("result: {}", result);

    let mut number = 3;

    while number != 0 {
        number -= 1;
    }
    println!("number: {}", number);

    // going over an array with a hile loop
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);
        index += 1;
    }

    for el in a.iter() {
        println!("el: {}", el);
    }

    // (1..4) will create a range
    // that is a type in the standard library
    // rev method will reverse a range
    for number in (1..4).rev() {
        println!("{}", number);
    }
    println!("TAKEOFF!!!!!!");

    println!("40ºC is {}ºF", c_to_f(40));
    println!("41ºF is {}ºC", f_to_c(41));
}

// parameters require type annoations
fn another_function(x: i32, y: i32) {
    println!("THe value of x is {} and teh value of y is {}", x, y);
}

fn c_to_f(c: i32) -> f32 {
    c as f32 * 1.8 + 32.0
}

fn f_to_c(f: i32) -> f32 {
    (f as f32 - 32.0)/1.8
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    // if there is a semicolon it will be considered a statement, not an expression (expressions return values)
    x + 1
}
