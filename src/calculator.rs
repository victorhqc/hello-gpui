use crate::round_button::RoundButton;
use gpui::{
    div, prelude::*, px, rgb, rgba, size, App, Application, Bounds, Context, Entity, Rgba,
    SharedString, TextStyle, Window, WindowBounds, WindowOptions,
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
        let light_gray = rgb(0x707070);
        let dark_gray = rgb(0x515251);
        let orange = rgb(0xff9600);

        let ac_btn: Entity<RoundButton<Rgba>> = cx.new(|_cx| RoundButton {
            text: "AC".into(),
            bg: light_gray,
        });
        let sign_btn: Entity<RoundButton<Rgba>> = cx.new(|_cx| RoundButton {
            text: "+/-".into(),
            bg: light_gray,
        });
        let percentage_btn: Entity<RoundButton<Rgba>> = cx.new(|_cx| RoundButton {
            text: "%".into(),
            bg: light_gray,
        });
        let division_btn: Entity<RoundButton<Rgba>> = cx.new(|_cx| RoundButton {
            text: "%".into(),
            bg: orange,
        });

        let seven_btn: Entity<RoundButton<Rgba>> = cx.new(|_cx| RoundButton {
            text: "7".into(),
            bg: dark_gray,
        });
        let eight_btn: Entity<RoundButton<Rgba>> = cx.new(|_cx| RoundButton {
            text: "8".into(),
            bg: dark_gray,
        });
        let nine_btn: Entity<RoundButton<Rgba>> = cx.new(|_cx| RoundButton {
            text: "9".into(),
            bg: dark_gray,
        });
        let times_btn: Entity<RoundButton<Rgba>> = cx.new(|_cx| RoundButton {
            text: "x".into(),
            bg: orange,
        });

        let four_btn: Entity<RoundButton<Rgba>> = cx.new(|_cx| RoundButton {
            text: "4".into(),
            bg: dark_gray,
        });
        let five_btn: Entity<RoundButton<Rgba>> = cx.new(|_cx| RoundButton {
            text: "5".into(),
            bg: dark_gray,
        });
        let six_btn: Entity<RoundButton<Rgba>> = cx.new(|_cx| RoundButton {
            text: "6".into(),
            bg: dark_gray,
        });
        let minus_btn: Entity<RoundButton<Rgba>> = cx.new(|_cx| RoundButton {
            text: "-".into(),
            bg: orange,
        });

        let one_btn: Entity<RoundButton<Rgba>> = cx.new(|_cx| RoundButton {
            text: "1".into(),
            bg: dark_gray,
        });
        let two_btn: Entity<RoundButton<Rgba>> = cx.new(|_cx| RoundButton {
            text: "2".into(),
            bg: dark_gray,
        });
        let three_btn: Entity<RoundButton<Rgba>> = cx.new(|_cx| RoundButton {
            text: "3".into(),
            bg: dark_gray,
        });
        let plus_btn: Entity<RoundButton<Rgba>> = cx.new(|_cx| RoundButton {
            text: "+".into(),
            bg: orange,
        });

        let calc_btn: Entity<RoundButton<Rgba>> = cx.new(|_cx| RoundButton {
            text: "calc".into(),
            bg: dark_gray,
        });
        let zero_btn: Entity<RoundButton<Rgba>> = cx.new(|_cx| RoundButton {
            text: "0".into(),
            bg: dark_gray,
        });
        let comma_btn: Entity<RoundButton<Rgba>> = cx.new(|_cx| RoundButton {
            text: ",".into(),
            bg: dark_gray,
        });
        let equals_btn: Entity<RoundButton<Rgba>> = cx.new(|_cx| RoundButton {
            text: "=".into(),
            bg: orange,
        });

        div()
            .items_center()
            .shadow_lg()
            .bg(rgba(0x45454580))
            .text_color(rgb(0xffffff))
            .p_1p5()
            .h_full()
            .children([
                div().w_full().mt_9(),
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
                div()
                    .w_full()
                    .flex()
                    .flex_wrap()
                    .flex_row()
                    .justify_center()
                    .items_center()
                    .gap(px(5.))
                    .children([
                        // Row 1
                        div().child(ac_btn),
                        div().child(sign_btn),
                        div().child(percentage_btn),
                        div().child(division_btn),
                        // Row 2
                        div().child(seven_btn),
                        div().child(eight_btn),
                        div().child(nine_btn),
                        div().child(times_btn),
                        // Row 3
                        div().child(four_btn),
                        div().child(five_btn),
                        div().child(six_btn),
                        div().child(minus_btn),
                        // Row 4
                        div().child(one_btn),
                        div().child(two_btn),
                        div().child(three_btn),
                        div().child(plus_btn),
                        // Row 5
                        div().child(calc_btn),
                        div().child(zero_btn),
                        div().child(comma_btn),
                        div().child(equals_btn),
                    ]),
            ])
    }
}
