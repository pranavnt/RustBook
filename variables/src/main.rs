// variables and mutability
fn main() {
  // immutable variables by default
  // takes advantage of concurrency and safety offered by rust
    let x = 5;
    println("x is equal to {}",x);
    x = 6;
    println("x is equal to {}",x);
}
