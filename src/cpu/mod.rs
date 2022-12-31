use super::*;
use crate::memory::_FONT_START;
use rand::Rng;
use time::{ Instant };


pub struct CPU {
    pub mem : memory::Mem, /* Memory for our cpu */
    pub display: display::Display, /* Repr of display */
    pub stack: Vec<u16>, /* Program stack */
    pub pc: u16, /* Program counter */
    pub regs: [u8; 16], /* Program variable registers */
    pub ireg: u16, /* Index registers */
    pub sound_timer : Timer, /* Sound timer */
    pub delay_timer : Timer, /* Delay timer */
}


/* A struct that breaks up the current instruction 
 * we are about to execute */
#[derive(Debug)]
#[derive(PartialEq)]
pub struct Instruction {
    inst : [u8; 2], /* The actual instruction */
    nib1 : u8, /* x... */
    nib2 : u8, /* .x.. */
    nib3 : u8, /* ..x. */
    nib4 : u8, /* ...x */
    nib234 : u16, /* Last 3 nibbles => .xxx */
    byte : u16, /* Instruction in 16 bit form */
}

/* Timer struct. Used for delay and sound timer. Uses start_time to get a DateTime
 * obj, then uses that to determine how much time has elapsed, and store that into 
 * CURR_SEC each second. Timers are both supposed to decrememnt at 60 ticks per 
 * second. Use BASE_VALUE for the start of the interval, and VALUE to determine
 * how many ticks have occured inthis interval */
pub struct Timer {
    start_time : Instant, /* Start time, used to calculate time elapsed */
    pub curr_second : u64, /* Place holder for How much time has elapsed */
    base_value : u8, // Base value that timer started at 
    value : u8, // Current value timer is at
}


/* Representation of the application state. 
 * Probably doesn't need to be a struct, 
 * only necessary fn is draw. In the future, 
 * would simply bake the draw fn into CPU, 
 * or if things actually start needing some more
 * info bake cpu into WORLD alongside other fields*/
pub struct World {
}

/* Make a cpu struct with MEM. Should be made more like a constructor
 * for CPU class => inside of an impl statement. Would change but
 * don't feel like updating every test to do. 
 */
pub fn make_cpu(mem: memory::Mem) -> CPU { 
    let dis: display::Display = display::make_display();

    let mut _stack: Vec<u16> = vec![];

    let mut _core: CPU = CPU{
        mem: mem,
        display: dis,
        stack: _stack,
        pc: memory::_PROG_START as u16,
        regs: [0x0 as u8; 16],
        ireg: 0x0,
        sound_timer : Timer::new(),
        delay_timer : Timer::new(),
    };
    return _core;
}

/* Make an instruction out of the two bytes. Similar to CPU struct,
 * should be a constructor for INSTRUCTION class 
 */
pub fn make_instruction(data : [u8; 2])-> Instruction {
    let mut out = Instruction {
        inst: data,
        nib1: data[0] >> 4,
        nib2: (data[0] << 4) >> 4,
        nib3: data[1] >> 4,
        nib4: (data[1] << 4) >> 4,
        nib234: 0x0,
        byte: (data[0] as u16) << 8 | data[1] as u16,

    };
    out.nib234 = (out.nib2 as u16) << 8 | data[1] as u16;
    return out;

}

