#![allow(clippy::needless_pass_by_value)]

/// Naive implementation
#[must_use]
pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    let mut return_vec: Vec<i32> = Vec::new();

    for num_1 in &nums1 {
        for num_2 in &nums2 {
            if num_1 == num_2 {
                return_vec.push(*num_1);
            }
        }
    }

    // Ensure there are no duplicates in the array
    return_vec.sort_unstable();
    return_vec.dedup();
    return_vec
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intersection_1() {
        let nums1 = vec![1, 2, 2, 1];
        let nums2 = vec![2, 2];

        let result = intersection(nums1, nums2);

        assert_eq!(result, vec![2]);
    }

    #[test]
    fn test_intersection_2() {
        let nums1 = vec![4, 9, 5];
        let nums2 = vec![9, 4, 9, 8, 4];

        let result = intersection(nums1, nums2);

        assert!(result == vec![9, 4] || result == vec![4,9]);
    }
}
