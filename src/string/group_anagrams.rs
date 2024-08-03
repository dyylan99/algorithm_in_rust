use std::collections::{HashMap};

pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut hash_map=HashMap::new();
    for s in strs{
        let mut sorted_s=s.clone().into_bytes();
        sorted_s.sort_unstable();
        hash_map.entry(sorted_s).or_insert(Vec::new()).push(s);
    }
    hash_map.into_values().collect()
}

#[test]
fn test(){
    let strs = vec!["eat".to_string(), "tea".to_string(), "tan".to_string(), "ate".to_string(), "nat".to_string(), "bat".to_string()];
    let res = group_anagrams(strs);
    let mut res = res.iter().map(|x| x.clone().into_iter().collect::<Vec<String>>()).collect::<Vec<Vec<String>>>();
    let mut expect = vec![vec!["bat".to_string()], vec!["nat".to_string(), "tan".to_string()], vec!["ate".to_string(), "eat".to_string(), "tea".to_string()]];
    assert_eq!(res, expect);
}