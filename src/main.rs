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
pub struct Snake {
  direction: u8,
}

#[derive(Component)]
pub struct Movement {
  movement_timer: Timer,
}

fn spawn_snake_part(mut commands: Commands, position: Vec3) {
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
    Snake {
      direction: 0,
    },
    Movement {
      movement_timer: Timer::from_seconds(0.5, TimerMode::Repeating),
    },
    Collision,
    Name::new("Snake"),
  ));
}

fn spawn_snake(mut commands: Commands) {
  spawn_snake_part(commands, Vec3::new(0.0, 0.0, 0.0));
}

fn move_snake(
  mut query: Query<(&mut Transform, &mut Movement, & Snake)>,
  time: Res<Time>,
) {
  for (mut transform, mut movement, snake) in &mut query {
    movement.movement_timer.tick(time.delta());
    if movement.movement_timer.finished() {
      if snake.direction == 0 {
        transform.translation.y += 20.0
      } else if snake.direction == 1 {
        transform.translation.x += 20.0
      } else if snake.direction == 2 {
        transform.translation.y -= 20.0
      } else if snake.direction == 3 {
        transform.translation.x -= 20.0
      }
      movement.movement_timer.reset();
    } 
  }
}

fn change_direction_snake(
  mut query: Query<&mut Snake>,
  input: Res<ButtonInput<KeyCode>>,
) {
  // 0 is up
  // 1 is right
  // 2 is down
  // 3 is left
  for mut snake in &mut query {
    if input.pressed(KeyCode::KeyW) {
      snake.direction = 0;
    } else if input.pressed(KeyCode::KeyS) {
      snake.direction = 2;
    } else if input.pressed(KeyCode::KeyD) {
      snake.direction = 1;
    } else if input.pressed(KeyCode::KeyA) {
      snake.direction = 3;
    }
  }
}

#[derive(Component)]
pub struct Apple;

fn spawn_apple(
  mut commands: Commands,
  query: Query<&Transform>,
  existing_apple: Query<&Apple>
) {
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

fn collision_check(
  mut commands: Commands,
  apple_query: Query<(&Transform, Entity), With<Apple>>,
  snake_query: Query<(&Transform, &Snake)>,
  wall_query: Query<(&Transform, &Wall)>
) {
  let (snake_transform, _) = snake_query.single();
  
  if let Ok((apple_transform, apple_entity)) = apple_query.get_single() {
    if snake_transform.translation.x == apple_transform.translation.x &&
      snake_transform.translation.y == apple_transform.translation.y {
      
      info!("apple entity is {:?}", apple_entity);
      commands.entity(apple_entity).despawn();
      info!("snake ate apple");
      return ();
    }
  }

  for (transform, _) in &wall_query {
    // 0.0 is basically center
    if (transform.translation.x != 0.0 && snake_transform.translation.x == transform.translation.x) ||
      (transform.translation.y != 0.0 && snake_transform.translation.y == transform.translation.y) {
      info!("snake collided with wall");
      return ();
    }
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
    .add_systems(Startup, (setup, spawn_walls, spawn_snake))
    .add_systems(Update, (change_direction_snake, move_snake, collision_check, spawn_apple).chain())
    .run();
}
