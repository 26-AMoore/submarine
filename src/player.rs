use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

#[derive(Bundle)]
pub struct PlayerBundle {
	pub player: Player,
	pub name: Name,
	pub position: Position,
	pub transform: Transform,
	pub mesh: Mesh2d,
	pub sprite: Sprite,
}

impl Default for PlayerBundle {
	fn default() -> Self {
		Self {
			player: Player,
			name: Name(String::from("Alex")),
			position: Default::default(),
			mesh: Default::default(),
			sprite: Sprite::default(),
			transform: Transform::default(),
		}
	}
}

impl PlayerBundle {
	pub fn new(name: Name, position: Position, mesh: Mesh2d, sprite: Sprite) -> Self {
		Self {
			player: Player,
			name,
			position,
			mesh,
			sprite: sprite,
			transform: Transform::default(),
		}
	}
}

#[derive(Component)]
pub struct Name(pub String);

#[derive(Component)]
pub struct Position {
	pub x: f32,
	pub y: f32,
}

impl Default for Position {
	fn default() -> Self {
		Self { x: 0.0, y: 0.0 }
	}
}

pub fn update_player_sprite(
	mut command: Commands,
	sprite: Query<&mut Sprite, With<Player>>,
	transform: Query<&Transform, With<Player>>,
) {
	let transform = transform.single().unwrap();
}

pub fn print_position(query: Query<&Position, With<Player>>) {
	println!(
		"pos x: {}, y: {}",
		query.single().unwrap().x,
		query.single().unwrap().y
	)
}

pub fn handle_player_movment(
	keyboard_input: Res<ButtonInput<KeyCode>>,
	mut position: Query<&mut Position, With<Player>>,
	mut transform: Query<&mut Transform, With<Player>>,
) {
	let position: &mut Position = position.single_mut().unwrap().into_inner();

	if keyboard_input.pressed(KeyCode::KeyW) {
		position.y = position.y + 1.0;
	}
	if keyboard_input.pressed(KeyCode::KeyA) {
		position.x = position.x - 1.0;
	}
	if keyboard_input.pressed(KeyCode::KeyS) {
		position.y = position.y - 1.0;
	}
	if keyboard_input.pressed(KeyCode::KeyD) {
		position.x = position.x + 1.0;
	}

	let transform = transform.single_mut().unwrap().into_inner();

	*transform = transform.with_translation(Vec3 {
		x: position.x,
		y: position.y,
		z: 1.0,
	});
}
