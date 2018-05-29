use std::collections::HashMap;
use std::fmt;

fn main() {
    let mut gameboy = GameBoy::new();
    gameboy.run();
}

struct GameBoy {
    // time/ticks since start
    t: u64,
    // instruction pointer/index
    i: u16,
    main_ram: [u8; 8192],
    video_ram: [u8; 8192],
    high_ram: [u8; 127],
    main_registers: [u8; 12],
    boot_rom: Vec<u8>,
    game_rom: Vec<u8>,
    debug_current_op_addr: u16,
    debug_current_code: Vec<u8>,
}

struct Operation {
    code: u8,
    execute: fn(gb: &mut GameBoy) -> (String, String),
}

fn getOperations() -> HashMap<u8, Operation> {
    let mut operations = HashMap::new();

    {
        let mut op = |code: u8, execute: fn(gb: &mut GameBoy) -> (String, String)| {
            let operation = Operation {
                code: code,
                execute: execute,
            };
            match operations.insert(operation.code, operation) {
                Some(existingOp) => panic!("duplicate opcode"),
                None => {}
            }
        };

        // 3.1.1. 8-bit loads
        {
            // 1. LD nn, n
            // Put value n into nn.
            {
                op(0x06, |gb| {
                    let b0 = gb.b();
                    let b1 = gb.read_immediate_u8();
                    gb.set_b(b1);
                    (
                        format!("LD B, ${:02x}", b1),
                        format!("B₀ = ${:02x}, B₁ = ${:02x}", b0, b1),
                    )
                });
                op(0x0E, |gb| {
                    let c0 = gb.c();
                    let c1 = gb.read_immediate_u8();
                    gb.set_c(c1);
                    (
                        format!("LD C, ${:02x}", c1),
                        format!("C₀ = ${:02x}, C₁ = ${:02x}", c0, c1),
                    )
                });
                op(0x16, |gb| {
                    let d0 = gb.d();
                    let d1 = gb.read_immediate_u8();
                    gb.set_d(d1);
                    (
                        format!("LD D, ${:02x}", d1),
                        format!("D₀ = ${:02x}, D₁ = ${:02x}", d0, d1),
                    )
                });
                op(0x1E, |gb| {
                    let e0 = gb.e();
                    let e1 = gb.read_immediate_u8();
                    gb.set_e(e1);
                    (
                        format!("LD E, ${:02x}", e1),
                        format!("E₀ = ${:02x}, E₁ = ${:02x}", e0, e1),
                    )
                });
                op(0x26, |gb| {
                    let h0 = gb.h();
                    let h1 = gb.read_immediate_u8();
                    gb.set_h(h1);
                    (
                        format!("LD H, ${:02x}", h1),
                        format!("H₀ = ${:02x}, H₁ = ${:02x}", h0, h1),
                    )
                });
                op(0x2E, |gb| {
                    let l0 = gb.l();
                    let l1 = gb.read_immediate_u8();
                    gb.set_l(l1);
                    (
                        format!("LD L, ${:02x}", l1),
                        format!("L₀ = ${:02x}, L₁ = ${:02x}", l0, l1),
                    )
                });
            }

            // 2. LD r1, r2
            // Put value r2 into r1.
            {
                op(0x7F, |gb| {
                    let a = gb.a();
                    (format!("LD A, A"), format!("A = ${:02x}", a))
                });
                op(0x78, |gb| {
                    let a0 = gb.a();
                    let b = gb.b();
                    gb.set_a(b);
                    (
                        format!("LD A, B"),
                        format!("A₀ = ${:02x}, B = ${:02x}", a0, b),
                    )
                });
                op(0x79, |gb| {
                    let a0 = gb.a();
                    let c = gb.c();
                    gb.set_a(c);
                    (
                        format!("LD A, C"),
                        format!("A₀ = ${:02x}, C = ${:02x}", a0, c),
                    )
                });
                op(0x7A, |gb| {
                    let a0 = gb.a();
                    let d = gb.d();
                    gb.set_a(d);
                    (
                        format!("LD A, D"),
                        format!("A₀ = ${:02x}, D = ${:02x}", a0, d),
                    )
                });
                op(0x7B, |gb| {
                    let a0 = gb.a();
                    let e = gb.e();
                    gb.set_a(e);
                    (
                        format!("LD A, E"),
                        format!("A₀ = ${:02x}, E = ${:02x}", a0, e),
                    )
                });
                op(0x7C, |gb| {
                    let a0 = gb.a();
                    let h = gb.h();
                    gb.set_a(h);
                    (
                        format!("LD A, H"),
                        format!("A₀ = ${:02x}, H = ${:02x}", a0, h),
                    )
                });
                op(0x7D, |gb| {
                    let a0 = gb.a();
                    let l = gb.l();
                    gb.set_a(l);
                    (
                        format!("LD A, L"),
                        format!("A₀ = ${:02x}, L = ${:02x}", a0, l),
                    )
                });
                op(0x7E, |gb| {
                    let a0 = gb.a();
                    let hl = gb.hl();
                    let a1 = gb.get_memory(hl);
                    gb.set_a(a1);
                    (
                        format!("LD A, (HL)"),
                        format!("A₀ = ${:02x}, HL = ${:04x}, (HL) = ${:04x}", a0, hl, a1),
                    )
                });
                op(0x40, |gb| {
                    let b = gb.b();
                    (format!("LD B, B"), format!("B = ${:02x}", b))
                });
                op(0x41, |gb| {
                    let b0 = gb.b();
                    let c = gb.c();
                    gb.set_b(c);
                    (
                        format!("LD B, C"),
                        format!("B₀ = ${:02x}, C = ${:02x}", b0, c),
                    )
                });
                op(0x42, |gb| {
                    let b0 = gb.b();
                    let d = gb.d();
                    gb.set_b(d);
                    (
                        format!("LD B, D"),
                        format!("B₀ = ${:02x}, D = ${:02x}", b0, d),
                    )
                });
                op(0x43, |gb| {
                    let b0 = gb.b();
                    let e = gb.e();
                    gb.set_b(e);
                    (
                        format!("LD B, E"),
                        format!("B₀ = ${:02x}, E = ${:02x}", b0, e),
                    )
                });
                op(0x44, |gb| {
                    let b0 = gb.b();
                    let h = gb.h();
                    gb.set_b(h);
                    (
                        format!("LD B, H"),
                        format!("B₀ = ${:02x}, H = ${:02x}", b0, h),
                    )
                });
                op(0x45, |gb| {
                    let b0 = gb.b();
                    let l = gb.l();
                    gb.set_b(l);
                    (
                        format!("LD B, L"),
                        format!("B₀ = ${:02x}, L = ${:02x}", b0, l),
                    )
                });
                op(0x46, |gb| {
                    let b0 = gb.b();
                    let hl = gb.hl();
                    let b1 = gb.get_memory(hl);
                    gb.set_b(b1);
                    (
                        format!("LD B, (HL)"),
                        format!("B₀ = ${:02x}, HL = ${:04x}, (HL) = ${:04x}", b0, hl, b1),
                    )
                });
            }
        }
    }

    operations
}

