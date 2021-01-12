use iced_graphics::{Backend, Defaults, Primitive, Renderer};
use iced_native::{
    layout, mouse, widget::svg::Handle, Background, Color, Element, Hasher, Layout, Length, Point,
    Rectangle, Size, Widget, Vector
};
use pleco::Board;

use std::collections::HashMap;
use std::fs;    

pub struct ChessBoard {
    board: Board,
    cells_size: f32,
    piece_assets: HashMap<String, Handle>
}

#[derive(Debug, Clone, Copy)]
pub enum Message {}

impl ChessBoard {
    pub fn new(cells_size: f32) -> Self {
        let piece_assets = ChessBoard::load_assets();
        Self {
            cells_size,
            piece_assets,
            board: Board::start_pos(),
        }
    }

    fn load_assets() -> HashMap<String, Handle> {
        let assets_dir = format!("{}/src/graphic/merida", env!("CARGO_MANIFEST_DIR"));
        let files = fs::read_dir(assets_dir.clone())
            .unwrap_or_else(|e| panic!("Couldn't read directory {}: {}", assets_dir, e))
            .map(|f| f.unwrap())
            .filter(|f| f.metadata().unwrap().is_file());
        let mut res: HashMap<String, Handle> = HashMap::new();
    
        for file in files {
            let svg = Handle::from_path(file.path());
            res.insert(
                file.path()
                    .file_stem()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_string(),
                svg,
            );
        }
    
        res
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
        let mut res: Vec<Primitive> = Vec::new();
        res.push(
            Primitive::Quad {
                bounds: layout.bounds(),
                background: Background::Color(Color::from_rgb8(214, 59, 96)),
                border_radius: 0.0,
                border_width: 0.0,
                border_color: Color::TRANSPARENT,
            }
        );

        for row in 0..8 {
            for col in 0..8 {
                let is_white_cell = (row+col) %2 == 0;
                let background = Background::Color(if is_white_cell {Color::from_rgb8(255, 206, 158)} else {Color::from_rgb8(209, 139, 71)});

                let x = self.cells_size * ((col as f32) + 0.5);
                let y = self.cells_size * ((row as f32) + 0.5);
                let position = layout.bounds().position() + Vector::new(x, y);
                let size = Size::new(self.cells_size, self.cells_size);
                let bounds = Rectangle::new(position, size);

                res.push(
                    Primitive::Quad {
                        bounds,
                        background,
                        border_radius: 0.0,
                        border_width: 0.0,
                        border_color: Color::TRANSPARENT,
                    }
                )
            }
        }

        (
            Primitive::Group { primitives: res },
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
    }
}
