use super::*;
use crate::memory::_FONT_START;

#[test]
fn test_make_cpu() {
    let mem: memory::Mem = memory::make_memory();
    let c : CPU = make_cpu(mem);
    let default_mem: memory::Mem = memory::make_memory();
    let default_dis: display::Display = display::make_display();
    assert_eq!(c.mem.data, default_mem.data);
    assert_eq!(c.display.screen, default_dis.screen);
}

#[test]
fn test_cpu_fetch() {
    let mem: memory::Mem = memory::make_memory();
    let mut c : CPU = make_cpu(mem);

    let w1 : [u8; 2] = [0xde, 0xad];
    let w2 : [u8; 2] = [0xbe, 0xef];

    c.mem.write16((c.pc).into(), w1);
    c.mem.write16((c.pc+2).into(), w2);

    let e1 = Instruction {
        inst: w1,
        nib1: 0xd,
        nib2: 0xe,
        nib3: 0xa,
        nib4: 0xd,
        nib234: 0x0ead,
        byte: 0xdead
    };
    let e2 = Instruction {
        inst: w2,
        nib1: 0xb,
        nib2: 0xe,
        nib3: 0xe,
        nib4: 0xf,
        nib234: 0xeef,
        byte: 0xbeef
    };

    assert_eq!(c.fetch_next_instruction(), e1, "Getting 1st instruction");
    assert_eq!(c.fetch_next_instruction(), e2, "Getting 2nd instruction");

}

#[test]
fn test_cpu_clear_display() {
    let mem: memory::Mem = memory::make_memory();
    let mut c : CPU = make_cpu(mem);
    let w1 : [u8; 2] = [0x00, 0xE0]; // Clear display
    let w2 : [u8; 2] = [0x00, 0x00]; // Exit
    c.mem.write16((c.pc).into(), w1);
    c.mem.write16((c.pc+2).into(), w2);
    c.display.screen[0][0] = true;
    c.display.screen[1][2] = true;
    c.display.screen[2][4] = true;

    c._run();

    let default_dis: display::Display = display::make_display();
    assert_eq!(c.display.screen, default_dis.screen);
}

#[test]
fn test_cpu_jump() {
    let mem: memory::Mem = memory::make_memory();
    let mut c : CPU = make_cpu(mem);
    let w1 : [u8; 2] = [0x19, 0x99]; // Jump to address 0x999
    let w2 : [u8; 2] = [0x00, 0xE0]; // Clear display
    let w3 : [u8; 2] = [0x00, 0x00]; // Exit
    c.mem.write16((c.pc).into(), w1);
    c.mem.write16(0x999, w2);
    c.mem.write16(0x999 + 2, w3);
    c.display.screen[0][0] = true;
    c.display.screen[1][2] = true;
    c.display.screen[2][4] = true;

    c._run();

    let default_dis: display::Display = display::make_display();
    assert_eq!(c.display.screen, default_dis.screen);
    assert_eq!(c.pc as u16, 0x999 + 4);
}

#[test]
fn test_cpu_jump_and_store() {
    let mem: memory::Mem = memory::make_memory();
    let mut c : CPU = make_cpu(mem);
    let w1 : [u8; 2] = [0x29, 0x99]; // Jump to address 0x999
    let w2 : [u8; 2] = [0x00, 0x00]; // Exit
    c.mem.write16((c.pc).into(), w1);
    c.mem.write16(0x999, w2);

    let og_stack = c.pc;
    c._run();

    assert_eq!(c.pc as u16, 0x999 + 2);
    assert_eq!(c.stack.len(), 1);
    assert_eq!(c.stack.pop(), Some(og_stack+2));
}

#[test]
fn test_cpu_load_and_jump() {
    let mem: memory::Mem = memory::make_memory();
    let mut c : CPU = make_cpu(mem);
    let w1 : [u8; 2] = [0x29, 0x99]; // Jump to address 0x999, store pc in stack
    let w2 : [u8; 2] = [0x00, 0xEE]; // Load address in stack, jump to address
    let w3 : [u8; 2] = [0x00, 0x00]; // Exit
    c.mem.write16((c.pc).into(), w1);
    c.mem.write16(0x999, w2);
    c.mem.write16((c.pc+2).into(), w3);

    let og_stack = c.pc;
    c._run();

    assert_eq!(c.pc as u16, og_stack+4);
    assert_eq!(c.stack.len(), 0);
}

