pub fn sum(nums: Vec<i32>) -> i32 {
    let mut s = 0;
    for n in nums {
        s += n;
    }
    s
}

pub fn fill(i: u32, n: usize) -> Vec<u32> {
    let mut v = vec![];
    for _ in 0..n {
        v.push(i);
    }
    v
}
