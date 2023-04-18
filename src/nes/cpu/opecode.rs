use once_cell::sync::Lazy;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Opecode {
    pub code: Code,
    pub mode: AddressingMode,
    pub cycle: u16,
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
    // Load
    LDA,
    LDX,
    LDY,
    // Store
    STA,
    STX,
    STY,
    // Transfer
    TAX,
    TAY,
    TXA,
    TYA,
    // Stack
    TSX,
    TXS,
    PHA,
    PHP,
    PLA,
    PLP,
    // Logical
    // Arithmetic
    // Increment
    INC,
    INX,
    INY,
    // Decrement
    DEC,
    DEX,
    DEY,
    // Jump
    JMP,
    // Branches
    BNE,
    // Flags
    SEI,
    // System
}

#[rustfmt::skip]
pub static OPECODE_MAP: Lazy<HashMap<u8, Opecode>> = Lazy::new(|| {
    let cycles: Vec<u16> =
    vec![
        7, 6, 2, 8, 3, 3, 5, 5, 3, 2, 2, 2, 4, 4, 6, 6, 2, 5, 2, 8, 4, 4, 6, 6, 2, 4, 2, 7,
        4, 4, 6, 7, 6, 6, 2, 8, 3, 3, 5, 5, 4, 2, 2, 2, 4, 4, 6, 6, 2, 5, 2, 8, 4, 4, 6, 6,
        2, 4, 2, 7, 4, 4, 6, 7, 6, 6, 2, 8, 3, 3, 5, 5, 3, 2, 2, 2, 3, 4, 6, 6, 2, 5, 2, 8,
        4, 4, 6, 6, 2, 4, 2, 7, 4, 4, 6, 7, 6, 6, 2, 8, 3, 3, 5, 5, 4, 2, 2, 2, 5, 4, 6, 6,
        2, 5, 2, 8, 4, 4, 6, 6, 2, 4, 2, 7, 4, 4, 6, 7, 2, 6, 2, 6, 3, 3, 3, 3, 2, 2, 2, 2,
        4, 4, 4, 4, 2, 6, 2, 6, 4, 4, 4, 4, 2, 4, 2, 5, 5, 4, 5, 5, 2, 6, 2, 6, 3, 3, 3, 3,
        2, 2, 2, 2, 4, 4, 4, 4, 2, 5, 2, 5, 4, 4, 4, 4, 2, 4, 2, 4, 4, 4, 4, 4, 2, 6, 2, 8,
        3, 3, 5, 5, 2, 2, 2, 2, 4, 4, 6, 6, 2, 5, 2, 8, 4, 4, 6, 6, 2, 4, 2, 7, 4, 4, 7, 7,
        2, 6, 3, 8, 3, 3, 5, 5, 2, 2, 2, 2, 4, 4, 6, 6, 2, 5, 2, 8, 4, 4, 6, 6, 2, 4, 2, 7,
        4, 4, 7, 7
    ];

    let mut m = HashMap::new();
    m.insert(0xA9, Opecode { code: Code::LDA, cycle: cycles[0xA9], mode: AddressingMode::Immediate });
    m.insert(0xA5, Opecode { code: Code::LDA, cycle: cycles[0xA5], mode: AddressingMode::ZeroPage });
    m.insert(0xB5, Opecode { code: Code::LDA, cycle: cycles[0xB5], mode: AddressingMode::ZeroPageIndexedX });
    m.insert(0xAD, Opecode { code: Code::LDA, cycle: cycles[0xAD], mode: AddressingMode::Absolute });
    m.insert(0xBD, Opecode { code: Code::LDA, cycle: cycles[0xBD], mode: AddressingMode::AbsoluteIndexedX });
    m.insert(0xB9, Opecode { code: Code::LDA, cycle: cycles[0xB9], mode: AddressingMode::AbsoluteIndexedY });
    m.insert(0xA1, Opecode { code: Code::LDA, cycle: cycles[0xA1], mode: AddressingMode::IndexedIndirect });
    m.insert(0xB1, Opecode { code: Code::LDA, cycle: cycles[0xB1], mode: AddressingMode::IndirectIndexed });
    m.insert(0xA2, Opecode { code: Code::LDX, cycle: cycles[0xA2], mode: AddressingMode::Immediate });
    m.insert(0xA6, Opecode { code: Code::LDX, cycle: cycles[0xA6], mode: AddressingMode::ZeroPage });
    m.insert(0xAE, Opecode { code: Code::LDX, cycle: cycles[0xAE], mode: AddressingMode::Absolute });
    m.insert(0xB6, Opecode { code: Code::LDX, cycle: cycles[0xB6], mode: AddressingMode::ZeroPageIndexedY });
    m.insert(0xBE, Opecode { code: Code::LDX, cycle: cycles[0xBE], mode: AddressingMode::AbsoluteIndexedY });
    m.insert(0xA0, Opecode { code: Code::LDY, cycle: cycles[0xA0], mode: AddressingMode::Immediate });
    m.insert(0xA4, Opecode { code: Code::LDY, cycle: cycles[0xA4], mode: AddressingMode::ZeroPage });
    m.insert(0xAC, Opecode { code: Code::LDY, cycle: cycles[0xAC], mode: AddressingMode::Absolute });
    m.insert(0xB4, Opecode { code: Code::LDY, cycle: cycles[0xB4], mode: AddressingMode::ZeroPageIndexedX });
    m.insert(0xBC, Opecode { code: Code::LDY, cycle: cycles[0xBC], mode: AddressingMode::AbsoluteIndexedX });
    m.insert(0x85, Opecode { code: Code::STA, cycle: cycles[0x85], mode: AddressingMode::ZeroPage });
    m.insert(0x8D, Opecode { code: Code::STA, cycle: cycles[0x8D], mode: AddressingMode::Absolute });
    m.insert(0x95, Opecode { code: Code::STA, cycle: cycles[0x95], mode: AddressingMode::ZeroPageIndexedX });
    m.insert(0x9D, Opecode { code: Code::STA, cycle: cycles[0x9D], mode: AddressingMode::AbsoluteIndexedX });
    m.insert(0x99, Opecode { code: Code::STA, cycle: cycles[0x99], mode: AddressingMode::AbsoluteIndexedY });
    m.insert(0x81, Opecode { code: Code::STA, cycle: cycles[0x81], mode: AddressingMode::IndexedIndirect });
    m.insert(0x91, Opecode { code: Code::STA, cycle: cycles[0x91], mode: AddressingMode::IndirectIndexed });
    // m.insert(0x86, Opecode { code: Code::STX, cycle: cycles[0x86], mode: AddressingMode::ZeroPage });
    // m.insert(0x8E, Opecode { code: Code::STX, cycle: cycles[0x8E], mode: AddressingMode::Absolute });
    // m.insert(0x96, Opecode { code: Code::STX, cycle: cycles[0x96], mode: AddressingMode::ZeroPageIndexedY });
    // m.insert(0x84, Opecode { code: Code::STY, cycle: cycles[0x84], mode: AddressingMode::ZeroPage });
    // m.insert(0x8C, Opecode { code: Code::STY, cycle: cycles[0x8C], mode: AddressingMode::Absolute });
    // m.insert(0x94, Opecode { code: Code::STY, cycle: cycles[0x94], mode: AddressingMode::ZeroPageIndexedX });
    // m.insert(0x8A, Opecode { code: Code::TXA, cycle: cycles[0x8A], mode: AddressingMode::Implied });
    // m.insert(0x98, Opecode { code: Code::TYA, cycle: cycles[0x98], mode: AddressingMode::Implied });
    m.insert(0x9A, Opecode { code: Code::TXS, cycle: cycles[0x9A], mode: AddressingMode::Implied });
    // m.insert(0xA8, Opecode { code: Code::TAY, cycle: cycles[0xA8], mode: AddressingMode::Implied });
    // m.insert(0xAA, Opecode { code: Code::TAX, cycle: cycles[0xAA], mode: AddressingMode::Implied });
    // m.insert(0xBA, Opecode { code: Code::TSX, cycle: cycles[0xBA], mode: AddressingMode::Implied });
    // m.insert(0x08, Opecode { code: Code::PHP, cycle: cycles[0x08], mode: AddressingMode::Implied });
    // m.insert(0x28, Opecode { code: Code::PLP, cycle: cycles[0x28], mode: AddressingMode::Implied });
    // m.insert(0x48, Opecode { code: Code::PHA, cycle: cycles[0x48], mode: AddressingMode::Implied });
    // m.insert(0x68, Opecode { code: Code::PLA, cycle: cycles[0x68], mode: AddressingMode::Implied });
    // m.insert(0x69, Opecode { code: Code::ADC, cycle: cycles[0x69], mode: AddressingMode::Immediate });
    // m.insert(0x65, Opecode { code: Code::ADC, cycle: cycles[0x65], mode: AddressingMode::ZeroPage });
    // m.insert(0x6D, Opecode { code: Code::ADC, cycle: cycles[0x6D], mode: AddressingMode::Absolute });
    // m.insert(0x75, Opecode { code: Code::ADC, cycle: cycles[0x75], mode: AddressingMode::ZeroPageIndexedX });
    // m.insert(0x7D, Opecode { code: Code::ADC, cycle: cycles[0x7D], mode: AddressingMode::AbsoluteIndexedX });
    // m.insert(0x79, Opecode { code: Code::ADC, cycle: cycles[0x79], mode: AddressingMode::AbsoluteIndexedY });
    // m.insert(0x61, Opecode { code: Code::ADC, cycle: cycles[0x61], mode: AddressingMode::IndexedIndirect });
    // m.insert(0x71, Opecode { code: Code::ADC, cycle: cycles[0x71], mode: AddressingMode::IndirectIndexed });
    // m.insert(0xE9, Opecode { code: Code::SBC, cycle: cycles[0xE9], mode: AddressingMode::Immediate });
    // m.insert(0xE5, Opecode { code: Code::SBC, cycle: cycles[0xE5], mode: AddressingMode::ZeroPage });
    // m.insert(0xED, Opecode { code: Code::SBC, cycle: cycles[0xED], mode: AddressingMode::Absolute });
    // m.insert(0xF5, Opecode { code: Code::SBC, cycle: cycles[0xF5], mode: AddressingMode::ZeroPageIndexedX });
    // m.insert(0xFD, Opecode { code: Code::SBC, cycle: cycles[0xFD], mode: AddressingMode::AbsoluteIndexedX });
    // m.insert(0xF9, Opecode { code: Code::SBC, cycle: cycles[0xF9], mode: AddressingMode::AbsoluteIndexedY });
    // m.insert(0xE1, Opecode { code: Code::SBC, cycle: cycles[0xE1], mode: AddressingMode::IndexedIndirect });
    // m.insert(0xF1, Opecode { code: Code::SBC, cycle: cycles[0xF1], mode: AddressingMode::IndirectIndexed });
    // m.insert(0xE0, Opecode { code: Code::CPX, cycle: cycles[0xE0], mode: AddressingMode::Immediate });
    // m.insert(0xE4, Opecode { code: Code::CPX, cycle: cycles[0xE4], mode: AddressingMode::ZeroPage });
    // m.insert(0xEC, Opecode { code: Code::CPX, cycle: cycles[0xEC], mode: AddressingMode::Absolute });
    // m.insert(0xC0, Opecode { code: Code::CPY, cycle: cycles[0xC0], mode: AddressingMode::Immediate });
    // m.insert(0xC4, Opecode { code: Code::CPY, cycle: cycles[0xC4], mode: AddressingMode::ZeroPage });
    // m.insert(0xCC, Opecode { code: Code::CPY, cycle: cycles[0xCC], mode: AddressingMode::Absolute });
    // m.insert(0xC9, Opecode { code: Code::CMP, cycle: cycles[0xC9], mode: AddressingMode::Immediate });
    // m.insert(0xC5, Opecode { code: Code::CMP, cycle: cycles[0xC5], mode: AddressingMode::ZeroPage });
    // m.insert(0xCD, Opecode { code: Code::CMP, cycle: cycles[0xCD], mode: AddressingMode::Absolute });
    // m.insert(0xD5, Opecode { code: Code::CMP, cycle: cycles[0xD5], mode: AddressingMode::ZeroPageIndexedX });
    // m.insert(0xDD, Opecode { code: Code::CMP, cycle: cycles[0xDD], mode: AddressingMode::AbsoluteIndexedX });
    // m.insert(0xD9, Opecode { code: Code::CMP, cycle: cycles[0xD9], mode: AddressingMode::AbsoluteIndexedY });
    // m.insert(0xC1, Opecode { code: Code::CMP, cycle: cycles[0xC1], mode: AddressingMode::IndexedIndirect });
    // m.insert(0xD1, Opecode { code: Code::CMP, cycle: cycles[0xD1], mode: AddressingMode::IndirectIndexed });
    // m.insert(0x29, Opecode { code: Code::AND, cycle: cycles[0x29], mode: AddressingMode::Immediate });
    // m.insert(0x25, Opecode { code: Code::AND, cycle: cycles[0x25], mode: AddressingMode::ZeroPage });
    // m.insert(0x2D, Opecode { code: Code::AND, cycle: cycles[0x2D], mode: AddressingMode::Absolute });
    // m.insert(0x35, Opecode { code: Code::AND, cycle: cycles[0x35], mode: AddressingMode::ZeroPageIndexedX });
    // m.insert(0x3D, Opecode { code: Code::AND, cycle: cycles[0x3D], mode: AddressingMode::AbsoluteIndexedX });
    // m.insert(0x39, Opecode { code: Code::AND, cycle: cycles[0x39], mode: AddressingMode::AbsoluteIndexedY });
    // m.insert(0x21, Opecode { code: Code::AND, cycle: cycles[0x21], mode: AddressingMode::IndexedIndirect });
    // m.insert(0x31, Opecode { code: Code::AND, cycle: cycles[0x31], mode: AddressingMode::IndirectIndexed });
    // m.insert(0x49, Opecode { code: Code::EOR, cycle: cycles[0x49], mode: AddressingMode::Immediate });
    // m.insert(0x45, Opecode { code: Code::EOR, cycle: cycles[0x45], mode: AddressingMode::ZeroPage });
    // m.insert(0x4D, Opecode { code: Code::EOR, cycle: cycles[0x4D], mode: AddressingMode::Absolute });
    // m.insert(0x55, Opecode { code: Code::EOR, cycle: cycles[0x55], mode: AddressingMode::ZeroPageIndexedX });
    // m.insert(0x5D, Opecode { code: Code::EOR, cycle: cycles[0x5D], mode: AddressingMode::AbsoluteIndexedX });
    // m.insert(0x59, Opecode { code: Code::EOR, cycle: cycles[0x59], mode: AddressingMode::AbsoluteIndexedY });
    // m.insert(0x41, Opecode { code: Code::EOR, cycle: cycles[0x41], mode: AddressingMode::IndexedIndirect });
    // m.insert(0x51, Opecode { code: Code::EOR, cycle: cycles[0x51], mode: AddressingMode::IndirectIndexed });
    // m.insert(0x09, Opecode { code: Code::ORA, cycle: cycles[0x09], mode: AddressingMode::Immediate });
    // m.insert(0x05, Opecode { code: Code::ORA, cycle: cycles[0x05], mode: AddressingMode::ZeroPage });
    // m.insert(0x0D, Opecode { code: Code::ORA, cycle: cycles[0x0D], mode: AddressingMode::Absolute });
    // m.insert(0x15, Opecode { code: Code::ORA, cycle: cycles[0x15], mode: AddressingMode::ZeroPageIndexedX });
    // m.insert(0x1D, Opecode { code: Code::ORA, cycle: cycles[0x1D], mode: AddressingMode::AbsoluteIndexedX });
    // m.insert(0x19, Opecode { code: Code::ORA, cycle: cycles[0x19], mode: AddressingMode::AbsoluteIndexedY });
    // m.insert(0x01, Opecode { code: Code::ORA, cycle: cycles[0x01], mode: AddressingMode::IndexedIndirect });
    // m.insert(0x11, Opecode { code: Code::ORA, cycle: cycles[0x11], mode: AddressingMode::IndirectIndexed });
    // m.insert(0x24, Opecode { code: Code::BIT, cycle: cycles[0x24], mode: AddressingMode::ZeroPage });
    // m.insert(0x2C, Opecode { code: Code::BIT, cycle: cycles[0x2C], mode: AddressingMode::Absolute });
    // m.insert(0x0A, Opecode { code: Code::ASL, cycle: cycles[0x0A], mode: AddressingMode::Accumulator });
    // m.insert(0x06, Opecode { code: Code::ASL, cycle: cycles[0x06], mode: AddressingMode::ZeroPage });
    // m.insert(0x0E, Opecode { code: Code::ASL, cycle: cycles[0x0E], mode: AddressingMode::Absolute });
    // m.insert(0x16, Opecode { code: Code::ASL, cycle: cycles[0x16], mode: AddressingMode::ZeroPageIndexedX });
    // m.insert(0x1E, Opecode { code: Code::ASL, cycle: cycles[0x1E], mode: AddressingMode::AbsoluteIndexedX });
    // m.insert(0x4A, Opecode { code: Code::LSR, cycle: cycles[0x4A], mode: AddressingMode::Accumulator });
    // m.insert(0x46, Opecode { code: Code::LSR, cycle: cycles[0x46], mode: AddressingMode::ZeroPage });
    // m.insert(0x4E, Opecode { code: Code::LSR, cycle: cycles[0x4E], mode: AddressingMode::Absolute });
    // m.insert(0x56, Opecode { code: Code::LSR, cycle: cycles[0x56], mode: AddressingMode::ZeroPageIndexedX });
    // m.insert(0x5E, Opecode { code: Code::LSR, cycle: cycles[0x5E], mode: AddressingMode::AbsoluteIndexedX });
    // m.insert(0x2A, Opecode { code: Code::ROL, cycle: cycles[0x2A], mode: AddressingMode::Accumulator });
    // m.insert(0x26, Opecode { code: Code::ROL, cycle: cycles[0x26], mode: AddressingMode::ZeroPage });
    // m.insert(0x2E, Opecode { code: Code::ROL, cycle: cycles[0x2E], mode: AddressingMode::Absolute });
    // m.insert(0x36, Opecode { code: Code::ROL, cycle: cycles[0x36], mode: AddressingMode::ZeroPageIndexedX });
    // m.insert(0x3E, Opecode { code: Code::ROL, cycle: cycles[0x3E], mode: AddressingMode::AbsoluteIndexedX });
    // m.insert(0x6A, Opecode { code: Code::ROR, cycle: cycles[0x6A], mode: AddressingMode::Accumulator });
    // m.insert(0x66, Opecode { code: Code::ROR, cycle: cycles[0x66], mode: AddressingMode::ZeroPage });
    // m.insert(0x6E, Opecode { code: Code::ROR, cycle: cycles[0x6E], mode: AddressingMode::Absolute });
    // m.insert(0x76, Opecode { code: Code::ROR, cycle: cycles[0x76], mode: AddressingMode::ZeroPageIndexedX });
    // m.insert(0x7E, Opecode { code: Code::ROR, cycle: cycles[0x7E], mode: AddressingMode::AbsoluteIndexedX });
    m.insert(0xE8, Opecode { code: Code::INX, cycle: cycles[0xE8], mode: AddressingMode::Implied });
    // m.insert(0xC8, Opecode { code: Code::INY, cycle: cycles[0xC8], mode: AddressingMode::Implied });
    // m.insert(0xE6, Opecode { code: Code::INC, cycle: cycles[0xE6], mode: AddressingMode::ZeroPage });
    // m.insert(0xEE, Opecode { code: Code::INC, cycle: cycles[0xEE], mode: AddressingMode::Absolute });
    // m.insert(0xF6, Opecode { code: Code::INC, cycle: cycles[0xF6], mode: AddressingMode::ZeroPageIndexedX });
    // m.insert(0xFE, Opecode { code: Code::INC, cycle: cycles[0xFE], mode: AddressingMode::AbsoluteIndexedX });
    // m.insert(0xCA, Opecode { code: Code::DEX, cycle: cycles[0xCA], mode: AddressingMode::Implied });
    m.insert(0x88, Opecode { code: Code::DEY, cycle: cycles[0x88], mode: AddressingMode::Implied });
    // m.insert(0xC6, Opecode { code: Code::DEC, cycle: cycles[0xC6], mode: AddressingMode::ZeroPage });
    // m.insert(0xCE, Opecode { code: Code::DEC, cycle: cycles[0xCE], mode: AddressingMode::Absolute });
    // m.insert(0xD6, Opecode { code: Code::DEC, cycle: cycles[0xD6], mode: AddressingMode::ZeroPageIndexedX });
    // m.insert(0xDE, Opecode { code: Code::DEC, cycle: cycles[0xDE], mode: AddressingMode::AbsoluteIndexedX });
    // m.insert(0x18, Opecode { code: Code::CLC, cycle: cycles[0x18], mode: AddressingMode::Implied });
    // m.insert(0x58, Opecode { code: Code::CLI, cycle: cycles[0x58], mode: AddressingMode::Implied });
    // m.insert(0xB8, Opecode { code: Code::CLV, cycle: cycles[0xB8], mode: AddressingMode::Implied });
    // m.insert(0x38, Opecode { code: Code::SEC, cycle: cycles[0x38], mode: AddressingMode::Implied });
    m.insert(0x78, Opecode { code: Code::SEI, cycle: cycles[0x78], mode: AddressingMode::Implied });
    // m.insert(0xEA, Opecode { code: Code::NOP, cycle: cycles[0xEA], mode: AddressingMode::Implied });
    // m.insert(0x00, Opecode { code: Code::BRK, cycle: cycles[0x00], mode: AddressingMode::Implied });
    // m.insert(0x20, Opecode { code: Code::JSR, cycle: cycles[0x20], mode: AddressingMode::Absolute });
    m.insert(0x4C, Opecode { code: Code::JMP, cycle: cycles[0x4C], mode: AddressingMode::Absolute });
    m.insert(0x6C, Opecode { code: Code::JMP, cycle: cycles[0x6C], mode: AddressingMode::AbsoluteIndirect });
    // m.insert(0x40, Opecode { code: Code::RTI, cycle: cycles[0x40], mode: AddressingMode::Implied });
    // m.insert(0x60, Opecode { code: Code::RTS, cycle: cycles[0x60], mode: AddressingMode::Implied });
    // m.insert(0x10, Opecode { code: Code::BPL, cycle: cycles[0x10], mode: AddressingMode::Relative });
    // m.insert(0x30, Opecode { code: Code::BMI, cycle: cycles[0x30], mode: AddressingMode::Relative });
    // m.insert(0x50, Opecode { code: Code::BVC, cycle: cycles[0x50], mode: AddressingMode::Relative });
    // m.insert(0x70, Opecode { code: Code::BVS, cycle: cycles[0x70], mode: AddressingMode::Relative });
    // m.insert(0x90, Opecode { code: Code::BCC, cycle: cycles[0x90], mode: AddressingMode::Relative });
    // m.insert(0xB0, Opecode { code: Code::BCS, cycle: cycles[0xB0], mode: AddressingMode::Relative });
    m.insert(0xD0, Opecode { code: Code::BNE, cycle: cycles[0xD0], mode: AddressingMode::Relative });
    // m.insert(0xF0, Opecode { code: Code::BEQ, cycle: cycles[0xF0], mode: AddressingMode::Relative });
    // m.insert(0xF8, Opecode { code: Code::SED, cycle: cycles[0xF8], mode: AddressingMode::Implied });
    // m.insert(0xD8, Opecode { code: Code::CLD, cycle: cycles[0xD8], mode: AddressingMode::Implied });
    // m.insert(0x1A, Opecode { code: Code::NOP, cycle: cycles[0x1A], mode: AddressingMode::Implied });
    // m.insert(0x3A, Opecode { code: Code::NOP, cycle: cycles[0x3A], mode: AddressingMode::Implied });
    // m.insert(0x5A, Opecode { code: Code::NOP, cycle: cycles[0x5A], mode: AddressingMode::Implied });
    // m.insert(0x7A, Opecode { code: Code::NOP, cycle: cycles[0x7A], mode: AddressingMode::Implied });
    // m.insert(0xDA, Opecode { code: Code::NOP, cycle: cycles[0xDA], mode: AddressingMode::Implied });
    // m.insert(0xFA, Opecode { code: Code::NOP, cycle: cycles[0xFA], mode: AddressingMode::Implied });
    // m.insert(0x02, Opecode { code: Code::NOP, cycle: cycles[0x02], mode: AddressingMode::Implied });
    // m.insert(0x12, Opecode { code: Code::NOP, cycle: cycles[0x12], mode: AddressingMode::Implied });
    // m.insert(0x22, Opecode { code: Code::NOP, cycle: cycles[0x22], mode: AddressingMode::Implied });
    // m.insert(0x32, Opecode { code: Code::NOP, cycle: cycles[0x32], mode: AddressingMode::Implied });
    // m.insert(0x42, Opecode { code: Code::NOP, cycle: cycles[0x42], mode: AddressingMode::Implied });
    // m.insert(0x52, Opecode { code: Code::NOP, cycle: cycles[0x52], mode: AddressingMode::Implied });
    // m.insert(0x62, Opecode { code: Code::NOP, cycle: cycles[0x62], mode: AddressingMode::Implied });
    // m.insert(0x72, Opecode { code: Code::NOP, cycle: cycles[0x72], mode: AddressingMode::Implied });
    // m.insert(0x92, Opecode { code: Code::NOP, cycle: cycles[0x92], mode: AddressingMode::Implied });
    // m.insert(0xB2, Opecode { code: Code::NOP, cycle: cycles[0xB2], mode: AddressingMode::Implied });
    // m.insert(0xD2, Opecode { code: Code::NOP, cycle: cycles[0xD2], mode: AddressingMode::Implied });
    // m.insert(0xF2, Opecode { code: Code::NOP, cycle: cycles[0xF2], mode: AddressingMode::Implied });
    // m.insert(0x80, Opecode { code: Code::NOP, cycle: cycles[0x80], mode: AddressingMode::Implied });
    // m.insert(0x82, Opecode { code: Code::NOP, cycle: cycles[0x82], mode: AddressingMode::Implied });
    // m.insert(0x89, Opecode { code: Code::NOP, cycle: cycles[0x89], mode: AddressingMode::Implied });
    // m.insert(0xC2, Opecode { code: Code::NOP, cycle: cycles[0xC2], mode: AddressingMode::Implied });
    // m.insert(0xE2, Opecode { code: Code::NOP, cycle: cycles[0xE2], mode: AddressingMode::Implied });
    // m.insert(0x04, Opecode { code: Code::NOP, cycle: cycles[0x04], mode: AddressingMode::Implied });
    // m.insert(0x44, Opecode { code: Code::NOP, cycle: cycles[0x44], mode: AddressingMode::Implied });
    // m.insert(0x64, Opecode { code: Code::NOP, cycle: cycles[0x64], mode: AddressingMode::Implied });
    // m.insert(0x14, Opecode { code: Code::NOP, cycle: cycles[0x14], mode: AddressingMode::Implied });
    // m.insert(0x34, Opecode { code: Code::NOP, cycle: cycles[0x34], mode: AddressingMode::Implied });
    // m.insert(0x54, Opecode { code: Code::NOP, cycle: cycles[0x54], mode: AddressingMode::Implied });
    // m.insert(0x74, Opecode { code: Code::NOP, cycle: cycles[0x74], mode: AddressingMode::Implied });
    // m.insert(0xD4, Opecode { code: Code::NOP, cycle: cycles[0xD4], mode: AddressingMode::Implied });
    // m.insert(0xF4, Opecode { code: Code::NOP, cycle: cycles[0xF4], mode: AddressingMode::Implied });
    // m.insert(0x0C, Opecode { code: Code::NOP, cycle: cycles[0x0C], mode: AddressingMode::Implied });
    // m.insert(0x1C, Opecode { code: Code::NOP, cycle: cycles[0x1C], mode: AddressingMode::Implied });
    // m.insert(0x3C, Opecode { code: Code::NOP, cycle: cycles[0x3C], mode: AddressingMode::Implied });
    // m.insert(0x5C, Opecode { code: Code::NOP, cycle: cycles[0x5C], mode: AddressingMode::Implied });
    // m.insert(0x7C, Opecode { code: Code::NOP, cycle: cycles[0x7C], mode: AddressingMode::Implied });
    // m.insert(0xDC, Opecode { code: Code::NOP, cycle: cycles[0xDC], mode: AddressingMode::Implied });
    // m.insert(0xFC, Opecode { code: Code::NOP, cycle: cycles[0xFC], mode: AddressingMode::Implied });
    // m.insert(0xA7, Opecode { code: Code::LAX, cycle: cycles[0xA7], mode: AddressingMode::ZeroPage });
    // m.insert(0xB7, Opecode { code: Code::LAX, cycle: cycles[0xB7], mode: AddressingMode::ZeroPageIndexedY });
    // m.insert(0xAF, Opecode { code: Code::LAX, cycle: cycles[0xAF], mode: AddressingMode::Absolute });
    // m.insert(0xBF, Opecode { code: Code::LAX, cycle: cycles[0xBF], mode: AddressingMode::AbsoluteIndexedY });
    // m.insert(0xA3, Opecode { code: Code::LAX, cycle: cycles[0xA3], mode: AddressingMode::IndexedIndirect });
    // m.insert(0xB3, Opecode { code: Code::LAX, cycle: cycles[0xB3], mode: AddressingMode::IndirectIndexed });
    // m.insert(0x87, Opecode { code: Code::SAX, cycle: cycles[0x87], mode: AddressingMode::ZeroPage });
    // m.insert(0x97, Opecode { code: Code::SAX, cycle: cycles[0x97], mode: AddressingMode::ZeroPageIndexedY });
    // m.insert(0x8F, Opecode { code: Code::SAX, cycle: cycles[0x8F], mode: AddressingMode::Absolute });
    // m.insert(0x83, Opecode { code: Code::SAX, cycle: cycles[0x83], mode: AddressingMode::IndexedIndirect });
    // m.insert(0xEB, Opecode { code: Code::SBC, cycle: cycles[0xEB], mode: AddressingMode::Immediate });
    // m.insert(0xC7, Opecode { code: Code::DCP, cycle: cycles[0xC7], mode: AddressingMode::ZeroPage });
    // m.insert(0xD7, Opecode { code: Code::DCP, cycle: cycles[0xD7], mode: AddressingMode::ZeroPageIndexedX });
    // m.insert(0xCF, Opecode { code: Code::DCP, cycle: cycles[0xCF], mode: AddressingMode::Absolute });
    // m.insert(0xDF, Opecode { code: Code::DCP, cycle: cycles[0xDF], mode: AddressingMode::AbsoluteIndexedX });
    // m.insert(0xDB, Opecode { code: Code::DCP, cycle: cycles[0xDB], mode: AddressingMode::AbsoluteIndexedY });
    // m.insert(0xC3, Opecode { code: Code::DCP, cycle: cycles[0xC3], mode: AddressingMode::IndexedIndirect });
    // m.insert(0xD3, Opecode { code: Code::DCP, cycle: cycles[0xD3], mode: AddressingMode::IndirectIndexed });
    // m.insert(0xE7, Opecode { code: Code::ISB, cycle: cycles[0xE7], mode: AddressingMode::ZeroPage });
    // m.insert(0xF7, Opecode { code: Code::ISB, cycle: cycles[0xF7], mode: AddressingMode::ZeroPageIndexedX });
    // m.insert(0xEF, Opecode { code: Code::ISB, cycle: cycles[0xEF], mode: AddressingMode::Absolute });
    // m.insert(0xFF, Opecode { code: Code::ISB, cycle: cycles[0xFF], mode: AddressingMode::AbsoluteIndexedX });
    // m.insert(0xFB, Opecode { code: Code::ISB, cycle: cycles[0xFB], mode: AddressingMode::AbsoluteIndexedY });
    // m.insert(0xE3, Opecode { code: Code::ISB, cycle: cycles[0xE3], mode: AddressingMode::IndexedIndirect });
    // m.insert(0xF3, Opecode { code: Code::ISB, cycle: cycles[0xF3], mode: AddressingMode::IndirectIndexed });
    // m.insert(0x07, Opecode { code: Code::SLO, cycle: cycles[0x07], mode: AddressingMode::ZeroPage });
    // m.insert(0x17, Opecode { code: Code::SLO, cycle: cycles[0x17], mode: AddressingMode::ZeroPageIndexedX });
    // m.insert(0x0F, Opecode { code: Code::SLO, cycle: cycles[0x0F], mode: AddressingMode::Absolute });
    // m.insert(0x1F, Opecode { code: Code::SLO, cycle: cycles[0x1F], mode: AddressingMode::AbsoluteIndexedX });
    // m.insert(0x1B, Opecode { code: Code::SLO, cycle: cycles[0x1B], mode: AddressingMode::AbsoluteIndexedY });
    // m.insert(0x03, Opecode { code: Code::SLO, cycle: cycles[0x03], mode: AddressingMode::IndexedIndirect });
    // m.insert(0x13, Opecode { code: Code::SLO, cycle: cycles[0x13], mode: AddressingMode::IndirectIndexed });
    // m.insert(0x27, Opecode { code: Code::RLA, cycle: cycles[0x27], mode: AddressingMode::ZeroPage });
    // m.insert(0x37, Opecode { code: Code::RLA, cycle: cycles[0x37], mode: AddressingMode::ZeroPageIndexedX });
    // m.insert(0x2F, Opecode { code: Code::RLA, cycle: cycles[0x2F], mode: AddressingMode::Absolute });
    // m.insert(0x3F, Opecode { code: Code::RLA, cycle: cycles[0x3F], mode: AddressingMode::AbsoluteIndexedX });
    // m.insert(0x3B, Opecode { code: Code::RLA, cycle: cycles[0x3B], mode: AddressingMode::AbsoluteIndexedY });
    // m.insert(0x23, Opecode { code: Code::RLA, cycle: cycles[0x23], mode: AddressingMode::IndexedIndirect });
    // m.insert(0x33, Opecode { code: Code::RLA, cycle: cycles[0x33], mode: AddressingMode::IndirectIndexed });
    // m.insert(0x47, Opecode { code: Code::SRE, cycle: cycles[0x47], mode: AddressingMode::ZeroPage });
    // m.insert(0x57, Opecode { code: Code::SRE, cycle: cycles[0x57], mode: AddressingMode::ZeroPageIndexedX });
    // m.insert(0x4F, Opecode { code: Code::SRE, cycle: cycles[0x4F], mode: AddressingMode::Absolute });
    // m.insert(0x5F, Opecode { code: Code::SRE, cycle: cycles[0x5F], mode: AddressingMode::AbsoluteIndexedX });
    // m.insert(0x5B, Opecode { code: Code::SRE, cycle: cycles[0x5B], mode: AddressingMode::AbsoluteIndexedY });
    // m.insert(0x43, Opecode { code: Code::SRE, cycle: cycles[0x43], mode: AddressingMode::IndexedIndirect });
    // m.insert(0x53, Opecode { code: Code::SRE, cycle: cycles[0x53], mode: AddressingMode::IndirectIndexed });
    // m.insert(0x67, Opecode { code: Code::RRA, cycle: cycles[0x67], mode: AddressingMode::ZeroPage });
    // m.insert(0x77, Opecode { code: Code::RRA, cycle: cycles[0x77], mode: AddressingMode::ZeroPageIndexedX });
    // m.insert(0x6F, Opecode { code: Code::RRA, cycle: cycles[0x6F], mode: AddressingMode::Absolute });
    // m.insert(0x7F, Opecode { code: Code::RRA, cycle: cycles[0x7F], mode: AddressingMode::AbsoluteIndexedX });
    // m.insert(0x7B, Opecode { code: Code::RRA, cycle: cycles[0x7B], mode: AddressingMode::AbsoluteIndexedY });
    // m.insert(0x63, Opecode { code: Code::RRA, cycle: cycles[0x63], mode: AddressingMode::IndexedIndirect });
    // m.insert(0x73, Opecode { code: Code::RRA, cycle: cycles[0x73], mode: AddressingMode::IndirectIndexed });
    m
});
