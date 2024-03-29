use std::collections::HashMap;

fn two_sum(arr: &[i32], target: i32) -> Vec<i32> {
    let mut cache_hash: HashMap<i32, i32> = HashMap::new();
    let mut return_vec: Vec<i32> = Vec::new();

    // x = target - current
    // current + x = target
    for (i, current) in arr.iter().enumerate() {
        let x = target - current;
        if cache_hash.contains_key(&x) && current + x == target {
            let other_index = cache_hash.get(&x).unwrap();
            return_vec.push(i32::try_from(i).unwrap());
            return_vec.push(*other_index);
            return return_vec;
        }
        cache_hash.insert(*current, i32::try_from(i).unwrap());
    }

    return_vec
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {

    use super::two_sum;

    #[test]
    fn test_four_item_list() {
        let list = &[2, 7, 11, 15];

        let target = 9;

        let result = two_sum(list, target);

        assert!(result == [0, 1].to_vec() || result == [1, 0].to_vec());
    }

    #[test]
    fn test_three_item_list() {
        let list = &[3, 2, 4];

        let target = 6;

        let result = two_sum(list, target);

        assert!(result == [1, 2].to_vec() || result == [2, 1].to_vec());
    }

    #[test]
    fn test_two_item_list() {
        let list = &[3, 3];

        let target = 6;

        let result = two_sum(list, target);

        assert!(result == [0, 1].to_vec() || result == [1, 0].to_vec());
    }

    #[test]
    fn test_two_hundred_item_list() {
        let list = &[
            0, 2, -3, -4, -5, -6, -7, -8, -9, -10, -11, -12, -13, -14, -15, -16, -17, -18, -19,
            -20, -21, -22, -23, -24, -25, -26, -27, -28, -29, -30, -31, -32, -33, -34, -35, -36,
            -37, -38, -39, -40, -41, -42, -43, -44, -45, -46, -47, -48, -49, -50, -51, -52, -53,
            -54, -55, -56, -57, -58, -59, -60, -61, -62, -63, -64, -65, -66, -67, -68, -69, -70,
            -71, -72, -73, -74, -75, -76, -77, -78, -79, -80, -81, -82, -83, -84, -85, -86, -87,
            -88, -89, -90, -91, -92, -93, -94, -95, -96, -97, -98, -99, -100, -101, -102, -103,
            -104, -105, -106, -107, -108, -109, -110, -111, -112, -113, -114, -115, -116, -117,
            -118, -119, -120, -121, -122, -123, -124, -125, -126, -127, -128, -129, -130, -131,
            -132, -133, -134, -135, -136, -137, -138, -139, -140, -141, -142, -143, -144, -145,
            -146, -147, -148, -149, -150, -151, -152, -153, -154, -155, -156, -157, -158, -159,
            -160, -161, -162, -163, -164, -165, -166, -167, -168, -169, -170, -171, -172, -173,
            -174, -175, -176, -177, -178, -179, -180, -181, -182, -183, -184, -185, -186, -187,
            -188, -189, -190, -191, -192, -193, -194, -195, -196, -197, -198, -199, -200,
        ];

        let target = -1;

        let result = two_sum(list, target);

        assert!(result == [1, 2].to_vec() || result == [2, 1].to_vec());
    }
}
