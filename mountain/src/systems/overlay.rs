use std::collections::HashMap;

use commons::color::Rgba;
use commons::geometry::{xy, XYRectangle, XY};
use commons::origin_grid::OriginGrid;
use engine::graphics::Graphics;

use crate::draw::terrain;
use crate::handlers::selection;
use crate::model::skiing::State;
use crate::model::{Direction, Lift, Piste, PisteCosts};

pub const CLEAR: Rgba<u8> = Rgba::new(0, 0, 0, 0);

pub struct System {
    updates: Vec<XYRectangle<u32>>,
    colors: Colors,
}

pub struct Colors {
    pub selection: Rgba<u8>,
    pub piste: Rgba<u8>,
    pub lift: Rgba<u8>,
}

impl System {
    pub fn new(colors: Colors) -> System {
        System {
            updates: vec![],
            colors,
        }
    }

    pub fn update(&mut self, update: XYRectangle<u32>) {
        self.updates.push(update);
    }

    pub fn run(
        &mut self,
        graphics: &mut dyn Graphics,
        drawing: Option<&terrain::Drawing>,
        pistes: &HashMap<usize, Piste>,
        piste_costs: &HashMap<usize, PisteCosts>,
        lifts: &HashMap<usize, Lift>,
        selection: &selection::Handler,
    ) {
        let Some(drawing) = drawing else {return};

        for update in self.updates.drain(..) {
            let mut image = OriginGrid::from_rectangle(update, CLEAR);

            let XYRectangle { from, to } = update;
            for x in from.x..=to.x {
                for y in from.y..=to.y {
                    let position = xy(x, y);
                    image[position] = selection_color(self.colors.selection, &position, selection)
                        .or_else(|| lift_color(self.colors.lift, &position, lifts))
                        .or_else(|| {
                            piste_color(self.colors.piste, &position, pistes, piste_costs, lifts)
                        })
                        .unwrap_or(CLEAR);
                }
            }

            drawing
                .modify_overlay(graphics, &image)
                .unwrap_or_else(|_| println!("WARN: Could not draw overlay"));
        }
    }
}

fn selection_color(
    color: Rgba<u8>,
    xy: &XY<u32>,
    selection: &selection::Handler,
) -> Option<Rgba<u8>> {
    let Some(rectangle) = selection.selected_rectangle() else {return None};

    if rectangle.contains(xy) {
        Some(color)
    } else {
        None
    }
}

fn piste_color(
    color: Rgba<u8>,
    xy: &XY<u32>,
    pistes: &HashMap<usize, Piste>,
    piste_costs: &HashMap<usize, PisteCosts>,
    lifts: &HashMap<usize, Lift>,
) -> Option<Rgba<u8>> {
    let Some(lift) = lifts.keys().next() else {return None};
    for (id, piste) in pistes.iter() {
        if piste_costs
            .get(id)
            .and_then(|costs| costs.costs(lift))
            .map(|costs| {
                costs.contains_key(&State {
                    position: *xy,
                    velocity: 0,
                    travel_direction: Direction::North,
                })
            })
            .unwrap_or(false)
        {
            if piste.grid.in_bounds(xy) && piste.grid[xy] {
                return Some(color);
            }
        }
    }

    None
}

fn lift_color(color: Rgba<u8>, xy: &XY<u32>, lifts: &HashMap<usize, Lift>) -> Option<Rgba<u8>> {
    for lift in lifts.values() {
        if lift.from == *xy || lift.to == *xy {
            return Some(color);
        }
    }

    None
}
