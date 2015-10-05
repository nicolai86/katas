// taken from http://rustbyexample.com/trait/iter.html
struct Fibonacci {
  curr: u32,
  next: u32,
}

impl Iterator for Fibonacci {
  type Item = u32;
  fn next(&mut self) -> Option<u32> {
    let new_next = self.curr + self.next;

    self.curr = self.next;
    self.next = new_next;

    Some(self.curr)
  }
}

fn fibonacci() -> Fibonacci {
  Fibonacci { curr: 1, next: 1 }
}

fn main() {
  let upper_limit = 4_000_000u32;
  let mut sum = 0u32;
  for i in fibonacci() {
    if i > upper_limit {
      break;
    }
    if i%2 == 0 {
      sum += i;
    }
  }

  println!("sum of even numbered numbers {}", sum);
}

#[test]
fn it_calculates_fibonacci() {
  let mut seq = fibonacci();
  assert_eq!(seq.next().unwrap(), 1);
  assert_eq!(seq.next().unwrap(), 2);
  assert_eq!(seq.next().unwrap(), 3);
  assert_eq!(seq.next().unwrap(), 5);
  assert_eq!(seq.next().unwrap(), 8);
  assert_eq!(seq.next().unwrap(), 13);
}