#[test]
fn test_cpu_skip3() {
    let mem: memory::Mem = memory::make_memory();
    let mut c : CPU = make_cpu(mem);
    let w1 : [u8; 2] = [0x68, 0x99]; // Set reg 8 to 0x99
    let w2 : [u8; 2] = [0x38, 0x99]; // Skip an instruction
    let w3 : [u8; 2] = [0x00, 0x00]; // Exit
    let w4 : [u8; 2] = [0x38, 0x10]; // Skip instruction but no skip
    let w5 : [u8; 2] = [0x00, 0x00]; // Exit
    c.mem.write16((c.pc).into(), w1);
    c.mem.write16((c.pc+2).into(), w2);
    c.mem.write16((c.pc+4).into(), w3);
    c.mem.write16((c.pc+6).into(), w4);
    c.mem.write16((c.pc+8).into(), w5);

    let og_pc = c.pc;
    c._run();

    assert_eq!(c.regs[8], 0x99);
    assert_eq!(c.pc, og_pc + 10);
}

#[test]
fn test_cpu_skip4() {
    let mem: memory::Mem = memory::make_memory();
    let mut c : CPU = make_cpu(mem);
    let w1 : [u8; 2] = [0x68, 0x99]; // Set reg 8 to 0x99
    let w2 : [u8; 2] = [0x48, 0x10]; // Skip an instruction
    let w3 : [u8; 2] = [0x00, 0x00]; // Exit
    let w4 : [u8; 2] = [0x48, 0x99]; // Skip instruction but no skip
    let w5 : [u8; 2] = [0x00, 0x00]; // Exit
    c.mem.write16((c.pc).into(), w1);
    c.mem.write16((c.pc+2).into(), w2);
    c.mem.write16((c.pc+4).into(), w3);
    c.mem.write16((c.pc+6).into(), w4);
    c.mem.write16((c.pc+8).into(), w5);

    let og_pc = c.pc;
    c._run();

    assert_eq!(c.regs[8], 0x99);
    assert_eq!(c.pc, og_pc + 10);
}

#[test]
fn test_cpu_skip5() {
    let mem: memory::Mem = memory::make_memory();
    let mut c : CPU = make_cpu(mem);
    let w1 : [u8; 2] = [0x68, 0x99]; // Set reg 8 to 0x99
    let w2 : [u8; 2] = [0x69, 0x99]; // Set reg 8 to 0x99
    let w3 : [u8; 2] = [0x58, 0x90]; // Skip an instruction
    let w4 : [u8; 2] = [0x00, 0x00]; // Exit
    let w5 : [u8; 2] = [0x98, 0x90]; // Skip instruction == false
    let w6 : [u8; 2] = [0x00, 0x00]; // Exit
    c.mem.write16((c.pc).into(), w1);
    c.mem.write16((c.pc+2).into(), w2);
    c.mem.write16((c.pc+4).into(), w3);
    c.mem.write16((c.pc+6).into(), w4);
    c.mem.write16((c.pc+8).into(), w5);
    c.mem.write16((c.pc+10).into(), w6);

    let og_pc = c.pc;
    c._run();

    assert_eq!(c.regs[8], 0x99);
    assert_eq!(c.regs[9], 0x99);
    assert_eq!(c.pc, og_pc + 12);
}

#[test]
fn test_cpu_skip9() {
    let mem: memory::Mem = memory::make_memory();
    let mut c : CPU = make_cpu(mem);
    let w1 : [u8; 2] = [0x68, 0xf9]; // Set reg 8 to 0x99
    let w2 : [u8; 2] = [0x69, 0x99]; // Set reg 8 to 0x99
    let w3 : [u8; 2] = [0x98, 0x90]; // Skip an instruction
    let w4 : [u8; 2] = [0x00, 0x00]; // Exit
    let w5 : [u8; 2] = [0x58, 0x90]; // Skip instruction == false
    let w6 : [u8; 2] = [0x00, 0x00]; // Exit
    c.mem.write16((c.pc).into(), w1);
    c.mem.write16((c.pc+2).into(), w2);
    c.mem.write16((c.pc+4).into(), w3);
    c.mem.write16((c.pc+6).into(), w4);
    c.mem.write16((c.pc+8).into(), w5);
    c.mem.write16((c.pc+10).into(), w6);

    let og_pc = c.pc;
    c._run();

    assert_eq!(c.regs[8], 0xf9);
    assert_eq!(c.regs[9], 0x99);
    assert_eq!(c.pc, og_pc + 12);
}

