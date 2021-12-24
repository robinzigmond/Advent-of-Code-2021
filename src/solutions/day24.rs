use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
enum Variable {
    W,
    X,
    Y,
    Z,
}

#[derive(Clone, Copy, Debug)]
enum Value {
    Var(Variable),
    Literal(isize),
}

#[derive(Clone, Copy, Debug)]
enum Instruction {
    Input(Variable),
    Add(Variable, Value),
    Multiply(Variable, Value),
    Divide(Variable, Value),
    Modulo(Variable, Value),
    IfEqual(Variable, Value),
}

struct Program(Vec<Instruction>);

impl Program {
    fn get_result(&self, inputs: Vec<isize>) -> Memory {
        let mut memory = Memory::new();
        let mut index = 0;
        for instruction in &self.0 {
            if let Instruction::Input(_) = instruction {
                let input = Some(inputs[index]);
                memory.process(*instruction, input);
                index += 1;
            } else {
                memory.process(*instruction, None);
            }
        }
        memory
    }

    fn get_result_expression(&self, input_vars: Vec<String>) -> Expression {
        let mut memory = ExpressionMemory::new();
        let mut index = 0;
        let mut step = 1;
        for instruction in &self.0 {
            println!("instruction #{}", step);
            if let Instruction::Input(_) = instruction {
                let input = &input_vars[index];
                memory.process(*instruction, input.to_string());
                index += 1;
            } else {
                memory.process(*instruction, String::new());
            }
            step += 1;
        }
        memory.0.get(&Variable::Z).unwrap().clone()
    }

    fn test_number(&self, number: u64, digits: usize) -> bool {
        println!("testing {}", format!("{:0width$}", number, width = digits));
        let inputs = format!("{:0width$}", number, width = digits)
            .chars()
            .map(|c| c.to_string().parse().unwrap())
            .collect();
        let res = self.get_result(inputs);
        println!("result is {:?}", res.0);
        res.0.get(&Variable::Z).unwrap() == &0
    }
}

struct Memory(HashMap<Variable, isize>);

impl Memory {
    fn new() -> Memory {
        let mut memory = HashMap::new();
        for var in vec![Variable::W, Variable::X, Variable::Y, Variable::Z] {
            memory.insert(var, 0);
        }
        Memory(memory)
    }

    fn read_value(&self, val: Value) -> isize {
        match val {
            Value::Literal(n) => n,
            Value::Var(var) => *self.0.get(&var).unwrap(),
        }
    }

    fn process(&mut self, instruction: Instruction, input: Option<isize>) {
        //println!("instruction is {:?}", instruction);
        match instruction {
            Instruction::Input(var) => {
                //println!("input is {}", input.unwrap());
                self.0.insert(var, input.unwrap());
            }
            Instruction::Add(var, val) => {
                let value = self.read_value(val);
                self.0.entry(var).and_modify(|e| *e += value);
            }
            Instruction::Multiply(var, val) => {
                let value = self.read_value(val);
                self.0.entry(var).and_modify(|e| *e *= value);
            }
            Instruction::Divide(var, val) => {
                let value = self.read_value(val);
                self.0.entry(var).and_modify(|e| *e /= value);
            }
            Instruction::Modulo(var, val) => {
                let value = self.read_value(val);
                self.0.entry(var).and_modify(|e| *e %= value);
            }
            Instruction::IfEqual(var, val) => {
                let value = self.read_value(val);
                self.0
                    .entry(var)
                    .and_modify(|e| *e = if *e == value { 1 } else { 0 });
            }
        }
        //println!("result is {:?}", self.0);
    }
}

#[derive(Clone)]
enum Expression {
    Var(String),
    Literal(isize),
    Add(Box<Expression>, Box<Expression>),
    Multiply(Box<Expression>, Box<Expression>),
    Divide(Box<Expression>, Box<Expression>),
    Modulo(Box<Expression>, Box<Expression>),
    IfEqual(Box<Expression>, Box<Expression>),
}

impl Expression {
    fn display(&self) -> String {
        match self {
            Expression::Var(s) => s.to_string(),
            Expression::Literal(n) => n.to_string(),
            Expression::Add(e1, e2) => format!("({} + {})", e1.display(), e2.display()),
            Expression::Multiply(e1, e2) => format!("({} * {})", e1.display(), e2.display()),
            Expression::Divide(e1, e2) => format!("({} / {})", e1.display(), e2.display()),
            Expression::Modulo(e1, e2) => format!("({} % {})", e1.display(), e2.display()),
            Expression::IfEqual(e1, e2) => {
                format!("(if {} = {} then 1 else 0)", e1.display(), e2.display())
            }
        }
    }
}

struct ExpressionMemory(HashMap<Variable, Expression>);

impl ExpressionMemory {
    fn new() -> ExpressionMemory {
        let mut memory = HashMap::new();
        for var in vec![Variable::W, Variable::X, Variable::Y, Variable::Z] {
            memory.insert(var, Expression::Literal(0));
        }
        ExpressionMemory(memory)
    }

