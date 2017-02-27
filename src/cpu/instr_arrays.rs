use cpu::*;

pub const INSTR_ARRAY_SIZE: usize = 256;

pub static INSTR: [fn(&mut Cpu) -> Result<(), ()> ; INSTR_ARRAY_SIZE] =
        [noop_instr, // 0x00
        ld_u16_imm_instr,
        ld_to_mem_instr,
        inc_u16_instr,
        inc_u8_instr,
        dec_u8_instr,
        ld_u8_imm_instr,
        rlca_instr,
        ld_sp_to_imm_mem_instr,
        add_hl_instr,
        ld_from_mem_instr,
        dec_u16_instr,
        inc_u8_instr,
        dec_u8_instr,
        ld_u8_imm_instr,
        noop_instr,
        stop_instr, // 0x10
        ld_u16_imm_instr,
        ld_to_mem_instr,
        inc_u16_instr,
        inc_u8_instr,
        dec_u8_instr,
        ld_u8_imm_instr,
        rlc_instr,
        jr_imm8_instr,
        add_hl_instr,
        ld_from_mem_instr,
        dec_u16_instr,
        inc_u8_instr,
        dec_u8_instr,
        ld_u8_imm_instr,
        noop_instr,
        jr_imm8_instr, // 0x20
        ld_u16_imm_instr,
        ld_to_mem_instr,
        inc_u16_instr,
        inc_u8_instr,
        dec_u8_instr,
        ld_u8_imm_instr,
        daa_instr,
        jr_imm8_instr,
        add_hl_instr,
        ld_from_mem_instr,
        dec_u16_instr,
        inc_u8_instr,
        dec_u8_instr,
        ld_u8_imm_instr,
        noop_instr,
        jr_imm8_instr, // 0x30
        ld_u16_imm_instr,
        ld_to_mem_instr,
        inc_u16_instr,
        inc_u8_instr,
        dec_u8_instr,
        ld_u8_imm_instr,
        add_hl_instr,
        jr_imm8_instr,
        noop_instr,
        ld_from_mem_instr,
        dec_u16_instr,
        inc_u8_instr,
        dec_u8_instr,
        ld_u8_imm_instr,
        noop_instr,
        ld_instr, // 0x40
        ld_instr,
        ld_instr,
        ld_instr,
        ld_instr,
        ld_instr,
        ld_instr,
        ld_instr,
        ld_instr,
        ld_instr,
        ld_instr,
        ld_instr,
        ld_instr,
        ld_instr,
        ld_instr,
        ld_instr,
        ld_instr, // 0x50
        ld_instr,
        ld_instr,
        ld_instr,
        ld_instr,
        ld_instr,
        ld_instr,
        ld_instr,
        ld_instr,
        ld_instr,
        ld_instr,
        ld_instr,
        ld_instr,
        ld_instr,
        ld_instr,
        ld_instr,
        ld_instr, // 0x60
        ld_instr,
        ld_instr,
        ld_instr,
        ld_instr,
        ld_instr,
        ld_instr,
        ld_instr,
        ld_instr,
        ld_instr,
        ld_instr,
        ld_instr,
        ld_instr,
        ld_instr,
        ld_instr,
        ld_instr,
        ld_instr, // 0x70
        ld_instr,
        ld_instr,
        ld_instr,
        ld_instr,
        ld_instr,
        noop_instr,
        ld_instr,
        ld_instr,
        ld_instr,
        ld_instr,
        ld_instr,
        ld_instr,
        ld_instr,
        ld_instr,
        ld_instr,
        add_instr, // 0x80
        add_instr,
        add_instr,
        add_instr,
        add_instr,
        add_instr,
        add_instr,
        add_instr,
        adc_instr,
        adc_instr,
        adc_instr,
        adc_instr,
        adc_instr,
        adc_instr,
        adc_instr,
        adc_instr,
        sub_instr, // 0x90
        sub_instr,
        sub_instr,
        sub_instr,
        sub_instr,
        sub_instr,
        sub_instr,
        sub_instr,
        sbc_instr,
        sbc_instr,
        sbc_instr,
        sbc_instr,
        sbc_instr,
        sbc_instr,
        sbc_instr,
        sbc_instr,
        and_instr, // 0xa0
        and_instr,
        and_instr,
        and_instr,
        and_instr,
        and_instr,
        and_instr,
        and_instr,
        xor_instr,
        xor_instr,
        xor_instr,
        xor_instr,
        xor_instr,
        xor_instr,
        xor_instr,
        xor_instr,
        or_instr, // 0xb0
        or_instr,
        or_instr,
        or_instr,
        or_instr,
        or_instr,
        or_instr,
        or_instr,
        cp_instr,
        cp_instr,
        cp_instr,
        cp_instr,
        cp_instr,
        cp_instr,
        cp_instr,
        cp_instr,
        noop_instr, // 0xc0
        noop_instr,
        jp_imm16_instr,
        jp_imm16_instr,
        noop_instr,
        noop_instr,
        add_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        jp_imm16_instr,
        cb_instr, // 0xcb
        noop_instr,
        noop_instr,
        adc_instr,
        noop_instr,
        noop_instr, // 0xd0
        noop_instr,
        jp_imm16_instr,
        undef_instr,
        noop_instr,
        noop_instr,
        sub_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        jp_imm16_instr,
        undef_instr,
        noop_instr,
        undef_instr,
        sbc_instr,
        noop_instr,
        ld_from_a_to_mem_instr, // 0xe0
        noop_instr,
        ld_from_a_to_mem_instr,
        undef_instr,
        undef_instr,
        noop_instr,
        and_instr,
        noop_instr,
        noop_instr,
        jp_hl_instr,
        ld_from_a_to_mem_instr,
        undef_instr,
        undef_instr,
        undef_instr,
        xor_instr,
        noop_instr,
        ld_from_mem_to_a_instr, // 0xf0
        noop_instr,
        ld_from_mem_to_a_instr,
        noop_instr,
        undef_instr,
        noop_instr,
        or_instr,
        noop_instr,
        ld_sp_plus_signed_imm_to_hl_instr,
        ld_hl_to_sp_instr,
        ld_from_mem_to_a_instr,
        noop_instr,
        undef_instr,
        undef_instr,
        cp_instr,
        noop_instr,
        ];

pub static CB_INSTR: [fn(&mut Cpu) -> Result<(), ()> ; INSTR_ARRAY_SIZE] =
        [noop_instr, // 0x00
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr, // 0x10
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr, // 0x20
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr, // 0x30
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr, // 0x40
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr, // 0x50
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr, // 0x60
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr, // 0x70
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr, // 0x80
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr, // 0x90
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr, // 0xa0
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr, // 0xb0
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr, // 0xc0
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr, // 0xd0
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr, // 0xe0
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr, // 0xf0
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        noop_instr,
        ];
