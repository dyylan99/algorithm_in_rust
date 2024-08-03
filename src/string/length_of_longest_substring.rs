use std::collections::HashMap;


// 3. 无重复字符的最长子串
// 给定一个字符串 s ，请你找出其中不含有重复字符的 最长子串的长度。
pub fn length_of_longest_substring(s: String) -> i32 {
    let mut left=0;
    let mut right=0;
    let mut res=0;
    let mut hash_map=HashMap::new();
    let s=s.as_bytes();
    while right<s.len(){
        if let Some(&v)=hash_map.get(&s[right]){
            //更新左指针
            left =left.max(v+1)
        }
        res=res.max(right-left+1);
        //将当前遍历的右指针放入map中
        hash_map.insert(s[right], right);
        right+=1;
    } 
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