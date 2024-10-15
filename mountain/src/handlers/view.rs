use engine::binding::Binding;
use engine::events::Event;

use crate::systems::{skier_colors, terrain_artist, tree_artist};

pub struct Parameters<'a> {
    pub bindings: &'a Bindings,
    pub terrain_artist: &'a mut terrain_artist::System,
    pub tree_artist: &'a mut tree_artist::System,
    pub skier_colors: &'a mut skier_colors::System,
    pub graphics: &'a mut dyn engine::graphics::Graphics,
}

pub fn handle(
    event: &Event,
    Parameters {
        bindings,
        terrain_artist,
        tree_artist,
        skier_colors,
        graphics,
    }: Parameters<'_>,
) {
    if bindings.toggle_pistes.binds_event(event) {
        terrain_artist.toggle_show_pistes();
        terrain_artist.update_whole_overlay();
    }

    if bindings.toggle_trees.binds_event(event) {
        tree_artist.toggle_visible(graphics);
    }

    if bindings.toggle_skier_ability.binds_event(event) {
        skier_colors.toggle_show_ability();
    }
}

pub struct Bindings {
    pub toggle_pistes: Binding,
    pub toggle_trees: Binding,
    pub toggle_skier_ability: Binding,
}
