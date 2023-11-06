use crate::bevygame::setup_res::*;
use bevy::input::keyboard::KeyboardInput;
use bevy::prelude::*;
use rand::Rng;
use rand::prelude::ThreadRng;



fn bird_move(
    mut query_bird: Query<(&mut Transform, &mut Bird, &mut TextureAtlasSprite)>,
    mut res_tower: ResMut<ResourceTower>,
    res_time: Res<Time>,
) {
    let move_speed = res_tower.bird_speed * res_time.delta_seconds();
    for (mut trans, mut brid, mut sprite) in query_bird.iter_mut() {
        let moving = move_speed * brid.turn;
        if brid.turn > 0. && trans.translation.x > 136. {
            for _ in 0..3 {
                let posion = res_tower.posions.pop_front();
                if posion.is_none() || brid.poops.len() >= 3 {
                    break;
                }
                brid.poops.push_back(posion.unwrap());
            }
            sprite.flip_x = false;
            brid.turn = -1.;
        } else if trans.translation.x < -155.5 {
            sprite.flip_x = true;
            brid.turn = 1.;
        }

        trans.translation += Vec3::new(moving, 0., 0.);
    }
}

fn bird_pooping(
    mut command: Commands,
    mut event_reader_keyboard: EventReader<KeyboardInput>,
    mut query_brid: Query<(&mut Bird, &Transform)>,
    res_poop: Res<ResourcePoop>,
) {
    for ev in event_reader_keyboard.read() {
        if let Some(KeyCode::A) = ev.key_code {
            for (mut brid, trans) in query_brid.iter_mut() {
                if let Some(poop) = brid.poops.pop_front() {
                    let sprite = match poop.property {
                        Stuff::Fire => res_poop.fire.clone(),
                        Stuff::Light => res_poop.light.clone(),
                        Stuff::Poison => res_poop.poison.clone(),
                        Stuff::Water => res_poop.water.clone(),
                    };
                    command.spawn((
                        SpriteBundle {
                            texture: sprite,
                            sprite: Sprite {
                                custom_size: Some(Vec2::new(14., 14.)),
                                ..default()
                            },
                            transform: Transform::from_translation(trans.translation.clone()),
                            ..default()
                        },
                        Poop(poop, 0.)
                    ));
                };
            }
        }
    }
}

fn poop_throw(
    mut commands: Commands,
    mut query_poops: Query<(&mut Poop, &mut Transform), With<Poop>>,
    query_slime: Query<(&Slime, &TextureAtlasSprite, &Transform, Entity), Without<Poop>>,
    res_image: Res<ResourceImage>,
    res_time: Res<Time>,
    mut event_writer: EventWriter<EventExplore>
) {
    let delta_sec = res_time.delta_seconds();
    let more_speed = delta_sec * 100.;
    for (mut poop, mut trans) in query_poops.iter_mut() {
        trans.translation += Vec3::NEG_Y * poop.1 * delta_sec;
        poop.1 += more_speed;
        let poop_pos = trans.translation;
        if poop_pos.z < -221.5 {
            event_writer.send(
                EventExplore{
                    stuff: poop.0.property.clone(),
                    pos: poop_pos,
                    stronger: poop.0.stronger as f32 * 48.
                }
            );
        }
        for (slime, sprite, slime_trans, entity) in query_slime.iter(){
            let slime_pos = slime_trans.translation;
            if poop_pos.distance(slime_pos) <= 35.{
                if slime.property == poop.0.property{
                    let mut death = res_image.slime_death.clone();
                    death.0.transform.translation = slime_pos;
                    death.0.sprite.color = sprite.color.clone();
                    commands.spawn(
                        death
                    );
                    let mut rng = rand::thread_rng();
                    let count = rng.gen_range(3..=10);
                    for _ in 0..count{
                        let mut pice = res_image.slime_pice.clone();
                        pice.0.sprite.color = sprite.color.clone();
                        pice.0.transform.translation = slime_pos;
                        commands.spawn(
                          pice
                        );
                    }
                    commands.entity(entity).despawn();
                }
            }
        }
    }
}

fn poop_explore(
    mut commands: Commands,
    query_slime: Query<(&Slime, &TextureAtlasSprite, &Transform, Entity)>,
    res_image: Res<ResourceImage>,
    res_poop: Res<ResourcePoop>,
    mut event_reader_explore: EventReader<EventExplore>
){
    for ev in event_reader_explore.read(){
        let mut boom = res_poop.boom.clone();
        boom.0.transform.translation = ev.pos;
        commands.spawn(
            boom
        );

        for (slime, sprite, trans, entity) in query_slime.iter(){
            if trans.translation.distance(ev.pos) > ev.stronger && ev.stuff == slime.property{
                let mut death = res_image.slime_death.clone();
                death.0.transform.translation = trans.translation;
                death.0.sprite.color = sprite.color.clone();
                commands.spawn(
                    death
                );
                let mut rng = rand::thread_rng();
                let count = rng.gen_range(3..=10);
                for _ in 0..count{
                    let mut pice = res_image.slime_pice.clone();
                    pice.0.sprite.color = sprite.color.clone();
                    pice.0.transform.translation = trans.translation;
                    commands.spawn(
                        pice
                    );
                }
                commands.entity(entity).despawn();
            }
        }
    }
}

pub struct BirdPlugin;
impl Plugin for BirdPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (bird_move, bird_pooping, poop_throw, poop_explore));
    }
}
