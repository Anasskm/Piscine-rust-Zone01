pub fn fibonacci2(n: u32) -> u32 {
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }
    let mut b_prev: u32 = 0;
    let mut prev: u32 = 1;
    let mut res: u32 = 0;
    for _ in 2..=n {
        res = b_prev + prev;
        b_prev = prev;
        prev = res;
    }
    res
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = fibonacci2(4);
        assert_eq!(result, 3);
    }
}
