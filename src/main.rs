extern crate rustd;

use rustd::{Screen, Draw, Button};

fn main() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                name: String::from("can_continue"),
                width: 250,
                height: 30,
                options: vec![
                    String::from("Yes"),
                    String::from("No"),
                    String::from("Maybe"),
                ],
            }),
            Box::new(Button {
                width: 100,
                height: 30,
                label: String::from("Done"),
            }),
        ],
    };
    screen.run();
}

struct SelectBox {
    name: String,
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!(
            "Name = {}, Width = {}, Height = {}, Options = {:?}",
            self.name,
            self.width,
            self.height,
            self.options
        );
    }
}
