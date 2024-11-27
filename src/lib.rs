pub use crossterm::{
    event::{
        read, poll,
        DisableMouseCapture, EnableMouseCapture, 
        Event, 
        MouseButton, MouseEvent, MouseEventKind
    },
    terminal::{
        disable_raw_mode, enable_raw_mode,
        EnterAlternateScreen, LeaveAlternateScreen,
        Clear, ClearType::Purge,
        size
    },
    cursor::{
        Hide,
        Show
    },
    ExecutableCommand
};
pub use std::{
    time::Duration,
    io::{Write, stdout},
    thread
};


/*
*  ____ _____ ____  _   _  ____ _____ ____  
* / ___|_   _|  _ \| | | |/ ___|_   _/ ___| 
* \___ \ | | | |_) | | | | |     | | \___ \ 
*  ___) || | |  _ <| |_| | |___  | |  ___) |
* |____/ |_| |_| \_\\___/ \____| |_| |____/ 
*/

pub struct BoolBox {
    pub val: bool,
    row: u32,
    col: u32
}

pub struct RadioBtn {
    pub txt: Vec<&'static str>,
    row: u32,
    col: u32,
    pub idx: usize
}

pub struct Label { }

pub struct Button {
    func: fn(),
    row: u32,
    col: u32,
    label: &'static str,
    isdown: bool,
}

/*
*  ___ __  __ ____  _     ____  
* |_ _|  \/  |  _ \| |   / ___| 
*  | || |\/| | |_) | |   \___ \ 
*  | || |  | |  __/| |___ ___) |
* |___|_|  |_|_|   |_____|____/ 
*/

impl BoolBox {
    pub fn new(col: u32, row: u32, label: &str) -> BoolBox { 
        let tsize = size().unwrap();
        if col >= tsize.0 as u32 || row >= tsize.1 as u32 || col <= 0 || row <= 0 {
            panic!("Invalid Position Given on Booleon Toggle");
        }
        if label == "" {
            print!("\x1b[s\x1b[{};{}H\x1b[38;2;255;255;255m\x1b[48;2;61;61;61m\x1b[0;38;2;61;61;61m█\x1b[0m\x1b[u",row,col); // Print the garbage
        }
        else {
            print!("\x1b[s\x1b[{};{}H\x1b[38;2;255;255;255m\x1b[48;2;61;61;61m\x1b[0;38;2;61;61;61m█\x1b[0;38;2;255;255;255m {}\x1b[0m\x1b[u", row, col, label); // Print the garbage
        }
        stdout().flush().unwrap();
        BoolBox {
            val: false,
            row,
            col
        }
    }
    pub fn update(&mut self, read: &Event) {
        if let Event::Mouse(MouseEvent { kind: MouseEventKind::Down(MouseButton::Left), row: mut y, column: mut x, modifiers: _, }) = read {
            x += 1;
            y += 1;
            if y == self.row as u16 && (x >= self.col as u16 && x <= self.col as u16 + 3) {
                self.val = !self.val;
                match self.val {
                    true  => {
                        print!("\x1b[s\x1b[{};{}H\x1b[38;2;150;200;255m█\x1b[38;2;255;255;255;48;2;150;200;255m\x1b[49m\x1b[0m\x1b[u",self.row, self.col);
                    }
                    false => {
                        print!("\x1b[s\x1b[{};{}H\x1b[38;2;255;255;255m\x1b[48;2;61;61;61m\x1b[0;38;2;61;61;61m█\x1b[0m\x1b[u",self.row, self.col);
                    }
                }
                stdout().flush().unwrap();
            }
        } 
    }
}

// Radio button shit
impl RadioBtn {
    // constructor
    pub fn new(col: u32, row: u32, txt: Vec<&'static str>) -> RadioBtn {
        // Check if the text is nothing
        if txt.len() < 1 {
            panic!("Cannot have Selection Field with length: {}", txt.len());
        }
        // Loop through the text
        for i in 0..txt.len() {
            // hightlight the first
            if i == 0 {
                // Cryptic ansi escape codes
                print!("\x1b[{};{}H\x1b[38;2;255;255;255m {}\x1b[0m", row as usize + i * 2, col, txt[i]);
            }
            // Don't hightlight the others
            else {
                // More cryptic ansi
                print!("\x1b[{};{}H\x1b[38;2;61;61;61m \x1b[38;2;255;255;255m{}\x1b[0m", row as usize + i * 2, col, txt[i]);
            }
            
        }
        // FLUSH THE STANDART OUTPUT WPOADPOLWIOWOWOOWOOOOOOOOO
        stdout().flush().unwrap();
        // Return the shittttt
        RadioBtn {
            txt,
            row,
            col,
            idx: 0
        }
    }
    pub fn update(&mut self, read: &Event) {
        // Mouse event D:
        if let Event::Mouse(MouseEvent { kind: MouseEventKind::Down(MouseButton::Left), row: mut y, column: mut x, modifiers: _}) = read {
            x += 1;
            y += 1;
           // uhhhhhh... 
            if (x == self.col as u16 || x == self.col as u16 + 1) && (y >= self.row as u16 && y < self.row as u16 + self.txt.len() as u16*2 && (y - self.row as u16) % 2 == 0) {
                print!("\x1b[{};{}H\x1b[38;2;61;61;61m\x1b[38;2;255;255;255m {}", self.idx * 2 + self.row as usize, self.col, self.txt[self.idx]);
                self.idx = ((y - self.row as u16) / 2) as usize;
                print!("\x1b[{};{}H\x1b[38;2;255;255;255m {}", self.idx * 2 + self.row as usize, self.col, self.txt[self.idx]);
                stdout().flush().unwrap();
            }
        }
    }
}

