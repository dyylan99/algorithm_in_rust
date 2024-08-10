
// 9. 回文数
pub fn is_palindrome(x: i32) -> bool {
    //不将数字转为字符串的解法
    if x < 0 || (x % 10 == 0 && x != 0) {
        return false;
    }
    let mut x = x;
    let mut reverted_number=0;
    while x>reverted_number {
        reverted_number = reverted_number*10 + x%10;
        x /= 10;
    }
    x == reverted_number || x == reverted_number/10
}