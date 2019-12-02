
fn main() {
    let data_1 = create_data();
    part_1(data_1);
}

fn part_1(mut program: Vec<usize>) {
    program[1] = 12;
    program[2] = 2;
    run_program(&mut program);
    println!("(pt1) Intcode @ position 0 = {}", program[0]);
}

fn run_program (program: &mut Vec<usize>) {
    let mut pc = 0;
    loop {
      // do stuff
      let opcode = program[pc];
      if opcode == 99 {
          break;
      }
      let a = program[program[pc + 1]];
      let b = program[program[pc + 2]];
      let d = program[pc + 3];
      let result = op(opcode, a, b);

      program[d] = result;

      pc += 4;
    }
}

fn op (o: usize, a: usize, b: usize) -> usize {
    if o == 1 {
        a + b
    } else if o == 2 {
        a * b
    } else {
        panic!("Unknown opcode: {}", o);
    }
}

fn create_data() -> Vec<usize> {
    vec![1,0,0,3,1,1,2,3,1,3,4,3,1,5,0,3,2,13,1,19,1,19,10,23,1,23,13,27,1,6,27,31,1,9,31,35,2,10,35,39,1,39,6,43,1,6,43,47,2,13,47,51,1,51,6,55,2,6,55,59,2,59,6,63,2,63,13,67,1,5,67,71,2,9,71,75,1,5,75,79,1,5,79,83,1,83,6,87,1,87,6,91,1,91,5,95,2,10,95,99,1,5,99,103,1,10,103,107,1,107,9,111,2,111,10,115,1,115,9,119,1,13,119,123,1,123,9,127,1,5,127,131,2,13,131,135,1,9,135,139,1,2,139,143,1,13,143,0,99,2,0,14,0]
}

#[test]
fn test_run_program_1() {
    let mut program = vec![1,1,1,4,99,5,6,0,99];
    run_program(&mut program);
    assert_eq!(program, vec![30,1,1,4,2,5,6,0,99]);
}

#[test]
fn test_run_program_2() {
    let mut program = vec![2,4,4,5,99,0];
    run_program(&mut program);
    assert_eq!(program, vec![2,4,4,5,99,9801]);
}
