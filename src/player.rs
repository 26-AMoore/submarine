use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

#[derive(Bundle)]
pub struct PlayerBundle {
	pub player: Player,
	pub name: Name,
	pub position: Position,
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
	position: Query<&Position, With<Player>>,
) {
	let position = position.single().unwrap();
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
	mut query: Query<&mut Position, With<Player>>,
) {
	for mut position in &mut query {
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
	}
}