impl GameBoy {
    fn new() -> GameBoy {
        GameBoy {
            t: 0,
            i: 0,
            main_ram: [0u8; 8192],
            video_ram: [0u8; 8192],
            high_ram: [0u8; 127],
            main_registers: [0u8; 12],
            boot_rom: load_boot_rom(),
            game_rom: load_game_rom("Pokemon Red (US)[:256]"),
            debug_current_op_addr: 0,
            debug_current_code: vec![],
        }
    }

    fn read_instruction(&mut self) -> u8 {
        self.debug_current_code.clear();
        self.debug_current_op_addr = self.i;
        self.read_immediate_u8()
    }

    fn read_immediate_u8(&mut self) -> u8 {
        let value = self.get_memory(self.i);
        self.debug_current_code.push(value);
        self.i += 1;
        value
    }

    fn relative_jump(&mut self, n: i32) {
        self.i = ((self.i as i32) + n) as u16;
    }

    fn print_current_code(&self, asm: String, info: String) {
        print!("{:32}", asm);
        print!(" ; ${:04x}", self.debug_current_op_addr);
        let code = self.debug_current_code
            .clone()
            .into_iter()
            .map(|c| format!("{:02x}", c))
            .collect::<Vec<String>>()
            .join("");
        print!(" ; ${:8}", code);
        print!(" ; {}", info);
        println!();
    }

