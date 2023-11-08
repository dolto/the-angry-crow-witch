use crate::bevygame::setup_res::*;
use bevy::audio::PlaybackMode;
use bevy::input::ButtonState;
use bevy::input::keyboard::KeyboardInput;
use bevy::prelude::*;
use rand::Rng;
use rand::prelude::ThreadRng;



fn bird_move(
    mut commands: Commands,
    mut query_bird: Query<(&mut Transform, &mut Bird, &mut TextureAtlasSprite, Entity)>,
    mut res_tower: ResMut<ResourceTower>,
    res_posion: Res<ResourcePosion>,
    res_time: Res<Time>,
) {
    let move_speed = res_tower.bird_speed * res_time.delta_seconds();
    for (mut trans, mut brid, mut sprite, entity) in query_bird.iter_mut() {
        let moving = move_speed * brid.turn;
        if brid.turn > 0. && trans.translation.x > 136. {
            for _ in 0..3 {
                let posion = res_tower.posions.pop_front();
                let len = brid.poops.len();
                if posion.is_none() || len >= 3 {
                    break;
                }
                let _posi = posion.unwrap();
                commands.entity(entity).with_children(|p|{
                    p.spawn(
                      SpriteBundle{
                        texture: res_posion.posion.clone(),
                        sprite: Sprite{
                            color: match _posi.property {
                                Stuff::Fire => {
                                    Color::RED
                                },
                                Stuff::Water => {
                                    Color::BLUE
                                },
                                Stuff::Poison => {
                                    Color::GREEN
                                },
                                Stuff::Light =>{
                                    Color::YELLOW
                                }
                            },
                            custom_size: Some(Vec2::new(8.,8.)),
                            ..default()
                        },
                        transform: Transform::from_translation(Vec3::new(0., 18.+ 10. * len as f32, 0.)),
                        ..default()
                      }
                    ).insert(BridPosion);
                });
                brid.poops.push_back(_posi);
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
    mut commands: Commands,
    mut event_reader_keyboard: EventReader<KeyboardInput>,
    mut query_brid: Query<(&mut Bird, &Transform, &Children), Without<BridPosion>>,
    mut query_bp: Query<(&mut Transform, Entity), With<BridPosion>>,
    res_poop: Res<ResourcePoop>,
    res_sound: Res<ResourceAudio>
) {
    for ev in event_reader_keyboard.read() {
        if let Some(KeyCode::A) = ev.key_code {
            if ev.state == ButtonState::Pressed{
                for (mut brid, trans, child) in query_brid.iter_mut() {
                    if let Some(poop) = brid.poops.pop_front() {
                        let sprite = match poop.property {
                            Stuff::Fire => res_poop.fire.clone(),
                            Stuff::Light => res_poop.light.clone(),
                            Stuff::Poison => res_poop.poison.clone(),
                            Stuff::Water => res_poop.water.clone(),
                        };
                        for &ch in child.iter(){
                            if let Ok((mut trans, entity)) = query_bp.get_mut(ch){
                                trans.translation += Vec3::NEG_Y * 10.;
                                if trans.translation.y < 10.{
                                    commands.entity(entity).despawn();
                                }
                            }
                        }
                        commands.spawn((
                            SpriteBundle {
                                texture: sprite,
                                sprite: Sprite {
                                    custom_size: Some(Vec2::new(14., 14.)),
                                    ..default()
                                },
                                transform: Transform::from_translation(trans.translation.clone()),
                                ..default()
                            },
                            AudioBundle {
                                source: res_sound.throw_poop_sound.clone(),
                                settings: PlaybackSettings{
                                    mode: PlaybackMode::Despawn,
                                    speed: 1.2,
                                    ..default()
                                },
                                ..default()
                            },
                            Poop(poop, 0.)
                        ));
                    };
                }
            }
        }
    }
}

fn poop_throw(
    mut commands: Commands,
    mut query_poops: Query<(&mut Poop, &mut Transform, Entity), With<Poop>>,
    query_slime: Query<(&Slime, &TextureAtlasSprite, &Transform, Entity), Without<Poop>>,
    res_image: Res<ResourceImage>,
    res_time: Res<Time>,
    mut event_writer: EventWriter<EventExplore>,
    res_sound: Res<ResourceAudio>
) {
    let delta_sec = res_time.delta_seconds();
    let more_speed = delta_sec * 100.;
    for (mut poop, mut trans, poop_entity) in query_poops.iter_mut() {
        trans.translation += Vec3::NEG_Y * poop.1 * delta_sec;
        poop.1 += more_speed;
        let poop_pos = trans.translation;
        if poop_pos.y < -213.5 {
            event_writer.send(
                EventExplore{
                    stuff: poop.0.property.clone(),
                    pos: poop_pos,
                    stronger: poop.0.stronger as f32 * 16.
                }
            );
            commands.entity(poop_entity).despawn();
        }
        for (slime, sprite, slime_trans, entity) in query_slime.iter(){
            let slime_pos = slime_trans.translation;
            if poop_pos.distance(slime_pos) <= 32.{
                if slime.property == poop.0.property{
                    let mut death = res_image.slime_death.clone();
                    death.0.transform.translation = slime_pos;
                    death.0.sprite.color = sprite.color.clone();
                    commands.spawn(
                        death
                    ).insert(Die);
                    commands.spawn(
                        AudioBundle{
                          source: res_sound.slime_daeth_sound.clone(),
                          settings:PlaybackSettings{
                              mode: PlaybackMode::Despawn,
                              ..default()
                          },
                          ..default()
                        }  
                      );
                    let mut rng = rand::thread_rng();
                    let count = rng.gen_range(3..=10);
                    for _ in 0..count{
                        let mut pice = res_image.slime_pice.clone();
                        pice.0.sprite.color = sprite.color.clone();
                        pice.0.transform.translation = slime_pos;
                        commands.spawn(
                          pice
                        ).insert(Exploer(rng.gen_range((16.)..=64.),rng.gen_range((16.)..=64.)));
                    }
                    commands.entity(entity).despawn();
                    commands.entity(poop_entity).despawn();
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
    mut event_reader_explore: EventReader<EventExplore>,
    res_sound: Res<ResourceAudio>
){
    for ev in event_reader_explore.read(){
        let mut boom = res_poop.boom.clone();
        boom.0.transform.translation = ev.pos;
        commands.spawn(
            boom
        ).insert(Die);
        commands.spawn(
          AudioBundle{
            source: res_sound.explosion_sound.clone(),
            settings:PlaybackSettings{
                mode: PlaybackMode::Despawn,
                ..default()
            },
            ..default()
          }  
        );

        for (slime, sprite, trans, entity) in query_slime.iter(){
            if trans.translation.distance(ev.pos) < ev.stronger && ev.stuff == slime.property{
                let mut death = res_image.slime_death.clone();
                death.0.transform.translation = trans.translation;
                death.0.sprite.color = sprite.color.clone();
                commands.spawn(
                    death
                ).insert(Die);
                commands.spawn(
                    AudioBundle{
                      source: res_sound.slime_daeth_sound.clone(),
                      settings:PlaybackSettings{
                          mode: PlaybackMode::Despawn,
                          ..default()
                      },
                      ..default()
                    }  
                  );
                let mut rng = rand::thread_rng();
                let count = rng.gen_range(3..=10);
                for _ in 0..count{
                    let mut pice = res_image.slime_pice.clone();
                    pice.0.sprite.color = sprite.color.clone();
                    pice.0.transform.translation = trans.translation + Vec3::Y;
                    commands.spawn(
                        pice
                    ).insert(Exploer(rng.gen_range((-64.)..=64.),rng.gen_range((16.)..=64.)));
                }
                commands.entity(entity).despawn();
            }
        }
    }
}

fn boom_die(
    mut command: Commands,
    query_boom: Query<(Entity, &TextureAtlasSprite, &AnimationIndices), With<Die>>
){
    for (entity, sprite, indices) in query_boom.iter(){
        if sprite.index >= indices.last - 1{
            command.entity(entity).despawn();
        }
    }
}

fn pice_flying(
    mut command: Commands,
    mut query_pice: Query<(Entity, &mut Transform, &mut Exploer)>,
    res_time: Res<Time>
){
    //슬라임 조각들이 날아가는 효과
    let delta_sec = res_time.delta_seconds();
    for (entity, mut trans, mut ex) in query_pice.iter_mut(){
        trans.translation += Vec3::new(ex.0 * delta_sec, ex.1 * delta_sec, 0.);
        //ex.0 -= delta_sec * 60.; x좌표는 굳이 속도가 줄어야 하나?
        ex.1 -= delta_sec * 60.;
        if trans.translation.y < -230.{
            command.entity(entity).despawn();
        }
    }
}

pub struct BirdPlugin;
impl Plugin for BirdPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (bird_move, bird_pooping, poop_throw, poop_explore, boom_die, pice_flying));
    }
}
