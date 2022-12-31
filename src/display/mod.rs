/* Really a useless struct. If i were to do this again,
 * I would simply put a [[bool;64];32] into the cpu
 * directly, and not use any extra class. It was nice
 * to use while I didn't have any graphics up though
 * 
 * It will stay despite its uselessness though.
 */
pub struct Display {
    pub screen: [[bool; 64]; 32],
}

// impl Display {
//     pub fn print_display(&self) {
//         for row in self.screen {
//             for pixel in row {
//                 if pixel == false {
//                     print!("0 ");
//                 } else {
//                     print!("1 ");
//                 }
//             }
//             println!();
//         }
//     }
// 
// }

pub fn make_display()-> Display {
    let s : [[bool; 64]; 32] = [[false; 64]; 32];
    let d = Display {screen: s };
    return d;
}

#[cfg(test)]
mod test;
