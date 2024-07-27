
pub fn pond_sizes(land: Vec<Vec<i32>>) -> Vec<i32> {
    let mut res = vec![];
    let mut land = land;
    for i in 0..land.len() {
        for j in 0..land[0].len() {
            if land[i][j] == 0 {
                let size = dfs(&mut land, i, j);
                res.push(size);
            }
        }
    }
    res.sort();
    res
}

/**
 * @description 横向,纵向和斜向的遍历
 * @author Dylan
 * @throws
 * @time 2024/5/19 11:19
 */

pub fn dfs(land: &mut Vec<Vec<i32>>, i: usize, j: usize) -> i32 {
    if i < 0 || j < 0 || i >= land.len() || j >= land[0].len() || land[i][j] != 0 {
        return 0;
    }
    //标记已经遍历过的位置
    land[i][j] = -1;
    let mut size = 1;
    for dx in -1..=1 {
        for dy in -1..=1 {
            size += dfs(land, (i as i32 + dx) as usize, (j as i32 + dy) as usize);
        }
    }
    size
}
#[test]
fn test_pond_sizes() {
    let land = vec![
        vec![0, 2, 1, 0],
        vec![0, 1, 0, 1],
        vec![1, 1, 0, 1],
        vec![0, 1, 0, 1],
    ];
    let res = pond_sizes(land);
    println!("{:?}", res);
}