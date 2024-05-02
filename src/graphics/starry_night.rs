use bevy::prelude::*;

pub fn entry() {
    build();
}

fn build() {
    App::new()
        .add_plugins(DefaultPlugins)
        .run();
}
