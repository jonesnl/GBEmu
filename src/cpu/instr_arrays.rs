use cpu::*;

pub const INSTR_ARRAY_SIZE: usize = 256;

pub struct Instruction {
    pub opcode: u8,
    pub func: fn(&mut Cpu) -> Result<(), ()>,
    pub cycles: u8,
}

pub static INSTR: [Instruction ; INSTR_ARRAY_SIZE] = [
    Instruction {
        opcode: 0x00,
        func: noop_instr,
        cycles: 4,
    },
    Instruction {
        opcode: 0x01,
        func: ld_u16_imm_instr,
        cycles: 12,
    },
    Instruction {
        opcode: 0x02,
        func: ld_to_mem_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0x03,
        func: inc_u16_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0x04,
        func: inc_u8_instr,
        cycles: 4,
    },
    Instruction {
        opcode: 0x05,
        func: dec_u8_instr,
        cycles: 4,
    },
    Instruction {
        opcode: 0x06,
        func: ld_u8_imm_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0x07,
        func: rlca_instr,
        cycles: 4,
    },
    Instruction {
        opcode: 0x08,
        func: ld_sp_to_imm_mem_instr,
        cycles: 20,
    },
    Instruction {
        opcode: 0x09,
        func: add_hl_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0x0a,
        func: ld_from_mem_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0x0b,
        func: dec_u16_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0x0c,
        func: inc_u8_instr,
        cycles: 4,
    },
    Instruction {
        opcode: 0x0d,
        func: dec_u8_instr,
        cycles: 4,
    },
    Instruction {
        opcode: 0x0e,
        func: ld_u8_imm_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0x0f,
        func: rrca_instr,
        cycles: 4,
    },
    Instruction {
        opcode: 0x10,
        func: stop_instr,
        cycles: 4,
    },
    Instruction {
        opcode: 0x11,
        func: ld_u16_imm_instr,
        cycles: 12,
    },
    Instruction {
        opcode: 0x12,
        func: ld_to_mem_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0x13,
        func: inc_u16_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0x14,
        func: inc_u8_instr,
        cycles: 4,
    },
    Instruction {
        opcode: 0x15,
        func: dec_u8_instr,
        cycles: 4,
    },
    Instruction {
        opcode: 0x16,
        func: ld_u8_imm_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0x17,
        func: rla_instr,
        cycles: 4,
    },
    Instruction {
        opcode: 0x18,
        func: jr_imm8_instr,
        cycles: 12,
    },
    Instruction {
        opcode: 0x19,
        func: add_hl_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0x1a,
        func: ld_from_mem_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0x1b,
        func: dec_u16_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0x1c,
        func: inc_u8_instr,
        cycles: 4,
    },
    Instruction {
        opcode: 0x1d,
        func: dec_u8_instr,
        cycles: 4,
    },
    Instruction {
        opcode: 0x1e,
        func: ld_u8_imm_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0x1f,
        func: rrc_instr,
        cycles: 4,
    },
    Instruction {
        opcode: 0x20,
        func: jr_imm8_instr,
        cycles: 12, //TODO or 8?
    },
    Instruction {
        opcode: 0x21,
        func: ld_u16_imm_instr,
        cycles: 12,
    },
    Instruction {
        opcode: 0x22,
        func: ld_to_mem_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0x23,
        func: inc_u16_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0x24,
        func: inc_u8_instr,
        cycles: 4,
    },
    Instruction {
        opcode: 0x25,
        func: dec_u8_instr,
        cycles: 4,
    },
    Instruction {
        opcode: 0x26,
        func: ld_u8_imm_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0x27,
        func: daa_instr,
        cycles: 4,
    },
    Instruction {
        opcode: 0x28,
        func: jr_imm8_instr,
        cycles: 12, //TODO or 8?
    },
    Instruction {
        opcode: 0x29,
        func: add_hl_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0x2a,
        func: ld_from_mem_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0x2b,
        func: dec_u16_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0x2c,
        func: inc_u8_instr,
        cycles: 4,
    },
    Instruction {
        opcode: 0x2d,
        func: dec_u8_instr,
        cycles: 4,
    },
    Instruction {
        opcode: 0x2e,
        func: ld_u8_imm_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0x2f,
        func: cpl_instr,
        cycles: 4,
    },
    Instruction {
        opcode: 0x30,
        func: jr_imm8_instr,
        cycles: 12, //TODO or 8?
    },
    Instruction {
        opcode: 0x31,
        func: ld_u16_imm_instr,
        cycles: 12,
    },
    Instruction {
        opcode: 0x32,
        func: ld_to_mem_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0x33,
        func: inc_u16_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0x34,
        func: inc_u8_instr,
        cycles: 12,
    },
    Instruction {
        opcode: 0x35,
        func: dec_u8_instr,
        cycles: 12,
    },
    Instruction {
        opcode: 0x36,
        func: ld_u8_imm_instr,
        cycles: 12,
    },
    Instruction {
        opcode: 0x37,
        func: scf_instr,
        cycles: 4,
    },
    Instruction {
        opcode: 0x38,
        func: jr_imm8_instr,
        cycles: 12, // XXX or 8?
    },
    Instruction {
        opcode: 0x39,
        func: add_hl_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0x3a,
        func: ld_from_mem_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0x3b,
        func: dec_u16_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0x3c,
        func: inc_u8_instr,
        cycles: 4,
    },
    Instruction {
        opcode: 0x3d,
        func: dec_u8_instr,
        cycles: 4,
    },
    Instruction {
        opcode: 0x3e,
        func: ld_u8_imm_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0x3f,
        func: ccf_instr,
        cycles: 4,
    },
    Instruction {
        opcode: 0x40,
        func: ld_instr,
        cycles: 4,
    },
    Instruction {
        opcode: 0x41,
        func: ld_instr,
        cycles: 4,
    },
    Instruction {
        opcode: 0x42,
        func: ld_instr,
        cycles: 4,
    },
    Instruction {
        opcode: 0x43,
        func: ld_instr,
        cycles: 4,
    },
    Instruction {
        opcode: 0x44,
        func: ld_instr,
        cycles: 4,
    },
    Instruction {
        opcode: 0x45,
        func: ld_instr,
        cycles: 4,
    },
    Instruction {
        opcode: 0x46,
        func: ld_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0x47,
        func: ld_instr,
        cycles: 4,
    },
    Instruction {
        opcode: 0x48,
        func: ld_instr,
        cycles: 4,
    },
    Instruction {
        opcode: 0x49,
        func: ld_instr,
        cycles: 4,
    },
    Instruction {
        opcode: 0x4a,
        func: ld_instr,
        cycles: 4,
    },
    Instruction {
        opcode: 0x4b,
        func: ld_instr,
        cycles: 4,
    },
    Instruction {
        opcode: 0x4c,
        func: ld_instr,
        cycles: 4,
    },
    Instruction {
        opcode: 0x4d,
        func: ld_instr,
        cycles: 4,
    },
    Instruction {
        opcode: 0x4e,
        func: ld_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0x4f,
        func: ld_instr,
        cycles: 4,
    },
    Instruction {
        opcode: 0x50,
        func: ld_instr,
        cycles: 4,
    },
    Instruction {
        opcode: 0x51,
        func: ld_instr,
        cycles: 4,
    },
    Instruction {
        opcode: 0x52,
        func: ld_instr,
        cycles: 4,
    },
    Instruction {
        opcode: 0x53,
        func: ld_instr,
        cycles: 4,
    },
    Instruction {
        opcode: 0x54,
        func: ld_instr,
        cycles: 4,
    },
    Instruction {
        opcode: 0x55,
        func: ld_instr,
        cycles: 4,
    },
    Instruction {
        opcode: 0x56,
        func: ld_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0x57,
        func: ld_instr,
        cycles: 4,
    },
    Instruction {
        opcode: 0x58,
        func: ld_instr,
        cycles: 4,
    },
    Instruction {
        opcode: 0x59,
        func: ld_instr,
        cycles: 4,
    },
    Instruction {
        opcode: 0x5a,
        func: ld_instr,
        cycles: 4,
    },
    Instruction {
        opcode: 0x5b,
        func: ld_instr,
        cycles: 4,
    },
    Instruction {
        opcode: 0x5c,
        func: ld_instr,
        cycles: 4,
    },
    Instruction {
        opcode: 0x5d,
        func: ld_instr,
        cycles: 4,
    },
    Instruction {
        opcode: 0x5e,
        func: ld_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0x5f,
        func: ld_instr,
        cycles: 4,
    },
    Instruction {
        opcode: 0x60,
        func: ld_instr,
        cycles: 4,
    },
    Instruction {
        opcode: 0x61,
        func: ld_instr,
        cycles: 4,
    },
    Instruction {
        opcode: 0x62,
        func: ld_instr,
        cycles: 4,
    },
    Instruction {
        opcode: 0x63,
        func: ld_instr,
        cycles: 4,
    },
    Instruction {
        opcode: 0x64,
        func: ld_instr,
        cycles: 4,
    },
    Instruction {
        opcode: 0x65,
        func: ld_instr,
        cycles: 4,
    },
    Instruction {
        opcode: 0x66,
        func: ld_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0x67,
        func: ld_instr,
        cycles: 4,
    },
    Instruction {
        opcode: 0x68,
        func: ld_instr,
        cycles: 4,
    },
    Instruction {
        opcode: 0x69,
        func: ld_instr,
        cycles: 4,
    },
    Instruction {
        opcode: 0x6a,
        func: ld_instr,
        cycles: 4,
    },
    Instruction {
        opcode: 0x6b,
        func: ld_instr,
        cycles: 4,
    },
    Instruction {
        opcode: 0x6c,
        func: ld_instr,
        cycles: 4,
    },
    Instruction {
        opcode: 0x6d,
        func: ld_instr,
        cycles: 4,
    },
    Instruction {
        opcode: 0x6e,
        func: ld_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0x6f,
        func: ld_instr,
        cycles: 4,
    },
    Instruction {
        opcode: 0x70,
        func: ld_instr,
        cycles: 4,
    },
    Instruction {
        opcode: 0x71,
        func: ld_instr,
        cycles: 4,
    },
    Instruction {
        opcode: 0x72,
        func: ld_instr,
        cycles: 4,
    },
    Instruction {
        opcode: 0x73,
        func: ld_instr,
        cycles: 4,
    },
    Instruction {
        opcode: 0x74,
        func: ld_instr,
        cycles: 4,
    },
    Instruction {
        opcode: 0x75,
        func: ld_instr,
        cycles: 4,
    },
    Instruction {
        opcode: 0x76,
        func: halt_instr,
        cycles: 4,
    },
    Instruction {
        opcode: 0x77,
        func: ld_instr,
        cycles: 4,
    },
    Instruction {
        opcode: 0x78,
        func: ld_instr,
        cycles: 4,
    },
    Instruction {
        opcode: 0x79,
        func: ld_instr,
        cycles: 4,
    },
    Instruction {
        opcode: 0x7a,
        func: ld_instr,
        cycles: 4,
    },
    Instruction {
        opcode: 0x7b,
        func: ld_instr,
        cycles: 4,
    },
    Instruction {
        opcode: 0x7c,
        func: ld_instr,
        cycles: 4,
    },
    Instruction {
        opcode: 0x7d,
        func: ld_instr,
        cycles: 4,
    },
    Instruction {
        opcode: 0x7e,
        func: ld_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0x7f,
        func: ld_instr,
        cycles: 4,
    },
    Instruction {
        opcode: 0x80,
        func: add_instr,
        cycles: 4,
    },
    Instruction {
        opcode: 0x81,
        func: add_instr,
        cycles: 4,
    },
    Instruction {
        opcode: 0x82,
        func: add_instr,
        cycles: 4,
    },
    Instruction {
        opcode: 0x83,
        func: add_instr,
        cycles: 4,
    },
    Instruction {
        opcode: 0x84,
        func: add_instr,
        cycles: 4,
    },
    Instruction {
        opcode: 0x85,
        func: add_instr,
        cycles: 4,
    },
    Instruction {
        opcode: 0x86,
        func: add_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0x87,
        func: add_instr,
        cycles: 4,
    },
    Instruction {
        opcode: 0x88,
        func: adc_instr,
        cycles: 4,
    },
    Instruction {
        opcode: 0x89,
        func: adc_instr,
        cycles: 4,
    },
    Instruction {
        opcode: 0x8a,
        func: adc_instr,
        cycles: 4,
    },
    Instruction {
        opcode: 0x8b,
        func: adc_instr,
        cycles: 4,
    },
    Instruction {
        opcode: 0x8c,
        func: adc_instr,
        cycles: 4,
    },
    Instruction {
        opcode: 0x8d,
        func: adc_instr,
        cycles: 4,
    },
    Instruction {
        opcode: 0x8e,
        func: adc_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0x8f,
        func: adc_instr,
        cycles: 4,
    },
    Instruction {
        opcode: 0x90,
        func: sub_instr,
        cycles: 4,
    },
    Instruction {
        opcode: 0x91,
        func: sub_instr,
        cycles: 4,
    },
    Instruction {
        opcode: 0x92,
        func: sub_instr,
        cycles: 4,
    },
    Instruction {
        opcode: 0x93,
        func: sub_instr,
        cycles: 4,
    },
    Instruction {
        opcode: 0x94,
        func: sub_instr,
        cycles: 4,
    },
    Instruction {
        opcode: 0x95,
        func: sub_instr,
        cycles: 4,
    },
    Instruction {
        opcode: 0x96,
        func: sub_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0x97,
        func: sub_instr,
        cycles: 4,
    },
    Instruction {
        opcode: 0x98,
        func: sbc_instr,
        cycles: 4,
    },
    Instruction {
        opcode: 0x99,
        func: sbc_instr,
        cycles: 4,
    },
    Instruction {
        opcode: 0x9a,
        func: sbc_instr,
        cycles: 4,
    },
    Instruction {
        opcode: 0x9b,
        func: sbc_instr,
        cycles: 4,
    },
    Instruction {
        opcode: 0x9c,
        func: sbc_instr,
        cycles: 4,
    },
    Instruction {
        opcode: 0x9d,
        func: sbc_instr,
        cycles: 4,
    },
    Instruction {
        opcode: 0x9e,
        func: sbc_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0x9f,
        func: sbc_instr,
        cycles: 4,
    },
    Instruction {
        opcode: 0xa0,
        func: and_instr,
        cycles: 4,
    },
    Instruction {
        opcode: 0xa1,
        func: and_instr,
        cycles: 4,
    },
    Instruction {
        opcode: 0xa2,
        func: and_instr,
        cycles: 4,
    },
    Instruction {
        opcode: 0xa3,
        func: and_instr,
        cycles: 4,
    },
    Instruction {
        opcode: 0xa4,
        func: and_instr,
        cycles: 4,
    },
    Instruction {
        opcode: 0xa5,
        func: and_instr,
        cycles: 4,
    },
    Instruction {
        opcode: 0xa6,
        func: and_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0xa7,
        func: and_instr,
        cycles: 4,
    },
    Instruction {
        opcode: 0xa8,
        func: xor_instr,
        cycles: 4,
    },
    Instruction {
        opcode: 0xa9,
        func: xor_instr,
        cycles: 4,
    },
    Instruction {
        opcode: 0xaa,
        func: xor_instr,
        cycles: 4,
    },
    Instruction {
        opcode: 0xab,
        func: xor_instr,
        cycles: 4,
    },
    Instruction {
        opcode: 0xac,
        func: xor_instr,
        cycles: 4,
    },
    Instruction {
        opcode: 0xad,
        func: xor_instr,
        cycles: 4,
    },
    Instruction {
        opcode: 0xae,
        func: xor_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0xaf,
        func: xor_instr,
        cycles: 4,
    },
    Instruction {
        opcode: 0xb0,
        func: or_instr,
        cycles: 4,
    },
    Instruction {
        opcode: 0xb1,
        func: or_instr,
        cycles: 4,
    },
    Instruction {
        opcode: 0xb2,
        func: or_instr,
        cycles: 4,
    },
    Instruction {
        opcode: 0xb3,
        func: or_instr,
        cycles: 4,
    },
    Instruction {
        opcode: 0xb4,
        func: or_instr,
        cycles: 4,
    },
    Instruction {
        opcode: 0xb5,
        func: or_instr,
        cycles: 4,
    },
    Instruction {
        opcode: 0xb6,
        func: or_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0xb7,
        func: or_instr,
        cycles: 4,
    },
    Instruction {
        opcode: 0xb8,
        func: cp_instr,
        cycles: 4,
    },
    Instruction {
        opcode: 0xb9,
        func: cp_instr,
        cycles: 4,
    },
    Instruction {
        opcode: 0xba,
        func: cp_instr,
        cycles: 4,
    },
    Instruction {
        opcode: 0xbb,
        func: cp_instr,
        cycles: 4,
    },
    Instruction {
        opcode: 0xbc,
        func: cp_instr,
        cycles: 4,
    },
    Instruction {
        opcode: 0xbd,
        func: cp_instr,
        cycles: 4,
    },
    Instruction {
        opcode: 0xbe,
        func: cp_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0xbf,
        func: cp_instr,
        cycles: 4,
    },
    Instruction {
        opcode: 0xc0,
        func: ret_instr,
        cycles: 20, // XXX or 8?
    },
    Instruction {
        opcode: 0xc1,
        func: pop_instr,
        cycles: 12,
    },
    Instruction {
        opcode: 0xc2,
        func: jp_imm16_instr,
        cycles: 16, // XXX or 12?
    },
    Instruction {
        opcode: 0xc3,
        func: jp_imm16_instr,
        cycles: 16,
    },
    Instruction {
        opcode: 0xc4,
        func: call_instr,
        cycles: 24, // XXX or 12?
    },
    Instruction {
        opcode: 0xc5,
        func: push_instr,
        cycles: 16,
    },
    Instruction {
        opcode: 0xc6,
        func: add_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0xc7,
        func: restart_instr,
        cycles: 16,
    },
    Instruction {
        opcode: 0xc8,
        func: ret_instr,
        cycles: 20, // XXX or 8?
    },
    Instruction {
        opcode: 0xc9,
        func: ret_instr,
        cycles: 16,
    },
    Instruction {
        opcode: 0xca,
        func: jp_imm16_instr,
        cycles: 16, // XXX or 12?
    },
    Instruction {
        opcode: 0xcb,
        func: cb_instr,
        cycles: 4,
    },
    Instruction {
        opcode: 0xcc,
        func: call_instr,
        cycles: 24, // XXX or 12?
    },
    Instruction {
        opcode: 0xcd,
        func: call_instr,
        cycles: 24,
    },
    Instruction {
        opcode: 0xce,
        func: adc_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0xcf,
        func: restart_instr,
        cycles: 16,
    },
    Instruction {
        opcode: 0xd0,
        func: ret_instr,
        cycles: 20, // XXX or 8?
    },
    Instruction {
        opcode: 0xd1,
        func: pop_instr,
        cycles: 12,
    },
    Instruction {
        opcode: 0xd2,
        func: jp_imm16_instr,
        cycles: 16, // XXX or 12?
    },
    Instruction {
        opcode: 0xd3,
        func: undef_instr,
        cycles: 0,
    },
    Instruction {
        opcode: 0xd4,
        func: call_instr,
        cycles: 24, // XXX or 12?
    },
    Instruction {
        opcode: 0xd5,
        func: push_instr,
        cycles: 16,
    },
    Instruction {
        opcode: 0xd6,
        func: sub_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0xd7,
        func: restart_instr,
        cycles: 16,
    },
    Instruction {
        opcode: 0xd8,
        func: ret_instr,
        cycles: 20, // XXX or 8?
    },
    Instruction {
        opcode: 0xd9,
        func: ret_instr,
        cycles: 16,
    },
    Instruction {
        opcode: 0xda,
        func: jp_imm16_instr,
        cycles: 16, // XXX or 12?
    },
    Instruction {
        opcode: 0xdb,
        func: undef_instr,
        cycles: 0,
    },
    Instruction {
        opcode: 0xdc,
        func: call_instr,
        cycles: 24, // XXX or 24?
    },
    Instruction {
        opcode: 0xdd,
        func: undef_instr,
        cycles: 0,
    },
    Instruction {
        opcode: 0xde,
        func: sbc_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0xdf,
        func: restart_instr,
        cycles: 16,
    },
    Instruction {
        opcode: 0xe0,
        func: ld_from_a_to_mem_instr,
        cycles: 12,
    },
    Instruction {
        opcode: 0xe1,
        func: pop_instr,
        cycles: 12,
    },
    Instruction {
        opcode: 0xe2,
        func: ld_from_a_to_mem_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0xe3,
        func: undef_instr,
        cycles: 0,
    },
    Instruction {
        opcode: 0xe4,
        func: undef_instr,
        cycles: 0,
    },
    Instruction {
        opcode: 0xe5,
        func: push_instr,
        cycles: 16,
    },
    Instruction {
        opcode: 0xe6,
        func: and_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0xe7,
        func: restart_instr,
        cycles: 16,
    },
    Instruction {
        opcode: 0xe8,
        func: add_sp_instr,
        cycles: 16,
    },
    Instruction {
        opcode: 0xe9,
        func: jp_hl_instr,
        cycles: 4,
    },
    Instruction {
        opcode: 0xea,
        func: ld_from_a_to_mem_instr,
        cycles: 16,
    },
    Instruction {
        opcode: 0xeb,
        func: undef_instr,
        cycles: 0,
    },
    Instruction {
        opcode: 0xec,
        func: undef_instr,
        cycles: 0,
    },
    Instruction {
        opcode: 0xed,
        func: undef_instr,
        cycles: 0,
    },
    Instruction {
        opcode: 0xee,
        func: xor_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0xef,
        func: restart_instr,
        cycles: 16,
    },
    Instruction {
        opcode: 0xf0,
        func: ld_from_mem_to_a_instr,
        cycles: 12,
    },
    Instruction {
        opcode: 0xf1,
        func: pop_instr,
        cycles: 12,
    },
    Instruction {
        opcode: 0xf2,
        func: ld_from_mem_to_a_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0xf3,
        func: di_instr,
        cycles: 4,
    },
    Instruction {
        opcode: 0xf4,
        func: undef_instr,
        cycles: 0,
    },
    Instruction {
        opcode: 0xf5,
        func: push_instr,
        cycles: 16,
    },
    Instruction {
        opcode: 0xf6,
        func: or_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0xf7,
        func: restart_instr,
        cycles: 16,
    },
    Instruction {
        opcode: 0xf8,
        func: ld_sp_plus_signed_imm_to_hl_instr,
        cycles: 12,
    },
    Instruction {
        opcode: 0xf9,
        func: ld_hl_to_sp_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0xfa,
        func: ld_from_mem_to_a_instr,
        cycles: 16,
    },
    Instruction {
        opcode: 0xfb,
        func: ei_instr,
        cycles: 4,
    },
    Instruction {
        opcode: 0xfc,
        func: undef_instr,
        cycles: 0,
    },
    Instruction {
        opcode: 0xfd,
        func: undef_instr,
        cycles: 0,
    },
    Instruction {
        opcode: 0xfe,
        func: cp_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0xff,
        func: restart_instr,
        cycles: 16,
    },
];

