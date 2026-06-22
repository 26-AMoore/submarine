use bevy::prelude::*;

pub mod player;

fn main() {
	App::new()
		.add_plugins((DefaultPlugins.set(ImagePlugin::default_nearest()),))
		.add_systems(Startup, (write_text, add_player))
		.add_systems(
			Update,
			(
				player::update_player_sprite,
				(player::handle_player_movment, player::print_position).chain(),
			),
		)
		.run();
}

fn write_text(mut command: Commands) {
	command.spawn(Camera2d);
	command.spawn((
		Text::new("This is Words"),
		Node {
			position_type: PositionType::Absolute,
			top: px(10),
			left: px(10),
			..default()
		},
	));
}

fn add_player(mut command: Commands, asset_server: Res<AssetServer>) {
	let player = player::PlayerBundle::new(
		player::Name(String::from("Alex")),
		player::Position::default(),
		Mesh2d::default(),
		Sprite::from_image(asset_server.load("Player.png")),
	);

	command.spawn(player);
}
