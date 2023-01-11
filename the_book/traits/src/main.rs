use traits::{notify, notify_long_form, NewsArticle, Summary, Tweet};

fn main() {
    let article1 = NewsArticle {
        headline: String::from("FAA Shuts Down"),
        location: String::from("Washington, DC"),
        author: String::from("Joe Blow"),
        content: String::from("How now brown cow?"),
    };

    let tweet1 = Tweet {
        username: String::from("kendopunk"),
        content: String::from("The quick brown fox jumped over the lazy dog."),
        reply: false,
        retweet: true,
    };

    println!("{}", article1.summarize());
    notify(&article1);
    println!("{}", tweet1.summarize());
    notify(&tweet1);
    notify_long_form(&tweet1);

    let s = 3i32.to_string();
    println!("{:?}", s);

    let s = "fred".to_string();
    println!("longest is {}", longest(s.as_str(), "marge"));
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
