use crate::round_button::RoundButton;
use gpui::{
    div, prelude::*, rgb, rgba, svg, ClickEvent, Context, EventEmitter, Rgba, SharedString, Timer,
    Window,
};
use std::time::Duration;

use super::calculation::Operation;

const DELAY: Duration = Duration::from_millis(50);

#[derive(Clone)]
pub enum ButtonLabel {
    String(SharedString),
    Svg(String),
}

pub struct Button {
    label: ButtonLabel,
    color: Rgba,
    is_active: bool,
    event_to_emit: Event,
}

impl Button {
    pub fn new(label: ButtonLabel, color: Rgba, event_to_emit: Event) -> Self {
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

    pub fn label(&mut self, label: ButtonLabel) {
        self.label = label;
    }

    pub fn render_label(&mut self) -> impl IntoElement {
        match &self.label {
            ButtonLabel::String(str) => div().child(str.clone()),
            ButtonLabel::Svg(path) => {
                div().child(svg().path(path).mt_1().size_4().text_color(rgb(0xffffff)))
            }
        }
    }
}

impl Render for Button {
    fn render(&mut self, _: &mut Window, cx: &mut Context<'_, Self>) -> impl IntoElement {
        let color = if self.is_active {
            self.color.blend(rgba(0xffffff30))
        } else {
            self.color
        };

        let btn = RoundButton::new("btn", self.render_label(), Some(color));

        div().child(btn.on_click(cx.listener(|this, evt, _, cx| {
            Self::handle_click(this, evt, cx);
        })))
    }
}

#[derive(Debug, Clone)]
pub enum Event {
    Number(usize),
    Clear,
    PlusMinus,
    Percent,
    Operation(Operation),
    Comma,
    Noop,
}

impl EventEmitter<Event> for Button {}
