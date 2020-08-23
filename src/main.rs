use iced::{
    button, executor, text_input, Align, Application, Button, Column, Command, Container, Element,
    HorizontalAlignment, Length, Row, Settings, Subscription, Text, TextInput,
};
extern crate rsreddit;
use futures::executor::block_on;
use rsreddit::client::Reddit;
use rsreddit::model::token::OAuthToken;
use rsreddit::oauth2::{AuthorizationTime, RedditApiScope, RedditOAuth};
use rsreddit::util::convert_scope_vec_to_string;

pub fn main() {
    /*let mut reddit_oauth = RedditOAuth::default().build();
    let mut scopes = Vec::new();
    scopes.push(RedditApiScope::identity);
    scopes.push(RedditApiScope::read);
    let scope_string = convert_scope_vec_to_string(&scopes);
    let bearer_token = reddit_oauth
        .authorize_client(&scope_string, Some(AuthorizationTime::permanent))
        .unwrap();
    println!("{:?}", bearer_token);*/
    ContentCreatoRS::run(Settings::default())
}

struct RedditRs {
    reddit_client: Reddit,
}

struct ContentCreatoRS {
    state: State,
    subreddit_input: text_input::State,
    subreddit_input_value: String,
    post_amount_input: text_input::State,
    post_amount_input_value: String,
    download_button: button::State,
    target_directory_value: String,
    target_directory_input: text_input::State,
    redditrs: Reddit,
}

enum State {
    Idle,
    Downloading,
}

#[derive(Debug, Clone)]
enum Message {
    InputChanged(String),
    DirectoryInputChanged(String),
    Download,
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
                post_amount_input: text_input::State::new(),
                post_amount_input_value: String::from(""),
                download_button: button::State::new(),
                target_directory_value: String::from(""),
                target_directory_input: text_input::State::new(),
                redditrs: Reddit::default()
                    .bearer_token(OAuthToken {
                        access_token: String::from("638802175-GgIMloeryPiX0vLj7PTyz0TVxS4"),
                        token_type: String::from("bearer"),
                        expires_in: 3600,
                        scope: String::from("identity read"),
                        refresh_token: String::from("638802175-H0tRwLO2KaH93h2x8B3MrYljDMA"),
                    })
                    .build(),
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
                State::Idle => {
                    self.subreddit_input_value = value;
                }
                _ => {}
            },
            Message::DirectoryInputChanged(value) => match self.state {
                State::Idle => {
                    self.target_directory_value = value;
                }
                _ => {}
            },
            Message::Download => match self.state {
                State::Idle => {
                    println!("Downloading highlights..");
                    let answer = self
                        .redditrs
                        .hot(
                            Some(&format!("/r/{}", self.subreddit_input_value)),
                            "",
                            "",
                            0,
                            1,
                            false,
                            false,
                        )
                        .unwrap();
                    println!("{:?}", answer);
                    //self.state = State::Downloading
                }
                _ => {}
            },
        }

        Command::none()
    }

    fn subscription(&self) -> Subscription<Message> {
        match self.state {
            State::Idle => Subscription::none(),
            State::Downloading => Subscription::none(),
        }
    }

    fn view(&mut self) -> Element<Message> {
        let title = Text::new("ContentCreatoRS")
            .width(Length::Fill)
            .size(25)
            .color([0.5, 0.5, 0.5])
            .horizontal_alignment(HorizontalAlignment::Center);

        let input_prefix = Text::new("/r/")
            .width(Length::Units(20))
            .size(25)
            .color([0.7, 0.7, 0.7])
            .horizontal_alignment(HorizontalAlignment::Center);
        let input = TextInput::new(
            &mut self.subreddit_input,
            "<subreddit>",
            &mut self.subreddit_input_value,
            Message::InputChanged,
        )
        .padding(15)
        .size(20);

        let post_amount = Text::new("Amount:")
            .width(Length::Units(65))
            .size(15)
            .color([0.7, 0.7, 0.7])
            .horizontal_alignment(HorizontalAlignment::Center);
        let post_amount_field = TextInput::new(
                &mut self.post_amount_input,
                "<amount>",
                &mut self.post_amount_input_value,
                Message::InputChanged,
            )
            .padding(10)
            .width(Length::Units(85))
            .size(15);

        let diretory_input = TextInput::new(
            &mut self.target_directory_input,
            "C:\\Users\\Monarch\\Downloads",
            &mut self.target_directory_value,
            Message::DirectoryInputChanged,
        )
        .padding(15)
        .size(20);

        let download = Button::new(&mut self.download_button, Text::new("Download").size(15))
            .width(Length::Units(86))
            .padding(10)
            .on_press(Message::Download);
        // .style(style::Button::Icon);

        let input_content = Row::new()
            .align_items(Align::Center)
            .max_width(540)
            .spacing(2)
            .push(input_prefix)
            .push(input);

        let amount_content = Row::new()
            .align_items(Align::Start)
            .max_width(540)
            .spacing(2)
            .push(post_amount)
            .push(post_amount_field);

        let content = Column::new()
            .align_items(Align::Center)
            .max_width(540)
            .spacing(20)
            .push(title)
            .push(input_content)
            .push(amount_content)
            .push(diretory_input)
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
