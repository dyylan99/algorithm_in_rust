
pub fn one_edit_away(first : String,second:String)->bool{
    let len1=first.len();
    let len2=second.len();
    if (len1 as i32-len2 as i32).abs()>1 {
        return false;
    }
    let mut count=0;
    let mut i=0;
    let mut j=0;
    while i<len1&&j<len2 {
        if first.chars().nth(i)!=second.chars().nth(j) {
            count+=1;
            if len1>len2 {
                i+=1;
            }else if len1<len2 {
                j+=1;
            }else {
                i+=1;
                j+=1;
            }
        }else {
            i+=1;
            j+=1;
        }
    }
    count<=1
}