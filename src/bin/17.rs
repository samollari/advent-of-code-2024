use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "17";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST_PART1: &str = "\
Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0
";

const TEST_PART2: &str = "\
Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0
";

fn assert_3bit(val: u8) {
    assert!((val >> 3) == 0)
}

#[derive(Debug)]
enum ComboOperand {
    Literal(u8),
    RegA,
    RegB,
    RegC,
}

impl ComboOperand {
    fn from(val: u8) -> Self {
        match val {
            0..=3 => Self::Literal(val),
            4 => Self::RegA,
            5 => Self::RegB,
            6 => Self::RegC,
            7 => panic!("Reserved unusable combo operand!"),
            _ => panic!("Combo operand not a 3-bit value!"),
        }
    }

    fn get_value(self: &Self, machine: &Machine) -> usize {
        match self {
            ComboOperand::Literal(val) => (*val).into(),
            ComboOperand::RegA => machine.reg_a,
            ComboOperand::RegB => machine.reg_b,
            ComboOperand::RegC => machine.reg_c,
        }
    }
}

#[derive(Debug)]
enum Instruction {
    /**
     * The adv instruction (opcode 0) performs division.
     * The numerator is the value in the A register.
     * The denominator is found by raising 2 to the power of the instruction's combo operand.
     * The result of the division operation is truncated to an integer and then written to the A register.
     */
    ADV(ComboOperand),
    /**
     *  The bxl instruction (opcode 1) calculates the bitwise XOR of register B and the instruction's literal operand, then stores the result in register B.
     */
    BXL(u8),
    /**
     * The bst instruction (opcode 2) calculates the value of its combo operand modulo 8 (thereby keeping only its lowest 3 bits), then writes that value to the B register.
     */
    BST(ComboOperand),
    /**
     * The jnz instruction (opcode 3) does nothing if the A register is 0.
     * However, if the A register is not zero, it jumps by setting the instruction pointer to the value of its literal operand; if this instruction jumps, the instruction pointer is not increased by 2 after this instruction.
     */
    JNZ(u8),
    /**
     * The bxc instruction (opcode 4) calculates the bitwise XOR of register B and register C, then stores the result in register B.
     * (For legacy reasons, this instruction reads an operand but ignores it.)
     */
    BXC,
    /**
     * The out instruction (opcode 5) calculates the value of its combo operand modulo 8, then outputs that value.
     * (If a program outputs multiple values, they are separated by commas.)
     */
    OUT(ComboOperand),
    /**
     * The bdv instruction (opcode 6) works exactly like the adv instruction except that the result is stored in the B register.
     * (The numerator is still read from the A register.)
     */
    BDV(ComboOperand),
    /**
     * The cdv instruction (opcode 7) works exactly like the adv instruction except that the result is stored in the C register.
     * (The numerator is still read from the A register.)
     */
    CDV(ComboOperand),
}

impl Instruction {
    fn from((instr, op): (u8, u8)) -> Self {
        assert_3bit(op);

        match instr {
            0 => Self::ADV(ComboOperand::from(op)),
            1 => Self::BXL(op),
            2 => Self::BST(ComboOperand::from(op)),
            3 => Self::JNZ(op),
            4 => Self::BXC,
            5 => Self::OUT(ComboOperand::from(op)),
            6 => Self::BDV(ComboOperand::from(op)),
            7 => Self::CDV(ComboOperand::from(op)),
            _ => panic!("Instruction is not a 3-bit value!"),
        }
    }
}

#[derive(Debug, Clone)]
struct Machine {
    reg_a: usize,
    reg_b: usize,
    reg_c: usize,
    output: Vec<u8>,
}

fn parse_reg_value(line: String) -> usize {
    line.trim()[12..].parse().unwrap()
}

fn parse_instructions(instructions: &String) -> Vec<Instruction> {
    instructions
        .split(',')
        .map(|val| val.to_string())
        .chunks(2)
        .into_iter()
        .map(|mut chunk| {
            let instr_string = chunk.next().expect("should have instruction character");
            let op_string = chunk.next().expect("should have operand character");
            let instr = instr_string
                .parse::<u8>()
                .expect("should be parsable to u8");
            let op = op_string.parse::<u8>().expect("should be parsable to u8");
            (instr, op)
        })
        .map(Instruction::from)
        .collect()
}

fn parse_program<R: BufRead>(reader: R) -> (Machine, String) {
    let mut lines = reader.lines();

    let reg_a = parse_reg_value(lines.next().expect("Register A").unwrap());
    let reg_b = parse_reg_value(lines.next().expect("Register B").unwrap());
    let reg_c = parse_reg_value(lines.next().expect("Register C").unwrap());
    let _ = lines.next().unwrap().unwrap();

    let instructions = lines.next().unwrap().unwrap()[9..].to_string();

    let machine = Machine {
        reg_a,
        reg_b,
        reg_c,
        output: vec![],
    };

    (machine, instructions)
}

fn calc_division(operand: &ComboOperand, machine: &Machine) -> usize {
    machine.reg_a / 2_usize.pow(operand.get_value(machine).try_into().unwrap())
}

fn execute(mut machine: Machine, instructions: &Vec<Instruction>) -> String {
    let mut ip: usize = 0;

    loop {
        assert!(ip % 2 == 0); // TODO: If not, have to parse instructions at runtime

        let idx = ip / 2;
        if idx >= instructions.len() {
            // HALT
            break;
        }

        let instruction = &instructions[idx];

        // println!("\t{:?}\n({}): {:?}", machine, ip, instruction);

        ip += 2;

        match instruction {
            Instruction::ADV(combo_operand) => {
                machine.reg_a = calc_division(combo_operand, &machine)
            }
            Instruction::BXL(operand) => machine.reg_b = machine.reg_b ^ (*operand as usize),
            Instruction::BST(combo_operand) => {
                machine.reg_b = combo_operand.get_value(&machine) & 0b111
            }
            Instruction::JNZ(operand) => {
                if machine.reg_a != 0 {
                    ip = *operand as usize
                }
            }
            Instruction::BXC => machine.reg_b = machine.reg_b ^ machine.reg_c,
            Instruction::OUT(combo_operand) => machine
                .output
                .push((combo_operand.get_value(&machine) & 0b111) as u8),
            Instruction::BDV(combo_operand) => {
                machine.reg_b = calc_division(combo_operand, &machine)
            }
            Instruction::CDV(combo_operand) => {
                machine.reg_c = calc_division(combo_operand, &machine)
            }
        }
    }

    machine.output.into_iter().join(",")
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<String> {
        let (machine, instructions) = parse_program(reader);
        let instructions = parse_instructions(&instructions);

        Ok(execute(machine, &instructions))
    }

    assert_eq!(
        "4,6,3,5,6,3,5,2,1,0",
        part1(BufReader::new(TEST_PART1.as_bytes()))?
    );

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let (machine, instruction_text) = parse_program(reader);
        let instructions = parse_instructions(&instruction_text);

        for i in 0.. {
            if i % 1_000_000 == 0 {
                println!("{}", i);
            }

            let mut machine = machine.clone();
            machine.reg_a = i;

            let execution_result = execute(machine, &instructions);

            if execution_result == instruction_text {
                return Ok(i);
            }

            // if i == 117440 {
            //     println!(
            //         "instructions: {}\tresult: {}",
            //         instruction_text, execution_result
            //     );
            //     panic!();
            // }
        }

        panic!("How did you get here?");
    }

    assert_eq!(117440, part2(BufReader::new(TEST_PART2.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
