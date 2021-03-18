// variables and mutability
fn main() {
  // immutable variables by default
  // takes advantage of concurrency and safety offered by rust
    let mut x = 5;
    println!("x is equal to {}",x);
    x = 6;
    println!("x is equal to {}",x);

    //constants
    // convention is to have uppercase with underscores
    // rather than commas, you can use underscores for readability
    const MAX_POINTS: u32 = 100_000;

    // shadowing
    let x = 5;
    let x = x + 1;
    let x = x*2;
    println!("x is equal to {}",x);
}
