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

        let next_pc = if let Some((instruction, _cc)) = Instruction::from_byte(instruction_byte, prefixed) {
            self.execute(instruction)
        } else {
            let desc = format!("0x{}{:x}", if prefixed {"cb"} else {""}, instruction_byte);
            panic!("Unknown instruction found for: {}", desc);  
        };

        self.pc = next_pc;
    }

    fn execute(&mut self, instruction: Instruction) -> u16 {
        match instruction {
           Instruction::NOP => {
               self.pc.wrapping_add(4)
           },
           Instruction::ADD(ref target) | Instruction::ADC(ref target) => {
               
               let carry = match instruction {
                   Instruction::ADC(_) => true,
                   _ => false
               };

                self.registers.a = match target {
                   ArithmeticTarget::A => self.add(self.registers.a, carry),
                   ArithmeticTarget::B => self.add(self.registers.b, carry),
                   ArithmeticTarget::C => self.add(self.registers.c, carry),
                   ArithmeticTarget::D => self.add(self.registers.d, carry),
                   ArithmeticTarget::E => self.add(self.registers.e, carry),
                   ArithmeticTarget::H => self.add(self.registers.h, carry),
                   ArithmeticTarget::L => self.add(self.registers.l, carry),
                   ArithmeticTarget::HL => self.add(self.bus.read_byte(self.registers.get_hl()), carry),
                   ArithmeticTarget::N8 => self.add(self.read_next_byte(), carry)
                };

                match target {
                    ArithmeticTarget::N8 => self.pc.wrapping_add(2),
                    _ => self.pc.wrapping_add(1)
                }
                
           },
           Instruction::SUB(ref target) | Instruction::SBC(ref target) => {
               
               let carry = match &instruction {
                   &Instruction::SBC(_) => true,
                   _ => false
               };

                self.registers.a = match target {
                   ArithmeticTarget::A => {
                       let old_carry = if carry {self.registers.f.carry} else {false};
                       let val = self.sub(self.registers.a, carry);
                       self.registers.f.carry = old_carry;
                       val
                   },
                   ArithmeticTarget::B => self.sub(self.registers.b, carry),
                   ArithmeticTarget::C => self.sub(self.registers.c, carry),
                   ArithmeticTarget::D => self.sub(self.registers.d, carry),
                   ArithmeticTarget::E => self.sub(self.registers.e, carry),
                   ArithmeticTarget::H => self.sub(self.registers.h, carry),
                   ArithmeticTarget::L => self.sub(self.registers.l, carry),
                   ArithmeticTarget::HL => self.sub(self.bus.read_byte(self.registers.get_hl()), carry),
                   ArithmeticTarget::N8 => self.sub(self.read_next_byte(), carry)
                };

                match target {
                    ArithmeticTarget::N8 => self.pc.wrapping_add(2),
                    _ => self.pc.wrapping_add(1)
                }
                
           },
           Instruction::AND(target) => {
                self.registers.a = match target {
                   ArithmeticTarget::A => self.and(self.registers.a),
                   ArithmeticTarget::B => self.and(self.registers.b),
                   ArithmeticTarget::C => self.and(self.registers.c),
                   ArithmeticTarget::D => self.and(self.registers.d),
                   ArithmeticTarget::E => self.and(self.registers.e),
                   ArithmeticTarget::H => self.and(self.registers.h),
                   ArithmeticTarget::L => self.and(self.registers.l),
                   ArithmeticTarget::HL => self.and(self.bus.read_byte(self.registers.get_hl())),
                   ArithmeticTarget::N8 => self.and(self.read_next_byte())
                };

                match target {
                    ArithmeticTarget::N8 => self.pc.wrapping_add(2),
                    _ => self.pc.wrapping_add(1)
                }
                
           },
           Instruction::OR(ref target) | Instruction::XOR(ref target) => {
               let not = match &instruction {
                   &Instruction::XOR(_) => true,
                   _ => false
               };

                self.registers.a = match target {
                   ArithmeticTarget::A => self.or(self.registers.a, not),
                   ArithmeticTarget::B => self.or(self.registers.b, not),
                   ArithmeticTarget::C => self.or(self.registers.c, not),
                   ArithmeticTarget::D => self.or(self.registers.d, not),
                   ArithmeticTarget::E => self.or(self.registers.e, not),
                   ArithmeticTarget::H => self.or(self.registers.h, not),
                   ArithmeticTarget::L => self.or(self.registers.l, not),
                   ArithmeticTarget::HL => self.or(self.bus.read_byte(self.registers.get_hl()), not),
                   ArithmeticTarget::N8 => self.or(self.read_next_byte(), not)
                };

                match target {
                    ArithmeticTarget::N8 => self.pc.wrapping_add(2),
                    _ => self.pc.wrapping_add(1)
                }
                
           },
           Instruction::CP(target) => {
                self.registers.a = match target {
                   ArithmeticTarget::A => self.sub(self.registers.a, false), //may need to set flags 1100
                   ArithmeticTarget::B => self.sub(self.registers.b, false),
                   ArithmeticTarget::C => self.sub(self.registers.c, false),
                   ArithmeticTarget::D => self.sub(self.registers.d, false),
                   ArithmeticTarget::E => self.sub(self.registers.e, false),
                   ArithmeticTarget::H => self.sub(self.registers.h, false),
                   ArithmeticTarget::L => self.sub(self.registers.l, false),
                   ArithmeticTarget::HL => self.sub(self.bus.read_byte(self.registers.get_hl()), false),
                   ArithmeticTarget::N8 => self.sub(self.read_next_byte(), false)
                };

                match target {
                    ArithmeticTarget::N8 => self.pc.wrapping_add(2),
                    _ => self.pc.wrapping_add(1)
                }
                
           },
           Instruction::ADDHL(target) => {
                let new_value = match target {
                   WordTarget::BC => self.addhl(self.registers.get_bc()),
                   WordTarget::DE => self.addhl(self.registers.get_de()),
                   WordTarget::HL => self.addhl(self.registers.get_hl()),
                   WordTarget::SP => self.addhl(self.sp),
                };

                self.registers.set_hl(new_value);

                self.pc.wrapping_add(1)
           },
           Instruction::ADDSP => {
               self.sp = self.add_sign_to_sp();

               self.pc.wrapping_add(2)
           }
           /*Instruction::ADC(target) => {
                self.registers.a = match target {
                   ArithmeticTarget::A => self.add(self.registers.a, true),
                   ArithmeticTarget::B => self.add(self.registers.b, true),
                   ArithmeticTarget::C => self.add(self.registers.c, true),
                   ArithmeticTarget::D => self.add(self.registers.d, true),
                   ArithmeticTarget::E => self.add(self.registers.e, true),
                   ArithmeticTarget::H => self.add(self.registers.h, true),
                   ArithmeticTarget::L => self.add(self.registers.l, true),
                   ArithmeticTarget::HL => self.add(self.bus.read_byte(self.registers.get_hl()), true),
                   ArithmeticTarget::N8 => self.add(self.read_next_byte(), true)
                };

                 match target {
                    ArithmeticTarget::N8 => self.pc.wrapping_add(2),
                    _ => self.pc.wrapping_add(1)
                }  
           },*/
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
                            LoadByteSource::B => self.registers.b,
                            LoadByteSource::C => self.registers.c,
                            LoadByteSource::D => self.registers.d,
                            LoadByteSource::E => self.registers.e,
                            LoadByteSource::H => self.registers.h,
                            LoadByteSource::L => self.registers.l,
                            LoadByteSource::D8 => self.read_next_byte(),
                            LoadByteSource::HL | LoadByteSource::HLI | LoadByteSource::HLD => self.bus.read_byte(self.registers.get_hl()),
                            LoadByteSource::BC => self.bus.read_byte(self.registers.get_bc()),
                            LoadByteSource::DE => self.bus.read_byte(self.registers.get_de()),
                            LoadByteSource::D16 => self.bus.read_byte(self.read_next_word()),
                            LoadByteSource::ADRC => self.bus.read_byte(0xFF00 | self.registers.c as u16),
                            LoadByteSource::A8 => self.bus.read_byte(0xFF00 | self.read_next_byte() as u16),

                        };

                        match target {
                             LoadByteTarget::A => self.registers.a = source_value,
                             LoadByteTarget::B => self.registers.b = source_value,
                             LoadByteTarget::C => self.registers.c = source_value,
                             LoadByteTarget::D => self.registers.d = source_value,
                             LoadByteTarget::E => self.registers.e = source_value,
                             LoadByteTarget::H => self.registers.h = source_value,
                             LoadByteTarget::L => self.registers.l = source_value,
                             LoadByteTarget::HL | LoadByteTarget::HLI | LoadByteTarget::HLD => self.bus.write_byte(self.registers.get_hl(), source_value),
                             LoadByteTarget::BC => self.bus.write_byte(self.registers.get_bc(), source_value),
                             LoadByteTarget::DE => self.bus.write_byte(self.registers.get_de(), source_value),
                             LoadByteTarget::D16 => self.bus.write_byte(self.read_next_word(), source_value),
                             LoadByteTarget::ADRC => self.bus.write_byte(0xFF00 | self.registers.c as u16, source_value),
                             LoadByteTarget::A8 => self.bus.write_byte(0xFF00 | self.read_next_byte() as u16, source_value),
                        };

                        if source == LoadByteSource::HLI || target == LoadByteTarget::HLI {
                            self.registers.set_hl(self.registers.get_hl().wrapping_add(1));
                        }

                        if source == LoadByteSource::HLD || target == LoadByteTarget::HLD {
                            self.registers.set_hl(self.registers.get_hl().wrapping_sub(1));
                        }


                       let skip = match source {
                            LoadByteSource::D16 => self.pc.wrapping_add(3),
                            LoadByteSource::D8 | LoadByteSource::A8 => self.pc.wrapping_add(2),
                            _                  => self.pc.wrapping_add(1),
                        };

                       match target {
                           LoadByteTarget::D16 => self.pc.wrapping_add(3),
                           LoadByteTarget::A8 => self.pc.wrapping_add(2),
                           _ => skip
                        }
                    },

                    LoadType::Word(target, source) => {
                         let source_value = match source {
                            LoadWordSource::D16 => self.read_next_word(),
                            LoadWordSource::SP => self.sp,
                            LoadWordSource::HL => self.registers.get_hl(),
                            LoadWordSource::SP8 => {
                                //Add the signed value e8 to SP and copy the result in HL.
                                let new_val = self.add_sign_to_sp();
                                self.sp = new_val;
                                new_val
                            },  
                         };

                         match target {
                             LoadWordTarget::BC => self.registers.set_bc(source_value),
                             LoadWordTarget::DE => self.registers.set_de(source_value),
                             LoadWordTarget::HL => self.registers.set_hl(source_value),
                             LoadWordTarget::A16 => self.bus.write_word(self.read_next_word(), source_value),
                             LoadWordTarget::SP => self.sp = source_value,
                         };

                         if target == LoadWordTarget::A16 { return self.pc.wrapping_add(3) }

                         match source {
                             LoadWordSource::D16 => self.pc.wrapping_add(3),
                             LoadWordSource::SP8 => self.pc.wrapping_add(2),
                             _ => self.pc.wrapping_add(1)
                         }
                    }
           }
           },
           Instruction::PUSH(target) => {
               let value = match target {
                   StackTarget::BC => { self.registers.get_bc() },
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
        self.bus.write_byte(self.sp, ((value & 0xFF00) >> 8 as u8).try_into().unwrap());

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

    fn add(&mut self, val: u8, carry: bool) -> u8 {
         let (new_value, did_overflow) = self.registers.a.overflowing_add(val + (if carry {1} else {0}));

        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.carry = did_overflow;
        self.registers.f.half_carry = (self.registers.a & 0b1111) + (val & 0b1111) > 0b1111;

        new_value
   }

    fn sub(&mut self, val: u8, carry: bool) -> u8 {
         let (new_value, did_overflow) = self.registers.a.overflowing_sub(val + (if carry {1} else {0}));

        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = true;
        self.registers.f.carry = did_overflow;
        self.registers.f.half_carry = (self.registers.a & 0b1111) + (val & 0b1111) > 0b1111;

        new_value
   }

    fn addhl(&mut self, val: u16) -> u16 {
        let (new_value, did_overflow) = self.registers.get_hl().overflowing_add(val);

        self.registers.f.subtract = false;
        self.registers.f.carry = did_overflow;
        self.registers.f.half_carry = (self.registers.get_hl() & 0b11111111) + (val & 0b11111111) > 0b11111111;

        new_value
    }

    fn and(&mut self, val: u8) -> u8 {
        let val = self.registers.a & val;
        self.registers.f.zero = if val == 0 {true} else {false};
        self.registers.f.half_carry = true;
        self.registers.f.subtract = false;
        self.registers.f.carry = false;

        val
    }

    fn or(&mut self, val: u8, not: bool) -> u8 {
        let val = if not {self.registers.a ^ val} else {self.registers.a | val};
        self.registers.f.zero = if val == 0 {true} else {false};
        self.registers.f.half_carry = false;
        self.registers.f.subtract = false;
        self.registers.f.carry = false;

        val
    }

    fn read_next_byte(&self) -> u8 {
        self.bus.read_byte(self.pc+1)//wrap???
    }

    fn read_next_word(&self) -> u16 {
       let lsb = self.bus.read_byte(self.pc + 1) as u16;
       let msb = self.bus.read_byte(self.pc + 2) as u16;
            
       (msb << 8) | lsb
    }

    fn add_sign_to_sp(&mut self) -> u16 {
        let val = self.read_next_byte();
        let mut did_overflow: bool;    
        let mut new_value: u16;
        let uval = (val & 0b0111_1111) as u16;

        if val & 0b1000_0000 == 0 { //neg
            (new_value, did_overflow) = self.sp.overflowing_add(uval);
        } else {//pos
            (new_value, did_overflow) = self.sp.overflowing_sub(uval);
        }

        self.registers.f.zero = false;
        self.registers.f.subtract = false;
        self.registers.f.carry = did_overflow;
        self.registers.f.half_carry = (self.sp & 0b1111) + ((val as u16) & 0b1111) > 0b1111;
        
        new_value
    }
}
