//! This example is a modified version of the Bevy UI ui.rs example:
//!     https://github.com/bevyengine/bevy/blob/v0.6.0/examples/ui/ui.rs
//! This example uses this `bevy_prototype_css` crate to specify the various `Style`s, `TextStyle`s,
//! and `Color`s using in code CSS (without selectors).  It is otherwise as close a replication of
//! the original as possible.

use bevy::{
    input::mouse::{MouseScrollUnit, MouseWheel},
    prelude::*,
};
use bevy_prototype_css::{CssContext, CssStyle};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(mouse_scroll)
        .run()
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // need a camera
    commands.spawn_bundle(Camera2dBundle::default());

    // a CssContext is needed to correctly parse relative css lengths (e.g. em)
    // you must provide one for this framework
    let css_context = CssContext::default();

    // root node
    let _background = commands
        .spawn_bundle(NodeBundle {
            style: CssStyle("width: 100%; height: 100%; justify-content: space-between;")
                .to_style(&css_context),
            color: CssStyle("color: transparent;").to_ui_color(),
            ..Default::default()
        })
        .with_children(|parent| {
            // left vertical fill (border)
            parent
                .spawn_bundle(NodeBundle {
                    style: CssStyle("width: 200px; height: 100%; border-width: 2px;")
                        .to_style(&css_context),
                    color: CssStyle("color: rgb(65%, 65%, 65%);").to_ui_color(),
                    ..Default::default()
                })
                .with_children(|parent| {
                    // left vertical fill (content)
                    parent
                        .spawn_bundle(NodeBundle {
                            style: CssStyle("width: 100%; height: 100%; align-items: flex-end;")
                                .to_style(&css_context),
                            color: CssStyle("color: rgb(15%, 15%, 15%);").to_ui_color(),
                            ..Default::default()
                        })
                        .with_children(|parent| {
                            // text
                            parent.spawn_bundle(TextBundle {
                                style: CssStyle("margin: 5px").to_style(&css_context),
                                text: Text::from_section(
                                    "Text Example",
                                    TextStyle {
                                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                        font_size: 30.0,
                                        color: Color::WHITE,
                                    },
                                ),
                                ..Default::default()
                            });
                        });
                });
            // right vertical fill
            parent
                .spawn_bundle(NodeBundle {
                    style: CssStyle("width: 200px; height: 100%; flex-direction: column-reverse; justify-content: center;")
                        .to_style(&css_context),
                    color: CssStyle("color: rgb(15%, 15%, 15%);").to_ui_color(),
                    ..Default::default()
                })
                .with_children(|parent| {
                    // Title
                    parent.spawn_bundle(TextBundle {
                        // @fixme should be centered, but isn't.  Maybe requires width: undefined?
                        style: CssStyle("height: 25px; margin-left: auto; margin-right: auto;")
                            .to_style(&css_context),
                        text: Text::from_section(
                            "Scrolling list",
                            TextStyle {
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: 25.,
                                color: Color::WHITE,
                            },
                        ),
                        ..Default::default()
                    });
                    // List with hidden overflow
                    parent
                        .spawn_bundle(NodeBundle {
                            // @fixme should be centered, but isn't.  Maybe requires width: undefined?
                            style: CssStyle("width: 100%; height: 50%; flex-direction: column-reverse; align-self: center; overflow: hidden;")
                                .to_style(&css_context),
                            color: CssStyle("color: rgb(10%, 10%, 10%);").to_ui_color(),
                            ..Default::default()
                        })
                        .with_children(|parent| {
                            // Moving panel
                            parent
                                .spawn_bundle(NodeBundle {
                                    // @fixme `Style` defines `size`, `min_size`, and `max_size` with `Val::Auto`, not `Val::Undefined`
                                    style: CssStyle("flex-direction: column-reverse; flex-grow: 1;")
                                        .to_style(&css_context),
                                    color: CssStyle("color: transparent;").to_ui_color(),
                                    ..Default::default()
                                })
                                .insert(ScrollingList::default())
                                .with_children(|parent| {
                                    // List items
                                    for i in 0..30 {
                                        parent.spawn_bundle(TextBundle {
                                            style: CssStyle("height: 20px; flex-shrink: 0; margin-left: auto; margin-right: auto;")
                                                .to_style(&css_context),
                                            text: Text::from_section(
                                                format!("Item {}", i),
                                                TextStyle {
                                                    font: asset_server
                                                        .load("fonts/FiraSans-Bold.ttf"),
                                                    font_size: 20.,
                                                    color: Color::WHITE,
                                                },
                                            ),
                                            ..Default::default()
                                        });
                                    }
                                });
                        });
                });
            // absolute positioning
            parent
                .spawn_bundle(NodeBundle {
                    style: CssStyle("width: 200px; height: 200px; position: absolute; left: 210px; bottom: 10px; border-width: 20px;")
                        .to_style(&css_context),
                    color: CssStyle("color: rgb(40%, 40%, 100%);").to_ui_color(),
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent.spawn_bundle(NodeBundle {
                        style: CssStyle("width: 100%; height: 100%;").to_style(&css_context),
                        color: CssStyle("color: rgb(80%, 80%, 100%);").to_ui_color(),
                        ..Default::default()
                    });
                });
            // render order test: reddest in the back, whitest in the front (flex center)
            parent
                .spawn_bundle(NodeBundle {
                    style: CssStyle("width: 100%; height: 100%; position: absolute; align-items: center; justify-content: center;")
                        .to_style(&css_context),
                    color: CssStyle("color: transparent;").to_ui_color(),
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent
                        .spawn_bundle(NodeBundle {
                            style: CssStyle("width: 100px; height: 100px;").to_style(&css_context),
                            color: CssStyle("color: red;").to_ui_color(),
                            ..Default::default()
                        })
                        .with_children(|parent| {
                            parent.spawn_bundle(NodeBundle {
                                style: CssStyle("width: 100px; height: 100px; position: absolute; left: 20px; bottom: 20px;")
                                    .to_style(&css_context),
                                color: CssStyle("color: rgb(100%, 30%, 30%);").to_ui_color(),
                                ..Default::default()
                            });
                            parent.spawn_bundle(NodeBundle {
                                style: CssStyle("width: 100px; height: 100px; position: absolute; left: 40px; bottom: 40px;")
                                    .to_style(&css_context),
                                color: CssStyle("color: rgb(100%, 50%, 50%);").to_ui_color(),
                                ..Default::default()
                            });
                            parent.spawn_bundle(NodeBundle {
                                style: CssStyle("width: 100px; height: 100px; position: absolute; left: 60px; bottom: 60px;")
                                    .to_style(&css_context),
                                color: CssStyle("color: rgb(100%, 70%, 70%);").to_ui_color(),
                                ..Default::default()
                            });
                            // alpha test
                            parent.spawn_bundle(NodeBundle {
                                style: CssStyle("width: 100px; height: 100px; position: absolute; left: 80px; bottom: 80px;")
                                    .to_style(&css_context),
                                color: CssStyle("color: rgba(100%, 90%, 90%, 0.4);").to_ui_color(),
                                ..Default::default()
                            });
                        });
                });
            // bevy logo (flex center)
            parent
                .spawn_bundle(NodeBundle {
                    style: CssStyle("width: 100%; height: 100%; position: absolute; justify-content: center; align-items: flex-end")
                        .to_style(&css_context),
                    color: CssStyle("color: transparent;").to_ui_color(),
                    ..Default::default()
                })
                .with_children(|parent| {
                    // bevy logo (image)
                    parent.spawn_bundle(ImageBundle {
                        style: CssStyle("width: 500px; height: auto;").to_style(&css_context),
                        image: asset_server.load("branding/bevy_logo_dark_big.png").into(),
                        ..Default::default()
                    });
                });
        });
}

#[derive(Component, Default)]
struct ScrollingList {
    position: f32,
}

fn mouse_scroll(
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mut query_list: Query<(&mut ScrollingList, &mut Style, &Children, &Node)>,
    query_item: Query<&Node>,
) {
    for mouse_wheel_event in mouse_wheel_events.iter() {
        for (mut scrolling_list, mut style, children, uinode) in query_list.iter_mut() {
            let items_height: f32 = children
                .iter()
                .map(|entity| query_item.get(*entity).unwrap().size.y)
                .sum();
            let panel_height = uinode.size.y;
            let max_scroll = (items_height - panel_height).max(0.);
            let dy = match mouse_wheel_event.unit {
                MouseScrollUnit::Line => mouse_wheel_event.y * 20.,
                MouseScrollUnit::Pixel => mouse_wheel_event.y,
            };
            scrolling_list.position += dy;
            scrolling_list.position = scrolling_list.position.clamp(-max_scroll, 0.);
            style.position.top = Val::Px(scrolling_list.position);
        }
    }
}