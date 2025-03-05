// println!("{}", colored_cells(69675));
fn colored_cells(n: i32) -> i64 {
    if n == 1 {
        return 1
    }
    let current: i64 = n.into();
    let prev: i64 = (n-1).into();
    let sum = (current * current) + (prev * prev);
    return sum;
}
