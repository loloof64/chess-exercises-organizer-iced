use iced_graphics::{Backend, Defaults, Primitive, Renderer};
use iced_native::{
    layout, mouse, Background, Color, Hasher, Layout, Length, Point, Rectangle, Size, Widget, Element,
};
use pleco::Board;

pub struct ChessBoard {
    board: Board,
    cells_size: f32,
}

#[derive(Debug, Clone, Copy)]
pub enum Message {}

impl ChessBoard {
    pub fn new(cells_size: f32) -> Self {
        Self {
            cells_size,
            board: Board::start_pos(),
        }
    }
}

impl<Message, B> Widget<Message, Renderer<B>> for ChessBoard
where
    B: Backend,
{
    fn width(&self) -> Length {
        Length::Shrink
    }

    fn height(&self) -> Length {
        Length::Shrink
    }

    fn layout(&self, _renderer: &Renderer<B>, _limits: &layout::Limits) -> layout::Node {
        let total_size = self.cells_size * 9.0;
        layout::Node::new(Size::new(total_size, total_size))
    }

    fn hash_layout(&self, state: &mut Hasher) {
        use std::hash::Hash;

        self.cells_size.to_bits().hash(state);
    }

    fn draw(
        &self,
        _renderer: &mut Renderer<B>,
        _defaults: &Defaults,
        layout: Layout<'_>,
        _cursor_position: Point,
        _viewport: &Rectangle,
    ) -> (Primitive, mouse::Interaction) {
        (
            Primitive::Quad {
                bounds: layout.bounds(),
                background: Background::Color(Color::from_rgb8(214, 59, 96)),
                border_radius: 0.0,
                border_width: 0.0,
                border_color: Color::TRANSPARENT,
            },
            mouse::Interaction::default(),
        )
    }
}

impl<'a, Message, B> Into<Element<'a, Message, Renderer<B>>> for ChessBoard
where
    B: Backend,
{
    fn into(self) -> Element<'a, Message, Renderer<B>> {
        Element::new(self)
    }}
