use std::time::Instant;
use std::collections::HashSet;

fn main() {
    const TOTAL_SIZE: usize = 100000;
    const NUMBER_1: usize = 343;
    const NUMBER_2: usize = 13;
    let mut a_set: HashSet<usize> = (1..=TOTAL_SIZE).collect();

    let now = Instant::now();
    for b in 0..=(TOTAL_SIZE / NUMBER_2) {
        for a in 0..=(TOTAL_SIZE / NUMBER_1) {
            let current_sum = a * NUMBER_1 + b * NUMBER_2;
            if current_sum > TOTAL_SIZE {
                break;
            }
            a_set.remove(&current_sum);
        }
    }
    
    match a_set.iter().max() {
        Some(val) => println!("The maximum value is: {}", val),
        None => println!("The set is empty."),
    }
    println!("Total sum is: {}", a_set.iter().sum::<usize>());
    println!("Elapsed: {:?}", now.elapsed());

}


