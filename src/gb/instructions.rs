#[derive(PartialEq)]
pub enum JumpTest {
    NotZero,
    Zero,
    NotCarry,
    Carry,
    Always
}

#[derive(PartialEq)]
pub enum ArithmeticTarget {
    A, B, C, D, E, H, L, HL, N8
}

#[derive(PartialEq)]
pub enum LoadByteTarget {
    A, B, C, D, E, H, L, HL, HLI, HLD, DE, BC, D16, ADRC, A8
}

#[derive(PartialEq)]
pub enum LoadByteSource {
    A, B, C, D, E, H, L, D8, HL, HLI, HLD, DE, BC, D16, ADRC, A8
}

#[derive(PartialEq)]
pub enum LoadWordSource {
    D16, SP, HL, SP8
}

#[derive(PartialEq)]
pub enum LoadWordTarget {
    BC, DE, HL, SP, A16
}

#[derive(PartialEq)]
pub enum WordTarget {
    BC, DE, HL, SP
}

#[derive(PartialEq)]
pub enum StackTarget {
    BC
}

#[derive(PartialEq)]
pub enum LoadType {
    Byte(LoadByteTarget, LoadByteSource),
    Word(LoadWordTarget, LoadWordSource),
}

#[derive(PartialEq)]
pub enum Instruction {
    ADD(ArithmeticTarget),
    ADC(ArithmeticTarget),
    SUB(ArithmeticTarget),
    SBC(ArithmeticTarget),
    ADDHL(WordTarget),
    ADDSP,
    JP(JumpTest),
    LD(LoadType),
    PUSH(StackTarget),
    POP(StackTarget),
    CALL(JumpTest),
    RET(JumpTest),
    NOP,
    AND(ArithmeticTarget),
    XOR(ArithmeticTarget),
    OR(ArithmeticTarget),
    CP(ArithmeticTarget)
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
      0x00 => Some((Instruction::NOP, 1)),
      0x01 => Some((Instruction::LD(LoadType::Word(LoadWordTarget::BC, LoadWordSource::D16)), 3)),
      0x02 => Some((Instruction::LD(LoadType::Byte(LoadByteTarget::BC, LoadByteSource::A)), 2)),
      0x06 => Some((Instruction::LD(LoadType::Byte(LoadByteTarget::B, LoadByteSource::D8)), 2)),
      0x08 => Some((Instruction::LD(LoadType::Word(LoadWordTarget::A16, LoadWordSource::SP)), 5)),
      0x09 => Some((Instruction::ADDHL(WordTarget::BC), 2)),
      0x0A => Some((Instruction::LD(LoadType::Byte(LoadByteTarget::A, LoadByteSource::BC)), 2)),
      0x0E => Some((Instruction::LD(LoadType::Byte(LoadByteTarget::C, LoadByteSource::D8)), 2)),

      0x11 => Some((Instruction::LD(LoadType::Word(LoadWordTarget::DE, LoadWordSource::D16)), 3)),
      0x12 => Some((Instruction::LD(LoadType::Byte(LoadByteTarget::DE, LoadByteSource::A)), 2)),
      0x16 => Some((Instruction::LD(LoadType::Byte(LoadByteTarget::D, LoadByteSource::D8)), 2)),
      0x19 => Some((Instruction::ADDHL(WordTarget::DE), 2)),
      0x1A => Some((Instruction::LD(LoadType::Byte(LoadByteTarget::A, LoadByteSource::DE)), 2)),
      0x1E => Some((Instruction::LD(LoadType::Byte(LoadByteTarget::E, LoadByteSource::D8)), 2)),

      0x21 => Some((Instruction::LD(LoadType::Word(LoadWordTarget::HL, LoadWordSource::D16)), 3)),
      0x22 => Some((Instruction::LD(LoadType::Byte(LoadByteTarget::HLI, LoadByteSource::A)), 2)),
      0x26 => Some((Instruction::LD(LoadType::Byte(LoadByteTarget::H, LoadByteSource::D8)), 2)),
      0x29 => Some((Instruction::ADDHL(WordTarget::HL), 2)),
      0x2A => Some((Instruction::LD(LoadType::Byte(LoadByteTarget::A, LoadByteSource::HLI)), 2)),
      0x2E => Some((Instruction::LD(LoadType::Byte(LoadByteTarget::L, LoadByteSource::D8)), 2)),

