use handling_duplication::{self, NewsArticle, Summary, Tweet, notify};
mod generic_types;
use generic_types::{Point, get_largest};

fn main() {
    //Generic types

    // Example of a generic struct to find largest element
    // in a list of numbers or characters
    let number_list = vec![34, 50, 25, 100, 65];

    let largest = get_largest(number_list);
    println!("The largest number is {largest}");

    let char_list = vec!['y', 'm', 'a', 'q'];

    let largest = get_largest(char_list);
    println!("The largest char is {largest}");

    // Example of a generic struct Point
    // with different types for x and y
    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

    //Traits and trait bounds
    // Example of a trait Summary with implementations for NewsArticle and Tweet
    let tweet = Tweet {
        username: String::from("@user123"),
        content: String::from("This is a tweet!"),
        reply: false,
        repost: false,
    };

    let article = NewsArticle {
        author: String::from("Author Name"),
        headline: String::from("This is the Headline..!!"),
        content: String::from("This is the content of the news article."),
    };

    println!("Tweet summary: {}", tweet.summarize());
    println!("Article summary: {}", article.summarize());

    notify(&article);
}
