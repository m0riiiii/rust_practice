extern crate phrases as sayings;

use sayings::english::{greetings, farewells};
use sayings::japanese;

fn main() {
    println!("Hello in English: {}", greetings::hello());
    println!("GoodBye in English: {}", farewells::goodbye());
    println!("Hello in Japanese: {}", japanese::hello());
    println!("GoodBye in Japanese: {}", japanese::goodbye());
}
