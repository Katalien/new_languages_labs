use std::io::{self, Write};

fn merge_sort(arr: Vec<&str>) -> Vec<&str> {
    let len = arr.len();
    if len <= 1 {
        return arr;
    }
    let mid = len / 2;
    let left = merge_sort(arr[..mid].to_vec());
    let right = merge_sort(arr[mid..].to_vec());
    let mut result = merge(left, right);
    return result;
}

fn merge<'a>(left: Vec<&'a str>, right: Vec<&'a str>) -> Vec<&'a str> {
    let mut result = Vec::new();
    let mut left_idx = 0;
    let mut right_idx = 0;
    while left_idx < left.len() && right_idx < right.len() {
        if left[left_idx] < right[right_idx] {
            result.push(left[left_idx]);
            left_idx += 1;
        } else {
            result.push(right[right_idx]);
            right_idx += 1;
        }
    }
    while left_idx < left.len() {
        result.push(left[left_idx]);
        left_idx += 1;
    }
    while right_idx < right.len() {
        result.push(right[right_idx]);
        right_idx += 1;
    }
    return result;
}

fn main() {
    let mut input_vec: Vec<String> = Vec::new();
    loop {
        print!("Type a word in english or in russian (empty string to finish): ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        if input.is_empty() {
            break;
        }
        input_vec.push(input.to_string());
    }
    println!("Your words:");
    for word in &input_vec {
        print!("{} ", word);
    }
    let words_vec: Vec<&str> = input_vec.iter().map(std::ops::Deref::deref).collect();
    let sorted = merge_sort(words_vec);
    println!("\nSorted words:");
    for item in sorted {
        print!("{} ", item);
    }
    println!()
}