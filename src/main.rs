use std::io;
use std::io::Read;

fn main() {
    let mut brainfuck_buffer: String = "".to_string();
    io::stdin().read_to_string(&mut brainfuck_buffer).expect("Something went wrong");

    let mut array: Vec<u8> = vec![0; 1000000];
    let mut ptr = 0;

    let brain_bytes = brainfuck_buffer.chars();
    let mut input: Vec<char> = Vec::new();
    for element in brain_bytes {
        input.push(element);
    }
    println!("{:?}", input);
    let mut index = 0;
    loop {
        if index >= input.len() {
            break;
        }
        match input[index] {
            '>' => {
                ptr += 1;
                index += 1;
            }
            '<' => {
                ptr -= 1;
                index += 1;
            }
            '+' => {
                array[ptr] += 1;
                index += 1;
            }
            '-' => {
                array[ptr] -= 1;
                index += 1;
            }
            '.' => {
                print!("{}", array[ptr] as char);
                index += 1;
            }
            ',' => {
            	array[ptr] = std::io::stdin().bytes().next().and_then(|result| result.ok()).map(|byte| byte as u8).expect("YOU FAIL");
            	index += 1;
            }
            '[' => {
                if array[ptr] > 0 {
                    index += 1;
                } else {
                    let mut countopen = 1;
                    for i in (index + 1)..array.len() {
                        if input[i] == '[' {
                            countopen += 1;
                        }
                        if input[i] == ']' {
                            countopen -= 1;
                        }
                        if countopen == 0 {
                            index = i + 1; // Place pointer after closing brace
                            break;
                        }
                    }
                }
            }
            ']' => {
                let mut countopen = 1;
                let mut i = index - 1;
                loop {
                    if input[i] == '[' {
                        countopen -= 1;
                    }
                    if input[i] == ']' {
                        countopen += 1;
                    }
                    if countopen == 0 {
                        index = i; // Place pointer on opening brace
                        break;
                    }
                    i -= 1;
                }
            }
            _ => {
                index += 1;
            }
        }
    }
}
