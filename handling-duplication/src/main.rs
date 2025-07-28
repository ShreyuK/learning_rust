fn get_largest<T: PartialOrd + Copy>(list: Vec<T>) -> T {
    let mut largest = list[0];

    for value in list {
        if value > largest {
            largest = value;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let largest = get_largest(number_list);
    println!("The largest number is {largest}");

    let char_list = vec!['y', 'm', 'a', 'q'];

    let largest = get_largest(char_list);
    println!("The largest char is {largest}");
}
