
// pub fn is_pal(x : i32) -> bool {
//     x.to_string() == x.to_string().chars().rev().collect::<String>()
// }

pub fn is_pal(x : i32) -> bool {
    if x < 0 {
        return false;
    } else {
        let (mut rev, mut num) = (0_i32, x);
        while num > 0 {
            rev = rev * 10 + num % 10;
            num /= 10;
        }
        return rev == x;
    }
}