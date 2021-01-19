use pleco::Board;

use iced::{
    canvas::{Cache, Cursor, Geometry, Path, Program},
    Canvas, Color, Element, Length, Point, Rectangle,
};

#[derive(Debug, Clone, Copy)]
enum Message {}

#[derive(Debug)]
struct GraphicElements {
    background: Cache,
}

#[derive(Debug)]
pub struct ChessBoard {
    position: Board,
    graphic_elements: GraphicElements,
    cells_size: u16,
}

impl ChessBoard {
    pub fn new(cells_size: u16) -> Self {
        Self {
            cells_size,
            position: Board::start_pos(),
            graphic_elements: GraphicElements {
                background: Cache::new(),
            },
        }
    }

    pub fn view(&mut self) -> Element<Message> {
        let total_size = self.cells_size * 9;
        let total_size = Length::Units(total_size);
        Canvas::new(&mut self)
            .width(total_size)
            .height(total_size)
            .into()
    }
}

impl<Message> Program<Message> for ChessBoard {
    fn draw(&self, bounds: Rectangle, _cursor: Cursor) -> Vec<Geometry> {
        let background = self
            .graphic_elements
            .background
            .draw(bounds.size(), |frame| {
                let zone = Path::rectangle(Point::new(0.0, 0.0), frame.size());
                let color = Color::from_rgb8(16, 68, 202);

                frame.fill(&zone, color);
            });

        vec![background]
    }
}