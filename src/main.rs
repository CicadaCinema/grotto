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
    indicator_value: usize,
    increment_button: button::State,
    decrement_button: button::State,

    glob_text_field_state: text_input::State,
    glob_user_input: String,
    submit_button: button::State,

    image_paths: Vec<PathBuf>,
    image_decisions: Vec<usize>,

    accept_button: button::State,
    reject_button: button::State,
    confirm_button: button::State,
}

#[derive(Debug, Clone)]
enum Message {
    IncrementPressed,
    DecrementPressed,

    GlobUserInputChanged(String),
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
        String::from("Grotto")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::IncrementPressed => {
                if self.image_paths.len() != self.indicator_value + 1 {
                    self.indicator_value += 1;
                }
            }
            Message::DecrementPressed => {
                if self.indicator_value != 0 {
                    self.indicator_value -= 1;
                }
            }
            Message::GlobUserInputChanged(new_string) => {
                self.glob_user_input = new_string;
            }
            Message::SubmitPressed => {
                self.indicator_value = 0;
                self.image_paths.clear();
                self.image_decisions.clear();

                for entry in glob(&*self.glob_user_input).expect("Failed to read glob pattern") {
                    match entry {
                        Ok(path) => {
                            self.image_paths.push(path);
                            self.image_decisions.push(0);
                        }
                        Err(e) => println!("{:?}", e),
                    }
                }

                for element in &self.image_paths {
                    println!("{}", (**element).to_str().unwrap());
                }
            }
            Message::AcceptPressed => {
                self.image_decisions[self.indicator_value] = 1;
            }
            Message::RejectPressed => {
                self.image_decisions[self.indicator_value] = 2;
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
                        Button::new(&mut self.decrement_button, Text::new("<-"))
                            .on_press(Message::DecrementPressed),
                    )
                    .push(Text::new(self.indicator_value.to_string()).size(50))
                    .push(
                        Button::new(&mut self.increment_button, Text::new("->"))
                            .on_press(Message::IncrementPressed),
                    ),
            )
            .push(TextInput::new(
                &mut self.glob_text_field_state,
                "Enter glob pattern",
                &*self.glob_user_input,
                Message::GlobUserInputChanged,
            ))
            .push(
                Button::new(&mut self.submit_button, Text::new("Submit"))
                    .on_press(Message::SubmitPressed),
            )
            .push(if self.image_paths.is_empty() {
                let a = "/images/ferris.png";
                let b = env!("CARGO_MANIFEST_DIR");
                let c = b.to_string() + &*a.to_string();
                Image::new(&*c).height(Length::from(400))
            } else {
                Image::new((*self.image_paths[self.indicator_value]).to_str().unwrap())
                    .height(Length::from(400))
            })
            .push(if self.image_paths.is_empty() {
                Row::new().push(
                    Text::new("█████████")
                        .size(50)
                        .color(Color::new(0.5, 0.5, 0.5, 1.0)),
                )
            } else {
                Row::new()
                    .spacing(50)
                    .push(
                        Button::new(&mut self.reject_button, Text::new("Reject"))
                            .on_press(Message::RejectPressed),
                    )
                    .push(Text::new("█████████").size(50).color(
                        match self.image_decisions[self.indicator_value] {
                            0 => Color::new(0.5, 0.5, 1.0, 1.0),
                            1 => Color::new(0.5, 1.0, 0.5, 1.0),
                            2 => Color::new(1.0, 0.5, 0.5, 1.0),
                            _ => panic!(),
                        },
                    ))
                    .push(
                        Button::new(&mut self.accept_button, Text::new("Accept"))
                            .on_press(Message::AcceptPressed),
                    )
            })
            .into()
    }
}