#[test]
fn test_cpu_set() {
    let mem: memory::Mem = memory::make_memory();
    let mut c : CPU = make_cpu(mem);
    let w1 : [u8; 2] = [0x68, 0x99]; // Set reg 8 to 0x99
    let w2 : [u8; 2] = [0x00, 0x00]; // Exit
    c.mem.write16((c.pc).into(), w1);
    c.mem.write16((c.pc+2).into(), w2);
    c._run();
    assert_eq!(c.regs[8], 0x99);
}

#[test]
fn test_cpu_add() {
    let mem: memory::Mem = memory::make_memory();
    let mut c : CPU = make_cpu(mem);
    let w1 : [u8; 2] = [0x78, 0x20]; // Jump to address 0x999
    let w2 : [u8; 2] = [0x78, 0x15]; // Jump to address 0x999
    let w3 : [u8; 2] = [0x00, 0x00]; // Exit
    c.mem.write16((c.pc).into(), w1);
    c.mem.write16((c.pc+2).into(), w2);
    c.mem.write16((c.pc+4).into(), w3);
    c._run();
    assert_eq!(c.regs[8], 0x20 + 0x15);
}

#[test]
fn test_cpu_set_reg() {
    let mem: memory::Mem = memory::make_memory();
    let mut c : CPU = make_cpu(mem);
    let w1 : [u8; 2] = [0x68, 0x99]; // Set reg 8 to 0x99
    let w2 : [u8; 2] = [0x89, 0x80]; // Set reg 9 to 0x99
    let w3 : [u8; 2] = [0x00, 0x00]; // Exit
    c.mem.write16((c.pc).into(), w1);
    c.mem.write16((c.pc+2).into(), w2);
    c.mem.write16((c.pc+4).into(), w3);
    c._run();
    assert_eq!(c.regs[8], 0x99);
    assert_eq!(c.regs[9], 0x99);
}

#[test]
fn test_cpu_bin_or() {
    let mem: memory::Mem = memory::make_memory();
    let mut c : CPU = make_cpu(mem);
    let w1 : [u8; 2] = [0x68, 0xbe]; // Set reg 8 to 0xde
    let w2 : [u8; 2] = [0x69, 0xef]; // Set reg 9 to 0xad
    let w3 : [u8; 2] = [0x88, 0x91]; // reg 8 (logical or) reg 9
    let w4 : [u8; 2] = [0x00, 0x00]; // Exit
    c.mem.write16((c.pc).into(), w1);
    c.mem.write16((c.pc+2).into(), w2);
    c.mem.write16((c.pc+4).into(), w3);
    c.mem.write16((c.pc+6).into(), w4);
    c._run();
    assert_eq!(c.regs[8], 0xbe | 0xef);
    assert_eq!(c.regs[9], 0xef);
}

#[test]
fn test_cpu_bin_and() {
    let mem: memory::Mem = memory::make_memory();
    let mut c : CPU = make_cpu(mem);
    let w1 : [u8; 2] = [0x68, 0xbe]; // Set reg 8 to 0xde
    let w2 : [u8; 2] = [0x69, 0xef]; // Set reg 9 to 0xad
    let w3 : [u8; 2] = [0x88, 0x92]; // reg 8 (logical and) reg 9
    let w4 : [u8; 2] = [0x00, 0x00]; // Exit
    c.mem.write16((c.pc).into(), w1);
    c.mem.write16((c.pc+2).into(), w2);
    c.mem.write16((c.pc+4).into(), w3);
    c.mem.write16((c.pc+6).into(), w4);
    c._run();
    assert_eq!(c.regs[8], 0xbe & 0xef);
    assert_eq!(c.regs[9], 0xef);
}

