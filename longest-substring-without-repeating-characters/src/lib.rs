struct Solution;

/////////////////////////////////////////////////////////////////
// Runtime: 4 ms, faster than 89.04% of Rust submissions.      //
// Memory Usage: 2.5 MB, less than 93.75% of Rust submissions. //
/////////////////////////////////////////////////////////////////

use std::collections::HashMap;

impl Solution {
  pub fn length_of_longest_substring(s: String) -> i32 {
    let mut left_pos = 0;
    let mut max_len = 0;
    let mut hash = HashMap::new();

    for (i, c) in s.chars().enumerate() {
      if let Some(&last_index) = hash.get(&c) {
        if last_index >= left_pos {
          left_pos = last_index + 1;
        }
      };

      if i - left_pos + 1 > max_len {
        max_len = i - left_pos + 1;
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
