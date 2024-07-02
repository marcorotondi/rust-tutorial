use std::{io, u32};

fn main() {
    println!("== String Calculator ==");
    let mut input_line = String::new();

    io::stdin()
        .read_line(&mut input_line)
        .expect("Error to read input line!");

    println!(
        "result form input {} is: {}",
        input_line.trim(),
        add(&input_line)
    );
}

fn add(input_buffer: &String) -> u32 {
    let parsing: Vec<&str> = input_buffer.split_terminator(',').collect();
    let mut sum: u32 = 0;

    for value in parsing {
        let v_number: i32 = value.trim().parse().unwrap();

        if v_number <= 1000 {
            if v_number.signum() != -1 {
                sum = sum + v_number as u32;
            } else {
                println!("negative value {} not allowed", v_number);
            }
        }
    }

    sum
}
