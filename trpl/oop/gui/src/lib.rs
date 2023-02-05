pub trait Draw {
    fn draw(&self);
}

// pub struct Screen<T: Draw> {
//     pub compnents: Vec<T>,
// }

// impl<T> Screen<T>
// where
//     T: Draw,
// {
//     pub fn run(&self) {
//         for component in self.compnents.iter() {
//             component.draw();
//         }
//     }
// }

pub struct Screen {
    pub compnents: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.compnents.iter() {
            component.draw();
        }
    }
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {}
}
