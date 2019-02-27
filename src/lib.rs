// for tail recursion support
#[macro_use]
extern crate tramp;
use tramp::{tramp, Rec};

pub fn nth(n: u32) -> u32 {
  if n == 0 {
    return 2;
  }
  find_prime(n)
}

pub fn find_prime(n: u32) -> u32 {
  const FIRST_PRIME: u32 = 3;
  const INIT_COUNT: i32 = 1;
  const INIT_NUMBER: u32 = 2;

  fn until_prime(next_number: u32, count: i32, until: u32, last_prime: u32) -> Rec<u32> {
    if count == until as i32 {
      rec_ret!(last_prime)
    }

    let (local_count, local_last_prime) = if is_prime(next_number) {
      (count + 1, next_number)
    } else {
      (count, last_prime)
    };

    rec_call!(until_prime(
      next_number + 1,
      local_count,
      until,
      local_last_prime
    ))
  }
  tramp(until_prime(INIT_NUMBER, INIT_COUNT, n, FIRST_PRIME))
}

pub fn is_prime(n: u32) -> bool {
  if n <= 1 {
    return false;
  }

  if n <= 3 {
    return true;
  }

  if n % 2 == 0 || n % 3 == 0 {
    return false;
  }

  fn is_prime_rec(n: u32, i: u32) -> Rec<bool> {
    if n % i == 0 || n % (i + 2) == 0 {
      rec_ret!(false);
    }

    if i * i <= n {
      rec_call!(is_prime_rec(n, i + 6));
    } else {
      rec_ret!(true);
    }
  }
  tramp(is_prime_rec(n, 5))
}
