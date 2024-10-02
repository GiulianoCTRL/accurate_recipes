use iced::widget::{button, column, container, row, text, Column, Container, Row};
use iced::{Center, Fill};

pub fn main() -> iced::Result {
    iced::run("A cool counter", Counter::update, Counter::view)
}

#[derive(Default)]
struct Counter {
    value: i64,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Increment,
    Decrement,
}

impl Counter {
    fn update(&mut self, message: Message) {
        match message {
            Message::Increment => {
                self.value += 1;
            }
            Message::Decrement => {
                self.value -= 1;
            }
        }
    }

    fn view(&self) -> Container<Message> {
        container(row![
            column![
                button("Increment").on_press(Message::Increment),
                text(self.value).size(50),
                button("Decrement").on_press(Message::Decrement),
            ]
            .padding(20)
            .align_x(Center),
            row!["Left", "Right"].padding(20).align_y(Center),
        ])
        .padding(10)
        .center_x(Fill)
        .center_y(Fill)
        .into()
    }
}
