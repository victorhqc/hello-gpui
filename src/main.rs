use gpui::{
    point, prelude::*, px, size, App, Application, Bounds, DisplayId, Pixels, Point, Size,
    TitlebarOptions, WindowBounds, WindowOptions,
};
use std::path::PathBuf;

#[macro_use]
extern crate dashu_macros;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate schemars;

mod assets;
mod calculator;
mod round_button;

fn main() {
    println!("Starting calculator");
    Application::new()
        .with_assets(assets::Assets::new(
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("icons"),
        ))
        .run(|cx: &mut App| {
            #[cfg(not(debug_assertions))]
            let bounds = Bounds::centered(None, size(px(198.), px(350.0)), cx);

            #[cfg(debug_assertions)]
            let bounds = bounds_top_right(
                cx,
                None,
                size(px(198.), px(350.0)),
                Point {
                    x: px(25.),
                    y: px(50.),
                },
            );

            cx.activate(true);
            calculator::component::init(cx);

            let window_options = WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                window_min_size: Some(size(px(198.), px(350.))),
                window_background: gpui::WindowBackgroundAppearance::Blurred,
                titlebar: Some(TitlebarOptions {
                    title: None,
                    traffic_light_position: None,
                    appears_transparent: true,
                }),
                app_id: Some("com.dev.hello-gpui".to_string()),
                kind: gpui::WindowKind::Normal,
                ..Default::default()
            };

            cx.open_window(window_options, |window, cx| {
                cx.new(|c| calculator::component::Calculator::new(window, c))
            })
            .unwrap();
        });
}

fn bounds_top_right(
    cx: &mut App,
    display_id: Option<DisplayId>,
    size: Size<Pixels>,
    offset: Point<Pixels>,
) -> Bounds<Pixels> {
    let display = display_id
        .and_then(|id| cx.find_display(id))
        .or_else(|| cx.primary_display());

    display
        .map(|display| {
            let origin = display.bounds().top_right();
            let origin: Point<Pixels> = Point {
                x: origin.x - offset.x,
                y: origin.y + offset.y,
            };

            Bounds::from_corner_and_size(gpui::Corner::TopRight, origin, size)
        })
        .unwrap_or_else(|| Bounds {
            origin: point(px(0.), px(0.)),
            size,
        })
}
