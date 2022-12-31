pub struct Display {
    pub screen: [[bool; 64]; 32],
}


impl Display {
    pub fn print_display(&self) {
        for row in self.screen {
            for pixel in row {
                if pixel == false {
                    print!("0 ");
                } else {
                    print!("1 ");
                }
            }
            println!();
        }
    }

}

pub fn make_display()-> Display {
    let s : [[bool; 64]; 32] = [[false; 64]; 32];
    let d = Display {screen: s };
    return d;
}

#[cfg(test)]
mod test;
