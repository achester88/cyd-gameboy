use super::ram::{MemoryBus};
use super::registers::{Registers};
use super::instructions::*;

#[derive(Debug)]
struct Cpu {
   registers: Registers,
   pc: u16,
   sp: u16,
   bus: MemoryBus,
}

impl Cpu {
    fn step(&mut self) {
        let mut instruction_byte = self.bus.read_byte(self.pc);
        let prefixed = instruction_byte == 0xCB;
        if prefixed {
            instruction_byte = self.bus.read_byte(self.pc + 1);
        }

        let next_pc = if let Some(instruction) = Instruction::from_byte(instruction_byte, prefixed) {
            self.execute(instruction)
        } else {
            let desc = format!("0x{}{:x}", if prefixed {"cb"} else {""}, instruction_byte);
            panic!("Unknown instruction found for: {}", desc);  
        };

        self.pc = next_pc;
    }

    fn execute(&mut self, instruction: Instruction) -> u16 {
        match instruction {
           Instruction::ADD(target) => {
               self.registers.a = match target {
                   ArithmeticTarget::A => self.add(self.registers.a),
                   ArithmeticTarget::B => self.add(self.registers.b),
                   ArithmeticTarget::C => self.add(self.registers.c),
                   ArithmeticTarget::D => self.add(self.registers.d),
                   ArithmeticTarget::E => self.add(self.registers.e),
                   ArithmeticTarget::H => self.add(self.registers.h),
                   ArithmeticTarget::L => self.add(self.registers.l),
                };

               self.pc.wrapping_add(1)
                
           },
           Instruction::JP(test) => {
               let jump_cond = match test {
                   JumpTest::NotZero => !self.registers.f.zero,
                   JumpTest::NotCarry => !self.registers.f.carry,
                   JumpTest::Zero => self.registers.f.zero,
                   JumpTest::Carry => self.registers.f.carry,
                   JumpTest::Always => true
               };
               self.jump(jump_cond)
           },
           Instruction::LD(load_type) => {
               match load_type {
                    LoadType::Byte(target, source) => {
                        let source_value = match source {
                            LoadByteSource::A => self.registers.a,
                            LoadByteSource::D8 => self.read_next_byte(),
                            LoadByteSource::HLI => self.bus.read_byte(self.registers.get_hl()),
                            _ => { todo!() }
                        };

                        match target {
                            LoadByteSource::A => self.registers.a = source_value,
                            LoadByteSource::D8 => self.bus.write_byte(self.registers.get_hl(), source_value),
                            _ => { todo!() }
                        };

                        match source {
                            LoadByteSource::D8 => self.pc.wrapping_add(2),
                            _                  => self.pc.wrapping_add(1),
                        }
                    }
               }
           },
           Instruction::PUSH(target) => {
               let value = match target {
                   StackTarget::BC => { self.registers::get_dr(self.registers.b, self.registers.c) },
                   _ => todo!()
               };
               self.push(value);
               self.pc.wrapping_add(1)
           }
           Instruction::POP(target) => {
               let result = self.pop();
               match target {
                   StackTarget::BC => self.registers.set_bc(result),
                   _ => todo!()
               }
               self.pc.wrapping_add(1)
           },
           Instruction::CALL(test) => {
               let jump_cond = match test {
                   JumpTest::NotZero => !self.registers.f.zero,
                   _ => { todo!() }
               };
               self.call(jump_cond)
           }
           Instruction::RET(test) => {
               let jump_cond = match test {
                   JumpTest::NotZero => !self.registers.f.zero,
                   _ => todo!()
               };
               self.return_(jump_cond)
           }
           //_ => {}
        }
    }

    //##########################################################################
    //
    //                       ##    Functions   ##
    //
    //##########################################################################

    fn call(&mut self, should_jump: bool) -> u16 {
        let next_pc = self.pc.wrapping_add(3);
        if should_jump {
            self.push(next_pc);
            self.read_next_word()
        } else {
            next_pc
        }
    }

    fn return_(&mut self, should_jump: bool) -> u16 {
        if should_jump {
            self.pop()
        } else {
            self.pc.wrapping_add(1)
        }
    }

    fn pop(&mut self) -> u16 {
        let lsb = self.bus.read_byte(self.sp) as u16;
        self.sp = self.sp.wrapping_add(1);

        let msb = self.bus.read_byte(self.sp) as u16;
        self.sp = self.sp.wrapping_add(1);

        (msb << 8) | lsb
    }

    fn push(&mut self, value: u16) {
        self.sp = self.sp.wrapping_sub(1);
        self.bus.write_byte(self.sp, (value & 0xFF00) >> 8) as u8;

        self.sp = self.sp.wrapping_sub(1);
        self.bus.write_byte(self.sp, (value & 0xFF) as u8);
    }

    fn jump(&self, should_jump: bool) -> u16 {
        if should_jump {
            let least_significant_byte = self.bus.read_byte(self.pc + 1) as u16;
            let most_significant_byte = self.bus.read_byte(self.pc + 2) as u16;
            
            (most_significant_byte << 8) | least_significant_byte
        } else {
            self.pc.wrapping_add(3)
        }
    }

    fn add(&mut self, val: u8) -> u8 {
        let (new_value, did_overflow) = self.registers.a.overflowing_add(val);

        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.carry = did_overflow;
        self.registers.f.half_carry = (self.registers.a & 0b1111) + (val & 0b1111) > 0b1111;

        new_value
    }
}