pub static CB_INSTR: [Instruction ; INSTR_ARRAY_SIZE] =
[
    Instruction {
        opcode: 0x00,
        func: rlc_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0x01,
        func: rlc_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0x02,
        func: rlc_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0x03,
        func: rlc_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0x04,
        func: rlc_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0x05,
        func: rlc_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0x06,
        func: rlc_instr,
        cycles: 16,
    },
    Instruction {
        opcode: 0x07,
        func: rlc_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0x08,
        func: rrc_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0x09,
        func: rrc_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0x0a,
        func: rrc_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0x0b,
        func: rrc_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0x0c,
        func: rrc_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0x0d,
        func: rrc_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0x0e,
        func: rrc_instr,
        cycles: 16,
    },
    Instruction {
        opcode: 0x0f,
        func: rrc_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0x10,
        func: rl_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0x11,
        func: rl_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0x12,
        func: rl_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0x13,
        func: rl_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0x14,
        func: rl_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0x15,
        func: rl_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0x16,
        func: rl_instr,
        cycles: 16,
    },
    Instruction {
        opcode: 0x17,
        func: rl_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0x18,
        func: rr_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0x19,
        func: rr_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0x1a,
        func: rr_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0x1b,
        func: rr_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0x1c,
        func: rr_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0x1d,
        func: rr_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0x1e,
        func: rr_instr,
        cycles: 16,
    },
    Instruction {
        opcode: 0x1f,
        func: rr_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0x20,
        func: sla_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0x21,
        func: sla_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0x22,
        func: sla_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0x23,
        func: sla_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0x24,
        func: sla_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0x25,
        func: sla_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0x26,
        func: sla_instr,
        cycles: 16,
    },
    Instruction {
        opcode: 0x27,
        func: sla_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0x28,
        func: sra_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0x29,
        func: sra_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0x2a,
        func: sra_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0x2b,
        func: sra_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0x2c,
        func: sra_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0x2d,
        func: sra_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0x2e,
        func: sra_instr,
        cycles: 16,
    },
    Instruction {
        opcode: 0x2f,
        func: sra_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0x30,
        func: swap_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0x31,
        func: swap_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0x32,
        func: swap_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0x33,
        func: swap_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0x34,
        func: swap_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0x35,
        func: swap_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0x36,
        func: swap_instr,
        cycles: 16,
    },
    Instruction {
        opcode: 0x37,
        func: swap_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0x38,
        func: srl_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0x39,
        func: srl_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0x3a,
        func: srl_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0x3b,
        func: srl_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0x3c,
        func: srl_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0x3d,
        func: srl_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0x3e,
        func: srl_instr,
        cycles: 16,
    },
    Instruction {
        opcode: 0x3f,
        func: srl_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0x40,
        func: bit0_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0x41,
        func: bit0_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0x42,
        func: bit0_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0x43,
        func: bit0_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0x44,
        func: bit0_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0x45,
        func: bit0_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0x46,
        func: bit0_instr,
        cycles: 16,
    },
    Instruction {
        opcode: 0x47,
        func: bit0_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0x48,
        func: bit1_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0x49,
        func: bit1_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0x4a,
        func: bit1_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0x4b,
        func: bit1_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0x4c,
        func: bit1_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0x4d,
        func: bit1_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0x4e,
        func: bit1_instr,
        cycles: 16,
    },
    Instruction {
        opcode: 0x4f,
        func: bit1_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0x50,
        func: bit2_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0x51,
        func: bit2_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0x52,
        func: bit2_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0x53,
        func: bit2_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0x54,
        func: bit2_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0x55,
        func: bit2_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0x56,
        func: bit2_instr,
        cycles: 16,
    },
    Instruction {
        opcode: 0x57,
        func: bit2_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0x58,
        func: bit3_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0x59,
        func: bit3_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0x5a,
        func: bit3_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0x5b,
        func: bit3_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0x5c,
        func: bit3_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0x5d,
        func: bit3_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0x5e,
        func: bit3_instr,
        cycles: 16,
    },
    Instruction {
        opcode: 0x5f,
        func: bit3_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0x60,
        func: bit4_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0x61,
        func: bit4_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0x62,
        func: bit4_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0x63,
        func: bit4_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0x64,
        func: bit4_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0x65,
        func: bit4_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0x66,
        func: bit4_instr,
        cycles: 16,
    },
    Instruction {
        opcode: 0x67,
        func: bit4_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0x68,
        func: bit5_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0x69,
        func: bit5_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0x6a,
        func: bit5_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0x6b,
        func: bit5_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0x6c,
        func: bit5_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0x6d,
        func: bit5_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0x6e,
        func: bit5_instr,
        cycles: 16,
    },
    Instruction {
        opcode: 0x6f,
        func: bit5_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0x70,
        func: bit6_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0x71,
        func: bit6_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0x72,
        func: bit6_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0x73,
        func: bit6_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0x74,
        func: bit6_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0x75,
        func: bit6_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0x76,
        func: bit6_instr,
        cycles: 16,
    },
    Instruction {
        opcode: 0x77,
        func: bit6_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0x78,
        func: bit7_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0x79,
        func: bit7_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0x7a,
        func: bit7_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0x7b,
        func: bit7_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0x7c,
        func: bit7_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0x7d,
        func: bit7_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0x7e,
        func: bit7_instr,
        cycles: 16,
    },
    Instruction {
        opcode: 0x7f,
        func: bit7_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0x80,
        func: res0_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0x81,
        func: res0_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0x82,
        func: res0_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0x83,
        func: res0_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0x84,
        func: res0_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0x85,
        func: res0_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0x86,
        func: res0_instr,
        cycles: 16,
    },
    Instruction {
        opcode: 0x87,
        func: res0_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0x88,
        func: res1_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0x89,
        func: res1_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0x8a,
        func: res1_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0x8b,
        func: res1_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0x8c,
        func: res1_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0x8d,
        func: res1_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0x8e,
        func: res1_instr,
        cycles: 16,
    },
    Instruction {
        opcode: 0x8f,
        func: res1_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0x90,
        func: res2_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0x91,
        func: res2_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0x92,
        func: res2_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0x93,
        func: res2_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0x94,
        func: res2_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0x95,
        func: res2_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0x96,
        func: res2_instr,
        cycles: 16,
    },
    Instruction {
        opcode: 0x97,
        func: res2_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0x98,
        func: res3_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0x99,
        func: res3_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0x9a,
        func: res3_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0x9b,
        func: res3_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0x9c,
        func: res3_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0x9d,
        func: res3_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0x9e,
        func: res3_instr,
        cycles: 16,
    },
    Instruction {
        opcode: 0x9f,
        func: res3_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0xa0,
        func: res4_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0xa1,
        func: res4_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0xa2,
        func: res4_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0xa3,
        func: res4_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0xa4,
        func: res4_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0xa5,
        func: res4_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0xa6,
        func: res4_instr,
        cycles: 16,
    },
    Instruction {
        opcode: 0xa7,
        func: res4_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0xa8,
        func: res5_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0xa9,
        func: res5_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0xaa,
        func: res5_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0xab,
        func: res5_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0xac,
        func: res5_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0xad,
        func: res5_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0xae,
        func: res5_instr,
        cycles: 16,
    },
    Instruction {
        opcode: 0xaf,
        func: res5_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0xb0,
        func: res6_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0xb1,
        func: res6_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0xb2,
        func: res6_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0xb3,
        func: res6_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0xb4,
        func: res6_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0xb5,
        func: res6_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0xb6,
        func: res6_instr,
        cycles: 16,
    },
    Instruction {
        opcode: 0xb7,
        func: res6_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0xb8,
        func: res7_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0xb9,
        func: res7_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0xba,
        func: res7_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0xbb,
        func: res7_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0xbc,
        func: res7_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0xbd,
        func: res7_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0xbe,
        func: res7_instr,
        cycles: 16,
    },
    Instruction {
        opcode: 0xbf,
        func: res7_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0xc0,
        func: set0_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0xc1,
        func: set0_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0xc2,
        func: set0_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0xc3,
        func: set0_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0xc4,
        func: set0_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0xc5,
        func: set0_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0xc6,
        func: set0_instr,
        cycles: 16,
    },
    Instruction {
        opcode: 0xc7,
        func: set0_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0xc8,
        func: set1_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0xc9,
        func: set1_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0xca,
        func: set1_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0xcb,
        func: set1_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0xcc,
        func: set1_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0xcd,
        func: set1_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0xce,
        func: set1_instr,
        cycles: 16,
    },
    Instruction {
        opcode: 0xcf,
        func: set1_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0xd0,
        func: set2_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0xd1,
        func: set2_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0xd2,
        func: set2_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0xd3,
        func: set2_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0xd4,
        func: set2_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0xd5,
        func: set2_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0xd6,
        func: set2_instr,
        cycles: 16,
    },
    Instruction {
        opcode: 0xd7,
        func: set2_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0xd8,
        func: set3_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0xd9,
        func: set3_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0xda,
        func: set3_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0xdb,
        func: set3_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0xdc,
        func: set3_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0xdd,
        func: set3_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0xde,
        func: set3_instr,
        cycles: 16,
    },
    Instruction {
        opcode: 0xdf,
        func: set3_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0xe0,
        func: set4_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0xe1,
        func: set4_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0xe2,
        func: set4_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0xe3,
        func: set4_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0xe4,
        func: set4_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0xe5,
        func: set4_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0xe6,
        func: set4_instr,
        cycles: 16,
    },
    Instruction {
        opcode: 0xe7,
        func: set4_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0xe8,
        func: set5_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0xe9,
        func: set5_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0xea,
        func: set5_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0xeb,
        func: set5_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0xec,
        func: set5_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0xed,
        func: set5_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0xee,
        func: set5_instr,
        cycles: 16,
    },
    Instruction {
        opcode: 0xef,
        func: set5_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0xf0,
        func: set6_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0xf1,
        func: set6_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0xf2,
        func: set6_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0xf3,
        func: set6_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0xf4,
        func: set6_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0xf5,
        func: set6_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0xf6,
        func: set6_instr,
        cycles: 16,
    },
    Instruction {
        opcode: 0xf7,
        func: set6_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0xf8,
        func: set7_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0xf9,
        func: set7_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0xfa,
        func: set7_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0xfb,
        func: set7_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0xfc,
        func: set7_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0xfd,
        func: set7_instr,
        cycles: 8,
    },
    Instruction {
        opcode: 0xfe,
        func: set7_instr,
        cycles: 16,
    },
    Instruction {
        opcode: 0xff,
        func: set7_instr,
        cycles: 8,
    },
];
