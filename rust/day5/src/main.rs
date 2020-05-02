use std::mem;
use std::io;
use std::convert::TryInto;

fn compute_output(input: &mut Vec<i32>, mut position: usize, opcode: u32) -> usize {
    let first = *input.get(position+1).unwrap() as usize;
    let second = *input.get(position+2).unwrap() as usize;
    let replace_index = *input.get(position+3).unwrap() as usize;

    let first_integer = input.get(first).unwrap();
    let second_integer = input.get(second).unwrap();

    match opcode {
        1 => {
            let third = first_integer + second_integer;
            let _ = mem::replace(&mut input[replace_index], third);
            println!("The integer at index {} will be replaced by {}", replace_index, third);
            position += 4;
        }
        2 => {
            let third = first_integer * second_integer; 
            let _ = mem::replace(&mut input[replace_index], third);
            println!("The integer at index {} will be replaced by {}", replace_index, third);
            position += 4;
        }
        3 => {
            let mut user_input = String::new();
            println!("Please enter an input: ");

            let replace_value = io::stdin().read_line(&mut user_input) 
                .expect("Unexpected value")
                .try_into()
                .unwrap();

            let _ = mem::replace(&mut input[first], replace_value);
            println!("The integer at index {} will be replaced by {}", first, user_input);
            position += 2;
        }
        4 => {
            println!("Value at address {} is {}", first, first_integer);
        }
        99 => {
            println!("Opcode 99 received. Terminating program...");
        }
        _ => {
            println!("Unexpected opcode received");
        }
    }
    position 
}

fn determine_mode(mode_value: u32, param: usize, input: &[i32]) -> i32 {
    match mode_value {
        0 => {
            let position = *input.get(param).unwrap() as usize;
            let value = input.get(position).unwrap();
            *value 
        },
        1 => { 
            let value = input.get(param).unwrap(); 
            *value 
        },
        _ => panic!("Unexpected mode encountered!"),
    }
}

fn main() {

    let mut input: Vec<i32> = vec![3,225,1,225,6,6,1100,1,238,225,104,0,1,192,154,224,101,-161,224,224,4,224,102,8,223,223,101,5,224,224,1,223,224,223,1001,157,48,224,1001,224,-61,224,4,224,102,8,223,223,101,2,224,224,1,223,224,223,1102,15,28,225,1002,162,75,224,1001,224,-600,224,4,224,1002,223,8,223,1001,224,1,224,1,224,223,223,102,32,57,224,1001,224,-480,224,4,224,102,8,223,223,101,1,224,224,1,224,223,223,1101,6,23,225,1102,15,70,224,1001,224,-1050,224,4,224,1002,223,8,223,101,5,224,224,1,224,223,223,101,53,196,224,1001,224,-63,224,4,224,102,8,223,223,1001,224,3,224,1,224,223,223,1101,64,94,225,1102,13,23,225,1101,41,8,225,2,105,187,224,1001,224,-60,224,4,224,1002,223,8,223,101,6,224,224,1,224,223,223,1101,10,23,225,1101,16,67,225,1101,58,10,225,1101,25,34,224,1001,224,-59,224,4,224,1002,223,8,223,1001,224,3,224,1,223,224,223,4,223,99,0,0,0,677,0,0,0,0,0,0,0,0,0,0,0,1105,0,99999,1105,227,247,1105,1,99999,1005,227,99999,1005,0,256,1105,1,99999,1106,227,99999,1106,0,265,1105,1,99999,1006,0,99999,1006,227,274,1105,1,99999,1105,1,280,1105,1,99999,1,225,225,225,1101,294,0,0,105,1,0,1105,1,99999,1106,0,300,1105,1,99999,1,225,225,225,1101,314,0,0,106,0,0,1105,1,99999,1108,226,226,224,102,2,223,223,1005,224,329,101,1,223,223,107,226,226,224,1002,223,2,223,1005,224,344,1001,223,1,223,107,677,226,224,102,2,223,223,1005,224,359,101,1,223,223,7,677,226,224,102,2,223,223,1005,224,374,101,1,223,223,108,226,226,224,102,2,223,223,1006,224,389,101,1,223,223,1007,677,677,224,102,2,223,223,1005,224,404,101,1,223,223,7,226,677,224,102,2,223,223,1006,224,419,101,1,223,223,1107,226,677,224,1002,223,2,223,1005,224,434,1001,223,1,223,1108,226,677,224,102,2,223,223,1005,224,449,101,1,223,223,108,226,677,224,102,2,223,223,1005,224,464,1001,223,1,223,8,226,677,224,1002,223,2,223,1005,224,479,1001,223,1,223,1007,226,226,224,102,2,223,223,1006,224,494,101,1,223,223,1008,226,677,224,102,2,223,223,1006,224,509,101,1,223,223,1107,677,226,224,1002,223,2,223,1006,224,524,1001,223,1,223,108,677,677,224,1002,223,2,223,1005,224,539,1001,223,1,223,1107,226,226,224,1002,223,2,223,1006,224,554,1001,223,1,223,7,226,226,224,1002,223,2,223,1006,224,569,1001,223,1,223,8,677,226,224,102,2,223,223,1006,224,584,101,1,223,223,1008,677,677,224,102,2,223,223,1005,224,599,101,1,223,223,1007,226,677,224,1002,223,2,223,1006,224,614,1001,223,1,223,8,677,677,224,1002,223,2,223,1005,224,629,101,1,223,223,107,677,677,224,102,2,223,223,1005,224,644,101,1,223,223,1108,677,226,224,102,2,223,223,1005,224,659,101,1,223,223,1008,226,226,224,102,2,223,223,1006,224,674,1001,223,1,223,4,223,99,226]; 
    
    let mut position = 0;

    while position < input.len() {
        let opcode = input[position];
        let opcode = opcode as u32;
        
        // Case 1: Opcode is single digit
        if (opcode / 10) < 1 {
            println!("Opcode is single digit");
            position = compute_output(&mut input, position, opcode);
            println!("{}", position);
        } 
        // Case 2: Opcode is 4/5-digit
        else {
            println!("Opcode has multiple digits");
            let mut instructions: Vec<u32> = opcode.to_string()
                .chars()
                .map(|z| z.to_digit(10).unwrap())
                .collect();
            // Inserting leading zero in cases where it is not specified(e.g. opcode is 4-digit)
            while instructions.len() < 5 {
                instructions.insert(0, 0);
            }

            instructions.reverse();
            assert_eq!(instructions.len(), 5);
            // Analyzing opcode and modes found in instructions
            let op = *instructions.get(0).unwrap();
            let first_param = determine_mode(*instructions.get(2).unwrap(), position+1, &input);
            let second_param = determine_mode(*instructions.get(3).unwrap(), position+2, &input);
            let third_param = determine_mode(*instructions.get(4).unwrap(), position+3, &input) as usize;
            match op {
                1 => {
                    let third = first_param + second_param;
                    let _ = mem::replace(&mut input[third_param], third);
                    position += 4;
                },
                2 => {
                    let third = first_param * second_param;
                    let _ = mem::replace(&mut input[third_param], third);
                    position += 4;
                }
                4 => println!("Output value: {}", first_param),
                9 => println!("Opcode 99 encountered. Terminating program..."),
                _ => panic!("Unexpected opcode {} encountered.", op),
            } 
        }
    }
}