      0x31 => Some((Instruction::LD(LoadType::Word(LoadWordTarget::SP, LoadWordSource::D16)), 3)),
      0x32 => Some((Instruction::LD(LoadType::Byte(LoadByteTarget::HLI, LoadByteSource::A)), 2)),
      0x36 => Some((Instruction::LD(LoadType::Byte(LoadByteTarget::HL, LoadByteSource::D8)), 3)), 
      0x39 => Some((Instruction::ADDHL(WordTarget::SP), 2)),
      0x3A => Some((Instruction::LD(LoadType::Byte(LoadByteTarget::A, LoadByteSource::HLD)), 2)),
      0x3E => Some((Instruction::LD(LoadType::Byte(LoadByteTarget::A, LoadByteSource::D8)), 2)),

      0x40 => Some((Instruction::LD(LoadType::Byte(LoadByteTarget::B, LoadByteSource::B)), 1)),
      0x41 => Some((Instruction::LD(LoadType::Byte(LoadByteTarget::B, LoadByteSource::C)), 1)),
      0x42 => Some((Instruction::LD(LoadType::Byte(LoadByteTarget::B, LoadByteSource::D)), 1)),
      0x43 => Some((Instruction::LD(LoadType::Byte(LoadByteTarget::B, LoadByteSource::E)), 1)),
      0x44 => Some((Instruction::LD(LoadType::Byte(LoadByteTarget::B, LoadByteSource::H)), 1)),
      0x45 => Some((Instruction::LD(LoadType::Byte(LoadByteTarget::B, LoadByteSource::L)), 1)),
      0x46 => Some((Instruction::LD(LoadType::Byte(LoadByteTarget::B, LoadByteSource::HL)), 2)),
      0x47 => Some((Instruction::LD(LoadType::Byte(LoadByteTarget::B, LoadByteSource::A)), 1)),
      0x48 => Some((Instruction::LD(LoadType::Byte(LoadByteTarget::C, LoadByteSource::B)), 1)),
      0x49 => Some((Instruction::LD(LoadType::Byte(LoadByteTarget::C, LoadByteSource::C)), 1)),
      0x4A => Some((Instruction::LD(LoadType::Byte(LoadByteTarget::C, LoadByteSource::D)), 1)),
      0x4B => Some((Instruction::LD(LoadType::Byte(LoadByteTarget::C, LoadByteSource::E)), 1)),
      0x4C => Some((Instruction::LD(LoadType::Byte(LoadByteTarget::C, LoadByteSource::H)), 1)),
      0x4D => Some((Instruction::LD(LoadType::Byte(LoadByteTarget::C, LoadByteSource::L)), 1)),
      0x4E => Some((Instruction::LD(LoadType::Byte(LoadByteTarget::C, LoadByteSource::HL)), 1)),
      0x4F => Some((Instruction::LD(LoadType::Byte(LoadByteTarget::C, LoadByteSource::A)), 1)),

