use std::collections::HashMap;

use commons::geometry::xy;

use crate::model::entrance::Entrance;
use crate::model::skiing::{Plan, State};

pub fn run(
    plans: &HashMap<usize, Plan>,
    entrances: &HashMap<usize, Entrance>,
    targets: &mut HashMap<usize, usize>,
    locations: &mut HashMap<usize, usize>,
) {
    for (id, plan) in plans {
        let Plan::Stationary(State {
            position: plan_position,
            ..
        }) = plan
        else {
            continue;
        };
        let Some(target) = targets.get(id).copied() else {
            continue;
        };

        let Some(entrance) = entrances.get(&target) else {
            continue;
        };

        if (entrance.from.x..=entrance.to.x)
            .flat_map(|x| (entrance.from.y..=entrance.to.y).map(move |y| xy(x, y)))
            .any(|position| position == *plan_position)
        {
            targets.remove(id);
            locations.insert(*id, entrance.piste);
        }
    }
}
