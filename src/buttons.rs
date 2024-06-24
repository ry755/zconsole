// buttons.rs

pub struct Buttons {
    up: bool,
    down: bool,
    left: bool,
    right: bool,
    select: bool,
    back: bool,
}

impl std::convert::From<&Buttons> for u8  {
    fn from(buttons: &Buttons) -> u8 {
        (if buttons.up     { 1 } else { 0 }) << 5 |
        (if buttons.down   { 1 } else { 0 }) << 4 |
        (if buttons.left   { 1 } else { 0 }) << 3 |
        (if buttons.right  { 1 } else { 0 }) << 2 |
        (if buttons.select { 1 } else { 0 }) << 1 |
        (if buttons.back   { 1 } else { 0 })
    }
}

#[derive(Debug)]
pub enum Button {
    Up,
    Down,
    Left,
    Right,
    Select,
    Back,
}

impl Buttons {
    pub fn new() -> Self {
        Buttons {
            up: false,
            down: false,
            left: false,
            right: false,
            select: false,
            back: false,
        }
    }

    pub fn press(&mut self, button: Button) {
        match button {
            Button::Up => self.up = true,
            Button::Down => self.down = true,
            Button::Left => self.left = true,
            Button::Right => self.right = true,
            Button::Select => self.select = true,
            Button::Back => self.back = true,
        }
    }

    pub fn release(&mut self, button: Button) {
        match button {
            Button::Up => self.up = false,
            Button::Down => self.down = false,
            Button::Left => self.left = false,
            Button::Right => self.right = false,
            Button::Select => self.select = false,
            Button::Back => self.back = false,
        }
    }
}
