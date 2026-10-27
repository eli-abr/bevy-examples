use bevy::prelude::*;
fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_plugins(game::GamePlugin)
    .run();
}

mod game {
    use bevy::{
        input::InputSystems,
        prelude::*,
    };
    pub struct GamePlugin;
    impl Plugin for GamePlugin {
        fn build(&self, app: &mut App) {
            app
            .add_systems(Startup, setup_system)
            .add_systems(PreUpdate, quit_input_system.after(InputSystems))
            .add_systems(PreUpdate, move_input_system.after(InputSystems))
            .add_systems(Update, move_system)
            ;
        }
    }
    #[derive(Component)]
    #[require(MoveInput, Transform)]
    struct MoveControllerTag;
    #[derive(Component)]
    struct MoveInput {
        dir: Vec2,
        speed: f32
    }
    impl Default for MoveInput {
        fn default() -> Self {
            Self {
                dir: Vec2::ZERO,
                speed: 200.
            }
        }
    }
    fn setup_system(
        mut cmds: Commands,
        asset: Res<AssetServer>,
    ) {
        cmds.spawn(Camera2d);
        cmds.spawn((
            Sprite::from_image(asset.load("images/ball.png")),
            Transform::from_xyz(0., 0., 0.).with_scale(Vec3::new(0.5, 0.5, 1.0)),
            MoveControllerTag
        ));
    }
    fn move_input_system(
        kb: Res<ButtonInput<KeyCode>>,
        mut query: Query<&mut MoveInput, With<MoveControllerTag>>
    ) {
        for mut input in &mut query {
            input.dir = Vec2::ZERO;
            if kb.pressed(KeyCode::KeyW) {
                input.dir.y += 1.;
            }
            if kb.pressed(KeyCode::KeyS) {
                input.dir.y -= 1.;
            }
            if kb.pressed(KeyCode::KeyA) {
                input.dir.x -= 1.;
            }
            if kb.pressed(KeyCode::KeyD) {
                input.dir.x += 1.;
            }
            let _ = input.dir.normalize();
        }
    }
    fn move_system(
        time: Res<Time>,
        mut query: Query<(&mut Transform, &MoveInput)>
    ) {
        let dt = time.delta_secs();
        for (mut transform, input) in &mut query {
            transform.translation += (input.dir * input.speed * dt).extend(0.);
        }
    }
    fn quit_input_system(
        kb: Res<ButtonInput<KeyCode>>,
        mut exit: MessageWriter<AppExit>,
    ) {
        if kb.just_released(KeyCode::Escape) {
            exit.write(AppExit::Success);
        }
    }
}