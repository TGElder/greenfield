use commons::color::Rgba;
use commons::geometry::XYRectangle;
use commons::grid::Grid;
use engine::graphics::Graphics;

const CLEAR_COLOR: Rgba<u8> = Rgba::new(0, 0, 0, 0);

pub struct SelectionArtist {
    pub drawn_selection: Option<XYRectangle<u32>>,
    pub selection_color: Rgba<u8>,
    pub texture: Option<usize>,
}

impl SelectionArtist {
    pub fn init(&mut self, terrain: &Grid<f32>, graphics: &mut dyn Graphics) {
        let clear = Grid::from_element(
            terrain.width() - 1,
            terrain.height() - 1,
            Rgba::new(0, 0, 0, 0),
        );
        self.texture = Some(graphics.load_texture(&clear).unwrap());
    }

    pub fn run(&mut self, graphics: &mut dyn Graphics, selection: &Option<XYRectangle<u32>>) {
        let Some(texture) = &self.texture else {return};

        if self.drawn_selection == *selection {
            return;
        }

        self.draw_selection(texture, graphics, CLEAR_COLOR);
        self.drawn_selection = *selection;
        self.draw_selection(texture, graphics, self.selection_color);
    }

    fn draw_selection(&self, texture: &usize, graphics: &mut dyn Graphics, color: Rgba<u8>) {
        if let Some(selection) = self.drawn_selection {
            graphics
                .modify_texture(
                    texture,
                    &selection.from,
                    &Grid::from_element(selection.width(), selection.height(), color),
                )
                .unwrap_or_else(|_| println!("WARN: Could not draw selection"));
        }
    }
}