    // Main Loop

    fn run(&mut self) {
        println!("ASM:                               ADDR:   CODES:      FLAGS:");
        println!("----                               -----   ------      ------");

        let operations = getOperations();

        while true {
            let opcode = self.read_instruction();

            let op = operations.get(&opcode);
            match op {
                Some(op) => {
                    let (asm, debug) = (op.execute)(self);
                    self.print_current_code(asm, debug);
                }
                None => {
                    match opcode {
                        // Jumps
                        // JR n
                        // Unconditional relative jump.
                        0x18 => {
                            let delta = self.read_immediate_u8() as i8;
                            self.print_current_code(format!("JR {})", delta), "".to_string());
                            self.relative_jump(delta as i32);
                        }
                        // JR cc, n
                        // Conditional relative jump.
                        0x20 => {
                            let delta = self.read_immediate_u8() as i8;
                            self.print_current_code(
                                format!("JR NZ, {}", delta),
                                format!("Z = {}", self.z_flag()),
                            );
                            if !self.z_flag() {
                                self.relative_jump(delta as i32);
                            }
                        }
                        0x28 => {
                            let delta = self.read_immediate_u8() as i8;
                            self.print_current_code(
                                format!("JR Z, {}", delta),
                                format!("Z = {}", self.z_flag()),
                            );
                            if self.z_flag() {
                                self.relative_jump(delta as i32);
                            }
                        }
                        0x30 => {
                            let delta = self.read_immediate_u8() as i8;
                            self.print_current_code(
                                format!("JR NC, {}", delta),
                                format!("C = {}", self.c_flag()),
                            );
                            if !self.c_flag() {
                                self.relative_jump(delta as i32);
                            }
                        }
                        0x38 => {
                            let delta = self.read_immediate_u8() as i8;
                            self.print_current_code(
                                format!("JR C, {}", delta),
                                format!("C = {}", self.c_flag()),
                            );
                            if self.c_flag() {
                                self.relative_jump(delta as i32);
                            }
                        }

                        0x21 => {
                            // LOAD HL, $1, $2
                            let h = self.read_immediate_u8();
                            let l = self.read_immediate_u8();
                            self.print_current_code(
                                format!("LOAD HL, ${:02x}, ${:02x}", h, l),
                                "".to_string(),
                            );
                            self.set_h_l(h, l);
                        }

                        0x31 => {
                            // LOAD SP, $1, $2
                            let s = self.read_immediate_u8();
                            let p = self.read_immediate_u8();
                            self.print_current_code(
                                format!("LOAD SP ${:02x}, ${:02x}", s, p),
                                "".to_string(),
                            );
                            self.set_s_p(s, p);
                        }

                        0x77 => {
                            // Put A into memory address HL.
                            self.print_current_code(
                                "LD (HL), A".to_string(),
                                format!("HL = ${:04x}, A = ${:02x}", self.hl(), self.a()),
                            );
                            let mut hl = self.hl();
                            let a = self.a();
                            self.set_memory(hl, a);
                        }

                        0x32 => {
                            // Put A into memory address HL.
                            self.print_current_code(
                                "LD (HL-), A".to_string(),
                                format!("HL₀ = ${:04x}, A = ${:02x}", self.hl(), self.a()),
                            );
                            let mut hl = self.hl();
                            let a = self.a();
                            self.set_memory(hl, a);
                            //  Decrement HL.
                            hl -= 1;
                            self.set_hl(hl);
                        }

                        0xE2 => {
                            // Put A into memory address 0xFF00 + C.
                            self.print_current_code(
                                "LD ($FF00+C), A ".to_string(),
                                format!("A = ${:02x}, C = ${:02x}", self.a(), self.c()),
                            );
                            let a = self.a();
                            let address = 0xFF00 + (self.c() as u16);
                            self.set_memory(address, a);
                        }

                        0xAF => {
                            self.print_current_code(
                                "XOR A A".to_string(),
                                format!("A₀ = ${:02x}, A₁ = $00", self.a()).to_string(),
                            );
                            self.set_a(0);
                        }

                        // 8-Bit Arithmatic
                        // Increment the value in register n.
                        // Z flag set iff result is 0.
                        // N flag cleared.
                        // H flag set iff value overflows and wraps.
                        0x3C => {
                            let oldValue = self.a();
                            let newValue = oldValue + 1;
                            self.print_current_code(
                                "INC A".to_string(),
                                format!("A₀ = ${:02x}, A₁ = ${:02x}", oldValue, newValue)
                                    .to_string(),
                            );
                            self.set_a(newValue);
                            self.set_z_flag(newValue == 0);
                            self.set_n_flag(false);
                            self.set_h_flag(oldValue > newValue);
                        }
                        0x04 => {
                            let oldValue = self.b();
                            let newValue = oldValue + 1;
                            self.print_current_code(
                                "INC B".to_string(),
                                format!("B₀ = ${:02x}, B₁ = ${:02x}", oldValue, newValue)
                                    .to_string(),
                            );
                            self.set_b(newValue);
                            self.set_z_flag(newValue == 0);
                            self.set_n_flag(false);
                            self.set_h_flag(oldValue > newValue);
                        }
                        0x0C => {
                            let oldValue = self.c();
                            let newValue = oldValue + 1;
                            self.print_current_code(
                                "INC C".to_string(),
                                format!("C₀ = ${:02x}, C₁ = ${:02x}", oldValue, newValue)
                                    .to_string(),
                            );
                            self.set_c(newValue);
                            self.set_z_flag(newValue == 0);
                            self.set_n_flag(false);
                            self.set_h_flag(oldValue > newValue);
                        }
                        0x14 => {
                            let oldValue = self.d();
                            let newValue = oldValue + 1;
                            self.print_current_code(
                                "INC D".to_string(),
                                format!("D₀ = ${:02x}, D₁ = ${:02x}", oldValue, newValue)
                                    .to_string(),
                            );
                            self.set_d(newValue);
                            self.set_z_flag(newValue == 0);
                            self.set_n_flag(false);
                            self.set_h_flag(oldValue > newValue);
                        }
                        0x1C => {
                            let oldValue = self.e();
                            let newValue = oldValue + 1;
                            self.print_current_code(
                                "INC E".to_string(),
                                format!("E₀ = ${:02x}, E₁ = ${:02x}", oldValue, newValue)
                                    .to_string(),
                            );
                            self.set_e(newValue);
                            self.set_z_flag(newValue == 0);
                            self.set_n_flag(false);
                            self.set_h_flag(oldValue > newValue);
                        }
                        0x24 => {
                            let oldValue = self.h();
                            let newValue = oldValue + 1;
                            self.print_current_code(
                                "INC H".to_string(),
                                format!("H₀ = ${:02x}, H₁ = ${:02x}", oldValue, newValue)
                                    .to_string(),
                            );
                            self.set_h(newValue);
                            self.set_z_flag(newValue == 0);
                            self.set_n_flag(false);
                            self.set_h_flag(oldValue > newValue);
                        }
                        0x2C => {
                            let oldValue = self.l();
                            let newValue = oldValue + 1;
                            self.print_current_code(
                                "INC L".to_string(),
                                format!("L₀ = ${:02x}, L₁ = ${:02x}", oldValue, newValue)
                                    .to_string(),
                            );
                            self.set_l(newValue);
                            self.set_z_flag(newValue == 0);
                            self.set_n_flag(false);
                            self.set_h_flag(oldValue > newValue);
                        }

                        0xCB => {
                            // 2-byte opcode

                            let opcode_2 = self.read_immediate_u8();

                            match opcode_2 {
                                0x7C => {
                                    let result = !u8_get_bit(self.h(), 7);
                                    self.print_current_code(
                                        "BIT 7, H".to_string(),
                                        format!("Z₁ = {}", result),
                                    );
                                    self.set_z_flag(result);
                                    self.set_n_flag(false);
                                    self.set_h_flag(true);
                                }

                                _ => {
                                    panic!("unsupported opcode: ${:02x}{:02x}", opcode, opcode_2);
                                }
                            }
                        }

                        _ => {
                            panic!("unsupported opcode: ${:02x}", opcode);
                        }
                    }
                }
            }

            self.t += 1;
        }
    }

