use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_inspector_egui::InspectorOptions;
use bevy_inspector_egui::prelude::ReflectInspectorOptions;
use bevy::input::common_conditions::input_toggle_active;
use rand::Rng;

fn setup(mut commands: Commands) {
  let mut camera = Camera2dBundle {
    camera: Camera {
      clear_color: ClearColorConfig::Custom(Color::srgba_u8(148, 188, 100, 255)),
      ..default()
    },
    ..default()
  };

  camera.projection.scaling_mode = ScalingMode::AutoMin {
    min_width: 1280.0,
    min_height: 720.0,
  };

  commands.spawn(camera);
}

#[derive(Component)]
pub struct Collision;

#[derive(Component, InspectorOptions, Default, Reflect)]
#[reflect(Component, InspectorOptions)]
pub struct Wall;

fn spawn_walls(mut commands: Commands, ) {
  commands.spawn((
    SpriteBundle {
      transform: Transform::from_translation(Vec3::new(-640.0, 0.0, 0.0)),
      sprite: Sprite {
        custom_size: Some(Vec2::new(20.0, 720.0)),
        ..default()
      },
      ..default()
    },
    Wall,
    Collision,
    Name::new("Left Wall"),
  ));

  commands.spawn((
    SpriteBundle {
      transform: Transform::from_translation(Vec3::new(0.0, -360.0, 0.0)),
      sprite: Sprite {
        custom_size: Some(Vec2::new(1280.0, 20.0)),
        ..default()
      },
      ..default()
    },
    Wall,
    Collision,
    Name::new("Bottom Wall"),
  ));


  commands.spawn((
    SpriteBundle {
      transform: Transform::from_translation(Vec3::new(640.0, 0.0, 0.0)),
      sprite: Sprite {
        custom_size: Some(Vec2::new(20.0, 720.0)),
        ..default()
      },
      ..default()
    },
    Wall,
    Collision,
    Name::new("Right Wall"),
  ));

  commands.spawn((
    SpriteBundle {
      transform: Transform::from_translation(Vec3::new(0.0, 360.0, 0.0)),
      sprite: Sprite {
        custom_size: Some(Vec2::new(1280.0, 20.0)),
        ..default()
      },
      ..default()
    },
    Wall,
    Collision,
    Name::new("Top Wall"),
  ));
}

#[derive(Component, InspectorOptions, Default, Reflect)]
#[reflect(Component, InspectorOptions)]
pub struct SnakeHead {
  direction: u8,
}

#[derive(Component, InspectorOptions, Default, Reflect)]
#[reflect(Component, InspectorOptions)]
pub struct SnakeSegment;

#[derive(Resource)]
pub struct Movement {
  internal_counter: u8,
}

fn spawn_snake_head(
  mut commands: Commands,
  mut state: ResMut<GameState>,
) {
  if (*state).state == State::Playing {
    if (*state).snake_head_spawned {
      return ();
    } else {
      commands.spawn((
        SpriteBundle {
          transform: Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)),
          sprite: Sprite {
            // color: Color::srgba_u8(20, 33, 18, 255),
            color: Color::srgba_u8(231, 161, 176 , 255), // piglet version
            custom_size: Some(Vec2::new(20.0, 20.0)),
            ..default()
          },
          ..default()
        },
        SnakeHead {
          direction: 0,
        },
        Collision,
        Name::new("Snake Head"),
      ));
      (*state).snake_head_spawned = true;
    }
  }
}

fn spawn_snake_segment(mut commands: Commands, position: Vec3) {
  commands.spawn((
    SpriteBundle {
      transform: Transform::from_translation(position),
      sprite: Sprite {
        // color: Color::srgba_u8(20, 33, 18, 255),
        color: Color::srgba_u8(231, 161, 176 , 255), // piglet version
        custom_size: Some(Vec2::new(20.0, 20.0)),
        ..default()
      },
      ..default()
    },
    SnakeSegment,
    Collision,
    Name::new("Snake Segment"),
  ));
}

