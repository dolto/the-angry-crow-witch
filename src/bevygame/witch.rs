use std::f32::consts::PI;
use bevy::{input::ButtonState, audio::PlaybackMode};
use bevy::input::keyboard::KeyboardInput;
use bevy::prelude::*;
use rand::Rng;
use crate::bevygame::setup_res::{AnimationIndices, Posion, ResourcePosion, ResourceTower, ResourceWitch, Rollat, RollatHandle, Stuff, Witch, WitchFailed};

use super::setup_res::{TowerPosion, HandleEffect, ResourceAudio, ResourceImage, Exploer};

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
                    rollatsprite.color = Color::rgb(1., 1., 1.);
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
                    rollatsprite.color = Color::rgb(0.7, 0.7, 0.7);
                    if res_push.lock && res_witch.stuff.is_none(){
                        if handle.0 < PI/2.{ // 이 각도를 지나기 전에 클릭을 풀면 선택
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

fn rollat_effect(
    mut commands: Commands,
    query_effect: Query<&GlobalTransform, With<HandleEffect>>,
    mut res_witch: ResMut<ResourceWitch>,
    res_sound: Res<ResourceAudio>,
    res_image: Res<ResourceImage>,
    res_time: Res<Time>,
    mut get_stronger: Local<i32>
){
    if let Some(stuff) = res_witch.stuff.clone(){
        res_witch.rollat_timer.tick(res_time.delta());
        if res_witch.rollat_timer.finished(){
            let trans = query_effect.get_single().unwrap();
            let mut rng = rand::thread_rng();
            let color = match stuff {
                Stuff::Fire => Color::RED,
                Stuff::Light => Color::YELLOW,
                Stuff::Poison => Color::GREEN,
                Stuff::Water => Color::BLUE,
            };
            let mut pice = res_image.slime_pice.clone();
            pice.0.sprite.color = color;
            pice.0.sprite.custom_size = Some(Vec2::splat(5.));
            pice.0.transform.translation = trans.translation();
            commands.spawn(
                pice
            ).insert(Exploer(rng.gen_range((-16.)..=16.),rng.gen_range((-32.)..=32.)));
        }else if *get_stronger < res_witch.stronger{
            let trans = query_effect.get_single().unwrap();
            *get_stronger = res_witch.stronger;
            let mut rng = rand::thread_rng();
            let color = match stuff {
                Stuff::Fire => Color::RED,
                Stuff::Light => Color::YELLOW,
                Stuff::Poison => Color::GREEN,
                Stuff::Water => Color::BLUE,
            };
            let mut pice = res_image.slime_pice.clone();
            let count = rng.gen_range(4..10);
            pice.0.sprite.color = color;
            pice.0.sprite.custom_size = Some(Vec2::splat(5.));
            pice.0.transform.translation = trans.translation();
            commands.spawn(
                AudioBundle{
                    source: res_sound.posion_up_sound.clone(),
                    settings: PlaybackSettings{
                        speed: res_witch.stronger as f32 / 2. + 1.,
                        mode: PlaybackMode::Despawn,
                        ..Default::default()
                    },
                    ..Default::default()
                }
            );
            for _ in 0..count{
                commands.spawn(
                    pice.clone()
                ).insert(Exploer(rng.gen_range((-16.)..=16.),rng.gen_range((-32.)..=32.)));
            }
        }
    }
    else if res_witch.stronger == 0{
        *get_stronger = 0;
    }
}

fn witch_failed(
    mut commands: Commands,
    mut query_witch: Query<(&mut WitchFailed, &mut TextureAtlasSprite, Entity)>,
    mut res_posion: ResMut<ResourcePosion>,
    mut res_witch: ResMut<ResourceWitch>,
    res_time: Res<Time>,
    res_sound: Res<ResourceAudio>
){
    for (mut timer, mut sprite, entity) in query_witch.iter_mut(){
        if timer.0 == 0. {
            sprite.color = Color::from([0.5,0.5,0.5]);
            res_witch.stuff = None;
            res_posion.lock = false;
            commands.spawn(
                AudioBundle{
                    source: res_sound.witch_failed_sound.clone(),
                    settings: PlaybackSettings{
                        mode: PlaybackMode::Despawn,
                        ..default()
                    },
                    ..default()
                }
            );
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

fn witch_print_posion(
    mut commands: Commands,
    mut len: Local<usize>,
    mut query_posions: Query<(&mut Sprite, Entity), With<TowerPosion>>,
    res_tower: Res<ResourceTower>,
    res_posion: Res<ResourcePosion>
){
    if *len != res_tower.posions.len(){
        let count = res_tower.posions.len();
        let mut temp: usize = 0;
        for (mut sprite, entity) in query_posions.iter_mut(){
            if temp >= count{
                commands.entity(entity).despawn();
            }
            else{
                match res_tower.posions[temp].property {
                    Stuff::Fire => {
                        sprite.color = Color::RED;
                    },
                    Stuff::Water => {
                        sprite.color = Color::BLUE;
                    },
                    Stuff::Poison => {
                        sprite.color = Color::GREEN;
                    },
                    Stuff::Light =>{
                        sprite.color = Color::YELLOW;
                    }
                }
            }
            temp += 1;
        }
        for i in temp..count{
            commands.spawn(
                SpriteBundle{
                    texture: res_posion.posion.clone(),
                    transform: Transform::from_translation(Vec3::new(84. + (10.*i as f32), -25., 40.)),
                    sprite: Sprite{
                        custom_size: Some(Vec2::new(8.,8.)),
                        color:match res_tower.posions[temp].property {
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
                        ..default()
                    },
                    ..default()
                }
            ).insert(TowerPosion);
        }
        *len = res_tower.posions.len();
    }
}

pub struct WitchPlugin;
impl Plugin for WitchPlugin{
    fn build(&self, app: &mut App) {
        app.
            add_systems(Update,(
                spin_rollat,
                witch_failed,
                witch_print_posion,
                rollat_effect
                ));
    }
}