      0x50 => Some((Instruction::LD(LoadType::Byte(LoadByteTarget::D, LoadByteSource::B)), 1)),
      0x51 => Some((Instruction::LD(LoadType::Byte(LoadByteTarget::D, LoadByteSource::C)), 1)),
      0x52 => Some((Instruction::LD(LoadType::Byte(LoadByteTarget::D, LoadByteSource::D)), 1)),
      0x53 => Some((Instruction::LD(LoadType::Byte(LoadByteTarget::D, LoadByteSource::E)), 1)),
      0x54 => Some((Instruction::LD(LoadType::Byte(LoadByteTarget::D, LoadByteSource::H)), 1)),
      0x55 => Some((Instruction::LD(LoadType::Byte(LoadByteTarget::D, LoadByteSource::L)), 1)),
      0x56 => Some((Instruction::LD(LoadType::Byte(LoadByteTarget::D, LoadByteSource::HL)), 2)),
      0x57 => Some((Instruction::LD(LoadType::Byte(LoadByteTarget::D, LoadByteSource::A)), 1)),
      0x58 => Some((Instruction::LD(LoadType::Byte(LoadByteTarget::E, LoadByteSource::B)), 1)),
      0x59 => Some((Instruction::LD(LoadType::Byte(LoadByteTarget::E, LoadByteSource::C)), 1)),
      0x5A => Some((Instruction::LD(LoadType::Byte(LoadByteTarget::E, LoadByteSource::D)), 1)),
      0x5B => Some((Instruction::LD(LoadType::Byte(LoadByteTarget::E, LoadByteSource::E)), 1)),
      0x5C => Some((Instruction::LD(LoadType::Byte(LoadByteTarget::E, LoadByteSource::H)), 1)),
      0x5D => Some((Instruction::LD(LoadType::Byte(LoadByteTarget::E, LoadByteSource::L)), 1)),
      0x5E => Some((Instruction::LD(LoadType::Byte(LoadByteTarget::E, LoadByteSource::HL)), 1)),
      0x5F => Some((Instruction::LD(LoadType::Byte(LoadByteTarget::E, LoadByteSource::A)), 1)),

      0x60 => Some((Instruction::LD(LoadType::Byte(LoadByteTarget::H, LoadByteSource::B)), 1)),
      0x61 => Some((Instruction::LD(LoadType::Byte(LoadByteTarget::H, LoadByteSource::C)), 1)),
      0x62 => Some((Instruction::LD(LoadType::Byte(LoadByteTarget::H, LoadByteSource::D)), 1)),
      0x63 => Some((Instruction::LD(LoadType::Byte(LoadByteTarget::H, LoadByteSource::E)), 1)),
      0x64 => Some((Instruction::LD(LoadType::Byte(LoadByteTarget::H, LoadByteSource::H)), 1)),
      0x65 => Some((Instruction::LD(LoadType::Byte(LoadByteTarget::H, LoadByteSource::L)), 1)),
      0x66 => Some((Instruction::LD(LoadType::Byte(LoadByteTarget::H, LoadByteSource::HL)), 2)),
      0x67 => Some((Instruction::LD(LoadType::Byte(LoadByteTarget::H, LoadByteSource::A)), 1)),
      0x68 => Some((Instruction::LD(LoadType::Byte(LoadByteTarget::L, LoadByteSource::B)), 1)),
      0x69 => Some((Instruction::LD(LoadType::Byte(LoadByteTarget::L, LoadByteSource::C)), 1)),
      0x6A => Some((Instruction::LD(LoadType::Byte(LoadByteTarget::L, LoadByteSource::D)), 1)),
      0x6B => Some((Instruction::LD(LoadType::Byte(LoadByteTarget::L, LoadByteSource::E)), 1)),
      0x6C => Some((Instruction::LD(LoadType::Byte(LoadByteTarget::L, LoadByteSource::H)), 1)),
      0x6D => Some((Instruction::LD(LoadType::Byte(LoadByteTarget::L, LoadByteSource::L)), 1)),
      0x6E => Some((Instruction::LD(LoadType::Byte(LoadByteTarget::L, LoadByteSource::HL)), 1)),
      0x6F => Some((Instruction::LD(LoadType::Byte(LoadByteTarget::L, LoadByteSource::A)), 1)),

