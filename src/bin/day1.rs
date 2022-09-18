fn main() {
    println!("Day 1: Sonar Sweep");
    let data = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

    let count = count_increases(data);

    println!("{}", count);
}

fn count_increases(data: Vec<i32>) -> i32 {
    let mut count = 0;
    let mut prev: i32 = data[0];

    for num in data.iter().skip(1) {
        if *num > prev {
            count += 1;
        }
        prev = *num;
    }

    count
}
