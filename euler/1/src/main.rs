fn main() {
  println!("{}", multiples(1000))
}

fn multiples(n: u32) -> u32 {
  (1..n).filter(|n| (n%3 == 0) || (n%5 == 0) ).fold(0, |sum, n| sum + n)
}

#[test]
fn it_calculates_the_example() {
  assert_eq!(multiples(10), 23);
}
