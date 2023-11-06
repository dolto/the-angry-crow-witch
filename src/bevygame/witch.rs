use std::f32::consts::PI;
use bevy::input::ButtonState;
use bevy::input::keyboard::KeyboardInput;
use bevy::prelude::*;
use crate::bevygame::setup_res::{AnimationIndices, Posion, ResourcePosion, ResourceTower, ResourceWitch, Rollat, RollatHandle, Stuff, Witch, WitchFailed};

fn spin_rollat( //포션 제조 컨트롤과 애니메이션 관리
    mut commands: Commands,
    mut query_rollat: Query<(&mut Sprite, &Rollat)>,
    mut query_handle: Query<(&mut RollatHandle, &mut Transform)>,
    mut query_witch: Query<(&mut TextureAtlasSprite, &mut AnimationIndices, Entity), With<Witch>>,
    mut res_push: ResMut<ResourcePosion>,
    mut res_witch: ResMut<ResourceWitch>,
    mut res_tower: ResMut<ResourceTower>,
    res_time: Res<Time>,
    mut event_reader_keyboard: EventReader<KeyboardInput>
){
    let (mut rollatsprite,rollat) = query_rollat.single_mut();
    let (mut handle, mut trans) = query_handle.single_mut();
    let (mut sprite, mut indices, entity) = query_witch.single_mut();

    if (res_witch.stuff.is_some() || res_push.push) && res_push.lock{
        let moving = rollat.0 * res_time.delta_seconds();
        handle.0 += moving;
        if handle.0 > PI*2.{
            handle.0 = 0.;
        }
        if res_witch.stuff.is_some(){ //어떤 재료로 만들지 정해졌다면
            match res_witch.stuff {
                None => {}
                Some(Stuff::Fire) => { //그 재료에서 약 50도 정도의 유예기간 동안 그냥 둔다면 포션 생성 성공
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
                    if handle.0 > 0.8 && handle.0 < 0.9{
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

        trans.rotation = Quat::from_euler(EulerRot::XYZ, 0.,0.,handle.0 * -1. + 0.2);
    }

    for ev in event_reader_keyboard.read(){
        match ev.state {
            ButtonState::Pressed => {
                if ev.key_code == Some(KeyCode::S){
                    res_push.push = true;
                    rollatsprite.color = Color::rgb(0.5, 0.5, 0.5);
                    if res_witch.stuff.is_some(){ //재료가 정해졌다면
                        match res_witch.stuff {
                            None => {}
                            Some(Stuff::Fire) => { 
                                if handle.0 < PI/2.{//그 재료의 범위 안에서 연타하면 포션 강화
                                    res_witch.stronger += 1;
                                }else {  //그 재료의 범위 안에서 실패하면 포션 제조 실패 및 경직 부여
                                    commands.entity(entity).insert(WitchFailed(0.));
                                }
                            },
                            Some(Stuff::Water) => {
                                if handle.0 < PI && handle.0 >= PI/2.{
                                    res_witch.stronger += 1;
                                }else {
                                    commands.entity(entity).insert(WitchFailed(0.));
                                }
                            },
                            Some(Stuff::Poison) => {
                                if handle.0 < PI*1.5 && handle.0 >= PI{
                                    res_witch.stronger += 1;
                                }else {
                                    commands.entity(entity).insert(WitchFailed(0.));
                                }
                            },
                            Some(Stuff::Light) => {
                                if handle.0 < PI * 2. && handle.0 >= PI*1.5{
                                    res_witch.stronger += 1;
                                }else {
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
                    rollatsprite.color = Color::rgb(1., 1., 1.);
                    if res_push.lock{
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
    mut res_witch: ResMut<ResourceWitch>,
    res_time: Res<Time>
){
    for (mut timer, mut sprite, entity) in query_witch.iter_mut(){
        if timer.0 == 0. {
            sprite.color = Color::from([0.5,0.5,0.5]);
            res_witch.stuff = None;
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
