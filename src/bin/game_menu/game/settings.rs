use bevy::{prelude::*, text::FontSourceTemplate, ui_widgets::{Activate, Button}};
use super::GameState;

pub fn settings_plugin(app: &mut App) {
    app.add_systems(OnEnter(super::GameState::Settings), ui_node.spawn());
}

fn ui_node() -> impl Scene {
    bsn! {
        #Root
        DespawnOnExit<GameState>(GameState::Settings)
        Node {

        }
        Children [
            #ReturnButton
            @text_button_node("返回")
            UiTransform {
                translation: Val2::px(20., 20.)
            }
            on(|_: On<Activate>, mut state: ResMut<NextState<GameState>>| {
                state.set(GameState::MainMenu);
            })
        ]
    }
}

fn text_button_node(label: &str) -> impl Scene {
    let text = label.to_string();
    bsn! {
        Button
        Node {
            padding: UiRect::all(px(12.))
        }
        BackgroundColor(Color::NONE)
        Children [
            Text({ text })
            TextFont {
                font_size: FontSize::Px(28.),
                font: FontSourceTemplate::Handle("fonts/wqy-zenhei.ttc"),
            }
            TextColor(Color::srgb(0.9, 0.7, 0.2))
        ]
    }
}


