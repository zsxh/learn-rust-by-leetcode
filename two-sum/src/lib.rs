struct Solution;

impl Solution {

  pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    use std::collections::HashMap;
    let mut wanted_seenIndex = HashMap::new();
    for (index, num) in nums.iter().enumerate() {
      if wanted_seenIndex.contains_key(num) {
        return vec![wanted_seenIndex[num] as i32, index as i32];
      }
      wanted_seenIndex.insert(target - num, index);
    }
    vec![]
  }
}


#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn it_works() {
    assert_eq!(vec![0,1], Solution::two_sum(vec![2,7,11,15], 9));
  }
}
