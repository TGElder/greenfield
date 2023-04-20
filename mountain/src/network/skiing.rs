use ::network::model::{Edge, Network};
use commons::{geometry::XY, grid::Grid};

use crate::model::skiing::State;
use crate::{
    model::Direction,
    network::velocity_encoding::{decode_velocity, encode_velocity},
    physics,
};

pub struct SkiingNetwork<'a> {
    pub terrain: &'a Grid<f32>,
}

impl<'a> Network<State> for SkiingNetwork<'a> {
    fn edges<'b>(
        &'b self,
        from: &'b State,
    ) -> Box<dyn Iterator<Item = ::network::model::Edge<State>> + 'b> {
        Box::new(
            [
                from.travel_direction.next_anticlockwise(),
                from.travel_direction,
                from.travel_direction.next_clockwise(),
            ]
            .into_iter()
            .flat_map(|travel_direction| self.get_edge(from, travel_direction)),
        )
    }
}

impl<'a> SkiingNetwork<'a> {
    fn get_edge(&self, from: &State, travel_direction: Direction) -> Option<Edge<State>> {
        let to_position = self.get_to_position(&from.position, &travel_direction)?;

        // dbg!(to_position);

        let initial_velocity: f32 = decode_velocity(&from.velocity)?;

        // dbg!(initial_velocity);

        let run = travel_direction.run();
        // dbg!(run);

        let rise = self.terrain[to_position] - self.terrain[from.position];
        // dbg!(rise);
        let physics::skiing::Solution { velocity, duration } =
            physics::skiing::solve(initial_velocity, run, rise)?;

        let out = Edge {
            from: *from,
            to: State {
                position: to_position,
                velocity: encode_velocity(&velocity)?,
                travel_direction,
            },
            cost: (duration * 1_000_000.0).round() as u32,
        };

        // dbg!(out);

        Some(out)
    }

    fn get_to_position(&self, position: &XY<u32>, travel_direction: &Direction) -> Option<XY<u32>> {
        let offset = travel_direction.offset();
        self.terrain.offset(position, offset)
    }
}
