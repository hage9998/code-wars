// Write a function that accepts an array of 10 integers (between 0 and 9), that returns a string of those numbers in the form of a phone number.
// create_phone_number(&[1,2,3,4,5,6,7,8,9,0]); // returns "(123) 456-7890"

fn create_phone_number(numbers: &[u8]) -> String {
    let string_con: String = numbers.into_iter().map(|i| i.to_string()).collect();
    let first = string_con[0..3].to_string();
    let second = string_con[3..6].to_string();
    let third = string_con[6..].to_string();
    
    format!("({}) {}-{}",first, second, third)
}

use std::str;

fn create_phone_number2(numbers: &[u8]) -> String {
    let s: String = numbers.into_iter().map(|i| i.to_string()).collect();
    
    format!("({}) {}-{}", &s[..3], &s[3..6], &s[6..])
}

fn create_phone_number3(x: &[u8]) -> String {
    format!("({}{}{}) {}{}{}-{}{}{}{}", x[0], x[1], x[2], x[3], x[4], x[5], x[6], x[7], x[8], x[9])
}