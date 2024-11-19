use std::collections::HashMap;

use commons::map::ContainsKeyValue;

pub fn is_editable<'a>(
    id: &'a usize,
    open: &'a HashMap<usize, bool>,
    locations: &'a HashMap<usize, usize>,
) -> Editable<'a> {
    if open.contains_key_value(id, true) {
        return Editable::False(Reason::Open);
    }

    if let Some((entity_id, _)) = locations
        .iter()
        .find(|(_, &location_id)| location_id == *id)
    {
        return Editable::False(Reason::Occupied(entity_id));
    }

    Editable::True
}

pub enum Editable<'a> {
    True,
    False(Reason<'a>),
}

pub enum Reason<'a> {
    Open,
    Occupied(&'a usize),
}
