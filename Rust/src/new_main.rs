use std::thread;

const SIZE: usize = 20;
const THREAD_SIZE: usize = 4;

fn combine_array(arr: &mut [i32], first: usize, mid_val: usize, end: usize) {
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
        if start[i] <= last[j] {
            arr[k] = start[i];
            i += 1;
        } else {
            arr[k] = last[j];
            j += 1;
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

fn sorting_threading(arr: &mut [i32], first: usize, end: usize) {
    if first < end {
        let mid_val = first + (end - first) / 2;
        sorting_threading(arr, first, mid_val);
        sorting_threading(arr, mid_val + 1, end);
        combine_array(arr, first, mid_val, end);
    }
}

fn main() {
    let arr = [67, 23, 89, 12, 45, 34, 76, 91, 2, 66, 50, 78, 33, 67, 99, 55, 88, 3, 21, 76];

    let mut threads = vec![];

    for _ in 0..THREAD_SIZE {
        let mut arr_clone = arr.clone();
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
    .fold(Vec::new(), |mut acc, v| {
        acc.extend_from_slice(&v);
        acc
    });

    combine_array(&mut result, 0, (SIZE / 2 - 1) / 2, SIZE / 2 - 1);
    combine_array(&mut result, SIZE / 2, SIZE / 2 + (SIZE - 1 - SIZE / 2) / 2, SIZE - 1);
    combine_array(&mut result, 0, (SIZE - 1) / 2, SIZE - 1);

    println!("Merge Sort using Multi-threading: {:?}", result);
}
