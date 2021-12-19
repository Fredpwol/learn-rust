pub fn map_arr(arr: [&str; 10]) -> [&str; 10]{
    let res : [&str; 10] = ["111";10];
    return res;
}

pub fn is_prime(num: i32) -> bool {
    let mut val = 2;
    while val < num{
        if num % val == 0{
            return false;
        }
        val+=1;
    }
    return true;
}