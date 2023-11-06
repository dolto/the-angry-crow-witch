use std::f32::consts::PI;
use bevy::input::ButtonState;
use bevy::input::keyboard::KeyboardInput;
use bevy::prelude::*;
use crate::bevygame::setup_res::{AnimationIndices, Posion, ResourcePosion, ResourceTower, ResourceWitch, Rollat, RollatHandle, Stuff, Witch, WitchFailed};

fn spin_rollat(
    mut commands: Commands,
    mut query_rollat: Query<&mut Rollat>,
    mut query_handle: Query<(&mut RollatHandle, &mut Transform)>,
    mut query_witch: Query<(&mut TextureAtlasSprite, &mut AnimationIndices, Entity), With<Witch>>,
    mut res_push: ResMut<ResourcePosion>,
    mut res_witch: ResMut<ResourceWitch>,
    mut res_tower: ResMut<ResourceTower>,
    res_time: Res<Time>,
    mut event_reader_keyboard: EventReader<KeyboardInput>
){
    let rollat = query_rollat.single_mut();
    let (mut handle, mut trans) = query_handle.single_mut();
    let (mut sprite, mut indices, entity) = query_witch.single_mut();

    if (res_witch.stuff.is_some() || res_push.push) && res_push.lock{
        let moving = rollat.0 * res_time.delta_seconds();
        handle.0 += moving;
        if handle.0 > PI*2.{
            handle.0 = 0.;
        }
        if res_witch.stuff.is_some(){
            match res_witch.stuff {
                None => {}
                Some(Stuff::Fire) => {
                    if handle.0 > PI/2. + 0.8{
                        let stuff = res_witch.stuff.clone().unwrap();
                        let posion = Posion{
                            stronger : res_witch.stronger,
                            property : stuff
                        };
                        res_tower.posions.push_back(posion);
                        indices.row = 0;
                        sprite.index = 0;
                        res_witch.stuff = None;
                        res_witch.stronger = 0;
                    }
                },
                Some(Stuff::Water) => {
                    if handle.0 > PI + 0.8{
                        let stuff = res_witch.stuff.clone().unwrap();
                        let posion = Posion{
                            stronger : res_witch.stronger,
                            property : stuff
                        };
                        res_tower.posions.push_back(posion);
                        indices.row = 0;
                        sprite.index = 0;
                        res_witch.stuff = None;
                        res_witch.stronger = 0;
                    }
                },
                Some(Stuff::Poison) => {
                    if handle.0 > PI*1.5 + 0.8{
                        let stuff = res_witch.stuff.clone().unwrap();
                        let posion = Posion{
                            stronger : res_witch.stronger,
                            property : stuff
                        };
                        res_tower.posions.push_back(posion);
                        indices.row = 0;
                        sprite.index = 0;
                        res_witch.stuff = None;
                        res_witch.stronger = 0;
                    }
                },
                Some(Stuff::Light) => {
                    if handle.0 < 0.8{
                        let stuff = res_witch.stuff.clone().unwrap();
                        let posion = Posion{
                            stronger : res_witch.stronger,
                            property : stuff
                        };
                        res_tower.posions.push_back(posion);
                        indices.row = 0;
                        sprite.index = 0;
                        res_witch.stuff = None;
                        res_witch.stronger = 0;
                    }
                },
            }
        }

        trans.rotation = Quat::from_euler(EulerRot::XYZ, 0.,0.,handle.0 * -1. - 0.2);
    }

    for ev in event_reader_keyboard.read(){
        match ev.state {
            ButtonState::Pressed => {
                if ev.key_code == Some(KeyCode::S){
                    res_push.push = true;

                    if res_witch.stuff.is_some(){
                        match res_witch.stuff {
                            None => {}
                            Some(Stuff::Fire) => {
                                if handle.0 < PI/2.{
                                    res_witch.stronger += 1;
                                }else {
                                    res_witch.stuff = None;
                                    commands.entity(entity).insert(WitchFailed(0.));
                                }
                            },
                            Some(Stuff::Water) => {
                                if handle.0 < PI && handle.0 >= PI/2.{
                                    res_witch.stronger += 1;
                                }else {
                                    res_witch.stuff = None;
                                    commands.entity(entity).insert(WitchFailed(0.));
                                }
                            },
                            Some(Stuff::Poison) => {
                                if handle.0 < PI*1.5 && handle.0 >= PI{
                                    res_witch.stronger += 1;
                                }else {
                                    res_witch.stuff = None;
                                    commands.entity(entity).insert(WitchFailed(0.));
                                }
                            },
                            Some(Stuff::Light) => {
                                if handle.0 < PI * 2. && handle.0 >= PI*1.5{
                                    res_witch.stronger += 1;
                                }else {
                                    res_witch.stuff = None;
                                    commands.entity(entity).insert(WitchFailed(0.));
                                }
                            }
                        }

                    }else{
                        indices.row = 1;
                        sprite.index = indices.row * indices.last;
                    }
                }
            }
            ButtonState::Released => {
                if ev.key_code == Some(KeyCode::S){
                    res_push.push = false;
                    if handle.0 < PI/2.{
                        res_witch.stuff = Some(Stuff::Fire);
                    }
                    else if handle.0 < PI {
                        res_witch.stuff = Some(Stuff::Water);
                    }
                    else if handle.0 < PI * 1.5{
                        res_witch.stuff = Some(Stuff::Poison);
                    }
                    else if handle.0 < PI * 2.{
                        res_witch.stuff = Some(Stuff::Light);
                    }

                    if res_witch.stuff.is_some(){
                        indices.row = 2;
                        sprite.index = indices.row * indices.last;
                    }else{
                        indices.row = 0;
                        sprite.index = 0;
                    }

                }
            }
        }
    }
}

fn witch_failed(
    mut commands: Commands,
    mut query_witch: Query<(&mut WitchFailed, &mut TextureAtlasSprite, Entity)>,
    mut res_posion: ResMut<ResourcePosion>,
    res_time: Res<Time>
){
    for (mut timer, mut sprite, entity) in query_witch.iter_mut(){
        if timer.0 == 0. {
            sprite.color = Color::from([0.5,0.5,0.5]);
            res_posion.lock = false;
        }
        else if timer.0 > 1.1{
            sprite.color = Color::from([1.,1.,1.]);
            commands.entity(entity).remove::<WitchFailed>();
            timer.0 = 0.;
            res_posion.lock = true;
        }
        timer.0 += res_time.delta_seconds();
    }
}

pub struct WitchPlugin;
impl Plugin for WitchPlugin{
    fn build(&self, app: &mut App) {
        app.
            add_systems(Update,(
                spin_rollat,
                witch_failed
                ));
    }
}
