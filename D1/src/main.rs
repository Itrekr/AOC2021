/// Crates for reading file content
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let numbers_p1 = get_numbers().unwrap();
    let result_p1 = count(&numbers_p1);
    println!("Result part 1 is {}",result_p1);
    let numbers_p2 = sum_numbers(numbers_p1);
    let result_p2 = count(&numbers_p2);
    println!("Result part 2 is {}",result_p2)
}

fn get_numbers() -> std::io::Result<Vec<u16>> {
    let file:File = File::open("src/ms.txt")?;
    let reader:BufReader<File> = BufReader::new(file);

    let mut numbers:Vec<u16> = Vec::new();
    for line in reader.lines() {
        if let Ok(number) = line?.parse::<u16>() {
            numbers.push(number);
        }
    }

    Ok(numbers)
}

/// Returns amount of increasing measurements as a 16bit unsigned integer
fn count(numbers:&Vec<u16>) -> u16 {
    // Set first iteration of prev_value to max value of 16bit unsigned integer and make it mutable
    let mut prev_value = u16::MAX;
    // Set first iteration of result to 0 and make it mutable
    let mut result = 0;
    // Compare each number (except the first in the index) to the number before and add 1 to the result every time the number is bigger than the previous one.
    for (index,value) in numbers.iter().enumerate() {
        if index == 0 {
            println!("{} (N/A - no previous measurement)",*value);
        }
        else if prev_value < *value {
            result = result+1;
            println!("{} (increased)",*value);
        }
        else if prev_value == *value {
            println!("{} #NOCHANGES",*value);
        }
        else if prev_value > *value {
            println!("{} (decreased)",*value);
        }
        prev_value = *value;
    }
    return result;
}

fn sum_numbers(numbers:Vec<u16>) -> Vec<u16> {
   let mut result = Vec::new();
   for index in 0..(numbers.len()-2) {
       let sum = numbers[index]+numbers[index+1]+numbers[index+2];
       result.push(sum);
   }
   return result;
}
