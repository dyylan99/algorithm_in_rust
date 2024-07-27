


pub fn is_unique(astr: String) -> bool {
    let mut mask = 0;
    //利用位运算，将字符串中的字符映射到一个32位的整数上
    for c in astr.chars() {
        let idx = c as u8 - 'a' as u8;
        if mask & (1 << idx) != 0 {
            return false;
        }
        mask |= 1 << idx;
    }
    true
}

//编写测试模块
#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_is_unique() {
        println!("{}", is_unique("leetcode".to_string()));
        println!("{}", is_unique("abc".to_string()));
    }
}