impl Button {
    pub fn new(col: u32, row: u32, func: fn(), label: &'static str) -> Button {
        print!("\x1b[{row};{col}H\x1b[38;2;255;255;255m█\x1b[1;48;2;255;255;255;38;2;0;0;0m{label}\x1b[0;38;2;255;255;255m█\x1b[0m");
        stdout().flush().unwrap();
        Button {
            func,
            col,
            row,
            label,
            isdown: false,
        }
    }
    pub fn update(&mut self, read: &Event) {
        if let Event::Mouse(MouseEvent { kind: MouseEventKind::Moved, row: mut y, column: mut x, modifiers: _, }) = read {
            x += 1;
            y += 1;
            if y == self.row as u16 && (x >= self.col as u16 && x <= self.col as u16 + 9) && !self.isdown {
                self.isdown = true;
                print!("\x1b[{};{}H\x1b[38;2;200;200;200m█\x1b[1;48;2;200;200;200;38;2;0;0;0m{}\x1b[0;38;2;200;200;200m█\x1b[0m", self.row, self.col, self.label);
                stdout().flush().unwrap();
            }
            else if self.isdown {
                self.isdown = false;
                print!("\x1b[{};{}H\x1b[38;2;255;255;255m█\x1b[1;48;2;255;255;255;38;2;0;0;0m{}\x1b[0;38;2;255;255;255m█\x1b[0m", self.row, self.col, self.label);
                stdout().flush().unwrap();
            }
        }
        if let Event::Mouse(MouseEvent { kind: MouseEventKind::Down(MouseButton::Left), row: mut y, column: mut x, modifiers: _, }) = read {
            x += 1;
            y += 1;
            if y == self.row as u16 && (x >= self.col as u16 && x <= self.col as u16 + 9) {
                let f = self.func;
                f(); 
            }
        } 
    }
}

impl Label {
    pub fn new(col: u32, row: u32, txt: &'static str) {
        print!("\x1b[{row};{col}H\x1b[38;2;255;255;255m{txt}\x1b[0m");   
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    fn test() {
        thread::spawn(|| {
            print!("\x1b[10;33H\x1b[38;2;255;255;255;1mclick!\x1b[0m");
            stdout().flush().unwrap();
            thread::sleep(Duration::from_millis(300));
            print!("\x1b[10;33H      ");
            stdout().flush().unwrap();
        });
    }
    #[test]
    fn main() {
        let mut stdout = stdout();
        enable_raw_mode().unwrap();
        stdout.execute(EnableMouseCapture).unwrap();
        stdout.execute(EnterAlternateScreen).unwrap();
        stdout.execute(Clear(Purge)).unwrap();
        stdout.execute(Hide).unwrap();
        let _blabel = Label::new(1, 3, "Booleans");
        let _rlabel = Label::new(14, 3, "Radio Buttons");
        let _btlabel = Label::new(33, 3, "Button");
        let _ddlabel = Label::new(47, 3, "Dropdown");

        let mut bboxes: [&mut BoolBox; 4] = [
            &mut BoolBox::new(3, 5,  ""),
            &mut BoolBox::new(3, 7,  ""),
            &mut BoolBox::new(3, 9,  ""),
            &mut BoolBox::new(3, 11, "")
        ];
        let mut radio_btn = RadioBtn::new(16, 5, vec![
            "1",
            "2",
            "3",
            "4",
            "5",
            "6",    
        ]);
        let mut btn = Button::new(31, 5, test, "Button");


        loop {
            if poll(Duration::from_millis(1)).unwrap() {
                let read = read().unwrap();
                for bbox in &mut bboxes {
                    bbox.update(&read); 
                }
                radio_btn.update(&read);
                btn.update(&read);
                if let Event::Key(_) = read {
                    break;
                }
            }
        }

        stdout.execute(DisableMouseCapture).unwrap();
        stdout.execute(LeaveAlternateScreen).unwrap();
        disable_raw_mode().unwrap();
        stdout.execute(Show).unwrap();
    }
}
