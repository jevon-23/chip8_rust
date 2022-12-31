use super::*;

#[test]
fn test_display() {
    let d1: Display = make_display();

    assert_eq!(d1.screen.len(), 32);
    assert_eq!(d1.screen[0].len(), 64);

    for pixel_row in d1.screen {
        for pixel_col in pixel_row {
            assert_eq!(pixel_col, false);
        }
    }
}
