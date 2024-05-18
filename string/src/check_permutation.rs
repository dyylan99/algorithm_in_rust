pub fn check_permutation(s1: String, s2: String) -> bool {
    if s1.len() != s2.len() {
        return false;
    }
    let mut arr1 = [0; 26];
    let mut arr2 = [0; 26];
    for c in s1.chars() {
        arr1[c as usize - 'a' as usize] += 1;
    }
    for c in s2.chars() {
        arr2[c as usize - 'a' as usize] += 1;
    }
    for i in 0..26 {
        if arr1[i] != arr2[i] {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_permutation() {
        println!(
            "{}",
            check_permutation("abc".to_string(), "bca".to_string())
        );
        assert_eq!(check_permutation("aa".to_string(), "bb".to_string()), false);
    }
}
