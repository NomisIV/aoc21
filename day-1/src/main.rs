use std::io::{self, BufRead};

fn main() -> io::Result<()> {

    let input: Vec<i32> = io::stdin().lock().lines().map(|line| {
        line.unwrap().parse().unwrap()
    }).collect();

    let mut prev = -1;
    let mut x = -1;
    for num in input.clone() {
        if num > prev {
            x += 1;
        }
        prev = num;
    }

    println!("{}", x);

    let avg_input: Vec<i32> = {
        let mut vec = Vec::new();
        for n in 1..(input.len() - 1) {
            vec.push(input[n-1] + input[n] + input[n+1]);
        };
        vec
    };

    let mut prev = -1;
    let mut x = -1;
    for num in avg_input {
        if num > prev {
            x += 1;
        }
        prev = num;
    }

    println!("{}", x);

    Ok(())
}
