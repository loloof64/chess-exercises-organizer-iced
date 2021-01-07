use iced::{Container, Element, Length, Sandbox, Text};

struct MainWindow {}

#[derive(Debug, Clone, Copy)]
enum Message {}

impl Sandbox for MainWindow {
    type Message = Message;

    fn new() -> Self {
        Self {}
    }

    fn title(&self) -> String {
        String::from("Chess exercises organizer")
    }

    fn update(&mut self, message: Message) {
        match message {}
    }

    fn view(&mut self) -> Element<Message> {
        let content = Text::new("Hello World !");

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}

pub fn start()  -> iced::Result {
    let window_settings = iced::window::Settings {
        size: (800_u32, 400_u32),
        always_on_top: false,
        resizable: false,
        decorations: true,
        transparent: false,
        min_size: None,
        max_size: None,
        icon: None,
    };
    MainWindow::run(iced::Settings {
        flags: (),
        window: window_settings,
        default_font: None,
        antialiasing: false,
        default_text_size: 12u16,
    })
}