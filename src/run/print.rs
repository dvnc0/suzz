extern crate colorful;

use colorful::Color;
use crate::run::print::colorful::Colorful;

pub fn message(message: String) {
    println!("{}", message.color(Color::Green));
}

pub fn info(message: String) {
    println!("{}", message.color(Color::Blue));
}

pub fn _error(message: String) {
    println!("{}", message.color(Color::Red));
}

pub fn _warning(message: String) {
    println!("{}", message.color(Color::Yellow));
}