fn move_snake(
  state: Res<GameState>,
  mut movement: ResMut<Movement>,
  mut snake_head_query: Query<(&mut Transform, &SnakeHead)>,
  mut snake_segment_query: Query<(&mut Transform, &SnakeSegment), Without<SnakeHead>>,
) {
  
  if state.state == State::Playing {
    if (*movement).internal_counter == 20 {
      let (mut snake_head_transform, snake) = snake_head_query.single_mut();
      let mut prev_location = (snake_head_transform.translation.x, snake_head_transform.translation.y, snake_head_transform.translation.z);
    
      if snake.direction == 0 {
        snake_head_transform.translation.y += 20.0
      } else if snake.direction == 1 {
        snake_head_transform.translation.x += 20.0
      } else if snake.direction == 2 {
        snake_head_transform.translation.y -= 20.0
      } else if snake.direction == 3 {
        snake_head_transform.translation.x -= 20.0
      }

      for (mut snake_segment_transform, _) in &mut snake_segment_query {
        let current_location = (snake_segment_transform.translation.x, snake_segment_transform.translation.y, snake_segment_transform.translation.z); 
        snake_segment_transform.translation.x = prev_location.0;
        snake_segment_transform.translation.y = prev_location.1;
        snake_segment_transform.translation.z = prev_location.2;
        prev_location = current_location;
      }
      (*movement).internal_counter = 0;
    }
    (*movement).internal_counter += 1;
  }  
}

fn change_direction_snake(
  state: Res<GameState>,
  mut query: Query<&mut SnakeHead>,
  input: Res<ButtonInput<KeyCode>>,
) {
  // 0 is up
  // 1 is right
  // 2 is down
  // 3 is left
  if state.state == State::Playing {
    for mut snake in &mut query {
      if input.pressed(KeyCode::KeyW) && snake.direction != 2 {
        snake.direction = 0;
      } else if input.pressed(KeyCode::KeyS) && snake.direction != 0 {
        snake.direction = 2;
      } else if input.pressed(KeyCode::KeyD) && snake.direction != 3 {
        snake.direction = 1;
      } else if input.pressed(KeyCode::KeyA) && snake.direction != 1 {
        snake.direction = 3;
      }
    }
  }
}

#[derive(Component)]
pub struct Apple;

fn spawn_apple(
  state: Res<GameState>,
  mut commands: Commands,
  query: Query<&Transform>,
  existing_apple: Query<&Apple>
) {
  if state.state == State::Playing {
    match existing_apple.get_single() {
      Err(_) => {
        let apple_location = random_apple_location(query);
        info!("apple location is {:?}", apple_location);
        commands.spawn((
          SpriteBundle {
            transform: Transform::from_translation(apple_location),
            sprite: Sprite {
              // color: Color::srgba_u8(20, 33, 18, 255),
              color: Color::srgba_u8(221, 21, 51 , 255), // piglet version
              custom_size: Some(Vec2::new(15.0, 15.0)),
              ..default()
            },
            ..default()
          },
          Apple,
          Collision,
          Name::new("Source of Discorc"),
        ));
      },
      Ok(_) => {
        return;
      }
    }
  }
}

fn random_apple_location(query: Query<&Transform>) -> Vec3 {
  let mut x: f32;
  let mut y: f32;
  let mut z: f32;
  let mut flag;
  loop {
    x = rand::thread_rng().gen_range(-31..31) as f32 * 20.0;
    y = rand::thread_rng().gen_range(-17..17) as f32 * 20.0;
    z = 0.0;
    flag = true;
    for transform in &query {
      flag = flag && (transform.translation.x != x || transform.translation.y != y);
    }
    if flag {
      break;
    }
  }
  return Vec3::new(x, y, z);
}

#[derive(Resource)]
pub struct GameState {
  state: State,
  snake_head_spawned: bool,
}

#[derive(PartialEq, Debug)]
enum State {
  Playing,
  GameOver,
}

