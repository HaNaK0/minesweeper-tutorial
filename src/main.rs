use bevy::prelude::*;
use board_plugin::BoardPlugin;

#[cfg(feature = "debug")]
use bevy_inspector_egui::WorldInspectorPlugin;

fn main() {
    let mut app = App::new();

    app.insert_resource(WindowDescriptor {
        title: "Mine Sweeper!".to_string(),
        width: 700.,
        height: 800.,
        ..Default::default()
    })
    .add_plugins(DefaultPlugins)
    .add_plugin(BoardPlugin);
    #[cfg(feature = "debug")]
    app.add_plugin(WorldInspectorPlugin::new());

    app.add_startup_system(camera_setup);

    app.run();
}

fn camera_setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}