#[test]
fn test_cpu_bin_xor() {
    let mem: memory::Mem = memory::make_memory();
    let mut c : CPU = make_cpu(mem);
    let w1 : [u8; 2] = [0x68, 0xbe]; // Set reg 8 to 0xde
    let w2 : [u8; 2] = [0x69, 0xef]; // Set reg 9 to 0xad
    let w3 : [u8; 2] = [0x88, 0x93]; // reg 8 (logical xor) reg 9
    let w4 : [u8; 2] = [0x00, 0x00]; // Exit
    c.mem.write16((c.pc).into(), w1);
    c.mem.write16((c.pc+2).into(), w2);
    c.mem.write16((c.pc+4).into(), w3);
    c.mem.write16((c.pc+6).into(), w4);
    c._run();
    assert_eq!(c.regs[8], 0xbe ^ 0xef);
    assert_eq!(c.regs[9], 0xef);
}

#[test]
fn test_cpu_reg_add() {
    let mem: memory::Mem = memory::make_memory();
    let mut c : CPU = make_cpu(mem);
    let w1 : [u8; 2] = [0x68, 0xbe]; // Set reg 8 to 0xde
    let w2 : [u8; 2] = [0x69, 0xef]; // Set reg 9 to 0xad
    let w3 : [u8; 2] = [0x88, 0x94]; // reg 8 (+) reg 9
    let w4 : [u8; 2] = [0x00, 0x00]; // Exit
    c.mem.write16((c.pc).into(), w1);
    c.mem.write16((c.pc+2).into(), w2);
    c.mem.write16((c.pc+4).into(), w3);
    c.mem.write16((c.pc+6).into(), w4);
    c._run();
    assert_eq!(c.regs[8], (0xbe + 0xef) as u8);
    assert_eq!(c.regs[9], 0xef);
    assert_eq!(c.regs[15], 0x01);

    let mem2: memory::Mem = memory::make_memory();
    let mut c2 : CPU = make_cpu(mem2);
    let w1 : [u8; 2] = [0x68, 0x02]; // Set reg 8 to 0xde
    let w2 : [u8; 2] = [0x69, 0x50]; // Set reg 9 to 0xad
    let w3 : [u8; 2] = [0x88, 0x94]; // reg 8 (+) reg 9
    let w4 : [u8; 2] = [0x00, 0x00]; // Exit
    c2.mem.write16((c2.pc).into(), w1);
    c2.mem.write16((c2.pc+2).into(), w2);
    c2.mem.write16((c2.pc+4).into(), w3);
    c2.mem.write16((c2.pc+6).into(), w4);
    c2._run();
    assert_eq!(c2.regs[8], (0x02 + 0x50) as u8);
    assert_eq!(c2.regs[9], 0x50);
    assert_eq!(c2.regs[15], 0x00);
}

#[test]
fn test_cpu_reg_subxy() {
    let mem: memory::Mem = memory::make_memory();
    let mut c : CPU = make_cpu(mem);
    let w1 : [u8; 2] = [0x68, 0xbe]; // Set reg 8 to 0xde
    let w2 : [u8; 2] = [0x69, 0xef]; // Set reg 9 to 0xad
    let w3 : [u8; 2] = [0x88, 0x95]; // reg 8 (+) reg 9
    let w4 : [u8; 2] = [0x00, 0x00]; // Exit
    c.mem.write16((c.pc).into(), w1);
    c.mem.write16((c.pc+2).into(), w2);
    c.mem.write16((c.pc+4).into(), w3);
    c.mem.write16((c.pc+6).into(), w4);
    c._run();
    assert_eq!(c.regs[8], (0xbe - 0xef) as u8, "Testing underflow");
    assert_eq!(c.regs[9], 0xef);
    assert_eq!(c.regs[15], 0x00);

    let mem2: memory::Mem = memory::make_memory();
    let mut c2 : CPU = make_cpu(mem2);
    let w1 : [u8; 2] = [0x68, 0x50]; // Set reg 8 to 0xde
    let w2 : [u8; 2] = [0x69, 0x02]; // Set reg 9 to 0xad
    let w3 : [u8; 2] = [0x88, 0x95]; // reg 8 (+) reg 9
    let w4 : [u8; 2] = [0x00, 0x00]; // Exit
    c2.mem.write16((c2.pc).into(), w1);
    c2.mem.write16((c2.pc+2).into(), w2);
    c2.mem.write16((c2.pc+4).into(), w3);
    c2.mem.write16((c2.pc+6).into(), w4);
    c2._run();
    assert_eq!(c2.regs[8], (0x50 - 0x02) as u8, "Testing no underflow");
    assert_eq!(c2.regs[9], 0x02);
    assert_eq!(c2.regs[15], 0x01);
}

