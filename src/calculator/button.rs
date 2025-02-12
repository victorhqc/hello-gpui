use crate::round_button::RoundButton;
use gpui::{
    div, prelude::*, rgba, ClickEvent, Context, EventEmitter, Rgba, SharedString, Timer, Window,
};
use std::time::Duration;

const DELAY: Duration = Duration::from_millis(50);

pub struct Button {
    label: SharedString,
    color: Rgba,
    is_active: bool,
    event_to_emit: Event,
}

impl Button {
    pub fn new(label: SharedString, color: Rgba, event_to_emit: Event) -> Self {
        Button {
            label,
            color,
            is_active: false,
            event_to_emit,
        }
    }

    pub fn set_clicked(&mut self, cx: &mut Context<Self>) {
        self.is_active = true;

        cx.spawn(|this, mut cx| async move {
            Timer::after(DELAY).await;
            if let Some(this) = this.upgrade() {
                this.update(&mut cx, |this, cx| this.set_unclick(cx)).ok();
            }
        })
        .detach();
    }

    pub fn set_unclick(&mut self, cx: &mut Context<Self>) {
        self.is_active = false;
        cx.notify();
    }

    fn handle_click(&mut self, _event: &ClickEvent, cx: &mut Context<Self>) {
        cx.emit(self.event_to_emit.clone());
    }
}

impl Render for Button {
    fn render(&mut self, _: &mut Window, cx: &mut Context<'_, Self>) -> impl IntoElement {
        let color = if self.is_active {
            self.color.blend(rgba(0xffffff30))
        } else {
            self.color
        };

        let btn = RoundButton::new("btn", self.label.clone(), Some(color));

        div().child(btn.on_click(cx.listener(|this, evt, _, cx| {
            Self::handle_click(this, evt, cx);
        })))
    }
}

#[derive(Debug, Clone)]
pub enum Event {
    Number(usize),
}

impl EventEmitter<Event> for Button {}
