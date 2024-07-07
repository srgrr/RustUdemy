fn main() {
  // Rust will figure out that this variable is an integer
  let some_integer = 4;
  // Although the language will still be capable of figuring this one out, too,
  // I explicitly specified that this is supposed to be an integer
  let another_integer: i32 = 4;
  // The two previous variables are IMMUTABLE, uncommenting this line will cause
  // the program to crash
  // some_integer = 7;
   
  // However, a mutable variable won't cause us any trouble
  let mut mutable_integer = 4;
  mutable_integer = 89;

  // Variables can also be declared as tuples following a python-like LHS-RHS matching
  let (first_integer, second_integer) = (1, 2);

  // We can also declare constants...
  // We MUST specify their type in the code
  const BIGASS_CONSTANT: i32 = 1237;

  // Let's add a print here so the compiler won't throw us ugly warnings
  // (or not too many at least)
  let total = some_integer + another_integer + mutable_integer + first_integer + second_integer + BIGASS_CONSTANT;

  println!("{}", total);
}
