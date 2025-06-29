
// pub fn two_sum(nums : Vec<i32>, target: i32) -> Vec<i32>{
//     // Brute Force
//     for (i, &a) in nums.iter().enumerate(){
//         for (j, &b) in nums.iter().enumerate(){
//             if i != j && a + b == target {
//                 return vec![i as i32, j as i32];
//             }
//         }
//     }
//     return vec![];
// }

use std::collections::HashMap;

pub fn two_sum(nums : Vec<i32>, target: i32) -> Vec<i32>{
    // Optimisted Solution
    let mut hm = HashMap::new();
    for (i, &val) in nums.iter().enumerate() {
        hm.insert(val, i as i32);
    }
    for (i, &val) in nums.iter().enumerate() {
        let look = target - val;
        if let Some(&j) = hm.get(&look) {
            let pos = j as usize;
            if i != pos {
                return vec![i as i32, j];
            }
        }
    }
    return vec![];
}