/* CPU functionality: fetch, decode, exec */
impl CPU {
    /* Decodes the instruction that is at the current program counter */
    fn decode_instruction(&mut self, instruction : Instruction, input_key : u8) {
        // Switch case for the first nibble of the instruction
        match instruction.nib1 {
            0 => match instruction.inst[1] {
                0x00 => println!("Finished the program"), // 0000 Exit
                0xE0 => { self.clear_screen(); },  // 00E0
                0xEE => { self.load_pc_and_jump(); }, // 00EE => Load and Jump
                _ => println!("Instruction {:#01x} has not been implemented!",
                              instruction.byte),

            }

            1 => { self.jump(instruction); }, // 1NNN => Jump
            2 => { self.jump_and_store_pc(instruction) }, // 2NNN 
            3 => {  // 3XNN 
                self.skip(self.regs[instruction.nib2 as usize],
                          instruction.inst[1],
                          true
                         ); 
            },
            4 => {  // 4XNN 
                self.skip(self.regs[instruction.nib2 as usize],
                          instruction.inst[1],
                          false
                         ); 
            },
            5 => {  // 5XY0
                self.skip(self.regs[instruction.nib2 as usize],
                          self.regs[instruction.nib3 as usize],
                          true
                         ); 
            },
            6 => { self.set(instruction); } // 6XNN => Set
            7 => { self.add(instruction); }, // 7XNN => Add 
            8 => {
                match instruction.nib4 {

                    0 => { self.set_reg(instruction); } // 8XY0
                    1 => { self.bin_or(instruction);  } // 8XY1
                    2 => { self.bin_and(instruction); } // 8XY2
                    3 => { self.bin_xor(instruction); } // 8XY3
                    4 => { self.reg_add(instruction); } // 8XY4
                                                        // 8XY5
                    5 => { self.reg_sub(instruction.nib2, instruction.nib3, true); }
                    6 => { self.reg_shift(instruction, false); } // 8XY6
                                                                 // 8XY7
                    7 => self.reg_sub(instruction.nib3, instruction.nib2, false),
                    0xE => { self.reg_shift(instruction, true); } // 8XYE
                    _ => println!("Instruction {:#01x} has not been implemented!",
                                  instruction.byte),
                } 
            },
            9 => {
                // 9XY0 
                self.skip(self.regs[instruction.nib2 as usize],
                          self.regs[instruction.nib3 as usize],
                          false
                         );
            },

            0xA => { self.set_ireg(instruction); }, // ANNN => Set I Register
            0xB => { self.jump_offset(instruction); }, // BNNN => Jump w/ offset
            0xC => { self.random(instruction); }, // CXNN => Random, bin. & w X reg
            0xD => { self.dxyn(instruction); }, // DXYN => Draw display
            0xE => { 
                match instruction.nib4 {
                    0x1 => {    /* EXA1 */
                        self.skip(self.regs[instruction.nib2 as usize],
                                  input_key, false); 
                    },
                    0xe => {   /* EX9E */
                        self.skip(self.regs[instruction.nib2 as usize],
                                  input_key, true); 
                    },
                    _ => println!("Instruction {:#01x} has not been implemented!",
                                  instruction.byte),
                }
            }, 
            0xF => {
                match instruction.nib4 {
                    3 => { self.binary_coded_decimal_conv(instruction); } // FX33 => 
                    5 => {
                        match instruction.nib3 {
                            1 => self.store_vx_timer(instruction, true), // FX15 => 
                            5 => self.store(instruction), // FX55 => 
                            6 => self.load(instruction), // FX65 => 
                            _ => println!("Instruction {:#01x} not implemented!",
                                          instruction.byte),
                        }
                    }  
                    7 => { self.set_vx_delaytimer(instruction); } // FX07 => 
                    8 => { self.store_vx_timer(instruction, false); } // FX18 => 
                    9 => { self.font_char(instruction); } // FX29 => 
                    0xA => { self.get_key(instruction, input_key); } // FX0A => 
                    0xE => { self.add_regi(instruction); } // FX1E => 

                    _ => println!("Instruction {:#01x} has not been implemented!",
                                  instruction.byte),
                }
            }
            /* Invalid instruction was passed in */
            _ => println!("Instruction {:#01x} has not been implemented!",
                          instruction.byte),
        }
    }

    /* Fetches the next instruction from the program, updates program counter */
    fn fetch_next_instruction(&mut self)->Instruction {
        let data : [u8; 2] = self.mem.read16(self.pc as usize);

        let out : Instruction = make_instruction(data);
        self.pc += 2;
        return out;
    }

