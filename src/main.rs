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
        use goblin::{elf::program_header::*, elf::section_header::*};

        let elf = goblin::elf::Elf::parse(binary_data)
            .map_err(|e| format!("Failed to parse ELF file: {}", e))?;

        // Find the program header and section header for the .text section
        let text_phdr = elf
            .program_headers
            .iter()
            .find(|phdr| phdr.p_type == PT_LOAD && phdr.p_flags == (PF_R | PF_X))
            .ok_or_else(|| "No loadable .text segment found in ELF file".to_string())?;
        let text_shdr = elf
            .section_headers
            .iter()
            .find(|shdr| {
                shdr.sh_type == SHT_PROGBITS && shdr.sh_flags == (SHF_ALLOC | SHF_EXECINSTR).into()
            })
            .ok_or_else(|| "No .text section found in ELF file".to_string())?;

        // Read the contents of the .text section into memory
        let text_offset = text_shdr.sh_offset as usize;
        let text_size = text_shdr.sh_size as usize;
        let text_addr = text_shdr.sh_addr as usize;
        self.memory[text_addr..(text_addr + text_size)]
            .copy_from_slice(&binary_data[text_offset..(text_offset + text_size)]);

        // Set the program counter to the start of the .text section
        self.pc = text_shdr.sh_addr as u32;

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
        println!("{:x?}, pc {:x?}", instruction & 0x7f, self.pc - 4);
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
                let rs1 = ((instruction >> 15) & 0x1f) as usize;
                let imm =
                    (((instruction >> 31) as i32) << 11) | (((instruction >> 7) & 0x1f) as i32);
                let offset = (imm << 19) >> 19; // Sign extend the 12-bit immediate field
                if self.registers[rs1] == 0 {
                    //let offset = (imm << 20) >> 20;
                    self.pc = (self.pc as i32 + offset) as u32;
                    println!("BEQZ: jumped to {:x?} (offset {})", self.pc, offset);
                } else {
                    println!("BEQZ: did not jump");
                }
            }

            // JAL rd, imm
            0x6f => {
                let rd = ((instruction >> 7) & 0x1f) as usize;
                let offset = (((instruction >> 31) as i32) << 31 >> 21)
                    | (((instruction >> 21) & 0x3ff) as i32) << 1 >> 1;
                let return_address = self.pc + 4;
                self.registers[rd] = return_address as u32;
                self.pc = (self.pc as i32 + offset * 2 - 4) as u32;
                println!("JAL: jumped to {:x?} (offset {})", self.pc, offset);
            }

            // SD rs2, offset(rs1)
            0x23 => {
                let rs1 = ((instruction >> 15) & 0x1f) as usize;
                let imm = (((instruction >> 7) & 0x1f) | ((instruction >> 25) << 5)) as i32;
                //let imm = ((((instruction >> 7) & 0x1f) | ((instruction >> 25) << 5)) as i32) << 20 >> 20;
                let rs2 = ((instruction >> 20) & 0x1f) as usize;

                let addr = (self.registers[rs1] as i32).wrapping_add(imm) as usize;
                let value0 = self.registers[rs2] as u32;
                let value1 = self.registers[rs2 + 1] as u32;

                println!(
                    "{:x?} (from {rs1}) with {imm} to {rs2}",
                    self.registers[rs1]
                );

                self.memory[addr..(addr + 4)].copy_from_slice(&value1.to_le_bytes());
                self.memory[(addr + 4)..(addr + 8)].copy_from_slice(&value0.to_le_bytes());

                println!(
                    "SD: stored {} to {}",
                    ((value0 as u64) << 32) | (value1 as u64),
                    addr
                );
            }

            // ECALL
            0x73 => {
                // The ecall instruction is used to call operating system services
                // In this emulator, we simply exit the program when an ecall instruction is encountered
                println!("ECALL");
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
