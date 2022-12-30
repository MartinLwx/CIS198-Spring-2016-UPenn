/// Find all prime numbers less than `n`.
/// For example, `sieve(7)` should return `[2, 3, 5]`
pub fn sieve(n: u32) -> Vec<u32> {
    let mut ans = Vec::new();
    let mut crossed = Vec::new();
    for x in 2..n {
        if !crossed.contains(&x) {
            ans.push(x);
            crossed.push(x);
            let mut scale = 2;
            while x * scale < n {
                crossed.push(x * scale);
                scale += 1;
            }
        }
    }
    ans
}