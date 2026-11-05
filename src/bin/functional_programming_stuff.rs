fn iter_test() {
    // iterate over elements of a Collection

    let a: Vec<f64> = vec![1.0, 2.0, 3.0];
    let b: Vec<f64> = vec![4.0, 5.0, 6.0];

    println!("{:?}", a.iter());
    let mut it = a.iter();
    println!("{:?}", it.next());
    println!("{:?}", it.next());
    println!("{:?}", it.next());

    let test_vec = vec![1, 2, 3, 4];
    for i in test_vec.iter() {
        println!("{}", i);
    }
}

fn zip_test() {
    //combine two iterators
    let names = vec!["Alice", "Bob", "Charlie"];
    let ages = vec![20, 25, 30];

    for (name, age) in names.iter().zip(ages.iter()) {
        println!("{} is {} years old", name, age);
    }
}

fn map_test() {
    //transform each element into something else
    let numbers = vec![1, 2, 3, 4];
    println!("Before: {:?}", numbers);
    let doubled: Vec<i32> = numbers.iter().map(|x| x * 2).collect();

    println!("After: {:?}", doubled);
}

fn sum_test() {
    //sum elements of a collection
    let numbers = vec![1, 2, 3, 4, 5];
    println!("{:?}", numbers);
    let total: i32 = numbers.iter().sum();
    println!("{}", total);
}

fn dot_product(first_vec: &[f64], second_vec: &[f64]) -> f64 {
    first_vec
        .iter()
        .zip(second_vec.iter())
        .map(|(first_element, second_element)| first_element * second_element)
        .sum()
}

fn main() {
    //iter()
    iter_test();

    //zip
    zip_test();

    //map
    map_test();

    //sum
    sum_test();

    // Bringing it all together, dot product
    let a: Vec<f64> = vec![1.0, 2.0];
    let b: Vec<f64> = vec![3.0, 4.0];

    println!("{}", dot_product(&a, &b));
    print!("{:?}", a);
}
