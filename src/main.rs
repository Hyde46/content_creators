use iced::{
    button, executor, text_input, time, Align, Application, Button, Column, Command, Container,
    Element, HorizontalAlignment, Length, Row, Settings, Subscription, Text, TextInput,
};

pub fn main() {
    ContentCreatoRS::run(Settings::default())
}

struct ContentCreatoRS {
    state: State,
    subreddit_input: text_input::State,
    subreddit_input_value: String,
    download_button: button::State,
}

enum State {
    Idle,
}

#[derive(Debug, Clone)]
enum Message {
    InputChanged(String),
}

impl Application for ContentCreatoRS {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (ContentCreatoRS, Command<Message>) {
        (
            ContentCreatoRS {
                state: State::Idle,
                subreddit_input: text_input::State::focused(),
                subreddit_input_value: String::from(""),
                download_button: button::State::new(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("ContentCreatoRS")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::InputChanged(value) => match self.state {
                State::Idle => {}
                _ => {}
            },
        }

        Command::none()
    }

    fn subscription(&self) -> Subscription<Message> {
        match self.state {
            State::Idle => Subscription::none(),
        }
    }

    fn view(&mut self) -> Element<Message> {
        let title = Text::new("ContentCreatoRS")
            .width(Length::Fill)
            .size(25)
            .color([0.5, 0.5, 0.5])
            .horizontal_alignment(HorizontalAlignment::Center);

        let input = TextInput::new(
            &mut self.subreddit_input,
            "Which subreddit?",
            &mut self.subreddit_input_value,
            Message::InputChanged,
        )
        .padding(15)
        .size(20);

        let download = Button::new(&mut self.download_button, Text::new("Download").size(15))
            .width(Length::Units(86))
            .padding(10);
        // .style(style::Button::Icon);

        let content = Column::new()
            .align_items(Align::Center)
            .max_width(540)
            .spacing(20)
            .push(title)
            .push(input)
            .push(download);

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}

mod style {
    use iced::{button, Background, Color, Vector};

    pub enum Button {
        Filter { selected: bool },
        Icon,
        Destructive,
    }

    impl button::StyleSheet for Button {
        fn active(&self) -> button::Style {
            match self {
                Button::Filter { selected } => {
                    if *selected {
                        button::Style {
                            background: Some(Background::Color(Color::from_rgb(0.2, 0.2, 0.7))),
                            border_radius: 10,
                            text_color: Color::WHITE,
                            ..button::Style::default()
                        }
                    } else {
                        button::Style::default()
                    }
                }
                Button::Icon => button::Style {
                    text_color: Color::from_rgb(0.5, 0.5, 0.5),
                    ..button::Style::default()
                },
                Button::Destructive => button::Style {
                    background: Some(Background::Color(Color::from_rgb(0.8, 0.2, 0.2))),
                    border_radius: 5,
                    text_color: Color::WHITE,
                    shadow_offset: Vector::new(1.0, 1.0),
                    ..button::Style::default()
                },
            }
        }

        fn hovered(&self) -> button::Style {
            let active = self.active();

            button::Style {
                text_color: match self {
                    Button::Icon => Color::from_rgb(0.2, 0.2, 0.7),
                    Button::Filter { selected } if !selected => Color::from_rgb(0.2, 0.2, 0.7),
                    _ => active.text_color,
                },
                shadow_offset: active.shadow_offset + Vector::new(0.0, 1.0),
                ..active
            }
        }
    }
}
