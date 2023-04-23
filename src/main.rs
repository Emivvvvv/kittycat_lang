use std::fs;
use std::collections::HashMap;

fn main() {
    let mut memory: [u8; 5000] = [0u8; 5000];
    let mut cell_index = 0usize;

    let instructions = parse_input(read_file_string("src/input.txt").unwrap());

    let mut loop_table: HashMap<usize, usize> = HashMap::new();
    let mut loop_stack: Vec<usize> = Vec::new();
    let mut loop_beginning_index = 0usize;

    for instruction_index in 0..(instructions.len()/4){
        let instruction = &instructions[instruction_index*4..instruction_index*4+4];
        if instruction == "Meow" {loop_stack.push(instruction_index)}
        else if instruction == "meoW" {
            loop_beginning_index = loop_stack.pop().expect("WAAOUUOOWWW MEOW MEOW (THIS CODE IS BROKEN!!! YOU MISS TYPED 'Meow' or 'meoW')");
            loop_table.insert(loop_beginning_index, instruction_index);
            loop_table.insert(instruction_index, loop_beginning_index);
        }
    }

    let mut instruction_index = 0usize;
    while instruction_index < (instructions.len()/4){
        let instruction = &instructions[instruction_index*4..instruction_index*4+4];
        match instruction {
            "prrr" => cell_index += 1,
            "mrrr" => {
                if cell_index < 0 {panic!("KIIIHHHH MEOW MEOW (CELL INDEX CAN'T BE LESS THAN 0)")}
                else {cell_index -= 1}
            },
            "MEOW" => memory[cell_index] += 1,
            "meow" => memory[cell_index] -= 1,
            "mEOw" => memory[cell_index] = input() as u8,
            "MeoW" => print!("{}", memory[cell_index] as char),
            "Meow" => if memory[cell_index] == 0 { instruction_index = loop_table[&instruction_index] },
            "meoW" => if memory[cell_index] != 0 { instruction_index = loop_table[&instruction_index] },
            _ => panic!("TISSSSS MEOW MEOW MEOW (I NO KNOW THIS HUMAN WURDS!!)"),
        }


        if cell_index > 4999 {panic!("MAAOOOUWW MEOW MEOW MEOW (CELL INDEX CAN'T BE MORE THAN 4999)")}

        instruction_index += 1;
    }
}


fn read_file_string(filepath: &str) -> Result<String, Box<dyn std::error::Error>> {
    let data = fs::read_to_string(filepath)?;
    Ok(data)
}

fn parse_input(input: String) -> String {
    let trimmed_vec = input.trim().split(&[' ', '\n', '\t', '\r']).collect::<Vec<&str>>();
    let mut trimmed = String::new();
    for command in trimmed_vec {
        trimmed += command;
    }
    trimmed
}

fn input() -> char {
    let mut io_input = String::new();
    std::io::stdin()
        .read_line(&mut io_input)
        .expect("failed to read from stdin");

    let input_trimmed = io_input.trim();
    match input_trimmed.parse::<char>() {
        Ok(input) => {
            if input as u8 > 255 as u8 { panic!("I NO KNOW THIS HUMAN KARACTER!") };
            input
        },
        Err(..) => panic!("I NO KNOW WHAT U SAY I ONLY KNOW MEOW MEOW!"),
    }
}
