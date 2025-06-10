pub enum JumpTest {
    NotZero,
    Zero,
    NotCarry,
    Carry,
    Always
}

pub enum ArithmeticTarget {
    A, B, C, D, E, H, L, HL, N8
}

pub enum LoadByteTarget {
    A, B, C, D, E, H, L, HLI
}

pub enum LoadByteSource {
    A, B, C, D, E, H, L, D8, HLI
}

pub enum WordTarget {
    BC, DE, HL, SP
}

pub enum StackTarget {
    BC
}

pub enum LoadType {
    Byte(LoadByteTarget, LoadByteSource)
}

pub enum Instruction {
    ADD(ArithmeticTarget),
    ADC(ArithmeticTarget),
    ADDHL(WordTarget),
    ADDSP,
    JP(JumpTest),
    LD(LoadType),
    PUSH(StackTarget),
    POP(StackTarget),
    CALL(JumpTest),
    RET(JumpTest),

    NOP
}

impl Instruction {
  pub fn from_byte(byte: u8, prefixed: bool) -> Option<(Instruction, u8)> {
    if prefixed {
      Instruction::from_byte_prefixed(byte)
    } else {
      Instruction::from_byte_not_prefixed(byte)
    }
  }

  fn from_byte_prefixed(byte: u8) -> Option<(Instruction, u8)> {
    match byte {
      //0x00 => Some(Instruction::RLC(PrefixTarget::B)),
      _ => None
    }
  }

  fn from_byte_not_prefixed(byte: u8) -> Option<(Instruction, u8)> {
    match byte {
      0x00 => Some((Instruction::NOP, 4)),

      0x09 => Some((Instruction::ADDHL(WordTarget::BC), 2)),

      0x19 => Some((Instruction::ADDHL(WordTarget::DE), 2)),

      0x29 => Some((Instruction::ADDHL(WordTarget::HL), 2)),

      0x39 => Some((Instruction::ADDHL(WordTarget::SP), 2)),

      0x80 => Some((Instruction::ADD(ArithmeticTarget::B), 1)),
      0x81 => Some((Instruction::ADD(ArithmeticTarget::C), 1)),
      0x82 => Some((Instruction::ADD(ArithmeticTarget::D), 1)),
      0x83 => Some((Instruction::ADD(ArithmeticTarget::E), 1)),
      0x84 => Some((Instruction::ADD(ArithmeticTarget::H), 1)),
      0x85 => Some((Instruction::ADD(ArithmeticTarget::L), 1)),
      0x86 => Some((Instruction::ADD(ArithmeticTarget::HL), 2)),
      0x87 => Some((Instruction::ADD(ArithmeticTarget::A), 1)),
      0x88 => Some((Instruction::ADC(ArithmeticTarget::B), 1)),
      0x89 => Some((Instruction::ADC(ArithmeticTarget::C), 1)),
      0x8A => Some((Instruction::ADC(ArithmeticTarget::D), 1)),
      0x8B => Some((Instruction::ADC(ArithmeticTarget::E), 1)),
      0x8C => Some((Instruction::ADC(ArithmeticTarget::H), 1)),
      0x8D => Some((Instruction::ADC(ArithmeticTarget::L), 1)),
      0x8E => Some((Instruction::ADC(ArithmeticTarget::HL), 2)),
      0x8F => Some((Instruction::ADC(ArithmeticTarget::A), 1)),

      0xC6 => Some((Instruction::ADC(ArithmeticTarget::N8), 2)),
      0xCE => Some((Instruction::ADC(ArithmeticTarget::N8), 2)),
      
      0xE8 => Some((Instruction::ADDSP, 4)),
      _ => None
    }
  }
}
