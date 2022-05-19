use once_cell::sync::Lazy;
use std::{collections::HashMap};

pub struct Opecode {
    pub mnemonic: Mnemonic,
    pub mode: AddressingMode,
}

pub enum Mnemonic {
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

#[derive(Debug)]
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

pub static OPECODE_MAP: Lazy<HashMap<u8, Opecode>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert(0xA9, Opecode { mnemonic: Mnemonic::LDA, mode: AddressingMode::Immediate });
    m.insert(0xA5, Opecode { mnemonic: Mnemonic::LDA, mode: AddressingMode::ZeroPage });
    m.insert(0xB5, Opecode { mnemonic: Mnemonic::LDA, mode: AddressingMode::ZeroPageIndexedX });
    m.insert(0xAD, Opecode { mnemonic: Mnemonic::LDA, mode: AddressingMode::Absolute });
    m.insert(0xBD, Opecode { mnemonic: Mnemonic::LDA, mode: AddressingMode::AbsoluteIndexedX });
    m.insert(0xB9, Opecode { mnemonic: Mnemonic::LDA, mode: AddressingMode::AbsoluteIndexedY });
    m.insert(0xA1, Opecode { mnemonic: Mnemonic::LDA, mode: AddressingMode::IndexedIndirect });
    m.insert(0xB1, Opecode { mnemonic: Mnemonic::LDA, mode: AddressingMode::IndirectIndexed });
    m.insert(0xA2, Opecode { mnemonic: Mnemonic::LDX, mode: AddressingMode::Immediate });
    m.insert(0xA6, Opecode { mnemonic: Mnemonic::LDX, mode: AddressingMode::ZeroPage });
    m.insert(0xAE, Opecode { mnemonic: Mnemonic::LDX, mode: AddressingMode::Absolute });
    m.insert(0xB6, Opecode { mnemonic: Mnemonic::LDX, mode: AddressingMode::ZeroPageIndexedY });
    m.insert(0xBE, Opecode { mnemonic: Mnemonic::LDX, mode: AddressingMode::AbsoluteIndexedY });
    m.insert(0xA0, Opecode { mnemonic: Mnemonic::LDY, mode: AddressingMode::Immediate });
    m.insert(0xA4, Opecode { mnemonic: Mnemonic::LDY, mode: AddressingMode::ZeroPage });
    m.insert(0xAC, Opecode { mnemonic: Mnemonic::LDY, mode: AddressingMode::Absolute });
    m.insert(0xB4, Opecode { mnemonic: Mnemonic::LDY, mode: AddressingMode::ZeroPageIndexedX });
    m.insert(0xBC, Opecode { mnemonic: Mnemonic::LDY, mode: AddressingMode::AbsoluteIndexedX });
    m.insert(0x85, Opecode { mnemonic: Mnemonic::STA, mode: AddressingMode::ZeroPage });
    m.insert(0x8D, Opecode { mnemonic: Mnemonic::STA, mode: AddressingMode::Absolute });
    m.insert(0x95, Opecode { mnemonic: Mnemonic::STA, mode: AddressingMode::ZeroPageIndexedX });
    m.insert(0x9D, Opecode { mnemonic: Mnemonic::STA, mode: AddressingMode::AbsoluteIndexedX });
    m.insert(0x99, Opecode { mnemonic: Mnemonic::STA, mode: AddressingMode::AbsoluteIndexedY });
    m.insert(0x81, Opecode { mnemonic: Mnemonic::STA, mode: AddressingMode::IndexedIndirect });
    m.insert(0x91, Opecode { mnemonic: Mnemonic::STA, mode: AddressingMode::IndirectIndexed });
    // m.insert(0x86, Opecode { mnemonic: Mnemonic::STX, mode: AddressingMode::ZeroPage });
    // m.insert(0x8E, Opecode { mnemonic: Mnemonic::STX, mode: AddressingMode::Absolute });
    // m.insert(0x96, Opecode { mnemonic: Mnemonic::STX, mode: AddressingMode::ZeroPageIndexedY });
    // m.insert(0x84, Opecode { mnemonic: Mnemonic::STY, mode: AddressingMode::ZeroPage });
    // m.insert(0x8C, Opecode { mnemonic: Mnemonic::STY, mode: AddressingMode::Absolute });
    // m.insert(0x94, Opecode { mnemonic: Mnemonic::STY, mode: AddressingMode::ZeroPageIndexedX });
    // m.insert(0x8A, Opecode { mnemonic: Mnemonic::TXA, mode: AddressingMode::Implied });
    // m.insert(0x98, Opecode { mnemonic: Mnemonic::TYA, mode: AddressingMode::Implied });
    m.insert(0x9A, Opecode { mnemonic: Mnemonic::TXS, mode: AddressingMode::Implied });
    // m.insert(0xA8, Opecode { mnemonic: Mnemonic::TAY, mode: AddressingMode::Implied });
    // m.insert(0xAA, Opecode { mnemonic: Mnemonic::TAX, mode: AddressingMode::Implied });
    // m.insert(0xBA, Opecode { mnemonic: Mnemonic::TSX, mode: AddressingMode::Implied });
    // m.insert(0x08, Opecode { mnemonic: Mnemonic::PHP, mode: AddressingMode::Implied });
    // m.insert(0x28, Opecode { mnemonic: Mnemonic::PLP, mode: AddressingMode::Implied });
    // m.insert(0x48, Opecode { mnemonic: Mnemonic::PHA, mode: AddressingMode::Implied });
    // m.insert(0x68, Opecode { mnemonic: Mnemonic::PLA, mode: AddressingMode::Implied });
    // m.insert(0x69, Opecode { mnemonic: Mnemonic::ADC, mode: AddressingMode::Immediate });
    // m.insert(0x65, Opecode { mnemonic: Mnemonic::ADC, mode: AddressingMode::ZeroPage });
    // m.insert(0x6D, Opecode { mnemonic: Mnemonic::ADC, mode: AddressingMode::Absolute });
    // m.insert(0x75, Opecode { mnemonic: Mnemonic::ADC, mode: AddressingMode::ZeroPageIndexedX });
    // m.insert(0x7D, Opecode { mnemonic: Mnemonic::ADC, mode: AddressingMode::AbsoluteIndexedX });
    // m.insert(0x79, Opecode { mnemonic: Mnemonic::ADC, mode: AddressingMode::AbsoluteIndexedY });
    // m.insert(0x61, Opecode { mnemonic: Mnemonic::ADC, mode: AddressingMode::IndexedIndirect });
    // m.insert(0x71, Opecode { mnemonic: Mnemonic::ADC, mode: AddressingMode::IndirectIndexed });
    // m.insert(0xE9, Opecode { mnemonic: Mnemonic::SBC, mode: AddressingMode::Immediate });
    // m.insert(0xE5, Opecode { mnemonic: Mnemonic::SBC, mode: AddressingMode::ZeroPage });
    // m.insert(0xED, Opecode { mnemonic: Mnemonic::SBC, mode: AddressingMode::Absolute });
    // m.insert(0xF5, Opecode { mnemonic: Mnemonic::SBC, mode: AddressingMode::ZeroPageIndexedX });
    // m.insert(0xFD, Opecode { mnemonic: Mnemonic::SBC, mode: AddressingMode::AbsoluteIndexedX });
    // m.insert(0xF9, Opecode { mnemonic: Mnemonic::SBC, mode: AddressingMode::AbsoluteIndexedY });
    // m.insert(0xE1, Opecode { mnemonic: Mnemonic::SBC, mode: AddressingMode::IndexedIndirect });
    // m.insert(0xF1, Opecode { mnemonic: Mnemonic::SBC, mode: AddressingMode::IndirectIndexed });
    // m.insert(0xE0, Opecode { mnemonic: Mnemonic::CPX, mode: AddressingMode::Immediate });
    // m.insert(0xE4, Opecode { mnemonic: Mnemonic::CPX, mode: AddressingMode::ZeroPage });
    // m.insert(0xEC, Opecode { mnemonic: Mnemonic::CPX, mode: AddressingMode::Absolute });
    // m.insert(0xC0, Opecode { mnemonic: Mnemonic::CPY, mode: AddressingMode::Immediate });
    // m.insert(0xC4, Opecode { mnemonic: Mnemonic::CPY, mode: AddressingMode::ZeroPage });
    // m.insert(0xCC, Opecode { mnemonic: Mnemonic::CPY, mode: AddressingMode::Absolute });
    // m.insert(0xC9, Opecode { mnemonic: Mnemonic::CMP, mode: AddressingMode::Immediate });
    // m.insert(0xC5, Opecode { mnemonic: Mnemonic::CMP, mode: AddressingMode::ZeroPage });
    // m.insert(0xCD, Opecode { mnemonic: Mnemonic::CMP, mode: AddressingMode::Absolute });
    // m.insert(0xD5, Opecode { mnemonic: Mnemonic::CMP, mode: AddressingMode::ZeroPageIndexedX });
    // m.insert(0xDD, Opecode { mnemonic: Mnemonic::CMP, mode: AddressingMode::AbsoluteIndexedX });
    // m.insert(0xD9, Opecode { mnemonic: Mnemonic::CMP, mode: AddressingMode::AbsoluteIndexedY });
    // m.insert(0xC1, Opecode { mnemonic: Mnemonic::CMP, mode: AddressingMode::IndexedIndirect });
    // m.insert(0xD1, Opecode { mnemonic: Mnemonic::CMP, mode: AddressingMode::IndirectIndexed });
    // m.insert(0x29, Opecode { mnemonic: Mnemonic::AND, mode: AddressingMode::Immediate });
    // m.insert(0x25, Opecode { mnemonic: Mnemonic::AND, mode: AddressingMode::ZeroPage });
    // m.insert(0x2D, Opecode { mnemonic: Mnemonic::AND, mode: AddressingMode::Absolute });
    // m.insert(0x35, Opecode { mnemonic: Mnemonic::AND, mode: AddressingMode::ZeroPageIndexedX });
    // m.insert(0x3D, Opecode { mnemonic: Mnemonic::AND, mode: AddressingMode::AbsoluteIndexedX });
    // m.insert(0x39, Opecode { mnemonic: Mnemonic::AND, mode: AddressingMode::AbsoluteIndexedY });
    // m.insert(0x21, Opecode { mnemonic: Mnemonic::AND, mode: AddressingMode::IndexedIndirect });
    // m.insert(0x31, Opecode { mnemonic: Mnemonic::AND, mode: AddressingMode::IndirectIndexed });
    // m.insert(0x49, Opecode { mnemonic: Mnemonic::EOR, mode: AddressingMode::Immediate });
    // m.insert(0x45, Opecode { mnemonic: Mnemonic::EOR, mode: AddressingMode::ZeroPage });
    // m.insert(0x4D, Opecode { mnemonic: Mnemonic::EOR, mode: AddressingMode::Absolute });
    // m.insert(0x55, Opecode { mnemonic: Mnemonic::EOR, mode: AddressingMode::ZeroPageIndexedX });
    // m.insert(0x5D, Opecode { mnemonic: Mnemonic::EOR, mode: AddressingMode::AbsoluteIndexedX });
    // m.insert(0x59, Opecode { mnemonic: Mnemonic::EOR, mode: AddressingMode::AbsoluteIndexedY });
    // m.insert(0x41, Opecode { mnemonic: Mnemonic::EOR, mode: AddressingMode::IndexedIndirect });
    // m.insert(0x51, Opecode { mnemonic: Mnemonic::EOR, mode: AddressingMode::IndirectIndexed });
    // m.insert(0x09, Opecode { mnemonic: Mnemonic::ORA, mode: AddressingMode::Immediate });
    // m.insert(0x05, Opecode { mnemonic: Mnemonic::ORA, mode: AddressingMode::ZeroPage });
    // m.insert(0x0D, Opecode { mnemonic: Mnemonic::ORA, mode: AddressingMode::Absolute });
    // m.insert(0x15, Opecode { mnemonic: Mnemonic::ORA, mode: AddressingMode::ZeroPageIndexedX });
    // m.insert(0x1D, Opecode { mnemonic: Mnemonic::ORA, mode: AddressingMode::AbsoluteIndexedX });
    // m.insert(0x19, Opecode { mnemonic: Mnemonic::ORA, mode: AddressingMode::AbsoluteIndexedY });
    // m.insert(0x01, Opecode { mnemonic: Mnemonic::ORA, mode: AddressingMode::IndexedIndirect });
    // m.insert(0x11, Opecode { mnemonic: Mnemonic::ORA, mode: AddressingMode::IndirectIndexed });
    // m.insert(0x24, Opecode { mnemonic: Mnemonic::BIT, mode: AddressingMode::ZeroPage });
    // m.insert(0x2C, Opecode { mnemonic: Mnemonic::BIT, mode: AddressingMode::Absolute });
    // m.insert(0x0A, Opecode { mnemonic: Mnemonic::ASL, mode: AddressingMode::Accumulator });
    // m.insert(0x06, Opecode { mnemonic: Mnemonic::ASL, mode: AddressingMode::ZeroPage });
    // m.insert(0x0E, Opecode { mnemonic: Mnemonic::ASL, mode: AddressingMode::Absolute });
    // m.insert(0x16, Opecode { mnemonic: Mnemonic::ASL, mode: AddressingMode::ZeroPageIndexedX });
    // m.insert(0x1E, Opecode { mnemonic: Mnemonic::ASL, mode: AddressingMode::AbsoluteIndexedX });
    // m.insert(0x4A, Opecode { mnemonic: Mnemonic::LSR, mode: AddressingMode::Accumulator });
    // m.insert(0x46, Opecode { mnemonic: Mnemonic::LSR, mode: AddressingMode::ZeroPage });
    // m.insert(0x4E, Opecode { mnemonic: Mnemonic::LSR, mode: AddressingMode::Absolute });
    // m.insert(0x56, Opecode { mnemonic: Mnemonic::LSR, mode: AddressingMode::ZeroPageIndexedX });
    // m.insert(0x5E, Opecode { mnemonic: Mnemonic::LSR, mode: AddressingMode::AbsoluteIndexedX });
    // m.insert(0x2A, Opecode { mnemonic: Mnemonic::ROL, mode: AddressingMode::Accumulator });
    // m.insert(0x26, Opecode { mnemonic: Mnemonic::ROL, mode: AddressingMode::ZeroPage });
    // m.insert(0x2E, Opecode { mnemonic: Mnemonic::ROL, mode: AddressingMode::Absolute });
    // m.insert(0x36, Opecode { mnemonic: Mnemonic::ROL, mode: AddressingMode::ZeroPageIndexedX });
    // m.insert(0x3E, Opecode { mnemonic: Mnemonic::ROL, mode: AddressingMode::AbsoluteIndexedX });
    // m.insert(0x6A, Opecode { mnemonic: Mnemonic::ROR, mode: AddressingMode::Accumulator });
    // m.insert(0x66, Opecode { mnemonic: Mnemonic::ROR, mode: AddressingMode::ZeroPage });
    // m.insert(0x6E, Opecode { mnemonic: Mnemonic::ROR, mode: AddressingMode::Absolute });
    // m.insert(0x76, Opecode { mnemonic: Mnemonic::ROR, mode: AddressingMode::ZeroPageIndexedX });
    // m.insert(0x7E, Opecode { mnemonic: Mnemonic::ROR, mode: AddressingMode::AbsoluteIndexedX });
    m.insert(0xE8, Opecode { mnemonic: Mnemonic::INX, mode: AddressingMode::Implied });
    // m.insert(0xC8, Opecode { mnemonic: Mnemonic::INY, mode: AddressingMode::Implied });
    // m.insert(0xE6, Opecode { mnemonic: Mnemonic::INC, mode: AddressingMode::ZeroPage });
    // m.insert(0xEE, Opecode { mnemonic: Mnemonic::INC, mode: AddressingMode::Absolute });
    // m.insert(0xF6, Opecode { mnemonic: Mnemonic::INC, mode: AddressingMode::ZeroPageIndexedX });
    // m.insert(0xFE, Opecode { mnemonic: Mnemonic::INC, mode: AddressingMode::AbsoluteIndexedX });
    // m.insert(0xCA, Opecode { mnemonic: Mnemonic::DEX, mode: AddressingMode::Implied });
    m.insert(0x88, Opecode { mnemonic: Mnemonic::DEY, mode: AddressingMode::Implied });
    // m.insert(0xC6, Opecode { mnemonic: Mnemonic::DEC, mode: AddressingMode::ZeroPage });
    // m.insert(0xCE, Opecode { mnemonic: Mnemonic::DEC, mode: AddressingMode::Absolute });
    // m.insert(0xD6, Opecode { mnemonic: Mnemonic::DEC, mode: AddressingMode::ZeroPageIndexedX });
    // m.insert(0xDE, Opecode { mnemonic: Mnemonic::DEC, mode: AddressingMode::AbsoluteIndexedX });
    // m.insert(0x18, Opecode { mnemonic: Mnemonic::CLC, mode: AddressingMode::Implied });
    // m.insert(0x58, Opecode { mnemonic: Mnemonic::CLI, mode: AddressingMode::Implied });
    // m.insert(0xB8, Opecode { mnemonic: Mnemonic::CLV, mode: AddressingMode::Implied });
    // m.insert(0x38, Opecode { mnemonic: Mnemonic::SEC, mode: AddressingMode::Implied });
    m.insert(0x78, Opecode { mnemonic: Mnemonic::SEI, mode: AddressingMode::Implied });
    // m.insert(0xEA, Opecode { mnemonic: Mnemonic::NOP, mode: AddressingMode::Implied });
    // m.insert(0x00, Opecode { mnemonic: Mnemonic::BRK, mode: AddressingMode::Implied });
    // m.insert(0x20, Opecode { mnemonic: Mnemonic::JSR, mode: AddressingMode::Absolute });
    m.insert(0x4C, Opecode { mnemonic: Mnemonic::JMP, mode: AddressingMode::Absolute });
    m.insert(0x6C, Opecode { mnemonic: Mnemonic::JMP, mode: AddressingMode::AbsoluteIndirect });
    // m.insert(0x40, Opecode { mnemonic: Mnemonic::RTI, mode: AddressingMode::Implied });
    // m.insert(0x60, Opecode { mnemonic: Mnemonic::RTS, mode: AddressingMode::Implied });
    // m.insert(0x10, Opecode { mnemonic: Mnemonic::BPL, mode: AddressingMode::Relative });
    // m.insert(0x30, Opecode { mnemonic: Mnemonic::BMI, mode: AddressingMode::Relative });
    // m.insert(0x50, Opecode { mnemonic: Mnemonic::BVC, mode: AddressingMode::Relative });
    // m.insert(0x70, Opecode { mnemonic: Mnemonic::BVS, mode: AddressingMode::Relative });
    // m.insert(0x90, Opecode { mnemonic: Mnemonic::BCC, mode: AddressingMode::Relative });
    // m.insert(0xB0, Opecode { mnemonic: Mnemonic::BCS, mode: AddressingMode::Relative });
    m.insert(0xD0, Opecode { mnemonic: Mnemonic::BNE, mode: AddressingMode::Relative });
    // m.insert(0xF0, Opecode { mnemonic: Mnemonic::BEQ, mode: AddressingMode::Relative });
    // m.insert(0xF8, Opecode { mnemonic: Mnemonic::SED, mode: AddressingMode::Implied });
    // m.insert(0xD8, Opecode { mnemonic: Mnemonic::CLD, mode: AddressingMode::Implied });
    // m.insert(0x1A, Opecode { mnemonic: Mnemonic::NOP, mode: AddressingMode::Implied });
    // m.insert(0x3A, Opecode { mnemonic: Mnemonic::NOP, mode: AddressingMode::Implied });
    // m.insert(0x5A, Opecode { mnemonic: Mnemonic::NOP, mode: AddressingMode::Implied });
    // m.insert(0x7A, Opecode { mnemonic: Mnemonic::NOP, mode: AddressingMode::Implied });
    // m.insert(0xDA, Opecode { mnemonic: Mnemonic::NOP, mode: AddressingMode::Implied });
    // m.insert(0xFA, Opecode { mnemonic: Mnemonic::NOP, mode: AddressingMode::Implied });
    // m.insert(0x02, Opecode { mnemonic: Mnemonic::NOP, mode: AddressingMode::Implied });
    // m.insert(0x12, Opecode { mnemonic: Mnemonic::NOP, mode: AddressingMode::Implied });
    // m.insert(0x22, Opecode { mnemonic: Mnemonic::NOP, mode: AddressingMode::Implied });
    // m.insert(0x32, Opecode { mnemonic: Mnemonic::NOP, mode: AddressingMode::Implied });
    // m.insert(0x42, Opecode { mnemonic: Mnemonic::NOP, mode: AddressingMode::Implied });
    // m.insert(0x52, Opecode { mnemonic: Mnemonic::NOP, mode: AddressingMode::Implied });
    // m.insert(0x62, Opecode { mnemonic: Mnemonic::NOP, mode: AddressingMode::Implied });
    // m.insert(0x72, Opecode { mnemonic: Mnemonic::NOP, mode: AddressingMode::Implied });
    // m.insert(0x92, Opecode { mnemonic: Mnemonic::NOP, mode: AddressingMode::Implied });
    // m.insert(0xB2, Opecode { mnemonic: Mnemonic::NOP, mode: AddressingMode::Implied });
    // m.insert(0xD2, Opecode { mnemonic: Mnemonic::NOP, mode: AddressingMode::Implied });
    // m.insert(0xF2, Opecode { mnemonic: Mnemonic::NOP, mode: AddressingMode::Implied });
    // m.insert(0x80, Opecode { mnemonic: Mnemonic::NOP, mode: AddressingMode::Implied });
    // m.insert(0x82, Opecode { mnemonic: Mnemonic::NOP, mode: AddressingMode::Implied });
    // m.insert(0x89, Opecode { mnemonic: Mnemonic::NOP, mode: AddressingMode::Implied });
    // m.insert(0xC2, Opecode { mnemonic: Mnemonic::NOP, mode: AddressingMode::Implied });
    // m.insert(0xE2, Opecode { mnemonic: Mnemonic::NOP, mode: AddressingMode::Implied });
    // m.insert(0x04, Opecode { mnemonic: Mnemonic::NOP, mode: AddressingMode::Implied });
    // m.insert(0x44, Opecode { mnemonic: Mnemonic::NOP, mode: AddressingMode::Implied });
    // m.insert(0x64, Opecode { mnemonic: Mnemonic::NOP, mode: AddressingMode::Implied });
    // m.insert(0x14, Opecode { mnemonic: Mnemonic::NOP, mode: AddressingMode::Implied });
    // m.insert(0x34, Opecode { mnemonic: Mnemonic::NOP, mode: AddressingMode::Implied });
    // m.insert(0x54, Opecode { mnemonic: Mnemonic::NOP, mode: AddressingMode::Implied });
    // m.insert(0x74, Opecode { mnemonic: Mnemonic::NOP, mode: AddressingMode::Implied });
    // m.insert(0xD4, Opecode { mnemonic: Mnemonic::NOP, mode: AddressingMode::Implied });
    // m.insert(0xF4, Opecode { mnemonic: Mnemonic::NOP, mode: AddressingMode::Implied });
    // m.insert(0x0C, Opecode { mnemonic: Mnemonic::NOP, mode: AddressingMode::Implied });
    // m.insert(0x1C, Opecode { mnemonic: Mnemonic::NOP, mode: AddressingMode::Implied });
    // m.insert(0x3C, Opecode { mnemonic: Mnemonic::NOP, mode: AddressingMode::Implied });
    // m.insert(0x5C, Opecode { mnemonic: Mnemonic::NOP, mode: AddressingMode::Implied });
    // m.insert(0x7C, Opecode { mnemonic: Mnemonic::NOP, mode: AddressingMode::Implied });
    // m.insert(0xDC, Opecode { mnemonic: Mnemonic::NOP, mode: AddressingMode::Implied });
    // m.insert(0xFC, Opecode { mnemonic: Mnemonic::NOP, mode: AddressingMode::Implied });
    // m.insert(0xA7, Opecode { mnemonic: Mnemonic::LAX, mode: AddressingMode::ZeroPage });
    // m.insert(0xB7, Opecode { mnemonic: Mnemonic::LAX, mode: AddressingMode::ZeroPageIndexedY });
    // m.insert(0xAF, Opecode { mnemonic: Mnemonic::LAX, mode: AddressingMode::Absolute });
    // m.insert(0xBF, Opecode { mnemonic: Mnemonic::LAX, mode: AddressingMode::AbsoluteIndexedY });
    // m.insert(0xA3, Opecode { mnemonic: Mnemonic::LAX, mode: AddressingMode::IndexedIndirect });
    // m.insert(0xB3, Opecode { mnemonic: Mnemonic::LAX, mode: AddressingMode::IndirectIndexed });
    // m.insert(0x87, Opecode { mnemonic: Mnemonic::SAX, mode: AddressingMode::ZeroPage });
    // m.insert(0x97, Opecode { mnemonic: Mnemonic::SAX, mode: AddressingMode::ZeroPageIndexedY });
    // m.insert(0x8F, Opecode { mnemonic: Mnemonic::SAX, mode: AddressingMode::Absolute });
    // m.insert(0x83, Opecode { mnemonic: Mnemonic::SAX, mode: AddressingMode::IndexedIndirect });
    // m.insert(0xEB, Opecode { mnemonic: Mnemonic::SBC, mode: AddressingMode::Immediate });
    // m.insert(0xC7, Opecode { mnemonic: Mnemonic::DCP, mode: AddressingMode::ZeroPage });
    // m.insert(0xD7, Opecode { mnemonic: Mnemonic::DCP, mode: AddressingMode::ZeroPageIndexedX });
    // m.insert(0xCF, Opecode { mnemonic: Mnemonic::DCP, mode: AddressingMode::Absolute });
    // m.insert(0xDF, Opecode { mnemonic: Mnemonic::DCP, mode: AddressingMode::AbsoluteIndexedX });
    // m.insert(0xDB, Opecode { mnemonic: Mnemonic::DCP, mode: AddressingMode::AbsoluteIndexedY });
    // m.insert(0xC3, Opecode { mnemonic: Mnemonic::DCP, mode: AddressingMode::IndexedIndirect });
    // m.insert(0xD3, Opecode { mnemonic: Mnemonic::DCP, mode: AddressingMode::IndirectIndexed });
    // m.insert(0xE7, Opecode { mnemonic: Mnemonic::ISB, mode: AddressingMode::ZeroPage });
    // m.insert(0xF7, Opecode { mnemonic: Mnemonic::ISB, mode: AddressingMode::ZeroPageIndexedX });
    // m.insert(0xEF, Opecode { mnemonic: Mnemonic::ISB, mode: AddressingMode::Absolute });
    // m.insert(0xFF, Opecode { mnemonic: Mnemonic::ISB, mode: AddressingMode::AbsoluteIndexedX });
    // m.insert(0xFB, Opecode { mnemonic: Mnemonic::ISB, mode: AddressingMode::AbsoluteIndexedY });
    // m.insert(0xE3, Opecode { mnemonic: Mnemonic::ISB, mode: AddressingMode::IndexedIndirect });
    // m.insert(0xF3, Opecode { mnemonic: Mnemonic::ISB, mode: AddressingMode::IndirectIndexed });
    // m.insert(0x07, Opecode { mnemonic: Mnemonic::SLO, mode: AddressingMode::ZeroPage });
    // m.insert(0x17, Opecode { mnemonic: Mnemonic::SLO, mode: AddressingMode::ZeroPageIndexedX });
    // m.insert(0x0F, Opecode { mnemonic: Mnemonic::SLO, mode: AddressingMode::Absolute });
    // m.insert(0x1F, Opecode { mnemonic: Mnemonic::SLO, mode: AddressingMode::AbsoluteIndexedX });
    // m.insert(0x1B, Opecode { mnemonic: Mnemonic::SLO, mode: AddressingMode::AbsoluteIndexedY });
    // m.insert(0x03, Opecode { mnemonic: Mnemonic::SLO, mode: AddressingMode::IndexedIndirect });
    // m.insert(0x13, Opecode { mnemonic: Mnemonic::SLO, mode: AddressingMode::IndirectIndexed });
    // m.insert(0x27, Opecode { mnemonic: Mnemonic::RLA, mode: AddressingMode::ZeroPage });
    // m.insert(0x37, Opecode { mnemonic: Mnemonic::RLA, mode: AddressingMode::ZeroPageIndexedX });
    // m.insert(0x2F, Opecode { mnemonic: Mnemonic::RLA, mode: AddressingMode::Absolute });
    // m.insert(0x3F, Opecode { mnemonic: Mnemonic::RLA, mode: AddressingMode::AbsoluteIndexedX });
    // m.insert(0x3B, Opecode { mnemonic: Mnemonic::RLA, mode: AddressingMode::AbsoluteIndexedY });
    // m.insert(0x23, Opecode { mnemonic: Mnemonic::RLA, mode: AddressingMode::IndexedIndirect });
    // m.insert(0x33, Opecode { mnemonic: Mnemonic::RLA, mode: AddressingMode::IndirectIndexed });
    // m.insert(0x47, Opecode { mnemonic: Mnemonic::SRE, mode: AddressingMode::ZeroPage });
    // m.insert(0x57, Opecode { mnemonic: Mnemonic::SRE, mode: AddressingMode::ZeroPageIndexedX });
    // m.insert(0x4F, Opecode { mnemonic: Mnemonic::SRE, mode: AddressingMode::Absolute });
    // m.insert(0x5F, Opecode { mnemonic: Mnemonic::SRE, mode: AddressingMode::AbsoluteIndexedX });
    // m.insert(0x5B, Opecode { mnemonic: Mnemonic::SRE, mode: AddressingMode::AbsoluteIndexedY });
    // m.insert(0x43, Opecode { mnemonic: Mnemonic::SRE, mode: AddressingMode::IndexedIndirect });
    // m.insert(0x53, Opecode { mnemonic: Mnemonic::SRE, mode: AddressingMode::IndirectIndexed });
    // m.insert(0x67, Opecode { mnemonic: Mnemonic::RRA, mode: AddressingMode::ZeroPage });
    // m.insert(0x77, Opecode { mnemonic: Mnemonic::RRA, mode: AddressingMode::ZeroPageIndexedX });
    // m.insert(0x6F, Opecode { mnemonic: Mnemonic::RRA, mode: AddressingMode::Absolute });
    // m.insert(0x7F, Opecode { mnemonic: Mnemonic::RRA, mode: AddressingMode::AbsoluteIndexedX });
    // m.insert(0x7B, Opecode { mnemonic: Mnemonic::RRA, mode: AddressingMode::AbsoluteIndexedY });
    // m.insert(0x63, Opecode { mnemonic: Mnemonic::RRA, mode: AddressingMode::IndexedIndirect });
    // m.insert(0x73, Opecode { mnemonic: Mnemonic::RRA, mode: AddressingMode::IndirectIndexed });
    m
});
