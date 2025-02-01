use crate::round_button::RoundButton;
use gpui::{div, prelude::*, px, rgb, rgba, ClickEvent, Context, SharedString, Window};

#[derive(Debug)]
pub struct Operand {
    symbol: Option<SharedString>,
    value: f64,
}

#[derive(Debug)]
pub struct Calculation {
    operands: Vec<Operand>,
}

impl Default for Calculation {
    fn default() -> Self {
        Self {
            operands: vec![Operand {
                value: 0.,
                symbol: None,
            }],
        }
    }
}

#[derive(Default, Debug)]
pub struct Calculator {
    result: Calculation,
}

impl Calculator {
    fn handle_number_press(&mut self, value: &str, _event: &ClickEvent, cx: &mut Context<Self>) {
        // let target = event.
        println!("target {}", value);

        // self.likes += 1;
        cx.notify();
    }

    fn render_result(&self) -> impl IntoElement {
        let mut result = div();

        for operand in &self.result.operands {
            let mut children: Vec<SharedString> = vec![format!("{}", operand.value).into()];

            if let Some(symbol) = &operand.symbol {
                children.push(symbol.clone())
            }

            result = result.children(children);
        }

        result
    }
}

impl Render for Calculator {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let light_gray = rgb(0x707070);
        let dark_gray = rgb(0x515251);
        let orange = rgb(0xff9600);

        let ac_btn = RoundButton::new("ac_btn", "AC".into(), Some(light_gray));
        let plus_minus_btn = RoundButton::new("plus_minus_btn", "±".into(), Some(light_gray));
        let percent_btn = RoundButton::new("percent_btn", "％".into(), Some(light_gray));
        let division_btn = RoundButton::new("division_btn", "÷".into(), Some(orange));

        let seven_btn = RoundButton::new("seven_btn", "7".into(), Some(dark_gray));
        let eight_btn = RoundButton::new("eight_btn", "8".into(), Some(dark_gray));
        let nine_btn = RoundButton::new("nine_btn", "9".into(), Some(dark_gray));
        let times_btn = RoundButton::new("times_btn", "⨉".into(), Some(orange));

        let four_btn = RoundButton::new("four_btn", "4".into(), Some(dark_gray));
        let five_btn = RoundButton::new("five_btn", "5".into(), Some(dark_gray));
        let six_btn = RoundButton::new("six_btn", "6".into(), Some(dark_gray));
        let minus_btn = RoundButton::new("minus_btn", "－".into(), Some(orange));

        let one_btn = RoundButton::new("one_btn", "1".into(), Some(dark_gray));
        let two_btn = RoundButton::new("two_btn", "2".into(), Some(dark_gray));
        let three_btn = RoundButton::new("three_btn", "3".into(), Some(dark_gray));
        let plus_btn = RoundButton::new("plus_btn", "＋".into(), Some(orange));

        let calc_btn = RoundButton::new("calc_btn", "calc".into(), Some(dark_gray));
        let zero_btn = RoundButton::new("zero_btn", "0".into(), Some(dark_gray));
        let comma_btn = RoundButton::new("comma_btn", ",".into(), Some(dark_gray));
        let equals_btn = RoundButton::new("equals_btn", "＝".into(), Some(orange));

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
                    .w_full()
                    .flex()
                    .flex_row()
                    .justify_end()
                    .text_lg()
                    .text_color(rgb(0xcccccc))
                    // .children([]),
                    .children([div().child("")]),
                div()
                    .w_full()
                    .flex()
                    .flex_row()
                    .justify_end()
                    .text_2xl()
                    .child(self.render_result()),
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
                        ac_btn,
                        plus_minus_btn,
                        percent_btn,
                        division_btn,
                        // Row 2
                        seven_btn.on_click(cx.listener(|this, evt, _, cx| {
                            Self::handle_number_press(this, "7", evt, cx)
                        })),
                        eight_btn,
                        nine_btn,
                        times_btn,
                        // Row 3
                        four_btn,
                        five_btn,
                        six_btn,
                        minus_btn,
                        // Row 4
                        one_btn,
                        two_btn,
                        three_btn,
                        plus_btn,
                        // Row 5
                        calc_btn,
                        zero_btn,
                        comma_btn,
                        equals_btn,
                    ]),
            ])
    }
}