    fn read_value(&self, val: Value) -> Expression {
        match val {
            Value::Literal(n) => Expression::Literal(n),
            Value::Var(var) => self.0.get(&var).unwrap().clone(),
        }
    }

    fn process(&mut self, instruction: Instruction, input: String) {
        match instruction {
            Instruction::Input(var) => {
                self.0.insert(var, Expression::Var(input));
            }
            Instruction::Add(var, val) => {
                let value = self.read_value(val);
                //let new_val = Expression::Add(Box::new(existing), Box::new(value));
                self.0
                    .entry(var)
                    .and_modify(|e| *e = Expression::Add(Box::new(e.clone()), Box::new(value)));
            }
            Instruction::Multiply(var, val) => {
                let value = self.read_value(val);
                self.0.entry(var).and_modify(|e| {
                    *e = Expression::Multiply(Box::new(e.clone()), Box::new(value))
                });
            }
            Instruction::Divide(var, val) => {
                let value = self.read_value(val);
                self.0
                    .entry(var)
                    .and_modify(|e| *e = Expression::Divide(Box::new(e.clone()), Box::new(value)));
            }
            Instruction::Modulo(var, val) => {
                let value = self.read_value(val);
                self.0
                    .entry(var)
                    .and_modify(|e| *e = Expression::Modulo(Box::new(e.clone()), Box::new(value)));
            }
            Instruction::IfEqual(var, val) => {
                let value = self.read_value(val);
                self.0
                    .entry(var)
                    .and_modify(|e| *e = Expression::IfEqual(Box::new(e.clone()), Box::new(value)));
            }
        }
    }
}

fn parse_variable(var: &str) -> Variable {
    match var {
        "w" => Variable::W,
        "x" => Variable::X,
        "y" => Variable::Y,
        "z" => Variable::Z,
        _ => panic!("unexpected variable {}", var),
    }
}

fn parse_value(val: &str) -> Value {
    match val {
        "w" | "x" | "y" | "z" => Value::Var(parse_variable(val)),
        num => Value::Literal(num.parse().unwrap()),
    }
}

fn parse_instruction(line: &str) -> Instruction {
    let parts: Vec<&str> = line.split_ascii_whitespace().collect();
    let first = parse_variable(parts[1]);
    let mut second = None;
    if parts.len() > 2 {
        second = Some(parse_value(parts[2]));
    }
    match parts[0] {
        "inp" => Instruction::Input(first),
        "add" => Instruction::Add(first, second.unwrap()),
        "mul" => Instruction::Multiply(first, second.unwrap()),
        "div" => Instruction::Divide(first, second.unwrap()),
        "mod" => Instruction::Modulo(first, second.unwrap()),
        "eql" => Instruction::IfEqual(first, second.unwrap()),
        _ => panic!("unexpected instruction {}", parts[0]),
    }
}

