use std::collections::HashMap;


/**
 * 给定一个字符串，编写一个函数判定其是否为某个回文串的排列之一。

回文串是指正反两个方向都一样的单词或短语。排列是指字母的重新排列。

回文串不一定是字典当中的单词。
 */

pub fn can_permute_palindrome(s:String)->bool{
    let mut map:HashMap<char,u8>=HashMap::new();
    for ele in s.chars() {
        //or_insert()方法返回一个可变引用，如果map中没有key对应的value，则插入key-value对，并返回value的可变引用
       *map.entry(ele).or_insert(0)+=1;
    }
    //出现次数为奇数的字符最多只能有一个
    map.iter().filter(|(k,v)| *v%2==1).count()<=1
}
