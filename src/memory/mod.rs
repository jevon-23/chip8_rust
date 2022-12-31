/* 
 * The struct that holds our memory. All memory is r/w
 * Holds up to 4096 bytes.
 */
pub struct Mem {
    pub data: [u8; 4096],
}

pub const _INTERP_START: usize = 0x00;
pub const _INTERP_END: usize   = 0x1FF;
pub const _PROG_START: usize   = 0x200;
pub const _PROG_END: usize     = 0x1FFF;
pub const _FONT_START: usize   = 0x050;
pub const _FONT_END: usize     = 0x09F;

pub const FONTS: [u8; 80] =  [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80  // F
];


pub fn make_memory() -> Mem {
    /* Create new memory data structure */
    let data:[u8; 4096] = [0x0; 4096];
    let mut new_mem: Mem = Mem{data: data};

    /* Store the fonts into the memory */
    let mut fonts_counter: usize = 0;
    for i in _FONT_START.._FONT_END {
        new_mem.data[i] = FONTS[fonts_counter];
        fonts_counter += 1;
    }
    return new_mem;
}

impl Mem {
    pub fn write8(&mut self, address : usize, data : u8)->bool {
        if (address < 1) || (address > self.data.len() - 1) {
            return false;
        }
        self.data[address] = data;
        return true;
    }
    /* Write 16 bits to memory @ ADDRESS */
    pub fn write16(&mut self, address : usize, data : [u8; 2])->bool {
        if (address < 1) || (address > self.data.len() - 2) {
            return false;
        }
        self.data[address] = data[0];
        self.data[address+1] = data[1];
        return true;
    }

    pub fn read8(&mut self, address : usize)->u8 {
        let mut out : u8 = 0x00;
        if (address < 1) || (address > self.data.len() - 1) {
            return out; 
        }
        out = self.data[address];
        return out;
    }

    /* Read 16 bits to memory @ ADDRESS */
    pub fn read16(&self, address : usize)->[u8; 2] {
        let mut out : [u8; 2] = [0x00, 0x00];
        if (address < 1) || (address > self.data.len() - 2) {
            return out; 
        }

        out[0] = self.data[address];
        out[1] = self.data[address+1];
        return out;
    }

    /* Write a game file to memory */
    pub fn store_game(&mut self, contents : Vec<u8>) {
        for i in 0..contents.len() {
            self.data[_PROG_START + i] = contents[i];
        }
    }
}

#[cfg(test)]
mod test;
