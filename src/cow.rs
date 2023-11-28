use bevy::prelude::*;

use crate::{Money, Player};
pub struct CowPlugin;

impl Plugin for CowPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_cow_parent)
            .add_systems(Update, (spawn_cow, cow_lifetime))
            .register_type::<Cow>();

    }
}

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct Cow {
    pub lifetime: Timer,
}

#[derive(Component)]
pub struct CowParent;

fn spawn_cow_parent(mut commands: Commands) {
    commands.spawn((SpatialBundle::default(), CowParent, Name::new("CowParent")));
}

fn spawn_cow(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    input: Res<Input<KeyCode>>,
    mut money: ResMut<Money>,
    player: Query<&Transform, With<Player>>,
    parent: Query<Entity, With<CowParent>>,
) {
    if !input.just_pressed(KeyCode::Space) {
        return;
    }

    let player_transform = player.single();
    let parent = parent.single();
    if money.0 >= 10.0 {
        money.0 -= 10.0;
        info!("Spent $10 on a cow, remaining money: ${:?}", money.0);
        let texture = asset_server.load("cow.png");

        commands.entity(parent).with_children(|commands| {
            commands.spawn((
                SpriteBundle {
                    texture,
                    transform: *player_transform,
                    ..default()
                },
                Cow {
                    lifetime: Timer::from_seconds(4.0, TimerMode::Once),
                },
                Name::new("Cow"),
            ));
        });
    } 
    
}

fn cow_lifetime(
    mut commands: Commands,
    time: Res<Time>,
    mut cows : Query<(Entity, &mut Cow)>,
    parent: Query<Entity, With<CowParent>>,
    mut money: ResMut<Money>,
    ) {
        let parent = parent.single();

        for(cow_entity, mut cow) in &mut cows {
            cow.lifetime.tick(time.delta());

            if cow.lifetime.finished() {
                money.0+= 15.0;

                commands.entity(parent).remove_children(&[cow_entity]);
                commands.entity(cow_entity).despawn();
                info!("Cow sold for $15, remaining money: ${:?}", money.0)
            }
        }
}