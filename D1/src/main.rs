use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let result = count();
    println!("Result is {}",result);
}

/// for x in measurments:
/// check value > prev_value
/// if true: aantal=aantal+1
/// set prev_value value

fn get_numbers() -> std::io::Result<Vec<u16>> {
    let file:File = File::open("src/ms.txt")?;
    let reader:BufReader<File> = BufReader::new(file);

    let mut numbers:Vec<u16> = Vec::new();
    for line in reader.lines() {
        if let Ok(number) = line?.parse::<u16>() {
            numbers.push(number);
        }
    }

    // println!("{:?}", numbers);

    Ok(numbers)
}

/// Returns amount of increasing measurements as a 16bit unsigned integer
fn count() -> u16 {
    let mut prev_value = u16::MAX;
    let mut result = 0;
    let numbers = get_numbers().unwrap();
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