    /* Execute one instruction */
    pub fn exec(&mut self, input_key : u8) ->bool {
        let next : Instruction = self.fetch_next_instruction();
        if next.nib1 == 0 && next.nib234 == 0x000 {
            return false;
        }
        self.decode_instruction(next, input_key);
        return true;
    }

    /* Used for testing, execute all instructions until stopped */
    pub fn _run(&mut self) {
        loop {
            let not_done : bool = self.exec(0xf0);
            if !not_done {
                break;
            }
        }     
    }

}

/* Instruction set */
impl CPU {

    /* Clears the display to be all false values */
    fn clear_screen(&mut self) {
        for pixel_row in 0..self.display.screen.len() {
            for pixel_col in 0..self.display.screen[pixel_row].len() {
                self.display.screen[pixel_row][pixel_col] = false;
            }
        }
    }

    /* Loads pc from stack and jumps to it */
    fn load_pc_and_jump(&mut self) {
        /* Pop program counter */
        if self.stack.len() > 0 {
            self.pc = self.stack.pop().unwrap();
        }
    }

    /* Used for jump statements */
    fn goto(&mut self, address : u16) {
        self.pc = address;
    }

    /* Sets PC to an address */
    fn jump(&mut self, instruction : Instruction) {
        if instruction.nib234 > self.mem.data.len() as u16 {
            println!("Invalid address to jump to");
        }
        self.goto(instruction.nib234);
    }

    /* Jump and stores pc onto the stack */
    fn jump_and_store_pc(&mut self, instruction : Instruction) {
        /* Push the program counter onto the stack */
        self.stack.push(self.pc as u16);

        /* Jump to desired address */
        self.jump(instruction);
    }

    /* Skips an instruction.
     * cheq_eq == true => skip if v1 == v2
     * !check_eq => skip if v1 != v2 
     */
    fn skip(&mut self, val1 : u8, val2 : u8, check_eq : bool) {
        let mut skip : bool = false;

        if check_eq {
            skip = if val1 == val2 { true } else { skip };
        } else {
            skip = if val1 != val2 { true } else { skip };
        }

        if skip {
            self.pc += 2;
        }

    }

    /* Set X register to an immediate */
    fn set(&mut self, instruction : Instruction) {
        if instruction.nib2 > 15 {
            println!("Failed to set a register");
        }
        self.regs[instruction.nib2 as usize] = instruction.inst[1];
    }

    /* Add an immediate to the x register */
    fn add(&mut self, instruction : Instruction) {
        if instruction.nib2 > 15 {
            println!("Failed to set a register");
        }

        let add_result : (u8, bool) = self.regs[instruction.nib2 as usize].
            overflowing_add(instruction.inst[1]);
        self.regs[instruction.nib2 as usize] = add_result.0;
    }

    /* Set the X register to be the Y register */
    fn set_reg(&mut self, instruction : Instruction) {
        if instruction.nib2 > 15 || instruction.nib3 > 15 {
            println!("Failed to set a register");
        }

        self.regs[instruction.nib2 as usize] = self.regs[instruction.nib3 as usize];
    }

    /* X |= Y */
    fn bin_or(&mut self, instruction : Instruction) {
        self.regs[instruction.nib2 as usize] |= self.regs[instruction.nib3 as usize];
    }

    /* X &&= Y */
    fn bin_and(&mut self, instruction : Instruction) {
        self.regs[instruction.nib2 as usize] &= self.regs[instruction.nib3 as usize];
    }

    /* X ^= Y */
    fn bin_xor(&mut self, instruction : Instruction) {
        self.regs[instruction.nib2 as usize] ^= self.regs[instruction.nib3 as usize];
    }

    /* X += Y */
    fn reg_add(&mut self, instruction : Instruction) {
        let add_result : (u8, bool) = self.regs[instruction.nib2 as usize].
            overflowing_add(self.regs[instruction.nib3 as usize]);

        self.regs[0x0f] = if add_result.1 { 0x01 } else {0x00};
        self.regs[instruction.nib2 as usize] = add_result.0;
    }