fn read_file() -> Program {
    let mut file = File::open("./input/input24.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let instructions = contents.lines().map(parse_instruction).collect();
    Program(instructions)
}

/*
There is a "single" set of 18 instructions, repeated 14 times. The only things which changes are 3 literals which
I call m, n and o.
On the right is the successive result of each step, starting at w,x,y,z and with the input called i.

below, some expressions are abbreviated:
- q0 is z if m is 1, or floor(z / 26) if m is 26
- q1 is 1 if (z%26)+n is equal to i, else 0
- q2 is 1 if q1 is 0, else 0

inp w               i,  x,        y,               z
mul x 0             i,  0,        y,               z
add x z             i,  z,        y,               z
mod x 26            i,  z%26,     y,               z
div z m             i,  z%26,     y,               q0
add x n             i,  (z%26)+n, y,               q0
eql x w             i,  q1,       y,               q0
eql x 0             i,  q2,       y,               q0
mul y 0             i,  q2,       0,               q0
add y 25            i,  q2,       25,              q0
mul y x             i,  q2,       25*q2,           q0
add y 1             i,  q2,       (25*q2)+1,       q0
mul z y             i,  q2,       (25*q2)+1,       (25*q0*q2)+q0
mul y 0             i,  q2,       0,               (25*q0*q2)+q0
add y w             i,  q2,       i,               (25*q0*q2)+q0
add y o             i,  q2,       i+o,             (25*q0*q2)+q0
mul y x             i,  q2,       (i*q2)+(o*q2),   (25*q0*q2)+q0
add z y             i,  q2,       (i*q2)+(10*q2),  (25*q0*q2)+(i*q2)+(o*q2)+q0

n's are successively 13,11,11,10,-14,-4,11,-3,12,-12,13,-12,-15,-12
m is either 1 or 26: specifically 26 on steps 5,6,8,10,12,13,14 - otherwise 1
o's are successively 10,16,0,13,7,11,11,10,16,8,15,2,5,10

so, starting with z = 0, and taking successive (i,n) pairs with i the digit input and ns above, the new z
is one of the following:
- if q2 = 0: q0
- if q2 = 1: 26*q0 + i + o
the last can NEVER be 0: i and o are always both positive
so we can only get 0 when q0=q2=0.
q2 being 0 means (z%26)+n is equal to i
q0 being 0 means either z was 0 OR z was < 26 and m=26
but if z was 0 then z%26+n equal to i means n is equal to i. i is 1..9 but none of the n's given meet this!
so we must have the prior z < 26 and m = 26. m = 26 does indeed happen on the last step (14).
so i, the last digit, must equal z%26+n = z+n = z-12. Where z is the *previous* z.

This is too hard by hand, but might be able to code it?
Maybe not, still have 10^14 possibilities to test!

in detail, given a z, the output at each stage is (depending on i, the digit):
1) (m,n,o) = (1,13,10)
q0=0
q2=0 if (z%26)+n is equal to i, ie i = z%26 + 13 - else 1
z starts at 0, so q2 is always 1 (i is NOT 13!)
so new z is (25*q0*q2)+(i*q2)+(o*q2)+q0 = i + o = i1 + 10

2) (m,n,o) = (1,11,16)
q0=z=i1+10
q2=0 if (z%26)+n is equal to i, else 1
z%26+n = (i1+10)+11 = i1+21, which can't equal i2. So q2=1
so new z is (25*q0*q2)+(i*q2)+(o*q2)+q0 = 25*(i1+10) + i2 + 16 + (i1 + 10)
= 26*i1 + i2 + 276

3) (m,n,o) = (1,11,0)
q0=z=25*(i1+10) + i2 + 16 + (i1 + 10)
q2=0 if (z%26)+n is equal to i, else 1
z%26+n = (i2+16)%26+11 = i2+27 (not i2+1 - i2 is at most 9). So it can't equal i3. so q2=1

*/

fn test_expression(program: Program) -> String {
    /*
    doesn't work - runs the first 100+ instructions very quickly but then slows to a crawl.
    (Probably due to all the cloning of complex expressions - but can't find a way to avoid that!)
    let input_vars = vec![
        "d1", "d2", "d3", "d4", "d5", "d6", "d7", "d8", "d9", "d10", "d11", "d12", "d13", "d14",
    ]
    .into_iter()
    .map(|s| String::from(s))
    .collect();
    let result = program.get_result_expression(input_vars);
    result.display()*/
    // use procedure outline above
    let mut z = Expression::Literal(0);
    let ms = vec![1, 1, 1, 1, 26, 26, 1, 26, 1, 26, 1, 26, 26, 26];
    let ns = vec![13, 11, 11, 10, -14, -4, 11, -3, 12, -12, 13, -12, -15, -12];
    let os = vec![10, 16, 0, 13, 7, 11, 11, 10, 16, 8, 15, 2, 5, 10];
    let mut idx = 0;
    while idx < 14 {
        println!("doing step {} of 14", idx + 1);
        let m = ms[idx];
        let n = ns[idx];
        let o = os[idx];
        let var = Expression::Var(format!("d{}", idx + 1));
        let q0 = if m == 1 {
            z.clone()
        } else {
            Expression::Divide(Box::new(z.clone()), Box::new(Expression::Literal(26)))
        };
        let q1 = Expression::IfEqual(
            Box::new(Expression::Add(
                Box::new(Expression::Modulo(
                    Box::new(z.clone()),
                    Box::new(Expression::Literal(26)),
                )),
                Box::new(Expression::Literal(n)),
            )),
            Box::new(var.clone()),
        );
        let q2 = Expression::IfEqual(Box::new(q1), Box::new(Expression::Literal(0)));
        //z = (((((25*q0)*q2)+(i*q2))+(o*q2))+q0)
        z = Expression::Add(
            Box::new(Expression::Add(
                Box::new(Expression::Add(
                    Box::new(Expression::Multiply(
                        Box::new(Expression::Multiply(
                            Box::new(Expression::Literal(25)),
                            Box::new(q0.clone()),
                        )),
                        Box::new(q2.clone()),
                    )),
                    Box::new(Expression::Multiply(Box::new(var), Box::new(q2.clone()))),
                )),
                Box::new(Expression::Multiply(
                    Box::new(Expression::Literal(o)),
                    Box::new(q2),
                )),
            )),
            Box::new(q0),
        );
        idx += 1;
    }
    z.display()
}

fn solve_part_1(program: Program) -> u64 {
    //println!("{}", test_expression(program));
    //obviously not a sensible way to solve!!
    /*let mut res = false;
    let mut num = 100_000_000_000_000;
    while !res {
        num -= 1;
        res = program.test_number(num, 14);
        /*if num % 100 == 0 {
            println!("down to {}", num);
        }*/
    }
    num*/
    //println!("{:?}", program.get_result(vec![9]).0);
    0
}

pub fn part_1() -> u64 {
    let program = read_file();
    solve_part_1(program)
}