#[test]
fn test_cpu_reg_subyx() {
    let mem: memory::Mem = memory::make_memory();
    let mut c : CPU = make_cpu(mem);
    let w1 : [u8; 2] = [0x68, 0xbe]; // Set reg 8 to 0xde
    let w2 : [u8; 2] = [0x69, 0xef]; // Set reg 9 to 0xad
    let w3 : [u8; 2] = [0x88, 0x97]; // reg 8 (+) reg 9
    let w4 : [u8; 2] = [0x00, 0x00]; // Exit
    c.mem.write16((c.pc).into(), w1);
    c.mem.write16((c.pc+2).into(), w2);
    c.mem.write16((c.pc+4).into(), w3);
    c.mem.write16((c.pc+6).into(), w4);
    c._run();
    assert_eq!(c.regs[8], (0xef - 0xbe) as u8, "Testing no underflow");
    assert_eq!(c.regs[9], 0xef);
    assert_eq!(c.regs[15], 0x01);

    let mem2: memory::Mem = memory::make_memory();
    let mut c2 : CPU = make_cpu(mem2);
    let w1 : [u8; 2] = [0x68, 0x50]; // Set reg 8 to 0xde
    let w2 : [u8; 2] = [0x69, 0x02]; // Set reg 9 to 0xad
    let w3 : [u8; 2] = [0x88, 0x97]; // reg 8 (+) reg 9
    let w4 : [u8; 2] = [0x00, 0x00]; // Exit
    c2.mem.write16((c2.pc).into(), w1);
    c2.mem.write16((c2.pc+2).into(), w2);
    c2.mem.write16((c2.pc+4).into(), w3);
    c2.mem.write16((c2.pc+6).into(), w4);
    c2._run();
    assert_eq!(c2.regs[8], (0x02 - 0x50) as u8, "Testing underflow");
    assert_eq!(c2.regs[9], 0x02);
    assert_eq!(c2.regs[15], 0x00);
}

#[test]
fn test_cpu_left_shift() {
    let mem: memory::Mem = memory::make_memory();
    let mut c : CPU = make_cpu(mem);
    let w1 : [u8; 2] = [0x68, 0x80]; // Set reg 8 to 0x80
    let w2 : [u8; 2] = [0x88, 0x0E]; // Shift reg 8 left 1
    let w3 : [u8; 2] = [0x00, 0x00]; // Exit
    c.mem.write16((c.pc).into(), w1);
    c.mem.write16((c.pc+2).into(), w2);
    c.mem.write16((c.pc+4).into(), w3);
    c._run();
    assert_eq!(c.regs[8], 0x00);
    assert_eq!(c.regs[15], 0x01);
    let w4 : [u8; 2] = [0x68, 0x01]; // Set reg 8 to 0x01
    let w5 : [u8; 2] = [0x88, 0x0E]; // Shift reg 8 left 1
    let w6 : [u8; 2] = [0x00, 0x00]; // Exit
    c.mem.write16((c.pc).into(), w4);
    c.mem.write16((c.pc+2).into(), w5);
    c.mem.write16((c.pc+4).into(), w6);
    c._run();
    assert_eq!(c.regs[8], 0x02);
    assert_eq!(c.regs[15], 0x00);
}

#[test]
fn test_cpu_right_shift() {
    let mem: memory::Mem = memory::make_memory();
    let mut c : CPU = make_cpu(mem);
    let w1 : [u8; 2] = [0x68, 0x80]; // Set reg 8 to 0x80
    let w2 : [u8; 2] = [0x88, 0x06]; // Shift reg 8 left 1
    let w3 : [u8; 2] = [0x00, 0x00]; // Exit
    c.mem.write16((c.pc).into(), w1);
    c.mem.write16((c.pc+2).into(), w2);
    c.mem.write16((c.pc+4).into(), w3);
    c._run();
    assert_eq!(c.regs[8], 0x40);
    assert_eq!(c.regs[15], 0x00);
    let w4 : [u8; 2] = [0x68, 0x01]; // Set reg 8 to 0x01
    let w5 : [u8; 2] = [0x88, 0x06]; // Shift reg 8 left 1
    let w6 : [u8; 2] = [0x00, 0x00]; // Exit
    c.mem.write16((c.pc).into(), w4);
    c.mem.write16((c.pc+2).into(), w5);
    c.mem.write16((c.pc+4).into(), w6);
    c._run();
    assert_eq!(c.regs[8], 0x00);
    assert_eq!(c.regs[15], 0x01);
}

