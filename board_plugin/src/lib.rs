pub mod components;
pub mod resources;

use bevy::prelude::*;
use bevy::log;
use resources::tile_map::TileMap;

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(Self::create_board);
        log::info!("Loaded Board plugin")
    }
}

impl BoardPlugin {
    ///System to generate the complete board
    pub fn create_board() {
        let mut tile_map = TileMap::empty(20, 20);
        tile_map.set_bombs(40);
        #[cfg(feature = "debug")]
        log::info!("{}", tile_map.console_output())
    }
}
