#![allow(dead_code, unused_variables)]

use std::fmt::{Debug, Display};

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
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }

    fn summarize_author(&self) -> String {
        format!("Not given.")
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
        format!("@{}", self.username)
    }
}


fn same_notify(item: &impl Summary) {
    println!("Breaking news: {}", item.summarize());
}

fn notify<T: Summary>(item: &T) {
    println!("Breaking news: {}", item.summarize());
}

fn test_notify_different_types_can_be_used(item1: &impl Summary, item2: &impl Summary) {
}

fn test_notify_only_same_type_can_be_used<T: Summary>(item1: &T, item2: &T) {
}


fn compare_prints_using_multiple_traits<T: Debug + Display>(t: &T) {
    println!("Debug: {:?}", t);
    println!("Display: {}", t);
}

fn compare_prints() {
    let s = "words";
    let arr = [1, 2, 3];
    let v = vec![5, 6, 7];

    compare_prints_using_multiple_traits(&s);
    // compare_prints_using_multiple_traits(&arr); // won't work since array doesn't implement Display, only Debug
    // compare_prints_using_multiple_traits(&v); // won't work since vector doesn't implement Display, only Debug
}


fn multiple_traits<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
    5
}

fn multiple_traits_elegently<T, U>(t: &T, u: &U) -> i32 
where 
    T: Display + Clone,
    U: Clone + Debug,
{
    5
}


fn return_trait() -> impl Summary {  // T: Summary doesn't work
    Tweet {
        username: String::from("Sona"),
        content: "Ghode ki shadi mein Gaddha diwana".to_string(),
        reply: false,
        retweet: false,
    }
}

// fn multiply_type_as_return_type_not_allowed(online: &bool) -> impl Summary {
//     if *online {
//         Tweet {
//             username: String::from("Sona"),
//             content: "Ghode ki shadi mein Gaddha diwana".to_string(),
//             reply: false,
//             retweet: false,
//         }
//     }
//     else {
//         NewsArticle {
//             headline: String::from("People are no longer eating out."),
//             location: String::from("Singapore"),
//             author: String::from("hidden_talent"),
//             content: String::from("copied from edmw."),
//         }
//     }
// }


struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self {
            x,
            y,
        }
    }
}

impl<T: PartialOrd + Display> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        }
        else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

fn conditionally_trait_bound_implementations() {
    let coord = Pair::new(10, 24);
    let relative = Pair::new("Shyam", "Shyama");

    coord.cmp_display();
    relative.cmp_display();

    let tup1 = (1, "Stop");
    let tup2 = (2, "Go");

    let race = Pair::new(tup1, tup2);

    // race.cmp_display(); // cmp_display is not implemented for this type.
}

pub fn entry_point() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("Very detailed clickbait information."),
        reply: true,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());


    let article = NewsArticle {
        headline: String::from("People are no longer eating out."),
        location: String::from("Singapore"),
        author: String::from("hidden_talent"),
        content: String::from("copied from edmw."),
    };

    println!("Article is: {}", article.summarize());
    println!("Article summary author: {}", article.summarize_author());


    notify(&tweet);
    notify(&article);

    same_notify(&tweet);
    same_notify(&article);


    compare_prints();

    return_trait();

    conditionally_trait_bound_implementations();

}