      0x70 => Some((Instruction::LD(LoadType::Byte(LoadByteTarget::HL, LoadByteSource::B)), 1)),
      0x71 => Some((Instruction::LD(LoadType::Byte(LoadByteTarget::HL, LoadByteSource::C)), 1)),
      0x72 => Some((Instruction::LD(LoadType::Byte(LoadByteTarget::HL, LoadByteSource::D)), 1)),
      0x73 => Some((Instruction::LD(LoadType::Byte(LoadByteTarget::HL, LoadByteSource::E)), 1)),
      0x74 => Some((Instruction::LD(LoadType::Byte(LoadByteTarget::HL, LoadByteSource::H)), 1)),
      0x75 => Some((Instruction::LD(LoadType::Byte(LoadByteTarget::HL, LoadByteSource::L)), 1)),
      //0x76 => Some((Instruction::HALT, 1)),
      0x77 => Some((Instruction::LD(LoadType::Byte(LoadByteTarget::HL, LoadByteSource::A)), 1)),
      0x78 => Some((Instruction::LD(LoadType::Byte(LoadByteTarget::A, LoadByteSource::B)), 1)),
      0x79 => Some((Instruction::LD(LoadType::Byte(LoadByteTarget::A, LoadByteSource::C)), 1)),
      0x7A => Some((Instruction::LD(LoadType::Byte(LoadByteTarget::A, LoadByteSource::D)), 1)),
      0x7B => Some((Instruction::LD(LoadType::Byte(LoadByteTarget::A, LoadByteSource::E)), 1)),
      0x7C => Some((Instruction::LD(LoadType::Byte(LoadByteTarget::A, LoadByteSource::H)), 1)),
      0x7D => Some((Instruction::LD(LoadType::Byte(LoadByteTarget::A, LoadByteSource::L)), 1)),
      0x7E => Some((Instruction::LD(LoadType::Byte(LoadByteTarget::A, LoadByteSource::HL)), 1)),
      0x7F => Some((Instruction::LD(LoadType::Byte(LoadByteTarget::A, LoadByteSource::A)), 1)),

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

      0x90 => Some((Instruction::SUB(ArithmeticTarget::B), 1)),
      0x91 => Some((Instruction::SUB(ArithmeticTarget::C), 1)),
      0x92 => Some((Instruction::SUB(ArithmeticTarget::D), 1)),
      0x93 => Some((Instruction::SUB(ArithmeticTarget::E), 1)),
      0x94 => Some((Instruction::SUB(ArithmeticTarget::H), 1)),
      0x95 => Some((Instruction::SUB(ArithmeticTarget::L), 1)),
      0x96 => Some((Instruction::SUB(ArithmeticTarget::HL), 2)),
      0x97 => Some((Instruction::SUB(ArithmeticTarget::A), 1)),
      0x98 => Some((Instruction::SBC(ArithmeticTarget::B), 1)),
      0x99 => Some((Instruction::SBC(ArithmeticTarget::C), 1)),
      0x9A => Some((Instruction::SBC(ArithmeticTarget::D), 1)),
      0x9B => Some((Instruction::SBC(ArithmeticTarget::E), 1)),
      0x9C => Some((Instruction::SBC(ArithmeticTarget::H), 1)),
      0x9D => Some((Instruction::SBC(ArithmeticTarget::L), 1)),
      0x9E => Some((Instruction::SBC(ArithmeticTarget::HL), 2)),
      0xDE => Some((Instruction::SBC(ArithmeticTarget::N8), 2)),
      0x9F => Some((Instruction::SBC(ArithmeticTarget::A), 1)),

      0xA0 => Some((Instruction::SUB(ArithmeticTarget::B), 1)),
      0xA1 => Some((Instruction::SUB(ArithmeticTarget::C), 1)),
      0xA2 => Some((Instruction::SUB(ArithmeticTarget::D), 1)),
      0xA3 => Some((Instruction::SUB(ArithmeticTarget::E), 1)),
      0xA4 => Some((Instruction::SUB(ArithmeticTarget::H), 1)),
      0xA5 => Some((Instruction::SUB(ArithmeticTarget::L), 1)),
      0xA6 => Some((Instruction::SUB(ArithmeticTarget::HL), 2)),
      0xA7 => Some((Instruction::SUB(ArithmeticTarget::A), 1)),
      0xA8 => Some((Instruction::XOR(ArithmeticTarget::B), 1)),
      0xA9 => Some((Instruction::XOR(ArithmeticTarget::C), 1)),
      0xAA => Some((Instruction::XOR(ArithmeticTarget::D), 1)),
      0xAB => Some((Instruction::XOR(ArithmeticTarget::E), 1)),
      0xAC => Some((Instruction::XOR(ArithmeticTarget::H), 1)),
      0xAD => Some((Instruction::XOR(ArithmeticTarget::L), 1)),
      0xAE => Some((Instruction::XOR(ArithmeticTarget::HL), 2)),
      0xAF => Some((Instruction::XOR(ArithmeticTarget::A), 1)),

