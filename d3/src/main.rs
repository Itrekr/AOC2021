use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
   println!("Result part 1: {}",solve_p1());
   // println!("Result part 1: {}",solve_p1());
   // println!("Result part 2: {}",solve_p2());
}

fn solve_p1() -> u32 {
    let file:File = File::open("src/input.txt").unwrap();
    let reader:BufReader<File> = BufReader::new(file);
    let mut diagnostic_report = DiagnosticReport::new();
    for line in reader.lines() {
        diagnostic_report.parse_line(&line.unwrap());
        }
    let result = diagnostic_report.compute_power_consumption();
    return result;
}

struct DiagnosticReport {
    gamma_rate: u32,
    epsilon_rate: u32,
    bit_vector: Vec<i32>,
}

impl DiagnosticReport {
    fn new() -> DiagnosticReport {
        DiagnosticReport { gamma_rate: 0, epsilon_rate: 0, bit_vector: Vec::new() }
    }

    fn parse_line(&mut self, binary:&str) {
        fn split_binary(bin: &str) -> Vec<u32> {
            bin.chars().map(|c| c.to_digit(10).unwrap()).collect()
        }
        let bits = split_binary(binary);
        let bit_amount = bits.len();

        self.bit_vector.resize(bit_amount, 0);

        for (i,&bit) in bits.iter().enumerate() {
            let modification: i32 = if bit == 1 { 1 } else { -1 };
            self.bit_vector[i] += modification;
        }
    }

    fn compute_power_consumption(&mut self) -> u32 {
        self.gamma_rate = DiagnosticReport::convert_bit_vector_to_bits(&self.bit_vector);
        self.epsilon_rate = (!self.gamma_rate) & !(u32::MAX << self.bit_vector.len());

        self.gamma_rate as u32 * self.epsilon_rate as u32
    }

    fn convert_bit_vector_to_bits(vector: &Vec<i32>) -> u32 {
        let mut result = 0;
        for (i, &number) in vector.iter().rev().enumerate() {
            let bit = if number > 0 { 1 } else { 0 };
            result += bit << i;
        }
        return result;
    }
}
