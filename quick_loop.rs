fn main() {
  let mut number = 1u128;
  for _ in 1..125 {
    println!("{}", number);
    number *= 2;
  }
}