    // Register Access

    fn a(&self) -> u8 {
        return self.main_registers[0];
    }

    fn set_a(&mut self, value: u8) {
        self.main_registers[0] = value;
    }

    fn flags(&self) -> u8 {
        return self.main_registers[1];
    }

    fn set_flags(&mut self, value: u8) {
        self.main_registers[1] = value;
    }

    fn b(&self) -> u8 {
        return self.main_registers[2];
    }

    fn set_b(&mut self, value: u8) {
        self.main_registers[2] = value;
    }

    fn c(&self) -> u8 {
        return self.main_registers[3];
    }

    fn set_c(&mut self, value: u8) {
        self.main_registers[3] = value;
    }

    fn d(&self) -> u8 {
        return self.main_registers[4];
    }

    fn set_d(&mut self, value: u8) {
        self.main_registers[4] = value;
    }

    fn e(&self) -> u8 {
        return self.main_registers[5];
    }

    fn set_e(&mut self, value: u8) {
        self.main_registers[5] = value;
    }

    fn h(&self) -> u8 {
        // XXX: this has been swapped with l as a test, clean-up required
        return self.main_registers[7];
    }

    fn set_h(&mut self, value: u8) {
        // XXX: this has been swapped with l as a test, clean-up required
        self.main_registers[7] = value;
    }

