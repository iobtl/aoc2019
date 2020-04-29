use std::mem;

fn compute_output(vec: &[u32], n: u32, v: u32) -> Vec<u32> {
    let mut newvec: Vec<u32> = vec.to_vec();
    // Replacing with noun and verb
    let _ = mem::replace(&mut newvec[1], n);
    let _ = mem::replace(&mut newvec[2], v);
    println!("Current noun and verbs are: {} and {}", n, v);

    let mut position = 0;

    while position < newvec.len() - 3 {
        let first = *newvec.get(position+1).unwrap() as usize;
        let second = *newvec.get(position+2).unwrap() as usize;
        let replace_index = *newvec.get(position+3).unwrap() as usize;

        let first_integer = newvec.get(first).unwrap();
        let second_integer = newvec.get(second).unwrap();
        match newvec[position] {
            1 => {
                // Don't need return value, only replacement of value in newvec
                let third = first_integer + second_integer;
                let _ = mem::replace(&mut newvec[replace_index], third);
                println!("The integer at index {} will be replaced by {}", replace_index, third);
            }
            2 => {
                let third = first_integer * second_integer; 
                let _ = mem::replace(&mut newvec[replace_index], third);
                println!("The integer at index {} will be replaced by {}", replace_index, third);
            }
            99 => {
                println!("Opcode 99 encountered. Terminating program.");
                break;
            }
            _ => panic!("Unexpected opcode encountered."),
        }
        position += 4;
        }
    newvec
}

pub struct Instructions {
    pub noun: u32,
    pub verb: u32,
}

impl Instructions {
    pub fn increment(self) -> Self {
        if self.verb < 99 {
            Instructions { noun: self.noun, verb: self.verb + 1 }
        } else {
            Instructions { noun: self.noun + 1, verb: 0 }
        }
    }
}

fn main() {

    let input: Vec<u32> = vec![1,12,2,3,1,1,2,3,1,3,4,3,1,5,0,3,2,1,6,19,1,9,19,23,1,6,23,27,1,10,27,31,1,5,31,35,2,6,35,39,1,5,39,43,1,5,43,47,2,47,6,51,1,51,5,55,1,13,55,59,2,9,59,63,1,5,63,67,2,67,9,71,1,5,71,75,2,10,75,79,1,6,79,83,1,13,83,87,1,10,87,91,1,91,5,95,2,95,10,99,2,9,99,103,1,103,6,107,1,107,10,111,2,111,10,115,1,115,6,119,2,119,9,123,1,123,6,127,2,127,10,131,1,131,6,135,2,6,135,139,1,139,5,143,1,9,143,147,1,13,147,151,1,2,151,155,1,10,155,0,99,2,14,0,0]; 
    
    // Finding values for noun and verbs given their range from 0-99
    let mut program: Vec<u32> = input.to_vec();

    let mut inputs = Instructions { noun: 0, verb: 0 };

    while program[0] != 19690720 {
        // Resetting the program every loop
        program = input.to_vec();
        program = compute_output(&program, inputs.noun, inputs.verb);
        inputs = inputs.increment();
    }
    println!("Noun is: {}; Verb is {}", inputs.noun, inputs.verb-1);
}