    /* X -= Y */
    fn reg_sub(&mut self, reg1 : u8, reg2 : u8, set_reg1 : bool) {
        let sub_result : (u8, bool) = self.regs[reg1 as usize].
            overflowing_sub(self.regs[reg2 as usize]);

        self.regs[0x0f] = if sub_result.1 { 0x00 } else {0x01};
        if set_reg1 { self.regs[reg1 as usize] = sub_result.0;}
        else { self.regs[reg2 as usize] = sub_result.0; }

    }

    /* X <<  Y or X >> Y, based on LEFT_SHIFT  */
    fn reg_shift(&mut self, instruction : Instruction, left_shift : bool) {
        if !left_shift {
            self.regs[0x0f] = self.regs[instruction.nib2 as usize] & 0x01;
            self.regs[instruction.nib2 as usize] >>= 1;
        } else {
            self.regs[0x0f] = (self.regs[instruction.nib2 as usize] & 0x80) >> 7;
            self.regs[instruction.nib2 as usize] <<= 1;
        }
    }

    /* Set the i register to an immediate */
    fn set_ireg(&mut self, instruction : Instruction) {
        self.ireg = 0;
        self.ireg = instruction.nib234;
    }

    /* Jump to value stored in the 0th register + immediate offset */
    fn jump_offset(&mut self, instruction : Instruction) {
        let address : u16 = (self.regs[0] as u16) + instruction.nib234;
        self.goto(address);
    }

    /* Gets a random number, and binary & it with value in X reg */
    fn random(&mut self, instruction : Instruction) {
        let mut rng = rand::thread_rng();
        let n: u8 = rng.gen();

        self.regs[instruction.nib2 as usize] = instruction.inst[1] & n;
    }


    /* Draws the display */
    fn dxyn(&mut self, instruction : Instruction) {
        /* Get the x and y coords of the instruction */
        let vx : u8 = self.regs[instruction.nib2 as usize] % 64;
        let vy : u8 = self.regs[instruction.nib3 as usize] % 32;

        self.regs[0xF] = 0; /* Set VF = 0 */
        for i in 0..instruction.nib4 as usize {
            let pixel_sprite : u8 = self.mem.data[self.ireg as usize + i];
            for pixel_i in 0..8 {
                let mask = 0x80 >> pixel_i;
                let pixel = pixel_sprite & mask;

                if pixel != 0 {
                    if self.display.screen[vy as usize+ i][vx as usize + pixel_i] != false {
                        self.regs[0xF] = 1;
                    }
                    self.display.screen[vy as usize+ i][vx as usize + pixel_i] ^= true;
                }
            }
        }
    }

    /* Stores the decimal version of value in reg X to the 
     * addy(i reg)...addy(i reg + 2) */
    fn binary_coded_decimal_conv(&mut self, instruction : Instruction) {
        let mut num_to_conv : u8 = self.regs[instruction.nib2 as usize];

        let multiplier : i16 = 10;
        for i in 0..3 {
            let dec_place : u8  = (num_to_conv as i16 % multiplier) as u8;
            self.mem.write8((self.ireg + (2-i as u16)) as usize, dec_place);
            num_to_conv /= 10;
        }
    }

    /* Stores the value of the Timer to be the value in reg X */
    fn store_vx_timer(&mut self, instruction : Instruction, is_delay : bool) {
        if is_delay {
            self.delay_timer.set_timer(self.regs[instruction.nib2 as usize]);
        } else {
            self.sound_timer.set_timer(self.regs[instruction.nib2 as usize]);
        }
    }

    /* Stores the values @ the address inside of ireg to the variable regs upto X*/
    fn store(&mut self, instruction : Instruction) {
        for i in 0..instruction.nib2+1 {
            self.mem.write8((self.ireg + (i as u16)) as usize,
            self.regs[i as usize]);
        }
    }

    /* Load the values from the variable regs into i reg upto X */
    fn load(&mut self, instruction : Instruction) {
        for i in 0..instruction.nib2+1 {
            self.regs[i as usize] = self.mem.read8(
                (self.ireg + (i as u16)) as usize);
        }
    }

