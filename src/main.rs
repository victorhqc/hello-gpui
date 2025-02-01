use gpui::{
    point, prelude::*, px, size, App, Application, Bounds, DisplayId, Pixels, Point, Size,
    TitlebarOptions, WindowBounds, WindowOptions,
};

mod calculator;
mod round_button;

fn main() {
    println!("Starting calculator");
    Application::new().run(|cx: &mut App| {
        let bounds = bounds_top_right(
            cx,
            None,
            size(px(198.), px(350.0)),
            Point {
                x: px(100.),
                y: px(100.),
            },
        );

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

        cx.open_window(window_options, |_, cx| {
            cx.new(|_| calculator::Calculator::default())
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