#[test]
fn test_cpu_set_ireg() {
    let mem: memory::Mem = memory::make_memory();
    let mut c : CPU = make_cpu(mem);
    let w1 : [u8; 2] = [0xA9, 0x99]; // Set I reg to 0x999
    let w2 : [u8; 2] = [0x00, 0x00]; // Exit
    c.mem.write16((c.pc).into(), w1);
    c.mem.write16((c.pc+2).into(), w2);
    c._run();
    assert_eq!(c.ireg, 0x999);
}

#[test]
fn test_cpu_jump_offset() {
    let mem: memory::Mem = memory::make_memory();
    let mut c : CPU = make_cpu(mem);
    c.regs[0] = 0x20;
    let w1 : [u8; 2] = [0xB8, 0x00]; // Jump to address 0x999
    let w2 : [u8; 2] = [0x00, 0xE0]; // Clear display
    let w3 : [u8; 2] = [0x00, 0x00]; // Exit
    c.mem.write16((c.pc).into(), w1);
    c.mem.write16(0x820, w2);
    c.mem.write16(0x822 + 2, w3);
    c.display.screen[0][0] = true;
    c.display.screen[1][2] = true;
    c.display.screen[2][4] = true;

    c._run();

    let default_dis: display::Display = display::make_display();
    assert_eq!(c.display.screen, default_dis.screen);
    assert_eq!(c.pc as u16, 0x820 + 4);
}

#[test]
fn test_cpu_radnom() {
    let mem: memory::Mem = memory::make_memory();
    let mut c : CPU = make_cpu(mem);
    c.regs[0] = 0x20;
    let w1 : [u8; 2] = [0xC8, 0x25]; // Jump to address 0x999
    let w2 : [u8; 2] = [0x00, 0x00]; // Exit
    c.mem.write16((c.pc).into(), w1);
    c.mem.write16((c.pc+2).into(), w2);
    c._run();

    assert_eq!(c.regs[8] | 0x25, 0x25);

}

#[test]
fn test_cpu_dxyn() {
    let mem: memory::Mem = memory::make_memory();
    let mut c : CPU = make_cpu(mem);
    let w1 : [u8; 2] = [0xA9, 0x99]; // Jump to address 0x999
    let w2 : [u8; 2] = [0x00, 0x00]; // Exit
    c.mem.write16((c.pc).into(), w1);
    c.mem.write16((c.pc+2).into(), w2);
    // c.mem.write16((c.pc+4).into(), w3);
    c._run();
    assert_eq!(c.ireg, 0x999);
}

#[test]
fn test_cpu_time() {
    let mem: memory::Mem = memory::make_memory();
    let mut c : CPU = make_cpu(mem);
    let w1 : [u8; 2] = [0x68, 0x80]; // Set reg 8 to 0x80
    let w2 : [u8; 2] = [0x69, 0x20]; // Set reg 9 to 0x20
    let w3 : [u8; 2] = [0xF8, 0x15]; // Set Delay timer to be value in reg 8
    let w4 : [u8; 2] = [0xF9, 0x18]; // Set Sound timer to be value in reg 9
    let w5 : [u8; 2] = [0xF2, 0x07]; // Set reg 2 to Delay timer
    let w6 : [u8; 2] = [0x00, 0x00]; // Exit
    c.mem.write16((c.pc).into(), w1);
    c.mem.write16((c.pc+2).into(), w2);
    c.mem.write16((c.pc+4).into(), w3);
    c.mem.write16((c.pc+6).into(), w4);
    c.mem.write16((c.pc+8).into(), w5);
    c.mem.write16((c.pc+10).into(), w6);
    c._run();
    assert_eq!(c.regs[8], 0x80);
    assert_eq!(c.regs[9], 0x20);
    assert_eq!(c.delay_timer.value, c.regs[8]);
    assert_eq!(c.sound_timer.value, c.regs[9]);
    assert_eq!(c.delay_timer.value, c.regs[2]);
}

