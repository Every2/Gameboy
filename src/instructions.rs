use crate::{cpu::Cpu, registers::FlagRegister};

pub static OPCODES: [(fn(&mut Cpu), &str); 0x100] = [
    //Opcodes 0X
    (nop, "NOP"),
    (ld_bc_nn, "LD BC, NN"),
    (ld_mbc_a, "LD [BC], A"),
    (inc_bc, "INC BC"),
    (inc_b, "INC B"),
    (dec_b, "DEC B"),
    (ld_b_n, "LD B, N"),
    (rlca, "RLCA"),
    (ld_mnn_sp, "LD [NN], SP"),
    (add_hl_bc, "ADD HL, BC"),
    (ld_a_mbc, "LD A, [BC]"),
    (dec_bc, "DEC BC"),
    (inc_c, "INC C"),
    (dec_c, "DEC C"),
    (ld_c_n, "LD C, N"),
    (rrca, "RRCA"),
    //Opcodes 1X
    (stop, "STOP"),
    (ld_de_nn, "LD DE, NN"),
    (ld_mde_a, "LD [DE], A"),
    (inc_de, "INC DE"),
    (inc_d, "INC D"),
    (dec_d, "DEC D"),
    (ld_d_n, "LD D, N"),
    (rla, "RLA"),
    (jr_sn, "JR SN"),
    (add_hl_de, "ADD HL, DE"),
    (ld_a_mde, "LD A, [DE]"),
    (dec_de, "DEC DE"),
    (inc_e, "INC E"),
    (dec_e, "DEC E"),
    (ld_e_n, "LD E, N"),
    (rra, "RRA"),
    (jr_nz_sn, "JR NZ, SN"),
    (ld_hl_nn, "LD HL, NN"),
    (ldi_mhl_a, "LDI [HL], A"),
    (inc_hl, "INC HL"),
    (inc_h, "INC H"),
    (dec_h, "DEC H"),
    (ld_h_n, "LD H, N"),
    (daa, "DAA"),
    (jr_z_sn, "JR Z, SN"),
    (add_hl_hl, "ADD HL, HL"),
    (ldi_a_mhl, "LDI A, [HL]"),
    (dec_hl, "DEC HL"),
    (inc_l, "INC L"),
    (dec_l, "DEC L"),
    (ld_l_n, "LD L, N"),
    (cpl, "CPL"),
    //Opcodes 3x
    (jr_nc_sn, "JR NC, SN"),
    (ld_sp_nn, "LD SP, NN"),
    (ldd_mhl_a, "LDD [HL], A"),
    (inc_sp, "INC SP"),
    (inc_mhl, "INC [HL]"),
    (dec_mhl, "DEC [HL]"),
    (ld_mhl_n, "LD [HL], N"),
    (scf, "SCF"),
    (jr_c_sn, "JR C, SN"),
    (add_hl_sp, "ADD HL, SP"),
    (ldd_a_mhl, "LLD A, [HL]"),
    (dec_sp, "DEC SP"),
    (inc_a, "INC A"),
    (dec_a, "DEC A"),
    (ld_a_n, "LD A, N"),
    (ccf, "CCF"),
    // Opcodes 4X
    (ld_b_b, "LD B, B"),
    (ld_b_c, "LD B, C"),
    (ld_b_d, "LD B, D"),
    (ld_b_e, "LD B, E"),
    (ld_b_h, "LD B, H"),
    (ld_b_l, "LD B, L"),
    (ld_b_mhl, "LD B, [HL]"),
    (ld_b_a, "LD B, A"),
    (ld_c_b, "LD C, B"),
    (ld_c_c, "LD C, C"),
    (ld_c_d, "LD C, D"),
    (ld_c_e, "LD C, E"),
    (ld_c_h, "LD C, H"),
    (ld_c_l, "LD C, L"),
    (ld_c_mhl, "LD C, [HL]"),
    (ld_c_a, "LD C, A"),
    // Opcodes 5X
    (ld_d_b, "LD D, B"),
    (ld_d_c, "LD D, C"),
    (ld_d_d, "LD D, D"),
    (ld_d_e, "LD D, E"),
    (ld_d_h, "LD D, H"),
    (ld_d_l, "LD D, L"),
    (ld_d_mhl, "LD D, [HL]"),
    (ld_d_a, "LD D, A"),
    (ld_e_b, "LD E, B"),
    (ld_e_c, "LD E, C"),
    (ld_e_d, "LD E, D"),
    (ld_e_e, "LD E, E"),
    (ld_e_h, "LD E, H"),
    (ld_e_l, "LD E, L"),
    (ld_e_mhl, "LD E, [HL]"),
    (ld_e_a, "LD E, A"),
    // Opcodes 6X
    (ld_h_b, "LD H, B"),
    (ld_h_c, "LD H, C"),
    (ld_h_d, "LD H, D"),
    (ld_h_e, "LD H, E"),
    (ld_h_h, "LD H, H"),
    (ld_h_l, "LD H, L"),
    (ld_h_mhl, "LD H, [HL]"),
    (ld_h_a, "LD H, A"),
    (ld_l_b, "LD L, B"),
    (ld_l_c, "LD L, C"),
    (ld_l_d, "LD L, D"),
    (ld_l_e, "LD L, E"),
    (ld_l_h, "LD L, H"),
    (ld_l_l, "LD L, L"),
    (ld_l_mhl, "LD L, [HL]"),
    (ld_l_a, "LD L, A"),
    //Opcodes 7X
    (ld_mhl_b, "LD [HL], B"),
    (ld_mhl_c, "LD [HL], C"),
    (ld_mhl_d, "LD [HL], D"),
    (ld_mhl_e, "LD [HL], E"),
    (ld_mhl_h, "LD [HL], H"),
    (ld_mhl_l, "LD [HL], L"),
    (halt, "HALT"),
    (ld_mhl_a, "LD [HL], A"),
    (ld_a_b, "LD A, B"),
    (ld_a_c, "LD A, C"),
    (ld_a_d, "LD A, D"),
    (ld_a_e, "LD A, E"),
    (ld_a_h, "LD A, H"),
    (ld_a_l, "LD A, L"),
    (ld_a_mhl, "LD A, [HL]"),
    (ld_a_a, "LD A, A"),
    //Opcodes 8X
    (add_a_b, "ADD A, B"),
    (add_a_c, "ADD A, C"),
    (add_a_d, "ADD A, D"),
    (add_a_e, "ADD A, E"),
    (add_a_h, "ADD A, H"),
    (add_a_l, "ADD A, L"),
    (add_a_mhl, "ADD A, [HL]"),
    (add_a_a, "ADD A, A"),
    (adc_a_b, "ADC A, B"),
    (adc_a_c, "ADC A, C"),
    (adc_a_d, "ADC A, D"),
    (adc_a_e, "ADC A, E"),
    (adc_a_h, "ADC A, H"),
    (adc_a_l, "ADC A, L"),
    (adc_a_mhl, "ADC A, [HL]"),
    (adc_a_a, "ADC A, A"),
    //Opcodes 9X
    (sub_a_b, "SUB A, B"),
    (sub_a_c, "SUB A, C"),
    (sub_a_d, "SUB A, D"),
    (sub_a_e, "SUB A, E"),
    (sub_a_h, "SUB A, H"),
    (sub_a_l, "SUB A, L"),
    (sub_a_mhl, "SUB A, [HL]"),
    (sub_a_a, "SUB A, A"),
    (sbc_a_b, "SBC A, B"),
    (sbc_a_c, "SBC A, C"),
    (sbc_a_d, "SBC A, D"),
    (sbc_a_e, "SBC A, E"),
    (sbc_a_h, "SBC A, H"),
    (sbc_a_l, "SBC A, L"),
    (sbc_a_mhl, "SBC A, [HL]"),
    (sbc_a_a, "SBC A, A"),
    //Opcodes AX
    (and_a_b, "AND A, B"),
    (and_a_c, "AND A, C"),
    (and_a_d, "AND A, D"),
    (and_a_e, "AND A, E"),
    (and_a_h, "AND A, H"),
    (and_a_l, "AND A, L"),
    (and_a_mhl, "AND A, [HL]"),
    (and_a_a, "AND A, A"),
    (xor_a_b, "XOR A, B"),
    (xor_a_c, "XOR A, C"),
    (xor_a_d, "XOR A, D"),
    (xor_a_e, "XOR A, E"),
    (xor_a_h, "XOR A, H"),
    (xor_a_l, "XOR A, L"),
    (xor_a_mhl, "XOR A, [HL]"),
    (xor_a_a, "XOR A, A"),
    //Opcodes BX
    (or_a_b, "OR A, B"),
    (or_a_c, "OR A, C"),
    (or_a_d, "OR A, D"),
    (or_a_e, "OR A, E"),
    (or_a_h, "OR A, H"),
    (or_a_l, "OR A, L"),
    (or_a_mhl, "OR A, [HL]"),
    (or_a_a, "OR A, A"),
    (cp_a_b, "CP A, B"),
    (cp_a_c, "CP A, C"),
    (cp_a_d, "CP A, D"),
    (cp_a_e, "CP A, E"),
    (cp_a_h, "CP A, H"),
    (cp_a_l, "CP A, L"),
    (cp_a_mhl, "CP A, [HL]"),
    (cp_a_a, "CP A, A"),
    //Opcodes CX
    (ret_nz, "RET NZ"),
    (pop_bc, "POP BC"),
    (jp_nz_nn, "JP NZ, NN"),
    (jp_nn, "JP NN"),
    (call_nz_nn, "CALL NZ, NN"),
    (push_bc, "PUSH BC"),
    (add_a_n, "ADD A, N"),
    (rst_00, "RST 00"),
    (ret_z, "RET Z"),
    (ret, "RET"),
    (jp_z_nn, "JP Z, NN"),
    (undefined, "UNDEFINED"),
    (call_z_nn, "CALL Z, NN"),
    (call_nn, "CALL NN"),
    (adc_a_n, "ADC A, N"),
    (rst_08, "RST 08"),
    //Opcodes DX
    (ret_nc, "RET NC"),
    (pop_de, "POP DE"),
    (jp_nc_nn, "JP NC, NN"),
    (undefined, "UNDEFINED"),
    (call_nc_nn, "CALL NC, NN"),
    (push_de, "PUSH DE"),
    (sub_a_n, "SUB A, N"),
    (rst_10, "RST 10"),
    (ret_c, "RET C"),
    (reti, "RETI"),
    (jp_c_nn, "JP C, NN"),
    (undefined, "UNDEFINED"),
    (call_c_nn, "CALL C, NN"),
    (undefined, "UNDEFINED"),
    (sbc_a_n, "SBC A, N"),
    (rst_18, "RST 18"),
    //OPCODES EX
    (ldh_mn_a, "LDH [N], A"),
    (pop_hl, "POP HL"),
    (ldh_mc_a, "LDH [C], A"),
    (undefined, "UNDEFINED"),
    (undefined, "UNDEFINED"),
    (push_hl, "PUSH HL"),
    (and_a_n, "AND A, N"),
    (rst_20, "RST 20"),
    (add_sp_sn, "ADD SP, SN"),
    (jp_hl, "JP HL"),
    (ld_mnn_a, "LD [NN], A"),
    (undefined, "UNDEFINED"),
    (undefined, "UNDEFINED"),
    (undefined, "UNDEFINED"),
    (xor_a_n, "XOR A, N"),
    (rst_28, "RST 28"),
    //Opcodes FX
    (ldh_a_mn, "LDH A, [N]"),
    (pop_af, "POP AF"),
    (ldh_a_mc, "LDH A, [C]"),
    (di, "DI"),
    (undefined, "UNDEFINED"),
    (push_a_f, "PUSH A, F"),
    (or_a_n, "OR A, N"),
    (rst_30, "RST 30"),
    (ld_hl_sp_sn, "LD HL, SP, SN"),
    (ld_sp_hl, "LD SP, HL"),
    (ld_a_mnn, "LD A, [NN]"),
    (ei, "EI"),
    (undefined, "UNDEFINED"),
    (undefined, "UNDEFINED"),
    (cp_a_n, "CP A, N"),
    (rst_38, "RST 38"),
];

fn undefined(cpu: &mut Cpu) {
    let pc = cpu.pc.wrapping_sub(1);

    println!("Invalid instruction called at 0x{:04x}. CPU stalled.", pc);

    cpu.set_pc(pc);
}

fn nop(_: &mut Cpu) {}

fn rlca(cpu: &mut Cpu) {
    let c = cpu.registers.a >> 7;

    cpu.registers.a = cpu.registers.a << 1 | c;

    cpu.registers.f.carry = c != 0;
    cpu.registers.f.zero = false;
    cpu.registers.f.half_carry = false;
    cpu.registers.f.subtract = false;
}