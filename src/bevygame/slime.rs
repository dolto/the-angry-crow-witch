use bevy::prelude::*;
use rand::Rng;

use super::setup_res::{Slime, Tower, ResourceImage, Stuff};

fn slime_move(
    mut commands: Commands,
    mut query_slime: Query<(&mut Transform, &mut Slime, Entity)>,
    mut query_tower: Query<&mut Tower>,
    res_time: Res<Time>
){
    let delta = res_time.delta();
    let mut tower = query_tower.get_single_mut().unwrap();
    for (mut slime_trans, mut slime, entity) in query_slime.iter_mut(){
        slime.move_time.tick(delta);
        if slime.move_time.finished(){
            let mut rng = rand::thread_rng();
            slime_trans.translation += Vec3::X * rng.gen_range(slime.move_min..slime.move_max);

            if slime_trans.translation.x > 100.{
                println!("슬라임 충돌!");
                tower.2 += 2;
                commands.entity(entity).despawn();
            }
        }
    }
}

fn slime_manger(
    mut commands: Commands,
    mut query_text: Query<&mut Text>,
    res_image: Res<ResourceImage>,
    mut level_time: Local<f32>,
    mut level: Local<i32>,
    res_time: Res<Time>
){
    let delta_sec = res_time.delta_seconds();
    *level_time += delta_sec;

    if *level_time > 2. {
        *level_time = 0.;
        *level += 1;
        
        let range = 1..(*level+1).max(3);
        println!("이건 count 범위입니다. {:?}", range);
        let mut rng = rand::thread_rng();
        let count = rng.gen_range(range);

        let mut text = query_text.get_single_mut().unwrap();
        text.sections[0].value = format!("Score: {}", *level);

        for _ in 0..count{
            let mut slime = res_image.slime.clone();
            let stuff_index = rng.gen_range(0..4);
            let stuff = match stuff_index{
                0 => {
                    slime.0.sprite.color = Color::RED;
                    Stuff::Fire
                },
                1 => {
                    slime.0.sprite.color = Color::BLUE;
                    Stuff::Water
                },
                2 => {
                    slime.0.sprite.color = Color::YELLOW;
                    Stuff::Light
                },
                3 => {
                    slime.0.sprite.color = Color::GREEN;
                    Stuff::Poison
                },
                _ => {Stuff::Fire}
            };

            let lv = (*level as f32).max(3.);
            let min = (-5.)..(-2.5);
            let max = (5.)..(10.);
            let time = (0.7 / lv)..(1.5 / lv);
            println!("이건 min 범위입니다. {:?}", min);
            println!("이건 max 범위입니다. {:?}", max);
            commands.spawn(
                    slime
            ).insert(
                Slime{
                    move_max: rng.gen_range(max),
                    move_min: rng.gen_range(min),
                    move_time: Timer::from_seconds(rng.gen_range(time), TimerMode::Repeating),
                    property: stuff
                }
            );
        }
    }
}

pub struct SlimePlugin;

impl Plugin for SlimePlugin {
    fn build(&self, app: &mut App) {
        app.
        add_systems(Update, (
            slime_move,
            slime_manger,
        ));
    }
}