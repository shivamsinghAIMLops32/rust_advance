pub fn print_array_using_iterator(arr: &Vec<i32>) {
    for item in arr.iter() {
        println!("{}", item);
    }
}

// better way to do using .next on iter()

pub fn print_array_using_next(arr: &Vec<i32>) {
    let mut iterr = arr.iter();

    while let Some(item) = iterr.next() {
        println!("{}", item);
    }
}

// mutable reference to iterator

pub fn print_array_using_next_mut(arr: &mut Vec<i32>) {
    let mut iterr = arr.iter_mut();

    while let Some(item) = iterr.next() {
        println!("{}", item);
    }
}

// into_iter() - takes ownership of the collection and returns an iterator that yields owned values
// example is filter,map,fold etc

pub fn print_array_using_into_iter(arr: Vec<i32>) {
    for item in arr.into_iter() {
        println!("{}", item);
    }
    // print!("Array after into_iter: {:?}", arr);  This will cause a compile-time error because arr has been moved
}

// sum of array usnig iterator => consume the iterator and return a single value
pub fn sum_array_using_iterator(arr: Vec<i32>) -> i32 {
    arr.into_iter().sum()
    // print!("Sum of array: {:?}", arr); cant access arr here because it has been moved or consumed by into_iter()
}

// iterator adaptors - map, filter, fold, etc
pub fn square_array_using_map(arr: Vec<i32>) -> Vec<i32> {
    arr.into_iter().map(|x| x * x).collect()
}

pub fn filter_even_numbers(arr: Vec<i32>) -> Vec<i32> {
    arr.into_iter().filter(|x| x % 2 == 0).collect()
}

// logic to first filter out odd values and then square those values and create a new vector

pub fn filter_and_square(arr: Vec<i32>) -> Vec<i32> {
    let odd_values = arr.iter().filter(|x| **x % 2 == 0); // return an iterator of even values
    odd_values.map(|x| x * x).collect() // collect values and convert from iter to vector
}

// iterating over an hashmap
pub fn hm_iter(hm: &std::collections::hash_map::HashMap<String, i32>) {
    for (key, value) in hm.iter() {
        println!("Key: {}, Value: {}", key, value);
    }
}
