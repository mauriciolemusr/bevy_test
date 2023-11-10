use bevy::{prelude::*, render::camera::ScalingMode};

#[derive(Component)]
pub struct Player{
    pub speed: f32,
}

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "We do a little bit of trolling...".into(),
                        resolution:(1280.0, 720.0).into(),
                        resizable: false,
                        ..default()
                    }),
                    ..default()
                })
                .build(),
        )
        .add_systems(Startup, setup)
        .add_systems(Update, character_movment)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut camera = Camera2dBundle::default();

    camera.projection.scaling_mode = ScalingMode::AutoMin {
        min_width: 12288.0,
        min_height: 6912.0,
    };

    commands.spawn(camera);

    let texture = asset_server.load("Trollface.png");

    commands.spawn((
        SpriteBundle {
            texture,
            ..default()
        },
        Player { speed: 2000.0 },
    ));
}


fn character_movment(
    mut characters: Query<(&mut Transform, &Player)>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    for(mut transform, player) in &mut characters {
        let movement_amount = player.speed * time.delta_seconds();
        if input.pressed(KeyCode::W) {
            transform.translation.y += movement_amount;
        }
        if input.pressed(KeyCode::S) {
            transform.translation.y -= movement_amount;
        }
        if input.pressed(KeyCode::A) {
            transform.translation.x -= movement_amount;
        }
        if input.pressed(KeyCode::D) {
            transform.translation.x += movement_amount;
        }
    }
}