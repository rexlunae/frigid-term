use iced::{
    executor, window, Application, Command, Font, Length,
    Subscription, Theme,

    widget::container,
    advanced::graphics::core::Element,
};

#[derive(Debug, Clone)]
pub enum Message {
    IcedTermEvent(iced_term::Event),
}

pub struct App {
    application_name: String,
    title: Option<String>,
    term: iced_term::Term,
}

impl Application for App {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        let system_shell = std::env::var("SHELL")
            .expect("SHELL variable is not defined");
        let term_id = 0;
        let term_settings = iced_term::TermSettings {
            font: iced_term::FontSettings {
                size: 12.0,
                font_type: Font::default(),
                ..Default::default()
            },
            theme: iced_term::ColorPalette::default(),
            backend: iced_term::BackendSettings {
                shell: system_shell.clone(),
            },
        };

        (
            Self {
                application_name: "Frigid Terminal".to_string(),
                title: Some(system_shell),
                term: iced_term::Term::new(term_id, term_settings.clone()),
            },
            Command::none(),
        )
    }

    // If the application name is set, set the title to that, otherwise just use the application name.
    fn title(&self) -> String {
        if let Some(title) = &self.title {
            return format!("{} - {}", self.application_name, title);
        }
        self.application_name.clone()
    }

    fn update(&mut self, message: Self::Message) -> Command<Message> {
        match message {
            Message::IcedTermEvent(iced_term::Event::CommandReceived(
                _,
                cmd,
            )) => match self.term.update(cmd) {
                iced_term::actions::Action::Shutdown => {
                    window::close(window::Id::MAIN)
                },
                iced_term::actions::Action::ChangeTitle(t) => {
                    if t.is_empty() {
                        self.title = None;
                    } else {
                        self.title = Some(t);
                    }
                    Command::none()
                },
                _ => Command::none(),
            },
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        self.term.subscription().map(Message::IcedTermEvent)
    }

    fn view(&self) -> Element<Message, Theme, iced::Renderer> {
        container(iced_term::term_view(&self.term).map(Message::IcedTermEvent))
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}