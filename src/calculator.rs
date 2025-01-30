use crate::round_button::RoundButton;
use gpui::{
    div, prelude::*, px, rgb, rgba, size, App, Application, Bounds, Context, Entity, SharedString,
    TextStyle, Window, WindowBounds, WindowOptions,
};

pub struct Operand {
    symbol: Option<SharedString>,
    value: f64,
}

pub struct Calculation {
    result: f64,
    operations: Vec<Operand>,
}

pub struct Calculator {
    calculation: Option<Calculation>,
}

impl Default for Calculator {
    fn default() -> Self {
        Self { calculation: None }
    }
}

impl Render for Calculator {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let ac_button: Entity<RoundButton> = cx.new(|_cx| RoundButton { text: "AC".into() });
        let sign_button: Entity<RoundButton> = cx.new(|_cx| RoundButton { text: "+/-".into() });

        div()
            .items_center()
            .shadow_lg()
            .bg(rgba(0x45454580))
            .text_color(rgb(0xffffff))
            .p_2()
            .children([
                div()
                    // .bg(gpui::blue())
                    .w_full()
                    .flex()
                    .flex_row()
                    .justify_end()
                    .text_lg()
                    .text_color(rgb(0xcccccc))
                    .children([div().child("7+"), div().child("6")]),
                div()
                    // .bg(gpui::red())
                    .w_full()
                    .flex()
                    .flex_row()
                    .justify_end()
                    .text_2xl()
                    .children([div().child("13")]),
                div().flex().flex_wrap().flex_row().children([
                    // Row 1
                    div().w_1_4().child(ac_button),
                    div().w_1_4().child(sign_button),
                    div().w_1_4().child("%"),
                    div().w_1_4().child("÷"),
                    // Row 2
                    div().w_1_4().child("7"),
                    div().w_1_4().child("8"),
                    div().w_1_4().child("9"),
                    div().w_1_4().child("x"),
                    // Row 3
                    div().w_1_4().child("4"),
                    div().w_1_4().child("5"),
                    div().w_1_4().child("6"),
                    div().w_1_4().child("-"),
                    // Row 4
                    div().w_1_4().child("1"),
                    div().w_1_4().child("2"),
                    div().w_1_4().child("3"),
                    div().w_1_4().child("+"),
                    // Row 5
                    div().w_1_4().child("calc"),
                    div().w_1_4().child("0"),
                    div().w_1_4().child(","),
                    div().w_1_4().child("="),
                ]),
            ])
    }
}
