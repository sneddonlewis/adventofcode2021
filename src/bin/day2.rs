fn main() {
    println!("Day 2: Dive");
    let instructions = "forward 5
down 5
forward 8
up 3
down 8
forward 2";

    let pos = Position::starting();
    let delta = parse_instructions(instructions);
    let result = pos.apply_delta(delta);

    println!("{:?}", result);
}

#[derive(Debug)]
struct Position {
    forward: i32,
    depth: i32,
}

impl Position {
    fn starting() -> Position {
        Position { forward: 0, depth: 0 }
    }

    fn apply_delta(&self, delta: Position) -> Position {
        Position {
            forward: self.forward + delta.forward,
            depth: self.depth + delta.depth       
        }
    }
}

fn parse_instructions(instructions: &str) -> Position {
    // split the str on newline
    // split each segment on whitespace
    // for key forward/backward +/- forward by amount
    // for key up/down +/- depth amount

    Position { forward: 5, depth: 5 }
}
