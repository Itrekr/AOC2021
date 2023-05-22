/// Crates for reading file content

use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
   println!("Result part 1: {}",solve_p1());
   println!("Result part 2: {}",solve_p2());
}

fn solve_p1() -> i32 {
    let file:File = File::open("src/input.txt").unwrap();
    let reader:BufReader<File> = BufReader::new(file);
    let mut submarine = Submarine{horizontal_position:0,depth:0,aim:0};
    for line in reader.lines() {
        submarine.execute(&line.unwrap());
        }
    let result = submarine.horizontal_position * submarine.depth;
    return result;
}

fn solve_p2() -> i32 {
    let file:File = File::open("src/input.txt").unwrap();
    let reader:BufReader<File> = BufReader::new(file);
    let mut submarine = Submarine{horizontal_position:0,depth:0,aim:0};
    for line in reader.lines() {
        submarine.execute2(&line.unwrap());
        }
    let result = submarine.horizontal_position * submarine.depth;
    return result;
    }

struct Submarine {
    horizontal_position: i32,
    depth: i32,
    aim: i32,
}

impl Submarine {
    // Mutable reference to self necessary to change horizontal_position and depth
    fn execute(&mut self, command:&str) {
        // split the content of command by whitespace and put it in a vector with collect()
        let words:Vec<&str> = command.split_whitespace().collect();
        let direction = words[0];
        // specifies that words[1] is an 32 bit signed integer and unwrap the value
        let amount = words[1].parse::<i32>().unwrap();
        if direction == "forward" {
           self.horizontal_position += amount;
        }
        else if direction == "up" {
           self.depth -= amount;
        }
        else if direction == "down" {
           self.depth += amount;
        }
    }
    fn execute2(&mut self, command:&str) {
        // split the content of command by whitespace and put it in a vector with collect()
        let words:Vec<&str> = command.split_whitespace().collect();
        let direction = words[0];
        // specifies that words[1] is an 32 bit signed integer and unwrap the value
        let amount = words[1].parse::<i32>().unwrap();
        if direction == "forward" {
            self.horizontal_position += amount;
            self.depth += self.aim * amount;
        }
        else if direction == "up" {
            self.aim -= amount;
        }
        else if direction == "down" {
            self.aim += amount;
        }
    }
}
