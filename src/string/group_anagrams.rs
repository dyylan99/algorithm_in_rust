use std::collections::HashMap;

pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut map = HashMap::new();
    for s in strs{
        let mut sorted_s=s.clone().into_bytes();
        sorted_s.sort_unstable();
        //如果有sorted_s,就返回sorted_s对应的value,没有就插入sorted_s和Vec::new()
        map.entry(sorted_s).or_insert(Vec::new()).push(s);
    }
    map.into_values().collect()
}