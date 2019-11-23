use std::collections::HashMap;

fn main() {
    let integers = vec![96, 49, 82, 100, 4, 4, 63, 71, 15, 40, 57, 71, 4];
    let mut sorted_integers = integers.to_vec();
    sorted_integers.sort();
    println!("mean of the list is {}", mean(&sorted_integers));
    println!("median of the list is {}", median(&sorted_integers));
    println!("mode of the list is {}", mode(&sorted_integers));
}

fn mean(integers:&Vec<i32>) -> f64 {
    let mut sum = 0;
    for i in integers {
        println!("{}", i);
        sum += i;
    }
    return sum as f64 / (integers.len() as f64);
}

fn median(integers:&Vec<i32>) -> i32 {
    let mid_index = integers.len() / 2;
    if let Some(mid) = integers.get(mid_index) {
        return *mid;
    } else {
        return 0;
    }
}

fn mode(integers:&Vec<i32>) -> i32 {
    let mut num_count = HashMap::new();
    for i in integers {
        let count = num_count.entry(i).or_insert(0);
        *count += 1;
    }

    let mut mode = 0;
    let mut max_count = 0;
    for (num, cnt) in num_count {
        if cnt > max_count {
            max_count = cnt;
            mode = *num;
        }
    }

    return mode;
}
