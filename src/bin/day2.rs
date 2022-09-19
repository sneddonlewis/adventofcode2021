fn main() {
    println!("Day 2: Dive");

    let result = get_input().lines().map(parse_instructions).fold(
        Position {
            forward: 0,
            depth: 0,
        },
        |mut acc, pos| {
            acc.forward += pos.forward;
            acc.depth += pos.depth;
            acc
        },
    );

    println!("{:?}", result);
}

fn get_input() -> &'static str {
    "forward 5
down 5
forward 8
up 3
down 8
forward 2"
}

#[derive(Debug)]
struct Position {
    forward: i32,
    depth: i32,
}

fn parse_instructions(instructions: &str) -> Position {
    let (dir, amt) = instructions
        .split_once(" ")
        .expect("must contain whitespace");
    let amount = str::parse::<i32>(amt).expect("must contain a number");

    match dir {
        "forward" => Position {
            forward: amount,
            depth: 0,
        },
        "down" => Position {
            forward: 0,
            depth: amount,
        },
        "up" => Position {
            forward: 0,
            depth: amount * -1,
        },
        _ => panic!("incorrect instructions"),
    }
}
