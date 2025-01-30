use gpui::{
    div, prelude::*, px, rgb, rgba, size, App, Application, Bounds, Context, SharedString,
    TextStyle, Window, WindowBounds, WindowOptions,
};

pub struct RoundButton {
    pub text: SharedString,
}

impl Render for RoundButton {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div().child(self.text.clone())
    }
}
