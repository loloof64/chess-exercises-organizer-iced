use iced::{
    widget::button::{Button, State},
    Column, Container, Element, Length, Sandbox,
};
use iced_native::widget::{svg::Handle, Svg};

use super::chess_board::ChessBoard;

#[derive(Debug, Clone)]
enum Message {
    ToggleBoardOrientation,
    SetPosition(String)
}

struct MainWindow {
    board_position: String,
    board_reversed: bool,
    reverse_board_button_state: State,
}

impl Sandbox for MainWindow {
    type Message = Message;

    fn new() -> Self {
        Self {
            board_position: String::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"),
            board_reversed: false,
            reverse_board_button_state: State::new(),
        }
    }

    fn title(&self) -> String {
        String::from("Chess exercises organizer")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::ToggleBoardOrientation => self.board_reversed = !self.board_reversed,
            Message::SetPosition(fen_string) => self.board_position = fen_string,
        }
    }

    fn view(&mut self) -> Element<Message> {
        let chess_board = ChessBoard::new(45f32, self.board_reversed, self.board_position.clone()).on_position_changed(Box::new(|position| Message::SetPosition(position)));
        let reverse_svg_path = format!(
            "{}/src/graphic/resources/reverseArrows.svg",
            env!("CARGO_MANIFEST_DIR")
        );
        let reverse_svg = Handle::from_path(reverse_svg_path);
        let reverse_board_button = Button::new(
            &mut self.reverse_board_button_state,
            Svg::new(reverse_svg)
                .width(Length::Units(20))
                .height(Length::Units(20)),
        )
        .on_press(Message::ToggleBoardOrientation);
        let content = Column::new()
            .padding(5)
            .spacing(20)
            .push(reverse_board_button)
            .push(chess_board);

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}

pub fn start() -> iced::Result {
    let window_settings = iced::window::Settings {
        size: (800_u32, 500_u32),
        always_on_top: false,
        resizable: true,
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
