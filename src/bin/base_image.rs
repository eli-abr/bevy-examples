use bevy::prelude::*;
fn main() {
    let plugin = DefaultPlugins.set(ImagePlugin::default_nearest());
    App::new()
        .add_plugins(plugin)
        .add_systems(Startup, setup_system)
        .run();
}
fn setup_system(mut cmds: Commands) {
    cmds.spawn(Camera2d);
    cmds.spawn_scene(img_node());
}
fn img_node() -> impl Scene {
    bsn! {
        Node {
            width: percent(100.),
            height: percent(100.),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
        }
        Children [
            ImageNode {
                image: "bundle/Ghostpixxells_pixelfood/06_apple_pie_dish.png",
                image_mode: NodeImageMode::Auto,
            }
            UiTransform {
                scale: Vec2::new(2.0, 2.0)
            }
        ]
    }
}
