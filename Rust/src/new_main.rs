use std::thread;
use std::cmp::Ordering;

const SIZE: usize = 20;
const THREAD_SIZE: usize = 4;

fn combine_array(arr: &mut [&str], first: usize, mid_val: usize, end: usize) {
    let mut start = Vec::with_capacity(mid_val - first + 1);
    let mut last = Vec::with_capacity(end - mid_val);

    for i in first..=mid_val {
        start.push(arr[i]);
    }

    for i in mid_val + 1..=end {
        last.push(arr[i]);
    }

    let mut i = 0;
    let mut j = 0;
    let mut k = first;

    while i < start.len() && j < last.len() {
        match start[i].cmp(last[j]) {
            Ordering::Less => {
                arr[k] = start[i];
                i += 1;
            },
            _ => {
                arr[k] = last[j];
                j += 1;
            },
        }
        k += 1;
    }

    while i < start.len() {
        arr[k] = start[i];
        i += 1;
        k += 1;
    }

    while j < last.len() {
        arr[k] = last[j];
        j += 1;
        k += 1;
    }
}

fn sorting_threading(arr: &mut [&str], first: usize, end: usize) {
    if first < end {
        let mid_val = first + (end - first) / 2;
        sorting_threading(arr, first, mid_val);
        sorting_threading(arr, mid_val + 1, end);
        combine_array(arr, first, mid_val, end);
    }
}

fn main() {
    println!("Type a list of words separated by spaces:");

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    // let input_vec: Vec<&str> = input.trim().split_whitespace().collect();
    let input_vec: Vec<String> = input.trim().split_whitespace().map(String::from).collect();


    let mut threads = vec![];

    for _ in 0..THREAD_SIZE {
        let mut arr_clone = input_vec.clone();
        let first = arr_clone.len() / THREAD_SIZE * threads.len();
        let end = if threads.len() == THREAD_SIZE - 1 {
            arr_clone.len() - 1
        } else {
            first + arr_clone.len() / THREAD_SIZE - 1
        };

        threads.push(thread::spawn(move || {
            sorting_threading(&mut arr_clone, first, end);
            arr_clone
        }));
    }

    let mut result = threads
        .into_iter()
        .map(|t| t.join().unwrap())
        .fold(Vec::new(), |mut acc, mut v| {
            acc.extend_from_slice(&v);
            acc
        });

    combine_array(&mut result, 0, (SIZE / 2 - 1) / 2, SIZE / 2 - 1);
    combine_array(&mut result, SIZE / 2, SIZE / 2 + (SIZE - 1 - SIZE / 2) / 2, SIZE - 1);
    combine_array(&mut result, 0, (SIZE - 1) / 2, SIZE - 1);

    println!("Sorted words: {:?}", result);
}







// fn main() {
//     // let mut input_vec: Vec<&str> = Vec::new();
//     // let mut input_vec_cur: Vec<&str> = Vec::new();
//     // loop {
//     //     print!("Type a word (empty string to finish): ");
//     //     let mut input = String::new();
//     //     std::io::stdin().read_line(&mut input).unwrap();
//     //     let input = input.trim();
//     //     if input.is_empty() {
//     //         break;
//     //     }
//     //     input_vec.push(input);
//     // }

//     let mut threads = vec![];

//     for _ in 0..THREAD_SIZE {
//         let mut arr_clone = input_vec.clone();
//         let first = arr_clone.len() / THREAD_SIZE * threads.len();
//         let end = if threads.len() == THREAD_SIZE - 1 {
//             arr_clone.len() - 1
//         } else {
//             first + arr_clone.len() / THREAD_SIZE - 1
//         };

//         threads.push(thread::spawn(move || {
//             sorting_threading(&mut arr_clone, first, end);
//             arr_clone
//         }));
//     }

//     let mut result = threads
//         .into_iter()
//         .map(|t| t.join().unwrap())
//         .fold(Vec::new(), |mut acc, mut v| {
//             acc.extend_from_slice(&v);
//             acc
//         });

//     combine_array(&mut result, 0, (SIZE / 2 - 1) / 2, SIZE / 2 - 1);
//     combine_array(&mut result, SIZE / 2, SIZE / 2 + (SIZE - 1 - SIZE / 2) / 2, SIZE - 1);
//     combine_array(&mut result, 0, (SIZE - 1) / 2, SIZE - 1);

//     println!("Merge Sort using Multi-threading: {:?}", result);
// }
