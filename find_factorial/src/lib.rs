pub fn factorial(num: u64) -> u64 {
    if num == 0 {
        return 1;
    }
    if num == 1 {
       return  1;
    }

    return num*factorial(num-1)
}
