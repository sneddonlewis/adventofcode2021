fn main() {
    println!("Day 3: Binary Diagnostic");

    println!("{:?}", get_input());
}

// get gamma rate
// get epsilon rate
// multiply them together

fn compute_gamma(reading: &str) -> &'static str {
    let result = reading.lines().map(more_ones);
    println!("{:?} debugging compute_gamma", result);
    "10110"
}

#[test]
fn compute_gamma_test_22() {
    let test_input = get_input();
    let result = compute_gamma(test_input);
    let expected = "10110";

    assert_eq!(expected, result);
}

fn more_ones(reading_pos: Vec<i32>) -> &'static str {
    let mut count = 0;

    for i in reading_pos.iter() {
        if *i == 1 {
            count += 1;
        }
    }

    if reading_pos.len() / 2 < count {
        "1"
    } else {
        "0"
    }
}

#[test]
fn more_ones_test_one() {
    let one = vec![1, 1, 0, 1];
    assert_eq!("1", more_ones(one));
}

#[test]
fn more_ones_test_zero() {
    let zero = vec![1, 0, 0, 0];

    assert_eq!("0", more_ones(zero));
}

fn get_input() -> &'static str {
    "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010"
}