#[test]
fn test_cpu_add_regi() {
    let mem: memory::Mem = memory::make_memory();
    let mut c : CPU = make_cpu(mem);
    let w1 : [u8; 2] = [0x68, 0x80]; // Set reg 8 to 0x80
    let w2 : [u8; 2] = [0xA0, 0x20]; // Set I reg to 0x020
    let w3 : [u8; 2] = [0xF8, 0x1E]; // reg 8 + i reg
    let w4 : [u8; 2] = [0x00, 0x00]; // Exit
    c.mem.write16((c.pc).into(), w1);
    c.mem.write16((c.pc+2).into(), w2);
    c.mem.write16((c.pc+4).into(), w3);
    c.mem.write16((c.pc+6).into(), w4);
    c._run();
    assert_eq!(c.regs[8], 0x80);
    assert_eq!(c.ireg, 0x80 + 0x20, "Failed the 1st test");
    assert_eq!(c.regs[0xf], 0x00);
    let w4 : [u8; 2] = [0x68, 0x80]; // Set reg 8 to 0x80
    let w5 : [u8; 2] = [0xAF, 0xFF]; // Set I reg to 0xFFF
    let w6 : [u8; 2] = [0xF8, 0x1E]; // reg 8 + i reg
    let w7 : [u8; 2] = [0x00, 0x00]; // Exit
    c.mem.write16((c.pc).into(), w4);
    c.mem.write16((c.pc+2).into(), w5);
    c.mem.write16((c.pc+4).into(), w6);
    c.mem.write16((c.pc+6).into(), w7);
    c._run();
    assert_eq!(c.regs[8], 0x80);
    assert_eq!(c.ireg, (0x080 + 0xfff), "Failed the second test");
    assert_eq!(c.regs[0xf], 0x01);
}

#[test]
fn test_cpu_font_char() {
    let mem: memory::Mem = memory::make_memory();
    let mut c : CPU = make_cpu(mem);
    let w1 : [u8; 2] = [0x68, 0x84]; // Set reg 8 to 0x84
    let w2 : [u8; 2] = [0xF8, 0x29]; // Set I reg to address of the 4th char
    let w3 : [u8; 2] = [0x00, 0x00]; // Exit
    c.mem.write16((c.pc).into(), w1);
    c.mem.write16((c.pc+2).into(), w2);
    c.mem.write16((c.pc+4).into(), w3);
    c._run();
    assert_eq!(c.regs[8], 0x84);
    assert_eq!(c.ireg, (_FONT_START as u16) + ((5 * 4) as u16) , "Checking for font");
    assert_eq!(c.mem.data[c.ireg as usize], 0x90 , "Checking for first byte of font");
}

#[test]
fn test_cpu_binary_coded_dec_conv() {
    let mem: memory::Mem = memory::make_memory();
    let mut c : CPU = make_cpu(mem);
    let w1 : [u8; 2] = [0x68, 0x9c]; // Set reg 8 to 0x9c
    let w2 : [u8; 2] = [0xA9, 0x99]; // Set I reg to 0x999
    let w3 : [u8; 2] = [0xF8, 0x33]; // Set mem[ireg] = dec version of 0x9c = 156
    let w4 : [u8; 2] = [0x00, 0x00]; // Exit
    c.mem.write16((c.pc).into(), w1);
    c.mem.write16((c.pc+2).into(), w2);
    c.mem.write16((c.pc+4).into(), w3);
    c.mem.write16((c.pc+6).into(), w4);
    c._run();
    assert_eq!(c.regs[8], 0x9c);
    assert_eq!(c.mem.read8(0x999), 0x01, "Checking first address");
    assert_eq!(c.mem.read8(0x999+1), 0x05, "Checking second address");
    assert_eq!(c.mem.read8(0x999+2), 0x06, "Checking third address");

    let w5 : [u8; 2] = [0x68, 0xff]; // Set reg 8 to 0x9c
    let w6 : [u8; 2] = [0xA9, 0x99]; // Set I reg to 0x999
    let w7 : [u8; 2] = [0xF8, 0x33]; // Set mem[ireg] = dec version of 0x9c = 156
    let w8 : [u8; 2] = [0x00, 0x00]; // Exit
    c.mem.write16((c.pc).into(), w5);
    c.mem.write16((c.pc+2).into(), w6);
    c.mem.write16((c.pc+4).into(), w7);
    c.mem.write16((c.pc+6).into(), w8);
    c._run();
    assert_eq!(c.regs[8], 0xff);
    assert_eq!(c.mem.read8(0x999), 0x02, "Checking first address");
    assert_eq!(c.mem.read8(0x999+1), 0x05, "Checking second address");
    assert_eq!(c.mem.read8(0x999+2), 0x05, "Checking third address");
}

