use gpui::{
    prelude::*, px, size, App, Application, Bounds, TitlebarOptions, WindowBounds, WindowOptions,
};

mod calculator;
mod round_button;

fn main() {
    println!("Starting calculator");
    Application::new().run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(198.), px(324.0)), cx);

        let window_options = WindowOptions {
            window_bounds: Some(WindowBounds::Windowed(bounds)),
            window_min_size: Some(size(px(198.), px(324.))),
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