    fn l(&self) -> u8 {
        // XXX: this has been swapped with h as a test, clean-up required
        return self.main_registers[6];
    }

    fn set_l(&mut self, value: u8) {
        // XXX: this has been swapped with h as a test, clean-up required
        self.main_registers[6] = value;
    }

    fn hl(&self) -> u16 {
        return u8s_to_u16(self.main_registers[6], self.main_registers[7]);
    }

    fn set_hl(&mut self, value: u16) {
        let (h, l) = u16_to_u8s(value);
        self.main_registers[6] = h;
        self.main_registers[7] = l;
    }

    fn set_h_l(&mut self, h: u8, l: u8) {
        self.main_registers[6] = h;
        self.main_registers[7] = l;
    }

    fn sp(&self) -> u16 {
        return u8s_to_u16(self.main_registers[8], self.main_registers[9]);
    }

    fn set_sp(&mut self, value: u16) {
        let (s, p) = u16_to_u8s(value);
        self.main_registers[8] = s;
        self.main_registers[9] = p;
    }

    fn set_s_p(&mut self, s: u8, p: u8) {
        self.main_registers[8] = s;
        self.main_registers[9] = p;
    }

    fn pc(&self) -> u16 {
        return u8s_to_u16(self.main_registers[10], self.main_registers[11]);
    }

    fn set_pc(&mut self, value: u16) {
        let (p, c) = u16_to_u8s(value);
        self.main_registers[10] = p;
        self.main_registers[11] = c;
    }

    fn set_p_c(&mut self, p: u8, c: u8) {
        self.main_registers[10] = p;
        self.main_registers[11] = c;
    }

    fn z_flag(&self) -> bool {
        u8_get_bit(self.flags(), 1)
    }

    fn set_z_flag(&mut self, value: bool) {
        let mut flags = self.flags();
        u8_set_bit(&mut flags, 1, value);
        self.set_flags(flags);
    }

    fn n_flag(&self) -> bool {
        u8_get_bit(self.flags(), 2)
    }

    fn set_n_flag(&mut self, value: bool) {
        let mut flags = self.flags();
        u8_set_bit(&mut flags, 2, value);
        self.set_flags(flags);
    }

    fn h_flag(&self) -> bool {
        u8_get_bit(self.flags(), 3)
    }

    fn set_h_flag(&mut self, value: bool) {
        let mut flags = self.flags();
        u8_set_bit(&mut flags, 3, value);
        self.set_flags(flags);
    }

