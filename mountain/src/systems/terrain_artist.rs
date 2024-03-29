use std::collections::{HashMap, HashSet};

use commons::color::Rgba;
use commons::geometry::{xy, XYRectangle, XY};
use commons::grid::Grid;
use commons::origin_grid::OriginGrid;
use engine::graphics::Graphics;

use crate::draw::terrain::Drawing;
use crate::handlers::selection;
use crate::model::ability::Ability;
use crate::utils::ability::cell_ability;

pub const CLEAR: Rgba<u8> = Rgba::new(0, 0, 0, 0);

pub struct System {
    drawing: Option<Drawing>,
    updates: Vec<XYRectangle<u32>>,
    show_pistes: bool,
    colors: Colors,
}

pub struct Colors {
    pub piste: AbilityColors,
    pub highlight: AbilityColors,
    pub cliff: Rgba<u8>,
}

pub struct AbilityColors {
    pub beginner: Rgba<u8>,
    pub intermedite: Rgba<u8>,
    pub advanced: Rgba<u8>,
    pub expert: Rgba<u8>,
    pub ungraded: Rgba<u8>,
}

impl Colors {
    fn selection_color(
        &self,
        xy: &XY<u32>,
        terrain: &Grid<f32>,
        selection: &selection::Handler,
    ) -> Option<Rgba<u8>> {
        let grid = selection.grid.as_ref()?;

        if grid.in_bounds(xy) && grid[xy] {
            let color = match cell_ability(terrain, xy) {
                Some(Ability::Beginner) => self.piste.beginner,
                Some(Ability::Intermediate) => self.piste.intermedite,
                Some(Ability::Advanced) => self.piste.advanced,
                Some(Ability::Expert) => self.piste.expert,
                None => self.piste.ungraded,
            };
            Some(color)
        } else {
            None
        }
    }

    fn piste_color(
        &self,
        position: &XY<u32>,
        piste_map: &Grid<Option<usize>>,
        highlights: &HashSet<usize>,
        abilities: &HashMap<usize, Ability>,
    ) -> Option<Rgba<u8>> {
        let piste_id = piste_map[position]?;

        let colors = if highlights.contains(&piste_id) {
            &self.highlight
        } else {
            &self.piste
        };

        let color = match abilities.get(&piste_id) {
            Some(Ability::Beginner) => colors.beginner,
            Some(Ability::Intermediate) => colors.intermedite,
            Some(Ability::Advanced) => colors.advanced,
            Some(Ability::Expert) => colors.expert,
            None => colors.ungraded,
        };

        Some(color)
    }

    fn cliff_color(&self, xy: &XY<u32>, terrain: &Grid<f32>) -> Option<Rgba<u8>> {
        if cell_ability(terrain, xy).is_none() {
            Some(self.cliff)
        } else {
            None
        }
    }
}

pub struct Parameters<'a> {
    pub graphics: &'a mut dyn Graphics,
    pub terrain: &'a Grid<f32>,
    pub piste_map: &'a Grid<Option<usize>>,
    pub highlights: &'a HashSet<usize>,
    pub abilities: &'a HashMap<usize, Ability>,
    pub selection: &'a selection::Handler,
}

impl System {
    pub fn new(colors: Colors) -> System {
        System {
            drawing: None,
            updates: vec![],
            show_pistes: true,
            colors,
        }
    }

    pub fn init(&mut self, graphics: &mut dyn Graphics, terrain: &Grid<f32>) {
        self.drawing = Some(Drawing::init(graphics, terrain));
        self.update_all();
    }

    pub fn update(&mut self, update: XYRectangle<u32>) {
        self.updates.push(update);
    }

    pub fn update_all(&mut self) {
        if let Some(drawing) = &self.drawing {
            self.updates.push(XYRectangle {
                from: xy(0, 0),
                to: xy(drawing.width() - 1, drawing.height() - 1), // -1 because rectangle is inclusive
            });
        }
    }

    pub fn toggle_show_pistes(&mut self) {
        self.show_pistes = !self.show_pistes;
    }

    pub fn run(
        &mut self,
        Parameters {
            graphics,
            terrain,
            piste_map,
            highlights,
            abilities,
            selection,
        }: Parameters<'_>,
    ) {
        let Some(drawing) = &self.drawing else { return };

        for update in self.updates.drain(..) {
            let mut image = OriginGrid::from_rectangle(update, CLEAR);

            for position in update.iter() {
                image[position] = self
                    .colors
                    .selection_color(&position, terrain, selection)
                    .or_else(|| {
                        if self.show_pistes {
                            self.colors
                                .piste_color(&position, piste_map, highlights, abilities)
                        } else {
                            None
                        }
                    })
                    .or_else(|| self.colors.cliff_color(&position, terrain))
                    .unwrap_or(CLEAR);
            }

            drawing
                .modify_overlay(graphics, &image)
                .unwrap_or_else(|_| println!("WARN: Could not draw overlay"));
        }
    }
}
