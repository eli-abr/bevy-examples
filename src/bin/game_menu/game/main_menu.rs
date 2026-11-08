use bevy::{
    prelude::*,
    text::{FontSourceTemplate, TextPlugin},
    ui_widgets::{Activate, Button},
};
use super::GameState;
pub fn main_menu_plugin(app: &mut App) {
    app.add_systems(OnEnter(super::GameState::MainMenu), ui_node.spawn());
}

#[derive(Component)]
struct MainMenuRoot;

#[derive(Component)]
struct MenuButton;

#[derive(Component, Clone, Copy)]
enum ButtonKind {
    Start,
    Settings,
    Quit,
}

#[derive(Event)]
enum ButtonAction {
    Start,
    Settings,
    Quit,
}

/// 布局:
/// root（100%×100% 竖排容器）
/// ├── spacer（上方留白）             ← Node，height 撑开
/// ├── 标题 Text                      ← 大字号，居中
/// ├── spacer
/// ├── 按钮列（竖排 flex）            ← Node
/// │   ├── "开始游戏" 按钮
/// │   ├── "设置" 按钮
/// │   └── "退出" 按钮
/// └── spacer（下方留白）
fn ui_node() -> impl Scene {
    bsn! {
        #Root
        DespawnOnExit<GameState>(GameState::MainMenu)
        Node {
            height: Val::Percent(100.),
            width: Val::Percent(100.),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            row_gap: Val::Px(36.),
        }
        Children [
            #Title
            Node {
                height: Val::Px(500.),
                width: Val::Px(500.),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
            }
            Children [
                Text("主菜单")
                TextFont {
                    font_size: FontSize::Px(36.),
                    font: FontSourceTemplate::Handle("fonts/wqy-zenhei.ttc"),
                }
                TextColor(Color::srgb(0.9, 0.5, 0.0))
            ]
            #ButtonList
            @panel_node()
        ]
    }
}
fn panel_node() -> impl Scene {
    bsn! {
        Node {
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            row_gap: Val::Px(12.)
        }
        Children [
            #StartButton
            @button_node("开始游戏")
            on(|_: On<Activate>| {
            })
            --
            #SettingsButton
            @button_node("设置")
            on(|_: On<Activate>, mut state: ResMut<NextState<GameState>>| {
                state.set(GameState::Settings);
            })
            --
            #QuitButton
            @button_node("退出")
            on(|_: On<Activate>, mut exit: MessageWriter<AppExit>| {
                exit.write(AppExit::Success);
            })
        ]
    }
}
fn button_node(label: &str) -> impl Scene {
    bsn! {
        Button
        Node {
            height: Val::Px(100.),
            width: Val::Px(200.),
            flex_direction: FlexDirection::Row,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
        }
        BackgroundColor(Color::srgb(0.9, 0.7, 0.3))
        Children [
            Text({ label })
            TextFont {
                font_size: FontSize::Px(24.),
                font: FontSourceTemplate::Handle("fonts/wqy-zenhei.ttc"),
            }
            TextColor(Color::BLACK)
        ]
    }
}
