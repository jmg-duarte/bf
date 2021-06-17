pub struct Compiler<'a> {
    code: &'a str,
    length: usize,
    position: usize,
    instructions: Vec<FoldedInstruction>,
}

impl Compiler<'_> {
    pub fn new(code: &str) -> Compiler {
        Compiler {
            code,
            length: code.len(),
            position: 0,
            instructions: Vec::new(),
        }
    }

    pub fn compile(&mut self) -> Vec<FoldedInstruction> {
        let mut loop_stack: Vec<usize> = Vec::new();
        while self.position < self.length {
            let instruction = self.code.chars().nth(self.position);
            match instruction {
                Some('+') => self.fold_instruction('+', Instruction::Plus),
                Some('-') => self.fold_instruction('-', Instruction::Minus),
                Some('<') => self.fold_instruction('<', Instruction::Left),
                Some('>') => self.fold_instruction('>', Instruction::Right),
                Some('.') => self.fold_instruction('.', Instruction::PutChar),
                Some(',') => self.fold_instruction(',', Instruction::ReadChar),
                Some('[') => {
                    let instruction_position =
                        self.emit_instruction(FoldedInstruction::new(Instruction::JumpIfZero, 0));
                    loop_stack.push(instruction_position);
                }
                Some(']') => {
                    if let Some(last_jmp_zero) = loop_stack.pop() {
                        let close_instruction_position = self.emit_instruction(
                            FoldedInstruction::new(Instruction::JumpIfNotZero, last_jmp_zero),
                        );
                        self.instructions[last_jmp_zero].argument = close_instruction_position;
                    }
                }
                None => panic!("error during compilation"),
                _ => {}
            }
            self.position += 1;
        }
        println!("{}", self.instructions.len());
        self.instructions.clone()
    }

    fn fold_instruction(&mut self, chr: char, instruction: Instruction) {
        let mut instruction_count = 1;
        while self.position < self.length - 1
            && self.code.chars().nth(self.position + 1) == Some(chr)
        {
            instruction_count += 1;
            self.position += 1;
        }
        self.emit_instruction(FoldedInstruction::new(instruction, instruction_count));
    }

    fn emit_instruction(&mut self, fi: FoldedInstruction) -> usize {
        self.instructions.push(fi);
        self.instructions.len() - 1
    }
}

pub struct Machine {
    int_ptr: usize,
    data_ptr: usize,
    memory: [u8; 30000],
    buffer: u8,
    code: Vec<FoldedInstruction>,
    input: Vec<u8>,
    output: Vec<u8>,
}

impl Machine {
    pub fn new(
        code: Vec<FoldedInstruction>,
        input: Vec<u8>,
        output: Vec<u8>,
    ) -> Self {
        Self {
            int_ptr: 0,
            data_ptr: 0,
            memory: [0; 30000],
            buffer: 0,
            code,
            input,
            output,
        }
    }

    pub fn execute(mut self) -> Vec<u8>{
        while self.int_ptr < self.code.len() {
            let folded_inst: FoldedInstruction = self.code[self.int_ptr];
            match folded_inst.instruction {
                Instruction::Plus => self.memory[self.data_ptr] += folded_inst.argument as u8,
                Instruction::Minus => self.memory[self.data_ptr] -= folded_inst.argument as u8,
                Instruction::Right => self.data_ptr += folded_inst.argument as usize,
                Instruction::Left => self.data_ptr -= folded_inst.argument as usize,
                Instruction::PutChar => {
                    for _ in 0..folded_inst.argument {
                        self.put_char()
                    }
                }
                Instruction::ReadChar => {
                    for _ in 0..folded_inst.argument {
                        self.read_char()
                    }
                }
                Instruction::JumpIfZero => {
                    if self.memory[self.data_ptr] == 0 {
                        self.int_ptr = folded_inst.argument as usize;
                        continue;
                    }
                }
                Instruction::JumpIfNotZero => {
                    if self.memory[self.data_ptr] != 0 {
                        self.int_ptr = folded_inst.argument as usize;
                        continue;
                    }
                }
            }
            self.int_ptr += 1;
        }
        self.output
    }

    fn read_char(&mut self) {
        let r = self.input.pop();
        match r {
            Some(byte) => self.memory[self.data_ptr] = byte,
            None => panic!("invalid read"),
        }
    }

    fn put_char(&mut self) {
        self.buffer = self.memory[self.data_ptr];
        self.output.push(self.buffer);
    }
}

#[derive(Clone, Copy)]
pub enum Instruction {
    Plus,
    Minus,
    Right,
    Left,
    PutChar,
    ReadChar,
    JumpIfZero,
    JumpIfNotZero,
}

#[derive(Clone, Copy)]
pub struct FoldedInstruction {
    pub instruction: Instruction,
    pub argument: usize,
}

impl FoldedInstruction {
    pub fn new(i: Instruction, arg: usize) -> FoldedInstruction {
        FoldedInstruction {
            instruction: i,
            argument: arg,
        }
    }
}
