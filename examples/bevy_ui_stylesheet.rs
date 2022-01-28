//! This example is a modified version of the Bevy UI ui.rs example:
//!     https://github.com/bevyengine/bevy/blob/v0.6.0/examples/ui/ui.rs
//! This example uses this `bevy_prototype_css` crate to specify the various `Style`s, `TextStyle`s,
//! and `Color`s using a CSS stylesheet loaded as an asset.  It is otherwise as close a replication
//! of the original as possible.

use bevy::{
    input::mouse::{MouseScrollUnit, MouseWheel},
    prelude::*,
};
use bevy_prototype_css::{CssClass, CssId, CssPlugin, CssStylesheet};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // Adds the `Stylesheet` asset (with loader for `.css` files), and relevant systems
        .add_plugin(CssPlugin)
        .add_startup_system(setup)
        .add_system(mouse_scroll)
        .run()
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    // ui camera
    commands.spawn_bundle(UiCameraBundle::default());

    // load .css file
    let _css_handle: Handle<CssStylesheet> = asset_server.load("styles/bevy_ui.css");

    // root node
    let background = commands
        .spawn_bundle(NodeBundle {
            color: Color::NONE.into(),
            ..Default::default()
        })
        .insert_bundle((
            CssId::from("background-container"), CssClass::from("fill-container"),
        ))
        .with_children(|parent| {
            // left vertical fill (border)
            parent
                .spawn_bundle(NodeBundle {
                    color: Color::rgb(0.65, 0.65, 0.65).into(),
                    ..Default::default()
                })
                .insert(CssClass::from("sidebar border-2px"))
                .with_children(|parent| {
                    // left vertical fill (content)
                    parent
                        .spawn_bundle(NodeBundle {
                            color: Color::rgb(0.15, 0.15, 0.15).into(),
                            ..Default::default()
                        })
                        .insert_bundle((
                            CssId::from("left-bar"), CssClass::from("fill-container"),
                        ))
                        .with_children(|parent| {
                            // text
                            parent
                                .spawn_bundle(TextBundle {
                                    text: Text::with_section(
                                        "Text Example",
                                        TextStyle {
                                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                            font_size: 30.0,
                                            color: Color::WHITE,
                                        },
                                        Default::default(),
                                    ),
                                    ..Default::default()
                                })
                                .insert(CssClass::from("margin-5px"));
                        });
                });
            // right vertical fill
            parent
                .spawn_bundle(NodeBundle {
                    color: Color::rgb(0.15, 0.15, 0.15).into(),
                    ..Default::default()
                })
                .insert_bundle((
                    CssId::from("right-bar"), CssClass::from("sidebar"),
                ))
                .with_children(|parent| {
                    // Title
                    parent
                        .spawn_bundle(TextBundle {
                            text: Text::with_section(
                                "Scrolling list",
                                TextStyle {
                                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                    font_size: 25.,
                                    color: Color::WHITE,
                                },
                                Default::default(),
                            ),
                            ..Default::default()
                        })
                        .insert_bundle((
                            CssId::from("scroll-title"), CssClass::from("margin-center")
                        ));
                    // List with hidden overflow
                    parent
                        .spawn_bundle(NodeBundle {
                            color: Color::rgb(0.10, 0.10, 0.10).into(),
                            ..Default::default()
                        })
                        .insert(CssId::from("scroll-list"))
                        .with_children(|parent| {
                            // Moving panel
                            parent
                                .spawn_bundle(NodeBundle {
                                    color: Color::NONE.into(),
                                    ..Default::default()
                                })
                                .insert(CssClass::from("scroller panel"))
                                .insert(ScrollingList::default())
                                .with_children(|parent| {
                                    // List items
                                    for i in 0..30 {
                                        parent
                                            .spawn_bundle(TextBundle {
                                                text: Text::with_section(
                                                    format!("Item {}", i),
                                                    TextStyle {
                                                        font: asset_server
                                                            .load("fonts/FiraSans-Bold.ttf"),
                                                        font_size: 20.,
                                                        color: Color::WHITE,
                                                    },
                                                    Default::default(),
                                                ),
                                                ..Default::default()
                                            })
                                            .insert(CssClass::from("scroller item margin-center"));
                                    }
                                });
                        });
                });
            // absolute positioning
            parent
                .spawn_bundle(NodeBundle {
                    color: Color::rgb(0.4, 0.4, 1.0).into(),
                    ..Default::default()
                })
                .insert_bundle((
                    CssId::from("square-bottom-left"),
                    CssClass::from("absolute square-200px border-20px"),
                ))
                .with_children(|parent| {
                    parent
                        .spawn_bundle(NodeBundle {
                            color: Color::rgb(0.8, 0.8, 1.0).into(),
                            ..Default::default()
                        })
                        .insert(CssClass::from("fill-container"));
                });
            // render order test: reddest in the back, whitest in the front (flex center)
            parent
                .spawn_bundle(NodeBundle {
                    color: Color::NONE.into(),
                    ..Default::default()
                })
                .insert_bundle((
                    CssId::from("order-container"),
                    CssClass::from("fill-container absolute"),
                ))
                .with_children(|parent| {
                    parent
                        .spawn_bundle(NodeBundle {
                            color: Color::rgb(1.0, 0.0, 0.0).into(),
                            ..Default::default()
                        })
                        .insert_bundle((
                            CssId::from("square-order-1"),
                            CssClass::from("square-100px")
                        ))
                        .with_children(|parent| {
                            parent
                                .spawn_bundle(NodeBundle {
                                    color: Color::rgb(1.0, 0.3, 0.3).into(),
                                    ..Default::default()
                                })
                                .insert_bundle((
                                    CssId::from("square-order-2"),
                                    CssClass::from("absolute square-100px")
                                ));
                            parent
                                .spawn_bundle(NodeBundle {
                                    color: Color::rgb(1.0, 0.5, 0.5).into(),
                                    ..Default::default()
                                })
                                .insert_bundle((
                                    CssId::from("square-order-3"),
                                    CssClass::from("absolute square-100px")
                                ));
                            parent
                                .spawn_bundle(NodeBundle {
                                    color: Color::rgb(1.0, 0.7, 0.7).into(),
                                    ..Default::default()
                                })
                                .insert_bundle((
                                    CssId::from("square-order-4"),
                                    CssClass::from("absolute square-100px")
                                ));
                            // alpha test
                            parent
                                .spawn_bundle(NodeBundle {
                                    color: Color::rgba(1.0, 0.9, 0.9, 0.4).into(),
                                    ..Default::default()
                                })
                                .insert_bundle((
                                    CssId::from("square-order-5"),
                                    CssClass::from("absolute square-100px")
                                ));
                        });
                });
            // bevy logo (flex center)
            parent
                .spawn_bundle(NodeBundle {
                    color: Color::NONE.into(),
                    ..Default::default()
                })
                .insert_bundle((
                    CssId::from("logo-container"),
                    CssClass::from("fill-container absolute")
                ))
                .with_children(|parent| {
                    // bevy logo (image)
                    parent
                        .spawn_bundle(ImageBundle {
                            image: asset_server.load("branding/bevy_logo_dark_big.png").into(),
                            ..Default::default()
                        })
                        .insert(CssId::from("logo"));
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