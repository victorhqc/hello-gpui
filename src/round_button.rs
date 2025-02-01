use gpui::{div, prelude::*, px, Context, Fill, SharedString, Window};

pub struct RoundButton<F> {
    pub text: SharedString,
    pub bg: F,
}

impl<F> Render for RoundButton<F>
where
    F: Into<Fill> + 'static + Sized + Copy,
{
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .bg(self.bg)
            .w(px(42.))
            .h(px(42.))
            .rounded_full()
            .child(
                div()
                    .flex()
                    .justify_center()
                    .items_center()
                    .py(px(9.))
                    .child(self.text.clone()),
            )
    }
}
