pub fn nth(n: u32) -> Option<u32> {
  if n == 0 {
    return None;
  }
  let mut prime_count = 0;
  for number in 1..u32::max_value() {
    if check_if_prime(number) {
      prime_count += 1;
    }
    if prime_count == n {
      return Some(number);
    }
  }
  None
}

pub fn check_if_prime(n: u32) -> bool {
  if n <= 1 {
    return false;
  }
  if n <= 3 {
    return true;
  }
  if n % 2 == 0 || n % 3 == 0 {
    return false;
  }
  let mut i = 5;
  while i * i <= n {
    if n % i == 0 || n % (i + 2) == 0 {
      return false;
    }
    i += 6;
  }
  true
}