fn collision_check(
  mut commands: Commands,
  mut game_state: ResMut<GameState>,
  movement: Res<Movement>,
  apple_query: Query<(&Transform, Entity), With<Apple>>,
  snake_query: Query<(&Transform, Entity, &SnakeHead)>,
  snake_segment_query: Query<(&Transform, Entity, &SnakeSegment)>,
  wall_query: Query<(&Transform, &Wall)>
) {
  if (*game_state).state == State::Playing && movement.internal_counter == 20 {
    let (snake_transform, snake_head_entity, _) = snake_query.single();

    for (transform, _) in &wall_query {
      // 0.0 is basically center
      if (transform.translation.x != 0.0 && snake_transform.translation.x == transform.translation.x) ||
        (transform.translation.y != 0.0 && snake_transform.translation.y == transform.translation.y) {
        (*game_state).state = State::GameOver;

        commands.entity(snake_head_entity).despawn();
        if let Ok((_, apple_entity)) = apple_query.get_single() {
          commands.entity(apple_entity).despawn();
        }
        for (_, snake_segment_entity, _) in &snake_segment_query {
          commands.entity(snake_segment_entity).despawn();
        }
        (*game_state).snake_head_spawned = false;

        return ();
      }
    }

    for (transform, snake_segment_entity, _) in &snake_segment_query {
      if transform.translation.x == snake_transform.translation.x && transform.translation.y == snake_transform.translation.y {
        (*game_state).state = State::GameOver;
      }
    }
    if (*game_state).state == State::GameOver {
      commands.entity(snake_head_entity).despawn();
      if let Ok((_, apple_entity)) = apple_query.get_single() {
        commands.entity(apple_entity).despawn();
      }
      for (_, snake_segment_entity, _) in &snake_segment_query {
        commands.entity(snake_segment_entity).despawn();
      }
      (*game_state).snake_head_spawned = false;
    }
  
    if let Ok((apple_transform, apple_entity)) = apple_query.get_single() {
      if snake_transform.translation.x == apple_transform.translation.x &&
        snake_transform.translation.y == apple_transform.translation.y {
        
        commands.entity(apple_entity).despawn();

        spawn_snake_segment(commands, Vec3::new(snake_transform.translation.x, snake_transform.translation.y, snake_transform.translation.z));
        return ();
      }
    }
  }
}

#[derive(Component)]
pub struct EndGameScreen;

fn end_game_screen(
  mut commands: Commands,
  mut game_state: ResMut<GameState>,
  input: Res<ButtonInput<KeyCode>>,
  query: Query<(Entity, &EndGameScreen)>,
) {
  if (*game_state).state == State::GameOver {
    commands
        .spawn((
          NodeBundle {
            style: Style {
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },

            ..default()
          },
        ))
        .with_children(|commands| {
            commands.spawn((
              TextBundle {
                text: Text::from_section(
                    "Press R to restart.",
                    TextStyle {
                        font_size: 32.0,
                        color: Color::BLACK,
                        ..default()
                    },
                ),
                ..default()
              },
              EndGameScreen,
            ));
        });
    
      if input.pressed(KeyCode::KeyR) {
        (*game_state).state = State::Playing;
      }
  } else {
    for (entity, _) in &query {
      commands.entity(entity).despawn();
    }
  }
}

#[derive(Component)]
pub struct PigLegs;

fn spawn_pig_legs(
  mut commands: Commands,
  snake_head_query: Query<(&Transform, &SnakeHead)>,
  snake_segment_query: Query<&Transform, With<SnakeSegment>>,
  legs_query: Query<(Entity, &PigLegs)>,
) {
  for (entity, _) in &legs_query {
    commands.entity(entity).despawn();
  }
  if let Ok((snake_head_transform, snake_head)) = snake_head_query.get_single() {
    let custom_size = Vec2::new(5.0, 30.0);
    commands.spawn((
      SpriteBundle {
        transform: Transform::from_translation(Vec3::new(snake_head_transform.translation.x, snake_head_transform.translation.y, snake_head_transform.translation.z + 1.0)),
        sprite: Sprite {
          // color: Color::srgba_u8(20, 33, 18, 255),
          color: Color::srgba_u8(231, 161, 176 , 255), // piglet version
          custom_size: Some(custom_size),
          ..default()
        },
        ..default()
      },
      PigLegs,
      Name::new("Snake Legs"),
    ));
  }
}

fn main() {
  App::new()
    .add_plugins(DefaultPlugins
      .set(ImagePlugin::default_nearest())
      .set(WindowPlugin {
        primary_window: Some(Window {
          title: "Simple Bevy Snake".into(),
          resolution: (1280.0, 720.0).into(),
          resizable: false,
          ..default()
        }),
        ..default()
      })
      .build()
    )
    .add_plugins(
      WorldInspectorPlugin::default().run_if(input_toggle_active(true, KeyCode::Escape))
    )
    .insert_resource(GameState { state: State::Playing, snake_head_spawned: false })
    .insert_resource(Movement {internal_counter: 0})
    .add_systems(Startup, (setup, spawn_walls))
    .add_systems(Update, (spawn_snake_head, spawn_pig_legs, change_direction_snake, move_snake, collision_check, spawn_apple, end_game_screen).chain())
    .run();
}