    fn c_flag(&self) -> bool {
        u8_get_bit(self.flags(), 4)
    }

    fn set_c_flag(&mut self, value: bool) {
        let mut flags = self.flags();
        u8_set_bit(&mut flags, 4, value);
        self.set_flags(flags);
    }

    // Memory Access

    fn get_memory(&self, address: u16) -> u8 {
        if address <= 0x00FF {
            return self.boot_rom[address as usize];
        } else if 0x8000 <= address && address <= 0x9FFF {
            let i: usize = (address - 0x8000) as usize;
            return self.video_ram[i];
        } else if 0xFF80 <= address && address <= 0xFFFE {
            let i: usize = (address - 0xFF80) as usize;
            return self.high_ram[i];
        } else {
            panic!("I don't know how to get memory address ${:04x}.", address);
        }
    }

    fn set_memory(&mut self, address: u16, value: u8) {
        if 0x8000 <= address && address <= 0x9FFF {
            let i: usize = (address - 0x8000) as usize;
            println!("  ; video_ram[${:04x}] = ${:02x}", i, value);
            self.video_ram[i] = value;
        } else if 0xFF80 <= address && address <= 0xFFFE {
            let i: usize = (address - 0xFF80) as usize;
            println!("  ; high_ram[${:04x}] = ${:02x}", i, value);
            self.high_ram[i] = value;
        } else if 0xFF10 <= address && address <= 0xFF26 {
            println!("  ; skipping write to sound control memory -- not implemented");
        } else {
            panic!("I don't know how to set memory address ${:04x}.", address);
        }
    }
}

fn u8s_to_u16(a: u8, b: u8) -> u16 {
    return a as u16 + ((b as u16) << 8);
}

fn u16_to_u8s(x: u16) -> (u8, u8) {
    (x as u8, (x >> 8) as u8)
}

fn u8_get_bit(x: u8, offset: u8) -> bool {
    if offset > 7 {
        panic!();
    }

    (x >> offset) & 1 == 1
}

fn u8_set_bit(x: &mut u8, offset: u8, value: bool) {
    if offset > 7 {
        panic!();
    }

    let mask = 1 << offset;
    if value {
        *x |= mask;
    } else {
        *x &= !mask;
    }
}

fn load_boot_rom() -> Vec<u8> {
    return vec![
        0x31, 0xFE, 0xFF, 0xAF, 0x21, 0xFF, 0x9F, 0x32, 0xCB, 0x7C, 0x20, 0xFB, 0x21, 0x26, 0xFF,
        0x0E, 0x11, 0x3E, 0x80, 0x32, 0xE2, 0x0C, 0x3E, 0xF3, 0xE2, 0x32, 0x3E, 0x77, 0x77, 0x3E,
        0xFC, 0xE0, 0x47, 0x11, 0x04, 0x01, 0x21, 0x10, 0x80, 0x1A, 0xCD, 0x95, 0, 0xCD, 0x96, 0,
        0x13, 0x7B, 0xFE, 0x34, 0x20, 0xF3, 0x11, 0xD8, 0, 0x06, 0x08, 0x1A, 0x13, 0x22, 0x23,
        0x05, 0x20, 0xF9, 0x3E, 0x19, 0xEA, 0x10, 0x99, 0x21, 0x2F, 0x99, 0x0E, 0x0C, 0x3D, 0x28,
        0x08, 0x32, 0x0D, 0x20, 0xF9, 0x2E, 0x0F, 0x18, 0xF3, 0x67, 0x3E, 0x64, 0x57, 0xE0, 0x42,
        0x3E, 0x91, 0xE0, 0x40, 0x04, 0x1E, 0x02, 0x0E, 0x0C, 0xF0, 0x44, 0xFE, 0x90, 0x20, 0xFA,
        0x0D, 0x20, 0xF7, 0x1D, 0x20, 0xF2, 0x0E, 0x13, 0x24, 0x7C, 0x1E, 0x83, 0xFE, 0x62, 0x28,
        0x06, 0x1E, 0xC1, 0xFE, 0x64, 0x20, 0x06, 0x7B, 0xE2, 0x0C, 0x3E, 0x87, 0xE2, 0xF0, 0x42,
        0x90, 0xE0, 0x42, 0x15, 0x20, 0xD2, 0x05, 0x20, 0x4F, 0x16, 0x20, 0x18, 0xCB, 0x4F, 0x06,
        0x04, 0xC5, 0xCB, 0x11, 0x17, 0xC1, 0xCB, 0x11, 0x17, 0x05, 0x20, 0xF5, 0x22, 0x23, 0x22,
        0x23, 0xC9, 0xCE, 0xED, 0x66, 0x66, 0xCC, 0x0D, 0, 0x0B, 0x03, 0x73, 0, 0x83, 0, 0x0C, 0,
        0x0D, 0, 0x08, 0x11, 0x1F, 0x88, 0x89, 0, 0x0E, 0xDC, 0xCC, 0x6E, 0xE6, 0xDD, 0xDD, 0xD9,
        0x99, 0xBB, 0xBB, 0x67, 0x63, 0x6E, 0x0E, 0xEC, 0xCC, 0xDD, 0xDC, 0x99, 0x9F, 0xBB, 0xB9,
        0x33, 0x3E, 0x3C, 0x42, 0xB9, 0xA5, 0xB9, 0xA5, 0x42, 0x3C, 0x21, 0x04, 0x01, 0x11, 0xA8,
        0, 0x1A, 0x13, 0xBE, 0x20, 0xFE, 0x23, 0x7D, 0xFE, 0x34, 0x20, 0xF5, 0x06, 0x19, 0x78,
        0x86, 0x23, 0x05, 0x20, 0xFB, 0x86, 0x20, 0xFE, 0x3E, 0x01, 0xE0, 0x50,
    ];
}

