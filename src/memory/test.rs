use super::*;

#[test]
fn test_make_mem() {
    let m1: Mem = make_memory();
    assert_eq!(m1.data.len(), 4096);

    // Ensure that the font was inserted in 
    let mut font_counter: usize = 0;
    for byte in 0..m1.data.len() {
        if byte >= _FONT_START && byte < _FONT_END {
            assert_eq!(m1.data[byte], FONTS[font_counter]);
            font_counter += 1;
        } else {
            assert_eq!(m1.data[byte], 0x0);
        }
    }
}


#[test]
fn test_mem_wr16() {
    let mut m1: Mem = make_memory();
    assert_eq!(m1.data.len(), 4096);

    let b1 : u8 = 0xbe;
    let b2 : u8 = 0xef;
    assert_eq!(m1.write16(0x100, [b1, b2]), true, "Testing write");
    assert_eq!(m1.read16(0x100), [b1, b2], "Testing read");

}

#[test]
fn test_mem_wr8() {
    let mut m1: Mem = make_memory();
    assert_eq!(m1.data.len(), 4096);

    let b1 : u8 = 0xbe;
    let b2 : u8 = 0xef;
    assert_eq!(m1.write8(0x100, b1), true, "Testing write");
    assert_eq!(m1.write8(0x101, b2), true, "Testing write");
    assert_eq!(m1.read16(0x100), [b1, b2], "Testing read");
    assert_eq!(m1.read8(0x100), b1, "Testing read");
    assert_eq!(m1.read8(0x101), b2, "Testing read");

}


