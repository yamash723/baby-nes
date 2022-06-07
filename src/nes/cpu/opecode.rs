use once_cell::sync::Lazy;
use std::{collections::HashMap};

#[derive(Debug)]
pub struct Opecode {
    pub code: Code,
    pub mode: AddressingMode,
}

#[derive(Debug, PartialEq)]
pub enum AddressingMode {
    Implied,
    Accumulator,
    Immediate,
    ZeroPage,
    ZeroPageIndexedX,
    ZeroPageIndexedY,
    Absolute,
    AbsoluteIndexedX,
    AbsoluteIndexedY,
    Relative,
    IndexedIndirect,
    IndirectIndexed,
    AbsoluteIndirect,
}

#[derive(Debug)]
pub enum Code {
    BNE,
    DEY,
    INX,
    JMP,
    LDA,
    LDX,
    LDY,
    SEI,
    STA,
    TXS,
}

pub static OPECODE_MAP: Lazy<HashMap<u8, Opecode>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert(0xA9, Opecode { code: Code::LDA, mode: AddressingMode::Immediate });
    m.insert(0xA5, Opecode { code: Code::LDA, mode: AddressingMode::ZeroPage });
    m.insert(0xB5, Opecode { code: Code::LDA, mode: AddressingMode::ZeroPageIndexedX });
    m.insert(0xAD, Opecode { code: Code::LDA, mode: AddressingMode::Absolute });
    m.insert(0xBD, Opecode { code: Code::LDA, mode: AddressingMode::AbsoluteIndexedX });
    m.insert(0xB9, Opecode { code: Code::LDA, mode: AddressingMode::AbsoluteIndexedY });
    m.insert(0xA1, Opecode { code: Code::LDA, mode: AddressingMode::IndexedIndirect });
    m.insert(0xB1, Opecode { code: Code::LDA, mode: AddressingMode::IndirectIndexed });
    m.insert(0xA2, Opecode { code: Code::LDX, mode: AddressingMode::Immediate });
    m.insert(0xA6, Opecode { code: Code::LDX, mode: AddressingMode::ZeroPage });
    m.insert(0xAE, Opecode { code: Code::LDX, mode: AddressingMode::Absolute });
    m.insert(0xB6, Opecode { code: Code::LDX, mode: AddressingMode::ZeroPageIndexedY });
    m.insert(0xBE, Opecode { code: Code::LDX, mode: AddressingMode::AbsoluteIndexedY });
    m.insert(0xA0, Opecode { code: Code::LDY, mode: AddressingMode::Immediate });
    m.insert(0xA4, Opecode { code: Code::LDY, mode: AddressingMode::ZeroPage });
    m.insert(0xAC, Opecode { code: Code::LDY, mode: AddressingMode::Absolute });
    m.insert(0xB4, Opecode { code: Code::LDY, mode: AddressingMode::ZeroPageIndexedX });
    m.insert(0xBC, Opecode { code: Code::LDY, mode: AddressingMode::AbsoluteIndexedX });
    m.insert(0x85, Opecode { code: Code::STA, mode: AddressingMode::ZeroPage });
    m.insert(0x8D, Opecode { code: Code::STA, mode: AddressingMode::Absolute });
    m.insert(0x95, Opecode { code: Code::STA, mode: AddressingMode::ZeroPageIndexedX });
    m.insert(0x9D, Opecode { code: Code::STA, mode: AddressingMode::AbsoluteIndexedX });
    m.insert(0x99, Opecode { code: Code::STA, mode: AddressingMode::AbsoluteIndexedY });
    m.insert(0x81, Opecode { code: Code::STA, mode: AddressingMode::IndexedIndirect });
    m.insert(0x91, Opecode { code: Code::STA, mode: AddressingMode::IndirectIndexed });
    // m.insert(0x86, Opecode { code: Code::STX, mode: AddressingMode::ZeroPage });
    // m.insert(0x8E, Opecode { code: Code::STX, mode: AddressingMode::Absolute });
    // m.insert(0x96, Opecode { code: Code::STX, mode: AddressingMode::ZeroPageIndexedY });
    // m.insert(0x84, Opecode { code: Code::STY, mode: AddressingMode::ZeroPage });
    // m.insert(0x8C, Opecode { code: Code::STY, mode: AddressingMode::Absolute });
    // m.insert(0x94, Opecode { code: Code::STY, mode: AddressingMode::ZeroPageIndexedX });
    // m.insert(0x8A, Opecode { code: Code::TXA, mode: AddressingMode::Implied });
    // m.insert(0x98, Opecode { code: Code::TYA, mode: AddressingMode::Implied });
    m.insert(0x9A, Opecode { code: Code::TXS, mode: AddressingMode::Implied });
    // m.insert(0xA8, Opecode { code: Code::TAY, mode: AddressingMode::Implied });
    // m.insert(0xAA, Opecode { code: Code::TAX, mode: AddressingMode::Implied });
    // m.insert(0xBA, Opecode { code: Code::TSX, mode: AddressingMode::Implied });
    // m.insert(0x08, Opecode { code: Code::PHP, mode: AddressingMode::Implied });
    // m.insert(0x28, Opecode { code: Code::PLP, mode: AddressingMode::Implied });
    // m.insert(0x48, Opecode { code: Code::PHA, mode: AddressingMode::Implied });
    // m.insert(0x68, Opecode { code: Code::PLA, mode: AddressingMode::Implied });
    // m.insert(0x69, Opecode { code: Code::ADC, mode: AddressingMode::Immediate });
    // m.insert(0x65, Opecode { code: Code::ADC, mode: AddressingMode::ZeroPage });
    // m.insert(0x6D, Opecode { code: Code::ADC, mode: AddressingMode::Absolute });
    // m.insert(0x75, Opecode { code: Code::ADC, mode: AddressingMode::ZeroPageIndexedX });
    // m.insert(0x7D, Opecode { code: Code::ADC, mode: AddressingMode::AbsoluteIndexedX });
    // m.insert(0x79, Opecode { code: Code::ADC, mode: AddressingMode::AbsoluteIndexedY });
    // m.insert(0x61, Opecode { code: Code::ADC, mode: AddressingMode::IndexedIndirect });
    // m.insert(0x71, Opecode { code: Code::ADC, mode: AddressingMode::IndirectIndexed });
    // m.insert(0xE9, Opecode { code: Code::SBC, mode: AddressingMode::Immediate });
    // m.insert(0xE5, Opecode { code: Code::SBC, mode: AddressingMode::ZeroPage });
    // m.insert(0xED, Opecode { code: Code::SBC, mode: AddressingMode::Absolute });
    // m.insert(0xF5, Opecode { code: Code::SBC, mode: AddressingMode::ZeroPageIndexedX });
    // m.insert(0xFD, Opecode { code: Code::SBC, mode: AddressingMode::AbsoluteIndexedX });
    // m.insert(0xF9, Opecode { code: Code::SBC, mode: AddressingMode::AbsoluteIndexedY });
    // m.insert(0xE1, Opecode { code: Code::SBC, mode: AddressingMode::IndexedIndirect });
    // m.insert(0xF1, Opecode { code: Code::SBC, mode: AddressingMode::IndirectIndexed });
    // m.insert(0xE0, Opecode { code: Code::CPX, mode: AddressingMode::Immediate });
    // m.insert(0xE4, Opecode { code: Code::CPX, mode: AddressingMode::ZeroPage });
    // m.insert(0xEC, Opecode { code: Code::CPX, mode: AddressingMode::Absolute });
    // m.insert(0xC0, Opecode { code: Code::CPY, mode: AddressingMode::Immediate });
    // m.insert(0xC4, Opecode { code: Code::CPY, mode: AddressingMode::ZeroPage });
    // m.insert(0xCC, Opecode { code: Code::CPY, mode: AddressingMode::Absolute });
    // m.insert(0xC9, Opecode { code: Code::CMP, mode: AddressingMode::Immediate });
    // m.insert(0xC5, Opecode { code: Code::CMP, mode: AddressingMode::ZeroPage });
    // m.insert(0xCD, Opecode { code: Code::CMP, mode: AddressingMode::Absolute });
    // m.insert(0xD5, Opecode { code: Code::CMP, mode: AddressingMode::ZeroPageIndexedX });
    // m.insert(0xDD, Opecode { code: Code::CMP, mode: AddressingMode::AbsoluteIndexedX });
    // m.insert(0xD9, Opecode { code: Code::CMP, mode: AddressingMode::AbsoluteIndexedY });
    // m.insert(0xC1, Opecode { code: Code::CMP, mode: AddressingMode::IndexedIndirect });
    // m.insert(0xD1, Opecode { code: Code::CMP, mode: AddressingMode::IndirectIndexed });
    // m.insert(0x29, Opecode { code: Code::AND, mode: AddressingMode::Immediate });
    // m.insert(0x25, Opecode { code: Code::AND, mode: AddressingMode::ZeroPage });
    // m.insert(0x2D, Opecode { code: Code::AND, mode: AddressingMode::Absolute });
    // m.insert(0x35, Opecode { code: Code::AND, mode: AddressingMode::ZeroPageIndexedX });
    // m.insert(0x3D, Opecode { code: Code::AND, mode: AddressingMode::AbsoluteIndexedX });
    // m.insert(0x39, Opecode { code: Code::AND, mode: AddressingMode::AbsoluteIndexedY });
    // m.insert(0x21, Opecode { code: Code::AND, mode: AddressingMode::IndexedIndirect });
    // m.insert(0x31, Opecode { code: Code::AND, mode: AddressingMode::IndirectIndexed });
    // m.insert(0x49, Opecode { code: Code::EOR, mode: AddressingMode::Immediate });
    // m.insert(0x45, Opecode { code: Code::EOR, mode: AddressingMode::ZeroPage });
    // m.insert(0x4D, Opecode { code: Code::EOR, mode: AddressingMode::Absolute });
    // m.insert(0x55, Opecode { code: Code::EOR, mode: AddressingMode::ZeroPageIndexedX });
    // m.insert(0x5D, Opecode { code: Code::EOR, mode: AddressingMode::AbsoluteIndexedX });
    // m.insert(0x59, Opecode { code: Code::EOR, mode: AddressingMode::AbsoluteIndexedY });
    // m.insert(0x41, Opecode { code: Code::EOR, mode: AddressingMode::IndexedIndirect });
    // m.insert(0x51, Opecode { code: Code::EOR, mode: AddressingMode::IndirectIndexed });
    // m.insert(0x09, Opecode { code: Code::ORA, mode: AddressingMode::Immediate });
    // m.insert(0x05, Opecode { code: Code::ORA, mode: AddressingMode::ZeroPage });
    // m.insert(0x0D, Opecode { code: Code::ORA, mode: AddressingMode::Absolute });
    // m.insert(0x15, Opecode { code: Code::ORA, mode: AddressingMode::ZeroPageIndexedX });
    // m.insert(0x1D, Opecode { code: Code::ORA, mode: AddressingMode::AbsoluteIndexedX });
    // m.insert(0x19, Opecode { code: Code::ORA, mode: AddressingMode::AbsoluteIndexedY });
    // m.insert(0x01, Opecode { code: Code::ORA, mode: AddressingMode::IndexedIndirect });
    // m.insert(0x11, Opecode { code: Code::ORA, mode: AddressingMode::IndirectIndexed });
    // m.insert(0x24, Opecode { code: Code::BIT, mode: AddressingMode::ZeroPage });
    // m.insert(0x2C, Opecode { code: Code::BIT, mode: AddressingMode::Absolute });
    // m.insert(0x0A, Opecode { code: Code::ASL, mode: AddressingMode::Accumulator });
    // m.insert(0x06, Opecode { code: Code::ASL, mode: AddressingMode::ZeroPage });
    // m.insert(0x0E, Opecode { code: Code::ASL, mode: AddressingMode::Absolute });
    // m.insert(0x16, Opecode { code: Code::ASL, mode: AddressingMode::ZeroPageIndexedX });
    // m.insert(0x1E, Opecode { code: Code::ASL, mode: AddressingMode::AbsoluteIndexedX });
    // m.insert(0x4A, Opecode { code: Code::LSR, mode: AddressingMode::Accumulator });
    // m.insert(0x46, Opecode { code: Code::LSR, mode: AddressingMode::ZeroPage });
    // m.insert(0x4E, Opecode { code: Code::LSR, mode: AddressingMode::Absolute });
    // m.insert(0x56, Opecode { code: Code::LSR, mode: AddressingMode::ZeroPageIndexedX });
    // m.insert(0x5E, Opecode { code: Code::LSR, mode: AddressingMode::AbsoluteIndexedX });
    // m.insert(0x2A, Opecode { code: Code::ROL, mode: AddressingMode::Accumulator });
    // m.insert(0x26, Opecode { code: Code::ROL, mode: AddressingMode::ZeroPage });
    // m.insert(0x2E, Opecode { code: Code::ROL, mode: AddressingMode::Absolute });
    // m.insert(0x36, Opecode { code: Code::ROL, mode: AddressingMode::ZeroPageIndexedX });
    // m.insert(0x3E, Opecode { code: Code::ROL, mode: AddressingMode::AbsoluteIndexedX });
    // m.insert(0x6A, Opecode { code: Code::ROR, mode: AddressingMode::Accumulator });
    // m.insert(0x66, Opecode { code: Code::ROR, mode: AddressingMode::ZeroPage });
    // m.insert(0x6E, Opecode { code: Code::ROR, mode: AddressingMode::Absolute });
    // m.insert(0x76, Opecode { code: Code::ROR, mode: AddressingMode::ZeroPageIndexedX });
    // m.insert(0x7E, Opecode { code: Code::ROR, mode: AddressingMode::AbsoluteIndexedX });
    m.insert(0xE8, Opecode { code: Code::INX, mode: AddressingMode::Implied });
    // m.insert(0xC8, Opecode { code: Code::INY, mode: AddressingMode::Implied });
    // m.insert(0xE6, Opecode { code: Code::INC, mode: AddressingMode::ZeroPage });
    // m.insert(0xEE, Opecode { code: Code::INC, mode: AddressingMode::Absolute });
    // m.insert(0xF6, Opecode { code: Code::INC, mode: AddressingMode::ZeroPageIndexedX });
    // m.insert(0xFE, Opecode { code: Code::INC, mode: AddressingMode::AbsoluteIndexedX });
    // m.insert(0xCA, Opecode { code: Code::DEX, mode: AddressingMode::Implied });
    m.insert(0x88, Opecode { code: Code::DEY, mode: AddressingMode::Implied });
    // m.insert(0xC6, Opecode { code: Code::DEC, mode: AddressingMode::ZeroPage });
    // m.insert(0xCE, Opecode { code: Code::DEC, mode: AddressingMode::Absolute });
    // m.insert(0xD6, Opecode { code: Code::DEC, mode: AddressingMode::ZeroPageIndexedX });
    // m.insert(0xDE, Opecode { code: Code::DEC, mode: AddressingMode::AbsoluteIndexedX });
    // m.insert(0x18, Opecode { code: Code::CLC, mode: AddressingMode::Implied });
    // m.insert(0x58, Opecode { code: Code::CLI, mode: AddressingMode::Implied });
    // m.insert(0xB8, Opecode { code: Code::CLV, mode: AddressingMode::Implied });
    // m.insert(0x38, Opecode { code: Code::SEC, mode: AddressingMode::Implied });
    m.insert(0x78, Opecode { code: Code::SEI, mode: AddressingMode::Implied });
    // m.insert(0xEA, Opecode { code: Code::NOP, mode: AddressingMode::Implied });
    // m.insert(0x00, Opecode { code: Code::BRK, mode: AddressingMode::Implied });
    // m.insert(0x20, Opecode { code: Code::JSR, mode: AddressingMode::Absolute });
    m.insert(0x4C, Opecode { code: Code::JMP, mode: AddressingMode::Absolute });
    m.insert(0x6C, Opecode { code: Code::JMP, mode: AddressingMode::AbsoluteIndirect });
    // m.insert(0x40, Opecode { code: Code::RTI, mode: AddressingMode::Implied });
    // m.insert(0x60, Opecode { code: Code::RTS, mode: AddressingMode::Implied });
    // m.insert(0x10, Opecode { code: Code::BPL, mode: AddressingMode::Relative });
    // m.insert(0x30, Opecode { code: Code::BMI, mode: AddressingMode::Relative });
    // m.insert(0x50, Opecode { code: Code::BVC, mode: AddressingMode::Relative });
    // m.insert(0x70, Opecode { code: Code::BVS, mode: AddressingMode::Relative });
    // m.insert(0x90, Opecode { code: Code::BCC, mode: AddressingMode::Relative });
    // m.insert(0xB0, Opecode { code: Code::BCS, mode: AddressingMode::Relative });
    m.insert(0xD0, Opecode { code: Code::BNE, mode: AddressingMode::Relative });
    // m.insert(0xF0, Opecode { code: Code::BEQ, mode: AddressingMode::Relative });
    // m.insert(0xF8, Opecode { code: Code::SED, mode: AddressingMode::Implied });
    // m.insert(0xD8, Opecode { code: Code::CLD, mode: AddressingMode::Implied });
    // m.insert(0x1A, Opecode { code: Code::NOP, mode: AddressingMode::Implied });
    // m.insert(0x3A, Opecode { code: Code::NOP, mode: AddressingMode::Implied });
    // m.insert(0x5A, Opecode { code: Code::NOP, mode: AddressingMode::Implied });
    // m.insert(0x7A, Opecode { code: Code::NOP, mode: AddressingMode::Implied });
    // m.insert(0xDA, Opecode { code: Code::NOP, mode: AddressingMode::Implied });
    // m.insert(0xFA, Opecode { code: Code::NOP, mode: AddressingMode::Implied });
    // m.insert(0x02, Opecode { code: Code::NOP, mode: AddressingMode::Implied });
    // m.insert(0x12, Opecode { code: Code::NOP, mode: AddressingMode::Implied });
    // m.insert(0x22, Opecode { code: Code::NOP, mode: AddressingMode::Implied });
    // m.insert(0x32, Opecode { code: Code::NOP, mode: AddressingMode::Implied });
    // m.insert(0x42, Opecode { code: Code::NOP, mode: AddressingMode::Implied });
    // m.insert(0x52, Opecode { code: Code::NOP, mode: AddressingMode::Implied });
    // m.insert(0x62, Opecode { code: Code::NOP, mode: AddressingMode::Implied });
    // m.insert(0x72, Opecode { code: Code::NOP, mode: AddressingMode::Implied });
    // m.insert(0x92, Opecode { code: Code::NOP, mode: AddressingMode::Implied });
    // m.insert(0xB2, Opecode { code: Code::NOP, mode: AddressingMode::Implied });
    // m.insert(0xD2, Opecode { code: Code::NOP, mode: AddressingMode::Implied });
    // m.insert(0xF2, Opecode { code: Code::NOP, mode: AddressingMode::Implied });
    // m.insert(0x80, Opecode { code: Code::NOP, mode: AddressingMode::Implied });
    // m.insert(0x82, Opecode { code: Code::NOP, mode: AddressingMode::Implied });
    // m.insert(0x89, Opecode { code: Code::NOP, mode: AddressingMode::Implied });
    // m.insert(0xC2, Opecode { code: Code::NOP, mode: AddressingMode::Implied });
    // m.insert(0xE2, Opecode { code: Code::NOP, mode: AddressingMode::Implied });
    // m.insert(0x04, Opecode { code: Code::NOP, mode: AddressingMode::Implied });
    // m.insert(0x44, Opecode { code: Code::NOP, mode: AddressingMode::Implied });
    // m.insert(0x64, Opecode { code: Code::NOP, mode: AddressingMode::Implied });
    // m.insert(0x14, Opecode { code: Code::NOP, mode: AddressingMode::Implied });
    // m.insert(0x34, Opecode { code: Code::NOP, mode: AddressingMode::Implied });
    // m.insert(0x54, Opecode { code: Code::NOP, mode: AddressingMode::Implied });
    // m.insert(0x74, Opecode { code: Code::NOP, mode: AddressingMode::Implied });
    // m.insert(0xD4, Opecode { code: Code::NOP, mode: AddressingMode::Implied });
    // m.insert(0xF4, Opecode { code: Code::NOP, mode: AddressingMode::Implied });
    // m.insert(0x0C, Opecode { code: Code::NOP, mode: AddressingMode::Implied });
    // m.insert(0x1C, Opecode { code: Code::NOP, mode: AddressingMode::Implied });
    // m.insert(0x3C, Opecode { code: Code::NOP, mode: AddressingMode::Implied });
    // m.insert(0x5C, Opecode { code: Code::NOP, mode: AddressingMode::Implied });
    // m.insert(0x7C, Opecode { code: Code::NOP, mode: AddressingMode::Implied });
    // m.insert(0xDC, Opecode { code: Code::NOP, mode: AddressingMode::Implied });
    // m.insert(0xFC, Opecode { code: Code::NOP, mode: AddressingMode::Implied });
    // m.insert(0xA7, Opecode { code: Code::LAX, mode: AddressingMode::ZeroPage });
    // m.insert(0xB7, Opecode { code: Code::LAX, mode: AddressingMode::ZeroPageIndexedY });
    // m.insert(0xAF, Opecode { code: Code::LAX, mode: AddressingMode::Absolute });
    // m.insert(0xBF, Opecode { code: Code::LAX, mode: AddressingMode::AbsoluteIndexedY });
    // m.insert(0xA3, Opecode { code: Code::LAX, mode: AddressingMode::IndexedIndirect });
    // m.insert(0xB3, Opecode { code: Code::LAX, mode: AddressingMode::IndirectIndexed });
    // m.insert(0x87, Opecode { code: Code::SAX, mode: AddressingMode::ZeroPage });
    // m.insert(0x97, Opecode { code: Code::SAX, mode: AddressingMode::ZeroPageIndexedY });
    // m.insert(0x8F, Opecode { code: Code::SAX, mode: AddressingMode::Absolute });
    // m.insert(0x83, Opecode { code: Code::SAX, mode: AddressingMode::IndexedIndirect });
    // m.insert(0xEB, Opecode { code: Code::SBC, mode: AddressingMode::Immediate });
    // m.insert(0xC7, Opecode { code: Code::DCP, mode: AddressingMode::ZeroPage });
    // m.insert(0xD7, Opecode { code: Code::DCP, mode: AddressingMode::ZeroPageIndexedX });
    // m.insert(0xCF, Opecode { code: Code::DCP, mode: AddressingMode::Absolute });
    // m.insert(0xDF, Opecode { code: Code::DCP, mode: AddressingMode::AbsoluteIndexedX });
    // m.insert(0xDB, Opecode { code: Code::DCP, mode: AddressingMode::AbsoluteIndexedY });
    // m.insert(0xC3, Opecode { code: Code::DCP, mode: AddressingMode::IndexedIndirect });
    // m.insert(0xD3, Opecode { code: Code::DCP, mode: AddressingMode::IndirectIndexed });
    // m.insert(0xE7, Opecode { code: Code::ISB, mode: AddressingMode::ZeroPage });
    // m.insert(0xF7, Opecode { code: Code::ISB, mode: AddressingMode::ZeroPageIndexedX });
    // m.insert(0xEF, Opecode { code: Code::ISB, mode: AddressingMode::Absolute });
    // m.insert(0xFF, Opecode { code: Code::ISB, mode: AddressingMode::AbsoluteIndexedX });
    // m.insert(0xFB, Opecode { code: Code::ISB, mode: AddressingMode::AbsoluteIndexedY });
    // m.insert(0xE3, Opecode { code: Code::ISB, mode: AddressingMode::IndexedIndirect });
    // m.insert(0xF3, Opecode { code: Code::ISB, mode: AddressingMode::IndirectIndexed });
    // m.insert(0x07, Opecode { code: Code::SLO, mode: AddressingMode::ZeroPage });
    // m.insert(0x17, Opecode { code: Code::SLO, mode: AddressingMode::ZeroPageIndexedX });
    // m.insert(0x0F, Opecode { code: Code::SLO, mode: AddressingMode::Absolute });
    // m.insert(0x1F, Opecode { code: Code::SLO, mode: AddressingMode::AbsoluteIndexedX });
    // m.insert(0x1B, Opecode { code: Code::SLO, mode: AddressingMode::AbsoluteIndexedY });
    // m.insert(0x03, Opecode { code: Code::SLO, mode: AddressingMode::IndexedIndirect });
    // m.insert(0x13, Opecode { code: Code::SLO, mode: AddressingMode::IndirectIndexed });
    // m.insert(0x27, Opecode { code: Code::RLA, mode: AddressingMode::ZeroPage });
    // m.insert(0x37, Opecode { code: Code::RLA, mode: AddressingMode::ZeroPageIndexedX });
    // m.insert(0x2F, Opecode { code: Code::RLA, mode: AddressingMode::Absolute });
    // m.insert(0x3F, Opecode { code: Code::RLA, mode: AddressingMode::AbsoluteIndexedX });
    // m.insert(0x3B, Opecode { code: Code::RLA, mode: AddressingMode::AbsoluteIndexedY });
    // m.insert(0x23, Opecode { code: Code::RLA, mode: AddressingMode::IndexedIndirect });
    // m.insert(0x33, Opecode { code: Code::RLA, mode: AddressingMode::IndirectIndexed });
    // m.insert(0x47, Opecode { code: Code::SRE, mode: AddressingMode::ZeroPage });
    // m.insert(0x57, Opecode { code: Code::SRE, mode: AddressingMode::ZeroPageIndexedX });
    // m.insert(0x4F, Opecode { code: Code::SRE, mode: AddressingMode::Absolute });
    // m.insert(0x5F, Opecode { code: Code::SRE, mode: AddressingMode::AbsoluteIndexedX });
    // m.insert(0x5B, Opecode { code: Code::SRE, mode: AddressingMode::AbsoluteIndexedY });
    // m.insert(0x43, Opecode { code: Code::SRE, mode: AddressingMode::IndexedIndirect });
    // m.insert(0x53, Opecode { code: Code::SRE, mode: AddressingMode::IndirectIndexed });
    // m.insert(0x67, Opecode { code: Code::RRA, mode: AddressingMode::ZeroPage });
    // m.insert(0x77, Opecode { code: Code::RRA, mode: AddressingMode::ZeroPageIndexedX });
    // m.insert(0x6F, Opecode { code: Code::RRA, mode: AddressingMode::Absolute });
    // m.insert(0x7F, Opecode { code: Code::RRA, mode: AddressingMode::AbsoluteIndexedX });
    // m.insert(0x7B, Opecode { code: Code::RRA, mode: AddressingMode::AbsoluteIndexedY });
    // m.insert(0x63, Opecode { code: Code::RRA, mode: AddressingMode::IndexedIndirect });
    // m.insert(0x73, Opecode { code: Code::RRA, mode: AddressingMode::IndirectIndexed });
    m
});
