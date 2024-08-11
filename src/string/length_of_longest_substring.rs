use std::collections::HashMap;


// 3. 无重复字符的最长子串
// 给定一个字符串 s ，请你找出其中不含有重复字符的 最长子串的长度。
pub fn length_of_longest_substring(s: String) -> i32 {
   let mut left=0;
   let mut right=0;
    let mut res=0;
    let mut map=HashMap::new();
    let s=s.as_bytes();
    while right<s.len(){
        if map.contains_key(&s[right]) {
            res=res.max((right-left) as i32);
            left=left.max(*map.get(&s[right]).unwrap()+1);
        }
        map.insert(s[right], right);
        right+=1;
    }
    res=res.max((right-left) as i32);
    res as i32
}
#[test]
fn test(){
    let s="abcabcbb".to_string();
    let res=length_of_longest_substring(s);
    assert_eq!(res, 3);
    let s="bbbbb".to_string();
    let res=length_of_longest_substring(s);
    assert_eq!(res, 1);
    let s="pwwkew".to_string();
    let res=length_of_longest_substring(s);
    assert_eq!(res, 3);
    let s="".to_string();
    let res=length_of_longest_substring(s);
    assert_eq!(res, 0);
}