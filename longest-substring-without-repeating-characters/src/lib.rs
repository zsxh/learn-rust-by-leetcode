struct Solution;

/////////////////////////////////////////////////////////////////
// Runtime: 4 ms, faster than 88.93% of Rust submissions.      //
// Memory Usage: 2.6 MB, less than 81.25% of Rust submissions. //
/////////////////////////////////////////////////////////////////

use std::collections::HashMap;

impl Solution {
  pub fn length_of_longest_substring(s: String) -> i32 {
    let mut left_pos = 0;
    let mut max_len = 0;
    let mut hash = HashMap::new();

    for (i, c) in s.chars().enumerate() {
      if hash.contains_key(&c) && hash[&c] >= left_pos {
        left_pos = hash[&c] + 1;
      } else {
        if i - left_pos + 1 > max_len {
          max_len = i - left_pos + 1
        }
      }
      hash.insert(c, i);
    }
    max_len as i32
  }
}


#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn it_works() {
    assert_eq!(3, Solution::length_of_longest_substring(String::from("abcabcbb")));
  }
}
