use std::io::BufRead;

fn main() {
    println!(
        "{}",
        std::io::stdin()
            .lock()
            .lines()
            .map(|line| {
                let line = line.unwrap();
                let line_iter = line.split(" ").collect::<Vec<&str>>();
                let direction = *line_iter.get(0).unwrap();
                let steps = line_iter.get(1).unwrap().parse::<i32>().unwrap();
                match direction {
                    "forward" => [steps, 0],
                    "up" => [0, -steps],
                    "down" => [0, steps],
                    _ => unreachable!(),
                }
            })
            .reduce(|acc, line| { [acc[0] + line[0], acc[1] + line[1]] })
            .unwrap()
            .iter()
            .product::<i32>()
    )
}
