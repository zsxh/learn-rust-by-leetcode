struct Solution;

/////////////////////////////////////////////////////////////////
// Runtime: 4 ms, faster than 100.00% of Rust submissions.     //
// Memory Usage: 2.5 MB, less than 71.43% of Rust submissions. //
/////////////////////////////////////////////////////////////////

impl Solution {
  pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let mut n1_len = nums1.len();
    let mut n2_len = nums2.len();
    let mut n1_min = 0;
    let mut n1_max = if n1_len > 0 { n1_len - 1 } else {0}; // cause vec len type is usize
    let mut n2_min = 0;
    let mut n2_max = if n2_len > 0 { n2_len - 1 } else {0};
    while n1_len + n2_len > 2 {
      if n1_len > 0 && n2_len > 0 {
        if nums1[n1_min] < nums2[n2_min] {
          n1_min += 1;
          n1_len -= 1;
        } else {
          n2_min += 1;
          n2_len -= 1;
        }
      } else if n1_len > 0 {
        n1_min += 1;
        n1_len -= 1;
      } else {
        n2_min += 1;
        n2_len -= 1;
      }

      if n1_len > 0 && n2_len > 0 {
        if nums1[n1_max] > nums2[n2_max] {
          n1_max -= 1;
          n1_len -= 1;
        } else {
          n2_max -= 1;
          n2_len -= 1;
        }
      } else if n1_len > 0 {
        n1_max -= 1;
        n1_len -= 1;
      } else {
        n2_max -= 1;
        n2_len -= 1;
      }
    }

    let mut median :f64 = 0.0;
    if n1_len + n2_len == 2 {
      if n1_len == 2 {
        median = (nums1[n1_min] + nums1[n1_max]) as f64;
      } else if n2_len == 2 {
        median = (nums2[n2_min] + nums2[n2_max]) as f64;
      } else {
        median = (nums1[n1_min] + nums2[n2_min]) as f64;
      }
      median /= 2.0;
    } else {
      if n1_len > 0 {
        median = nums1[n1_min] as f64;
      } else {
        median = nums2[n2_min] as f64;
      }
    }
    median
  }
}


#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn it_works() {
    assert_eq!(2.0, Solution::find_median_sorted_arrays(vec![1,3], vec![2]));
    assert_eq!(2.5, Solution::find_median_sorted_arrays(vec![1,2], vec![3,4]));
    assert_eq!(1.0, Solution::find_median_sorted_arrays(vec![1], vec![]));
    assert_eq!(1.5, Solution::find_median_sorted_arrays(vec![], vec![1,2]));
  }
}
