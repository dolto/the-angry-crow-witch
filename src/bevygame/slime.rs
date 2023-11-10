use bevy::{prelude::*, input::keyboard::KeyboardInput};
use rand::Rng;

use super::setup_res::{Slime, Tower, ResourceImage, Stuff, Score, AppSet, CloneBird, Bird};

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
                tower.2 += 1;
                commands.entity(entity).despawn();
            }
        }
    }
}

fn tower_hit(
    mut query_tower: Query<(&mut Tower, &mut Sprite)>,
    res_time: Res<Time>
){
    let (mut tower, mut sprite) = query_tower.get_single_mut().unwrap();
    
    if tower.2 > 0{
        let delta = res_time.delta();
        tower.1.tick(delta);
        if tower.1.finished(){
            if sprite.color == Color::WHITE{
                sprite.color = Color::from([2.,2.,2.]);
            }else{
                sprite.color = Color::WHITE;
                tower.2 -= 1;
                tower.0 -= 1;
            }
        }
    }
}

fn slime_manger(
    mut commands: Commands,
    mut query_text: Query<&mut Text, With<Score>>,
    res_image: Res<ResourceImage>,
    mut level_time: Local<f32>,
    mut level: Local<i32>,
    res_time: Res<Time>
){
    let delta_sec = res_time.delta_seconds();
    *level_time += delta_sec;

    if *level_time > 3. {
        let lv = ((*level/5).max(1) as f32).max(3.);
        *level_time = 0.;
        *level += 1;
        
        let range = 1..((*level/5).max(2)).min(5);
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

            
            let min = (-5.)..(-2.5);
            let max = (5.)..(10.);
            let time = (0.7)..(1.5);
            // println!("이건 min 범위입니다. {:?}", min);
            // println!("이건 max 범위입니다. {:?}", max);
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

fn game_over(
    mut next_state: ResMut<NextState<AppSet>>,
    mut commands: Commands,
    query_reset: Query<Entity, With<Slime>>,
    query_bird: Query<Entity, With<CloneBird>>,
    mut query_bird_set: Query<&mut Bird, (With<Bird>, Without<CloneBird>)>,
    mut query_tower: Query<&mut Tower>,
    mut time: Local<f32>,
    res_time: Res<Time>,
    mut event_reader: EventReader<KeyboardInput>
){
    let mut tower = query_tower.get_single_mut().unwrap();
    if tower.0 < 0{
        let _time = res_time.delta_seconds();
        *time += _time;

        for _ in event_reader.read(){
            if *time > 0.5{
                tower.0 = 10;
                next_state.set(AppSet::Reset);
                for entity in query_reset.iter(){
                    commands.entity(entity).despawn();
                }
                for entity in query_bird.iter(){
                    commands.entity(entity).despawn();
                }
                let mut brid = query_bird_set.get_single_mut().unwrap();
                brid.poops.clear();
            }
        }
        
        commands.spawn(
            NodeBundle{
                style: Style{
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    flex_direction: FlexDirection::Column,
                    ..Default::default()
                },
                ..default()
            }
        ).with_children(|p|{
            p.spawn(
                TextBundle{
                    text: Text::from_section(
                        "Game Over", 
                        TextStyle { font_size: 64., ..default() }
                    ),
                    ..default()
                }
            );
            p.spawn(
                TextBundle{
                    text: Text::from_section(
                        "any key is restart", 
                        TextStyle { font_size: 12., ..default() }
                    ),
                    ..default()
                }
            );
        });
    }
}

pub struct SlimePlugin;

impl Plugin for SlimePlugin {
    fn build(&self, app: &mut App) {
        app.
        add_systems(Update, (
            slime_move,
            slime_manger,
            tower_hit,
            //game_over
        ).run_if(in_state(AppSet::Set)));
    }
}