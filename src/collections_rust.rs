pub fn even_filter(arr: Vec<i32>) -> Vec<i32> {
    let mut res_arr: Vec<i32> = Vec::new();
    for i in arr {
        if i % 2 == 0 {
            res_arr.push(i);
        }
    }
    return res_arr;
}

pub fn odd_filter_using_refrence(arr: &Vec<i32>) -> Vec<i32> {
    let mut ans_array = Vec::new();
    for val in arr {
        if *val % 2 != 0 {
            ans_array.push(*val);
        }
    }
    ans_array
}
