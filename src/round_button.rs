use gpui::{
    div, prelude::*, px, rgb, rgba, Action, App, ClickEvent, ElementId, Rgba, SharedString, Window,
};

type ClickFn = dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static;
// type ActionFn<A: Action> = dyn Fn(&A, &mut Window, &mut App) + 'static;

#[derive(IntoElement)]
pub struct RoundButton {
    pub id: ElementId,
    pub bg: Rgba,
    active_bg: Rgba,
    is_disabled: bool,
    on_click: Option<Box<ClickFn>>,
    // on_action: Option<Box<ActionFn<A>>>,
    label: SharedString,
}

impl RoundButton {
    pub fn new(id: impl Into<ElementId>, label: SharedString, bg: Option<Rgba>) -> Self {
        let bg = bg.unwrap_or_else(|| rgb(0x000000));

        let active_bg = bg.blend(rgba(0xffffff30));

        RoundButton {
            id: id.into(),
            label,
            bg,
            active_bg,
            is_disabled: false,
            on_click: None,
            // on_action: None,
        }
    }

    pub fn on_click(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_click = Some(Box::new(handler));
        self
    }

    // pub fn on_action(mut self, handler: impl Fn(&A, &mut Window, &mut App) + 'static) -> Self {
    //     self.on_action = Some(Box::new(handler));

    //     self
    // }

    pub fn label(mut self, label: SharedString) -> Self {
        self.label = label;

        self
    }
}

impl RenderOnce for RoundButton {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .id(self.id.clone())
            .bg(self.bg)
            .w(px(42.))
            .h(px(42.))
            .rounded_full()
            .when_some(self.on_click, |this, on_click| {
                this.on_click(move |evt, win, app| (on_click)(evt, win, app))
            })
            // .when_some(self.on_action, |this, on_action| {
            //     this.on_action(move |act, win, app| (on_action)(act, win, app))
            // })
            .when(!self.is_disabled, |this| {
                this.active(|this| this.bg(self.active_bg))
            })
            .child(
                div()
                    .flex()
                    .justify_center()
                    .items_center()
                    .py(px(9.))
                    .child(self.label),
            )
    }
}