      0xB0 => Some((Instruction::OR(ArithmeticTarget::B), 1)),
      0xB1 => Some((Instruction::OR(ArithmeticTarget::C), 1)),
      0xB2 => Some((Instruction::OR(ArithmeticTarget::D), 1)),
      0xB3 => Some((Instruction::OR(ArithmeticTarget::E), 1)),
      0xB4 => Some((Instruction::OR(ArithmeticTarget::H), 1)),
      0xB5 => Some((Instruction::OR(ArithmeticTarget::L), 1)),
      0xB6 => Some((Instruction::OR(ArithmeticTarget::HL), 2)),
      0xB7 => Some((Instruction::OR(ArithmeticTarget::A), 1)),
      0xB8 => Some((Instruction::CP(ArithmeticTarget::B), 1)),
      0xB9 => Some((Instruction::CP(ArithmeticTarget::C), 1)),
      0xBA => Some((Instruction::CP(ArithmeticTarget::D), 1)),
      0xBB => Some((Instruction::CP(ArithmeticTarget::E), 1)),
      0xBC => Some((Instruction::CP(ArithmeticTarget::H), 1)),
      0xBD => Some((Instruction::CP(ArithmeticTarget::L), 1)),
      0xBE => Some((Instruction::CP(ArithmeticTarget::HL), 2)),
      0xBF => Some((Instruction::CP(ArithmeticTarget::A), 1)),

      0xC6 => Some((Instruction::ADD(ArithmeticTarget::N8), 2)),
      0xCE => Some((Instruction::ADC(ArithmeticTarget::N8), 2)),

      0xD6 => Some((Instruction::SUB(ArithmeticTarget::N8), 2)),
      0xDE => Some((Instruction::SBC(ArithmeticTarget::N8), 2)),

      0xE0 => Some((Instruction::LD(LoadType::Byte(LoadByteTarget::A8, LoadByteSource::A)), 2)),
      0xE2 => Some((Instruction::LD(LoadType::Byte(LoadByteTarget::ADRC, LoadByteSource::A)), 2)),
      0xE6 => Some((Instruction::AND(ArithmeticTarget::N8), 2)),      
      0xE8 => Some((Instruction::ADDSP, 4)),
      0xEA => Some((Instruction::LD(LoadType::Byte(LoadByteTarget::D16, LoadByteSource::A)), 4)),
      0xEE => Some((Instruction::XOR(ArithmeticTarget::N8), 2)),

      0xF0 => Some((Instruction::LD(LoadType::Byte(LoadByteTarget::A, LoadByteSource::A8)), 2)),
      0xF2 => Some((Instruction::LD(LoadType::Byte(LoadByteTarget::A, LoadByteSource::ADRC)), 2)),
      0xF6 => Some((Instruction::OR(ArithmeticTarget::N8), 2)),
      0xF8 => Some((Instruction::LD(LoadType::Word(LoadWordTarget::HL, LoadWordSource::SP8)), 4)),
      0xF9 => Some((Instruction::LD(LoadType::Word(LoadWordTarget::SP, LoadWordSource::HL)), 2)),
      0xFA => Some((Instruction::LD(LoadType::Byte(LoadByteTarget::A, LoadByteSource::D16)), 4)),
      0xFE => Some((Instruction::CP(ArithmeticTarget::N8), 2)),

      _ => None
    }
  }
}
