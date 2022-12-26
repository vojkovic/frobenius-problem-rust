use std::time::Instant;

fn main() {
    const TOTAL_SIZE: usize = 100000;
    const NUMBER_1: usize = 121;
    const NUMBER_2: usize = 53;
    
    let mut loop_a: usize = 0;
    let mut loop_b: usize = 0;
    let mut loop_c: usize = 0;

    let mut a_list: Vec<usize> = Vec::new();
    let now = Instant::now();

    for i in 1..=TOTAL_SIZE {
        a_list.insert(i - 1, i)
    }

    loop {
        if a_list[loop_a] == loop_b*NUMBER_1+loop_c*NUMBER_2 {
            a_list.remove(loop_a);
        }
        loop_a = loop_a + 1;
        if loop_a >= a_list.len() {
            loop_a = 0;
            loop_b = loop_b + 1;
            if loop_b >= (TOTAL_SIZE / NUMBER_1) + 1 {
                loop_c = loop_c + 1;
                loop_b = 0;
                if loop_c >= (TOTAL_SIZE / NUMBER_2) + 1 {
                    println!("Completed! {:?}", a_list);
                    println!("Elapsed: {:.2?}", now.elapsed());
                    println!("Total sum is: {}", a_list.iter().fold(0u64, |sum, i| sum + (*i as u64)));
                    break
                }
            }
        }
    }
}
