struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

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

    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}
