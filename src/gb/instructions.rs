pub enum JumpTest {
    NotZero,
    Zero,
    NotCarry,
    Carry,
    Always
}

pub enum ArithmeticTarget {
    A, B, C, D, E, H, L,
}

pub enum LoadByteTarget {
    A, B, C, D, E, H, L, HLI
}

pub enum LoadByteSource {
    A, B, C, D, E, H, L, HLI
}

pub enum StackTarget {
    BC
}

pub enum LoadType {
    Byte(LoadByteTarget, LoadByteSource)
}

pub enum Instruction {
    ADD(ArithmeticTarget),
    //ADDHL,
    JP(JumpTest),
    LD(LoadType),
    PUSH(StackTarget),
    POP(StackTarget),
    CALL(JumpTest),
    RET(JumpTest),
}

impl Instruction {
  pub fn from_byte(byte: u8, prefixed: bool) -> Option<Instruction> {
    if prefixed {
      Instruction::from_byte_prefixed(byte)
    } else {
      Instruction::from_byte_not_prefixed(byte)
    }
  }

  fn from_byte_prefixed(byte: u8) -> Option<Instruction> {
    match byte {
      //0x00 => Some(Instruction::RLC(PrefixTarget::B)),
      _ => None
    }
  }

  fn from_byte_not_prefixed(byte: u8) -> Option<Instruction> {
    match byte {
      //0x02 => Some(Instruction::INC(IncDecTarget::BC)),
      _ => None
    }
  }
}
