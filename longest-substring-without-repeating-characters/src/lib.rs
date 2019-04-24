struct Solution;

/////////////////////////////////////////////////////////////////
// Runtime: 4 ms, faster than 88.93% of Rust submissions.      //
// Memory Usage: 2.6 MB, less than 81.25% of Rust submissions. //
/////////////////////////////////////////////////////////////////

use std::collections::HashMap;

impl Solution {
  pub fn length_of_longest_substring(s: String) -> i32 {
    let mut left_pos :i32 = -1;
    let mut max_left :i32 = -1;
    let mut max_right :i32 = -1;
    let mut hash = HashMap::new();
    for (i, c) in s.chars().enumerate() {
      let i = i as i32;
      if hash.contains_key(&c) && *hash.get(&c).unwrap() > left_pos {
        left_pos = *hash.get(&c).unwrap();
        hash.insert(c, i);
      } else {
        hash.insert(c, i);
        if i - left_pos > max_right - max_left {
          max_left = left_pos;
          max_right = i;
        }
      }
    }
    max_right - max_left
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