    /* Set the value reg X to be the value of the delay timer */
    fn set_vx_delaytimer(&mut self, instruction : Instruction) {
        self.regs[instruction.nib2 as usize] = self.delay_timer.value;
    }

    /* Find the font corresponding to last nibble of VX */
    fn font_char(&mut self, instruction : Instruction) {
        /* Look at the last nibble of VX, and set 
         * ireg to the font corresponding to it */
        let font_num : u8 = (self.regs[instruction.nib2 as usize] << 4) >> 4;
        self.ireg = (_FONT_START as u16) + ((5 * font_num) as u16);
        dbg!(font_num);
    }

    /* Stall until a key is sent through, and when it is store key value in VX */
    fn get_key(&mut self, instruction : Instruction, input_key : u8) {
        if input_key == 0xf0 { self.pc -= 2; return; }
        self.regs[instruction.nib2 as usize] = input_key;
    }

    /* I reg += VX */
    fn add_regi(&mut self, instruction : Instruction) {
        let add_result : (u16, bool) = self.ireg.
            overflowing_add(self.regs[instruction.nib2 as usize] as u16);

        self.ireg = add_result.0;
        /* Overflow past normal addressing */
        if add_result.1 || add_result.0 > 0x0fff { self.regs[0xf] = 0x01; }
    }

}


/*
   https://github.com/parasyte/pixels/blob/864a9c3491cb2aa778a8c0ae5742f760bcfac622/examples/minimal-winit/src/main.rs
   */
impl World {
    // Create a new `World` instance.
    pub fn new() -> Self {
        return Self {
        }
    }


    // Draw the `World` state to the frame buffer.
    // Assumes the default texture format: `wgpu::TextureFormat::Rgba8UnormSrgb`
    pub fn draw(&self, core : &CPU, frame: &mut [u8]) {

        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            let x = (i % WIDTH as usize) as usize;
            let y = (i / WIDTH as usize) as usize;

            let in_screen = x < core.display.screen[0].len() && y < core.display.screen.len();
            let rgba = if in_screen && core.display.screen[y][x] == false {
                [0x00, 0x00, 0x00, 0x00]
            } else {
                [0xff, 0xff, 0xff, 0xff]
            };

            pixel.copy_from_slice(&rgba);
        }
    }
}

impl Timer {
    pub fn new() -> Self {
        return Self {
            start_time : Instant::now(),
            curr_second : 0,
            base_value : 0x00,
            value : 0x00,
        };
    }

    /* Get the amount of elapsed time since we have started this timer */
    pub fn get_elapsed_time(&self) -> u64 {
        return self.start_time.elapsed().as_seconds_f32().floor() as u64;
    }

    /* Do one tick. If we have executed more than 60 ticks in a second, hold off
     * until 1 second passes, then update Timer.curr_sec += 1, and continue 
     * ticking */
    pub fn tick(&mut self)  {
        let elapsed = self.get_elapsed_time();

        /* If >= 1 second has elapsed and 60 ticks haven't went by yet */
        if elapsed >= self.curr_second && self.base_value - self.value <= 60 &&
            self.base_value - self.value > 0 && self.value != 0 {
                self.value -= 1;
            }
        else if elapsed > self.curr_second && self.base_value - self.value > 60 {
            /* if > 1 sec has elapsed and we have been dec. 60x  */
            if self.value != 0 {
                self.base_value = self.value+1;
                self.value = self.base_value;
            }  else { self.base_value = 0; }

            /* Update curr_sec for the next 60 tick interval */
            self.curr_second = elapsed;
        }
    }

    /* Set the timer to TIME_AMOUNT */
    pub fn set_timer(&mut self, time_amount : u8) {
        /* If we set them to be the same value, tick will never dec */
        if time_amount == 0xff {
            self.base_value = time_amount;
            self.value = time_amount - 1;
        } else {

            self.base_value = time_amount+1;
            self.value = time_amount;
        }
    }
}


#[cfg(test)]
mod test;

