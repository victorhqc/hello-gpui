use gpui::{
    div, prelude::*, px, rgb, App, ClickEvent, ElementId, Fill, Rgba, SharedString, Window,
};

#[derive(IntoElement)]
pub struct RoundButton<F>
where
    F: Into<Fill> + 'static + Sized + Copy,
{
    pub id: ElementId,
    pub label: SharedString,
    pub bg: F,
    on_click: Option<Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
}

impl RoundButton<Rgba> {
    pub fn new(id: impl Into<ElementId>, label: SharedString, bg: Option<Rgba>) -> Self {
        RoundButton {
            id: id.into(),
            label,
            bg: bg.unwrap_or_else(|| rgb(0x000000)),
            on_click: None,
        }
    }

    pub fn on_click(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_click = Some(Box::new(handler));
        self
    }
}

impl<F> RenderOnce for RoundButton<F>
where
    F: Into<Fill> + 'static + Sized + Copy,
{
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .id(self.id)
            .bg(self.bg)
            .w(px(42.))
            .h(px(42.))
            .rounded_full()
            .when_some(self.on_click, |this, on_click| {
                this.on_click(move |evt, win, app| (on_click)(evt, win, app))
            })
            .child(
                div()
                    .flex()
                    .justify_center()
                    .items_center()
                    .py(px(9.))
                    .child(self.label.clone()),
            )
    }
}
