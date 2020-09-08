use rand::Rng;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io;
use std::io::ErrorKind;

fn main() {
    //println!("Hello, world!");
    // println!("Guess the number!");

    // let secret_number = rand::thread_rng().gen_range(1, 101);
    // println!("The secret number is: {}", secret_number);

    // loop {
    //     println!("Please input your guess.");

    //     let mut guess = String::new();

    //     io::stdin()
    //         .read_line(&mut guess)
    //         .expect("Failed to read line");

    //     let guess: u32 = match guess.trim().parse() {
    //         Ok(num) => num,
    //         Err(_) => continue,
    //     };

    //     println!("You guessed: {}", guess);

    //     match guess.cmp(&secret_number) {
    //         Ordering::Less => println!("Too small!"),
    //         Ordering::Greater => println!("Too big!"),
    //         Ordering::Equal => {
    //             println!("You win!");
    //             break;
    //         }
    //     }
    // }

    // let mut x = 5;
    // println!("The value of x is: {}", x);
    // x = 6;
    // println!("The value of x is: {}", x);

    // let x = 5;

    // let x = x + 1;

    // let x = x * 2;

    // println!("The value of x is: {}", x);

    // let guess: u32 = "42".parse().expect("Not a number!");

    // let tup = (500, 6.4, 1);

    // let (x, y, z) = tup;

    // println!("The value of y is: {} {} {}", y,x,z);

    // let x: (i32, f64, u8) = (500, 6.4, 1);

    // let five_hundred = x.0;

    // let six_point_four = x.1;

    // let one = x.2;

    // println!(
    //     "The value of y is: {} {} {}",
    //     five_hundred, six_point_four, one
    // );

    // let a = [1, 2, 3, 4, 5];
    // let index = 10;

    // let element = a[a.len()];

    // println!("The value of element is: {}", element);

    // let x = plus_one(5);

    // println!("The value of x is: {}", x);

    // let a = [10, 20, 30, 40, 50];

    // for element in a.iter() {
    //     println!("the value is: {}", element);
    // }

    // for number in (1..3).rev() {
    //     println!("{}!", number);
    // }
    // println!("LIFTOFF!!!");

    // let mut s = String::from("hello");

    // s.push_str(", world!");
    // s.push_str("1");

    // println!("{}", s);

    // let s1 = String::from("hello");
    // let s2 = s1;

    // println!("{}, world!", s2);

    // let mut s1 = String::from("hello");
    // let r1 = &s1; // no problem
    // let r2 = &s1;

    // let r3 = &mut s1;
    // //println!("{},{}", r1,r2);
    // r3.push_str("ccac");
    // let len = calculate_length(&mut s1);

    // println!("The length of '{}' is {}.", s1, len);

    // let mut s = String::from("hello world");

    // let word = first_word(&s);

    //s.clear(); // error!

    // let my_string = String::from("hellowor ld");

    // // first_word works on slices of `String`s
    // let word = first_word(&my_string[..]);

    // let my_string_literal = "helloworld";

    // // first_word works on slices of string literals
    // let word = first_word(&my_string_literal[..]);

    // // Because string literals *are* string slices already,
    // // this works too, without the slice syntax!
    // let word = first_word(&my_string_literal[0..10]);

    // println!("the first word is: {}", word);

    // let a = [1, 2, 3, 4, 5];

    // let slice = &a[0..2];

    // println!("{:?}", slice);

    // let user1 = User {
    //     email: String::from("someone@example.com"),
    //     username: String::from("someusername123"),
    //     active: true,
    //     sign_in_count: 1,
    // };

    // let rect1 = (30, 50);

    // println!(
    //     "The area of the rectangle is {} square pixels.",
    //     area(rect1)
    // );

    // let rect1 = Rectangle {
    //     width: 30,
    //     height: 50,
    // };

    // println!("rect1 is {:?}", rect1);

    // let rect1 = Rectangle {
    //     width: 30,
    //     height: 50,
    // };

    // println!(
    //     "The area of the rectangle is {} square pixels.",
    //     rect1.area()
    // );

    // let rect1 = Rectangle {
    //     width: 30,
    //     height: 50,
    // };
    // let rect2 = Rectangle {
    //     width: 10,
    //     height: 40,
    // };
    // let rect3 = Rectangle {
    //     width: 60,
    //     height: 45,
    // };

    // println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    // println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    // let rect4 = Rectangle::square(3);
    // println!("{:?}", rect4)

    // let m = Message::Write(String::from("hello"));
    // m.call();

    // value_in_cents(Coin::Dime);

    // let v = vec![1, 2, 3, 4, 5];

    // let third: &i32 = &v[2];
    // println!("The third element is {}", third);

    // match v.get(2) {
    //     Some(third) => println!("The third element is {}", third),
    //     None => println!("There is no third element."),
    // }

    // for c in "超尼玛".chars() {
    //     println!("{}", c);
    // }

    // let str = String::from("abcef");
    // println!("string {}", &str[1..3]);
    // for a in str.bytes() {
    //     println!("byte: {}", a);
    // }

    // let mut scores = HashMap::new();

    // scores.insert(String::from("Blue"), 10);
    // scores.insert(String::from("Yellow"), 50);

    // let mut scores = HashMap::new();
    // scores.insert(String::from("Blue"), 10);

    // scores.entry(String::from("Yellow")).or_insert(50);
    // scores.entry(String::from("Blue")).or_insert(50);

    // println!("{:?}", scores);

    // let text = "hello hello world wonderful world";

    // let mut map = HashMap::new();

    // for word in text.split_whitespace() {
    //     let count = map.entry(word).or_insert(0);
    //     *count += 1;
    // }

    // println!("{:?}", map);

    // let f = File::open("hello.txt");

    // let f = match f {
    //     Ok(file) => file,
    //     Err(error) => panic!("Problem opening the file: {:?}", error),
    // };

    // let f = File::open("hello.txt");

    // let f = match f {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => panic!("Problem creating the file: {:?}", e),
    //         },
    //         other_error => {
    //             panic!("Problem opening the file: {:?}", other_error)
    //         }
    //     },
    // };

    // let number_list = vec![34, 50, 25, 100, 65];

    // let mut largest = number_list[0];

    // for number in number_list {
    //     if number > largest {
    //         largest = number;
    //     }
    // }

    // println!("The largest number is {}", largest);

    // let number_list = vec![34, 50, 25, 100, 65];

    // let result = largest_i32(&number_list);
    // println!("The largest number is {}", result);

    // let char_list = vec!['y', 'm', 'a', 'q'];

    // let result = largest_char(&char_list);
    // println!("The largest char is {}", result);

    // let p = Point { x: 5, y: 10 };

    // println!("p.x = {}", p.x());

    // let p1 = Point { x: 5, y: 10.4 };
    // let p2 = Point { x: "Hello", y: '啊' };

    // let p3 = p1.mixup(p2);

    // println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

    // let tweet = Tweet {
    //     username: String::from("horse_ebooks"),
    //     content: String::from("of course, as you probably already know, people"),
    //     reply: false,
    //     retweet: false,
    // };

    // println!("1 new tweet: {}", tweet.summarize());

    // {
    //     let x = 5;
    //     let r = &x;
    //     println!("r: {}", r);
    // }

    let string1 = String::from("long string is long");
    //let result;
    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());

        println!("The longest string is {}", result);
    }
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// fn longest_with_an_announcement<'a, T>(
//     x: &'a str,
//     y: &'a str,
//     ann: T,
// ) -> &'a str
// where
//     T: Display,
// {
//     println!("Announcement! {}", ann);
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }

pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("what the fuck you {}", self.author)
    }

    fn summarize(&self) -> String {
        format!(
            "{}, by {} ({})",
            self.headline,
            self.summarize_author(),
            self.location
        )
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("fuck you ggggggg {}", self.username)
    }

    fn summarize(&self) -> String {
        format!("{}: {}", self.summarize_author(), self.content)
    }
}

// fn largest_i32(list: &[i32]) -> i32 {
//     let mut largest = list[0];

//     for &item in list {
//         if item > largest {
//             largest = item;
//         }
//     }

//     largest
// }

// fn largest_char(list: &[char]) -> char {
//     let mut largest = list[0];

//     for &item in list {
//         if item > largest {
//             largest = item;
//         }
//     }

//     largest
// }

// struct Point<T, U> {
//     x: T,
//     y: U,
// }

// impl<T, U> Point<T, U> {
//     fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
//         Point {
//             x: self.x,
//             y: other.y,
//         }
//     }
// }

// fn calculate_length( s: &mut String) -> usize {
//     s.push_str("abc");
//     s.len() // len() returns the length of a String
//     //let mut s = String::from(s);
//     //s.push_str("abc");

// }

// fn plus_one(x: i32) -> i32 {
//     x + 1
// }

// fn first_word(s: &str) -> &str {
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate() {
//         println!("{},{}", &s[0..i], i);
//         if item == b' ' {
//             return &s[0..i];
//         }
//     }

//     &s[..]
// }

// struct User {
//     username: String,
//     email: String,
//     sign_in_count: u64,
//     active: bool,
// }

// fn area(dimensions: (u32, u32)) -> u32 {
//     dimensions.0 * dimensions.1
// }

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

impl Message {
    fn call(&self) {
        // method body would be defined here
        println!("{:#?}", self)
    }
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

fn read_username_from_file() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}
