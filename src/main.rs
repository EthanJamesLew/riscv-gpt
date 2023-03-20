use goblin::elf::Elf;
use std::env;
use std::fs;

const MEMORY_SIZE: usize = 1024 * 1024; // 1MB
const REGISTER_COUNT: usize = 32;
const ENTRY_POINT: u32 = 0x1000;

struct Emulator {
    memory: [u8; MEMORY_SIZE],
    registers: [u32; REGISTER_COUNT],
    pc: u32,
}

impl Emulator {
    fn new() -> Self {
        Self {
            memory: [0; MEMORY_SIZE],
            registers: [0; REGISTER_COUNT],
            pc: ENTRY_POINT,
        }
    }

    fn load_program(&mut self, binary_data: &[u8]) -> Result<(), String> {
        let elf_file =
            Elf::parse(&binary_data).map_err(|e| format!("Failed to parse ELF file: {}", e))?;

        let mut pc: u64 = 0;
        for ph in &elf_file.program_headers {
            if ph.p_type == goblin::elf::program_header::PT_LOAD {
                let start = ph.p_vaddr as usize;
                let end = start + ph.p_memsz as usize;
                let section_data =
                    &binary_data[ph.p_offset as usize..(ph.p_offset + ph.p_filesz) as usize];

                self.memory[start..end].copy_from_slice(&section_data);

                // Print out the contents of the memory region that was just loaded
                println!("{:x?}", &self.memory[start..end]);

                // Set the program counter to the start of the .text section
                if ph.p_flags & goblin::elf::program_header::PF_X != 0 {
                    pc = ph.p_vaddr;
                }
            }
        }

        // Initialize the program counter
        // TODO: this is hard coded for now
        self.pc = 0x10078;

        Ok(())
    }

    fn fetch(&mut self) -> u32 {
        // Read a 32-bit instruction from memory at the current program counter
        let instruction = u32::from_le_bytes([
            self.memory[self.pc as usize],
            self.memory[(self.pc + 1) as usize],
            self.memory[(self.pc + 2) as usize],
            self.memory[(self.pc + 3) as usize],
        ]);
        // Increment the program counter by 4
        self.pc += 4;
        instruction
    }

    fn decode_execute(&mut self, instruction: u32) {
        println!("{:x?}, pc {:x?}", instruction & 0x7f, self.pc);
        // Decode and execute the instruction
        match instruction & 0x7f {
            // ADD rd, rs1, rs2
            0x33 => {
                let rd = ((instruction >> 7) & 0x1f) as usize;
                let rs1 = ((instruction >> 15) & 0x1f) as usize;
                let rs2 = ((instruction >> 20) & 0x1f) as usize;
                self.registers[rd] = self.registers[rs1] + self.registers[rs2];

                println!("ADD: {:?} from {rs1} and {rs2}", self.registers[rd])
            }

            // ADDI 
            0x13 => {
                let rd = ((instruction >> 7) & 0x1f) as usize;
                let funct3 = ((instruction >> 12) & 0x7) as u32;
            
                match funct3 {
                    // ADDI rd, rs1, imm
                    0x0 => {
                        let rs1 = ((instruction >> 15) & 0x1f) as usize;
                        let imm = ((instruction >> 20) as i32) << 20 >> 20;
                        self.registers[rd] = self.registers[rs1].wrapping_add(imm as u32);
            
                        println!("ADDI: added {imm} to {rs1} and stored result in {rd}");
                    }
                    // MV rd, rs1
                    0x1 => {
                        let rs1 = ((instruction >> 15) & 0x1f) as usize;
                        self.registers[rd] = self.registers[rs1];
            
                        println!("MV: moved contents of {rs1} to {rd}");
                    }
                    _ => panic!("Unknown instruction: {:08x}", instruction),
                }
            }

            // BEQZ rs1, offset
            0x63 => {
                let imm = ((instruction >> 8) & 0xfff) as i32;
                let rs1 = ((instruction >> 15) & 0x1f) as usize;
                if self.registers[rs1] == 0 {
                    let offset = (imm << 20) >> 20;
                    self.pc = (self.pc as i32 + offset) as u32;
                    println!("BEQZ: jumped to {:x?} (offset {})", self.pc, offset);
                } else {
                    println!("BEQZ: did not jump");
                }
            }

            // JAL rd, imm
            0x6f => {
                let rd = ((instruction >> 7) & 0x1f) as usize;
                let offset = (((instruction >> 31) as i32) << 31 >> 21) | (((instruction >> 21) & 0x3ff) as i32) << 1 >> 1;
                let return_address = self.pc + 4;
                self.registers[rd] = return_address as u32;
                self.pc = (self.pc as i32 + offset * 2 - 4) as u32;
                println!("JAL: jumped to {:x?} (offset {})", self.pc, offset);
            }

            // ECALL
            0x73 => {
                // The ecall instruction is used to call operating system services
                // In this emulator, we simply exit the program when an ecall instruction is encountered
                std::process::exit(0);
            }

            // Unknown instruction
            _ => panic!("Unknown instruction: {:08x}", instruction),
        }
    }
}

fn main() {
    // Read command-line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: cargo run <binary>");
        std::process::exit(1);
    }
    let binary_path = &args[1];

    // Load program from binary file
    let binary_data = fs::read(binary_path).expect("Failed to read binary file");

    let mut emulator = Emulator::new();
    let _program = emulator.load_program(&binary_data);

    // Run the program
    loop {
        let instruction = emulator.fetch();
        emulator.decode_execute(instruction);
        if instruction == 0x73 {
            // ecall instruction halts the program
            break;
        }
    }
    // Print the result (11)
    println!("{}", emulator.registers[10]);
}
