use std::io::{self, Write};
use std::thread;
use std::sync::{mpsc, Arc, Mutex};

const MAX_THREADS: usize = 4;

fn merge_mt(arr_left: Vec<String>, arr_right: Vec<String>) -> Vec<String> {
    let mut result = Vec::new();
    let mut left_ptr = 0;
    let mut right_ptr = 0;

    while left_ptr < arr_left.len() && right_ptr < arr_right.len() {
        if arr_left[left_ptr] < arr_right[right_ptr] {
            result.push(arr_left[left_ptr].clone());
            left_ptr += 1;
        } else {
            result.push(arr_right[right_ptr].clone());
            right_ptr += 1;
        }
    }

    while left_ptr < arr_left.len() {
        result.push(arr_left[left_ptr].clone());
        left_ptr += 1;
    }

    while right_ptr < arr_right.len() {
        result.push(arr_right[right_ptr].clone());
        right_ptr += 1;
    }

    result
}

fn mergesort_mt_helper(arr: Vec<String>, left: usize, right: usize, depth: usize) -> Vec<String> {
    if right - left > 1 {
        let mid = (left + right) / 2;
        let new_depth = depth + 1;

        let (mut arr_left, mut arr_right) = match arr.split_at(mid) {
            (l, r) => (l.to_vec(), r.to_vec())
        };
        let arr_left_len = arr_left.len();
        let arr_right_len = arr_right.len();

        if new_depth < MAX_THREADS {
            let (sender, receiver) = mpsc::channel();
            let left_ptr = Arc::new(Mutex::new(arr_left));
            let _ = thread::spawn(move || {
                let left_sorted = mergesort_mt_helper(left_ptr.lock().unwrap().clone(), 0, arr_left_len, new_depth);
                sender.send(left_sorted).unwrap();
            });
            arr_right = mergesort_mt_helper(arr_right, 0, arr_right_len, new_depth);
            arr_left = receiver.recv().unwrap();
        } else {
            arr_left = mergesort_mt_helper(arr_left, 0, arr_left_len, new_depth);
            arr_right = mergesort_mt_helper(arr_right, 0, arr_right_len, new_depth);
        }

        return merge_mt(arr_left, arr_right);
    }
    arr
}

pub fn mergesort_mt(arr: Vec<String>, left: usize, right: usize) -> Vec<String> {
    mergesort_mt_helper(arr, left, right, 0)
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
    let sorted = mergesort_mt(input_vec.clone(), 0, input_vec.len());
    println!("\nSorted words:");
    for item in sorted {
        print!("{} ", item);
    }
    println!()
}