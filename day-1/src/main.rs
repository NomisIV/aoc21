use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let mut prev = -1;
    let mut x = -1;

    for line in io::stdin().lock().lines() {
        let line = line?;
        let num = line.parse::<i32>().unwrap();
        if num > prev {
            x += 1;
        }
        prev = num;
    }

    println!("{}", x);

    Ok(())
}
