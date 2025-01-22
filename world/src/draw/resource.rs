use std::collections::HashMap;

use commons::geometry::{xyz, Rectangle, XY, XYZ};
use commons::grid::Grid;
use engine::graphics::elements::Billboard;
use engine::graphics::{DrawMode, Graphics};

use crate::model::resource::{Resource, RESOURCES};

#[derive(Default)]
pub struct Artist {
    textures: HashMap<Resource, usize>,
}

impl Artist {
    pub fn init(&mut self, graphics: &mut dyn Graphics) {
        self.textures = RESOURCES
            .into_iter()
            .map(|resource| (resource, texture(&resource, graphics)))
            .collect();
    }

    pub fn draw(
        &self,
        resources: &Grid<Option<Resource>>,
        tile_heights: &Grid<f32>,
        graphics: &mut dyn Graphics,
    ) {
        // drawing by resource to avoid constantly changing texture
        for resource in RESOURCES {
            resources
                .iter()
                .filter_map(|xy| resources[xy].as_ref().map(|resource| (xy, resource)))
                .filter(|(_, &candidate)| candidate == resource)
                .for_each(|(xy, resource)| self.draw_one(resource, tile_heights, &xy, graphics));
        }
    }

    fn draw_one(
        &self,
        resource: &Resource,
        tile_heights: &Grid<f32>,
        position: &XY<u32>,
        graphics: &mut dyn Graphics,
    ) {
        let Some(texture) = self.textures.get(resource) else {
            return;
        };

        let billboard = Billboard {
            position: xyz_position(tile_heights, position),
            dimensions: Rectangle {
                width: 1.0,
                height: 1.0,
            },
            texture: *texture,
        };
        let index = graphics.create_billboards().unwrap();
        graphics
            .draw_billboard(&index, DrawMode::Solid, &billboard)
            .unwrap();
    }
}

fn texture(resource: &Resource, graphics: &mut dyn Graphics) -> usize {
    graphics
        .load_texture_from_file(texture_file(resource))
        .unwrap()
}

fn texture_file(resource: &Resource) -> &'static str {
    match resource {
        Resource::Apple => "resources/resources/apple.png",
        Resource::Banana => "resources/resources/banana.png",
        Resource::Blueberry => "resources/resources/blueberry.png",
        Resource::Cherry => "resources/resources/cherry.png",
        Resource::Coconut => "resources/resources/coconut.png",
        Resource::Grapes => "resources/resources/grapes.png",
        Resource::Kiwi => "resources/resources/kiwi.png",
        Resource::Lemon => "resources/resources/lemon.png",
        Resource::Lime => "resources/resources/lime.png",
        Resource::Orange => "resources/resources/orange.png",
        Resource::Peach => "resources/resources/peach.png",
        Resource::Pear => "resources/resources/pear.png",
        Resource::Pineapple => "resources/resources/pineapple.png",
        Resource::Strawberry => "resources/resources/strawberry.png",
        Resource::Tomato => "resources/resources/tomato.png",
        Resource::Watermelon => "resources/resources/watermelon.png",
    }
}

fn xyz_position(tile_heights: &Grid<f32>, position: &XY<u32>) -> XYZ<f32> {
    xyz(
        position.x as f32 + 0.5,
        position.y as f32 + 0.5,
        tile_heights[position] + 1.5,
    )
}
