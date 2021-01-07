use iced::{Container, Element, Length, Sandbox, Settings, Text};

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
    MainWindow::run(Settings::default())
}