#[test]
fn test_cpu_store() {
    let mem: memory::Mem = memory::make_memory();
    let mut c : CPU = make_cpu(mem);
    /* Store values into the registers */
    for i in 0..16 {
        let w : [u8; 2] = [0x60 + i, i as u8]; // Set reg 8 to 0x9c
        c.mem.write16((c.pc+((2*i) as u16)).into(), w);
    }
    c._run();
    assert_eq!(c.regs[0], 0x0);
    assert_eq!(c.regs[1], 0x1);
    assert_eq!(c.regs[0xf], 0xf);

    /* Load values from the registers into the address of 0x999 */
    let w1 : [u8; 2] = [0xA9, 0x99]; // Set I reg to 0x999
    let w2 : [u8; 2] = [0xF8, 0x55]; // Load the values from registers 0-8 into memory @ i reg
    let w3 : [u8; 2] = [0x00, 0x00]; // Exit
    c.mem.write16((c.pc).into(), w1);
    c.mem.write16((c.pc+2).into(), w2);
    c.mem.write16((c.pc+4).into(), w3);
    c._run();
    assert_eq!(c.mem.read8(0x999), 0x0, "Checking 0 address");
    assert_eq!(c.mem.read8(0x999+1), 0x01, "Checking 1 address");
    assert_eq!(c.mem.read8(0x999+2), 0x02, "Checking 2 address");
    assert_eq!(c.mem.read8(0x999+3), 0x03, "Checking 3 address");
    assert_eq!(c.mem.read8(0x999+4), 0x04, "Checking 4 address");
    assert_eq!(c.mem.read8(0x999+5), 0x05, "Checking 5 address");
    assert_eq!(c.mem.read8(0x999+6), 0x06, "Checking 6 address");
    assert_eq!(c.mem.read8(0x999+7), 0x07, "Checking 7 address");
    assert_eq!(c.mem.read8(0x999+8), 0x08, "Checking 8 address");
    assert_eq!(c.mem.read8(0x999+9), 0x00, "Checking 9 address");
}

#[test]
fn test_cpu_load() {
    let mem: memory::Mem = memory::make_memory();
    let mut c : CPU = make_cpu(mem);
    /* Store values into 0x999 + i */
    for i in 0..16 {
        c.mem.write8((0x999+i as u16).into(), i as u8);
    }
    assert_eq!(c.mem.read8(0x999), 0x0);
    assert_eq!(c.mem.read8(0x999+1), 0x1);
    assert_eq!(c.mem.read8(0x999+0xf), 0xf);

    /* Load values from the registers into the address of 0x999 */
    let w1 : [u8; 2] = [0xA9, 0x99]; // Set I reg to 0x999
    let w2 : [u8; 2] = [0xF6, 0x65]; // Load the values from registers 0-6 into memory @ i reg
    let w3 : [u8; 2] = [0x00, 0x00]; // Exit
    c.mem.write16((c.pc).into(), w1);
    c.mem.write16((c.pc+2).into(), w2);
    c.mem.write16((c.pc+4).into(), w3);
    c._run();
    assert_eq!(c.regs[0], 0x00, "Checking 0 address");
    assert_eq!(c.regs[1], 0x01, "Checking 1 address");
    assert_eq!(c.regs[2], 0x02, "Checking 2 address");
    assert_eq!(c.regs[3], 0x03, "Checking 3 address");
    assert_eq!(c.regs[4], 0x04, "Checking 4 address");
    assert_eq!(c.regs[5], 0x05, "Checking 5 address");
    assert_eq!(c.regs[6], 0x06, "Checking 6 address");
    assert_eq!(c.regs[7], 0x00, "Checking 7 address");
    assert_eq!(c.regs[8], 0x00, "Checking 8 address");
}