fn load_game_rom(game_name: &str) -> Vec<u8> {
    match game_name {
        "Pokemon Red (US)[:256]" => {
            return vec![
                0xFF, 0, 0, 0, 0, 0, 0, 0, 0xFF, 0, 0, 0, 0, 0, 0, 0, 0xFF, 0, 0, 0, 0, 0, 0, 0,
                0xFF, 0, 0, 0, 0, 0, 0, 0, 0xFF, 0, 0, 0, 0, 0, 0, 0, 0xFF, 0, 0, 0, 0, 0, 0, 0,
                0xFF, 0, 0, 0, 0, 0, 0, 0, 0xFF, 0, 0, 0, 0, 0, 0, 0, 0xC3, 0x24, 0x20, 0, 0, 0, 0,
                0, 0xFF, 0, 0, 0, 0, 0, 0, 0, 0xC3, 0x06, 0x23, 0, 0, 0, 0, 0, 0xC3, 0x25, 0x21, 0,
                0, 0, 0, 0, 0xD9, 0xAF, 0xE0, 0x0F, 0xF0, 0xFF, 0x47, 0xCB, 0x87, 0xE0, 0xFF, 0xF0,
                0x44, 0xFE, 0x91, 0x20, 0xFA, 0xF0, 0x40, 0xE6, 0x7F, 0xE0, 0x40, 0x78, 0xE0, 0xFF,
                0xC9, 0xF0, 0x40, 0xCB, 0xFF, 0xE0, 0x40, 0xC9, 0xAF, 0x21, 0, 0xC3, 0x06, 0xA0,
                0x22, 0x05, 0x20, 0xFC, 0xC9, 0x3E, 0xA0, 0x21, 0, 0xC3, 0x11, 0x04, 0, 0x06, 0x28,
                0x77, 0x19, 0x05, 0x20, 0xFB, 0xC9, 0xEA, 0xE9, 0xCE, 0xF0, 0xB8, 0xF5, 0xFA, 0xE9,
                0xCE, 0xE0, 0xB8, 0xEA, 0, 0x20, 0xCD, 0xB5, 0, 0xF1, 0xE0, 0xB8, 0xEA, 0, 0x20,
                0xC9, 0x2A, 0x12, 0x13, 0x0B, 0x79, 0xB0, 0x20, 0xF8, 0xC9, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0,
            ];
        }

        _ => panic!("Game ROM Not Available: {}", game_name),
    }
}
