use glob::glob;
use iced::{
    button, image, text_input, Align, Button, Color, Column, Element, Image, Length, Rectangle,
    Row, Sandbox, Settings, Text, TextInput,
};
use std::path::PathBuf;
use std::ptr::null;

pub fn main() -> iced::Result {
    Counter::run(Settings::default())
}

#[derive(Default)]
struct Counter {
    value: usize,
    textvalue: String,
    imagepaths: Vec<PathBuf>,
    imagedecision: Vec<usize>,
    increment_button: button::State,
    decrement_button: button::State,
    accept_button: button::State,
    reject_button: button::State,
    confirm_button: button::State,
    submit_button: button::State,
    text_field: text_input::State,
}

#[derive(Debug, Clone)]
enum Message {
    IncrementPressed,
    DecrementPressed,
    InputChanged(String),
    SubmitPressed,
    AcceptPressed,
    RejectPressed,
    ConfirmPressed,
}

impl Sandbox for Counter {
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("Title")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::IncrementPressed => {
                if self.imagepaths.len() != self.value + 1 {
                    self.value += 1;
                }
            }
            Message::DecrementPressed => {
                if self.value != 0 {
                    self.value -= 1;
                }
            }
            Message::InputChanged(new_string) => {
                self.textvalue = new_string;
            }
            Message::SubmitPressed => {
                self.value = 0;

                for entry in glob(&*self.textvalue).expect("Failed to read glob pattern") {
                    match entry {
                        Ok(path) => {
                            self.imagepaths.push(path);
                            self.imagedecision.push(0);
                        }
                        Err(e) => println!("{:?}", e),
                    }
                }

                for element in &self.imagepaths {
                    println!("{}", (**element).to_str().unwrap());
                }
            }
            Message::AcceptPressed => {
                self.imagedecision[self.value] = 1;
            }
            Message::RejectPressed => {
                self.imagedecision[self.value] = 2;
            }
            Message::ConfirmPressed => {
                println!("confirm!");
            }
        }
    }

    fn view(&mut self) -> Element<Message> {
        Column::new()
            .padding(20)
            .align_items(Align::Center)
            .push(
                Row::new()
                    .spacing(50)
                    .push(
                        Button::new(&mut self.increment_button, Text::new("Increment"))
                            .on_press(Message::IncrementPressed),
                    )
                    .push(Text::new(self.value.to_string()).size(50))
                    .push(
                        Button::new(&mut self.decrement_button, Text::new("Decrement"))
                            .on_press(Message::DecrementPressed),
                    ),
            )
            .push(Text::new("some text").size(50))
            .push(TextInput::new(
                &mut self.text_field,
                "placeholder",
                &*self.textvalue,
                Message::InputChanged,
            ))
            .push(
                Button::new(&mut self.submit_button, Text::new("Submit"))
                    .on_press(Message::SubmitPressed),
            )
            .push(if self.imagepaths.is_empty() {
                let a = "/images/ferris.png";
                let b = env!("CARGO_MANIFEST_DIR");
                let c = b.to_string() + &*a.to_string();
                Image::new(&*c)
            } else {
                Image::new((*self.imagepaths[self.value]).to_str().unwrap())
            })
            .push(if self.imagepaths.is_empty() {
                Row::new().push(
                    Text::new("###########")
                        .size(50)
                        .color(Color::new(0.5, 0.5, 0.5, 1.0)),
                )
            } else {
                Row::new()
                    .spacing(50)
                    .push(
                        Button::new(&mut self.accept_button, Text::new("Accept"))
                            .on_press(Message::AcceptPressed),
                    )
                    .push(Text::new("###########").size(50).color(
                        match self.imagedecision[self.value] {
                            0 => Color::new(0.5, 0.5, 1.0, 1.0),
                            1 => Color::new(0.5, 1.0, 0.5, 1.0),
                            2 => Color::new(1.0, 0.5, 0.5, 1.0),
                            _ => panic!(),
                        },
                    ))
                    .push(
                        Button::new(&mut self.reject_button, Text::new("Reject"))
                            .on_press(Message::RejectPressed),
                    )
            })
            .into()
    }
}
