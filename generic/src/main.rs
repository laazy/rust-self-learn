use generic::{Tweet, NewsArticle, Summary, notify};

fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

#[derive(Debug)]
#[allow(dead_code)]
struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f32, f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

impl<X1, Y1> Point<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}


fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);

    let p = Point { x: 5, y: 4.0};
    println!("wont_work = ({}, {:})", p.x(), p.y);
    println!("wont_work = ({}, {:})", p.x(), p.y);
    let p: Point<f32, f32> = Point {x: 3.0, y: 4.0};
    println!("distance_from_origin = {}", p.distance_from_origin());
    let p2 = p.mixup(Point {x: "abc", y: 'd'});
    println!("p2 = {:?}", p2);

    let tweet = Tweet{
        username: String::from("tweet"),
        content: String::from("tweet"),
        reply: true,
        retweet: true,
    };
    println!("1 new tweet: {}", tweet.summarize());
    let article = NewsArticle{
        headline: String::from("article"),
        location: String::from("article"),
        author:  String::from("article"),
        content:  String::from("article"),
    };
    println!("1 new article: {}", article.summarize());
    notify(&tweet);

    println!("\nlife time:");

    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    println!("i = {}", i.part);
    let s: &'static str = "I have a static lifetime.";
    println!("s = {}", s);
}

struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}