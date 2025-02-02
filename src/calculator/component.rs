use std::fmt::Display;

use crate::round_button::RoundButton;
use gpui::{div, prelude::*, px, rgb, rgba, ClickEvent, Context, SharedString, Window};

#[derive(Debug)]
pub enum OperandValue {
    Integer(i128),
    Float(f64),
}

#[derive(Debug)]
pub struct Operand {
    symbol: Option<SharedString>,
    value: OperandValue,
}

#[derive(Debug)]
pub struct Calculation {
    result: Option<OperandValue>,
    operands: Vec<Operand>,
}

impl Default for Calculation {
    fn default() -> Self {
        Self {
            result: None,
            operands: vec![Operand {
                symbol: None,
                value: OperandValue::Integer(0),
            }],
        }
    }
}

#[derive(Default, Debug)]
pub struct Calculator {
    calculation: Calculation,
}

impl Calculator {
    fn handle_number_press(&mut self, value: Button, _event: &ClickEvent, cx: &mut Context<Self>) {
        // let target = event.
        println!("target {:?}", value);

        if let Button::Number(num) = value {
            self.append_number(num, cx);
        };
    }

    fn handle_operation_press(
        &mut self,
        value: Button,
        _event: &ClickEvent,
        cx: &mut Context<Self>,
    ) {
        if let Button::Operation(op) = value {
            match op {
                OperationButton::Equals => {
                    println!("Calculate")
                }
                _ => self.append_operation(op, cx),
            }
        }
    }

    fn handle_ac_press(&mut self, value: Button, _event: &ClickEvent, cx: &mut Context<Self>) {
        if let Button::Ac = value {
            self.remove_from_result();
        };

        cx.notify();
    }

    fn render_result(&self) -> impl IntoElement {
        let mut result = div().w_full().flex().flex_row().justify_end();

        if let Some(calculation) = &self.calculation.result {
            // if self.calculation.operands.is_empty() {
            let res: SharedString = format!("{}", calculation).into();

            return result.child(res);
        }

        for operand in &self.calculation.operands {
            let mut children: Vec<SharedString> = vec![format!("{}", operand.value).into()];

            if let Some(symbol) = &operand.symbol {
                children.push(symbol.clone())
            }

            result = result.children(children);
        }

        result
    }

    fn append_number(&mut self, value: NumericButton, cx: &mut Context<Self>) {
        let current_operand = self.calculation.operands.last_mut();

        if let Some(&mut ref mut operand) = current_operand {
            if operand.symbol.is_some() {
                return self.calculation.operands.push(Operand {
                    value: OperandValue::Integer(value.into()),
                    symbol: None,
                });
            }

            match operand.value {
                OperandValue::Integer(val) => {
                    let value: i128 = value.into();
                    let new_value = format!("{}{}", val, value);
                    // operand.

                    let new_value: i128 = new_value.parse().unwrap();

                    operand.value = OperandValue::Integer(new_value);
                }
                OperandValue::Float(val) => {
                    let value: f64 = value.into();
                    let new_value = format!("{}{}", val, value);
                    let new_value: f64 = new_value.parse().unwrap();

                    operand.value = OperandValue::Float(new_value);
                }
            }
        } else {
            self.calculation.operands.push(Operand {
                value: OperandValue::Integer(value.into()),
                symbol: None,
            });
        }

        cx.notify();
    }

    fn append_operation(&mut self, operation: OperationButton, cx: &mut Context<Self>) {
        let symbol: SharedString = operation.into();
        let current_operand = self.calculation.operands.last_mut();

        if let Some(&mut ref mut operand) = current_operand {
            operand.symbol = Some(symbol)
        }

        // match operation {
        //     OperationButton::Plus => {

        //     }
        //     _ => println!("Not implemented yet")
        // }
    }

    fn remove_from_result(&mut self) {
        let current_operand = self.calculation.operands.last_mut();

        if let Some(&mut ref mut operand) = current_operand {
            match operand.value {
                OperandValue::Integer(val) => {}
                OperandValue::Float(val) => {}
            }
        };
    }
}

impl Render for Calculator {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let light_gray = rgb(0x707070);
        let dark_gray = rgb(0x515251);
        let orange = rgb(0xff9600);

        // ␡

        let ac_btn = RoundButton::new("ac_btn", "AC".into(), Some(light_gray));
        let plus_minus_btn = RoundButton::new("plus_minus_btn", "±".into(), Some(light_gray));
        let percent_btn = RoundButton::new("percent_btn", "％".into(), Some(light_gray));
        let division_btn = RoundButton::new(
            "division_btn",
            OperationButton::Division.into(),
            Some(orange),
        );

        let seven_btn = RoundButton::new("seven_btn", "7".into(), Some(dark_gray));
        let eight_btn = RoundButton::new("eight_btn", "8".into(), Some(dark_gray));
        let nine_btn = RoundButton::new("nine_btn", "9".into(), Some(dark_gray));
        let times_btn = RoundButton::new("times_btn", OperationButton::Times.into(), Some(orange));

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
        let equals_btn =
            RoundButton::new("equals_btn", OperationButton::Equals.into(), Some(orange));

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
                        ac_btn.on_click(cx.listener(|this, evt, _, cx| {
                            Self::handle_ac_press(this, Button::Ac, evt, cx)
                        })),
                        plus_minus_btn,
                        percent_btn,
                        division_btn.on_click(cx.listener(|this, evt, _, cx| {
                            Self::handle_operation_press(
                                this,
                                Button::Operation(OperationButton::Division),
                                evt,
                                cx,
                            );
                        })),
                        // Row 2
                        seven_btn.on_click(cx.listener(|this, evt, _, cx| {
                            Self::handle_number_press(
                                this,
                                Button::Number(NumericButton::Seven),
                                evt,
                                cx,
                            )
                        })),
                        eight_btn.on_click(cx.listener(|this, evt, _, cx| {
                            Self::handle_number_press(
                                this,
                                Button::Number(NumericButton::Eight),
                                evt,
                                cx,
                            )
                        })),
                        nine_btn.on_click(cx.listener(|this, evt, _, cx| {
                            Self::handle_number_press(
                                this,
                                Button::Number(NumericButton::Nine),
                                evt,
                                cx,
                            )
                        })),
                        times_btn.on_click(cx.listener(|this, evt, _, cx| {
                            Self::handle_operation_press(
                                this,
                                Button::Operation(OperationButton::Times),
                                evt,
                                cx,
                            );
                        })),
                        // Row 3
                        four_btn.on_click(cx.listener(|this, evt, _, cx| {
                            Self::handle_number_press(
                                this,
                                Button::Number(NumericButton::Four),
                                evt,
                                cx,
                            )
                        })),
                        five_btn.on_click(cx.listener(|this, evt, _, cx| {
                            Self::handle_number_press(
                                this,
                                Button::Number(NumericButton::Five),
                                evt,
                                cx,
                            )
                        })),
                        six_btn.on_click(cx.listener(|this, evt, _, cx| {
                            Self::handle_number_press(
                                this,
                                Button::Number(NumericButton::Six),
                                evt,
                                cx,
                            )
                        })),
                        minus_btn.on_click(cx.listener(|this, evt, _, cx| {
                            Self::handle_operation_press(
                                this,
                                Button::Operation(OperationButton::Minus),
                                evt,
                                cx,
                            );
                        })),
                        // Row 4
                        one_btn.on_click(cx.listener(|this, evt, _, cx| {
                            Self::handle_number_press(
                                this,
                                Button::Number(NumericButton::One),
                                evt,
                                cx,
                            )
                        })),
                        two_btn.on_click(cx.listener(|this, evt, _, cx| {
                            Self::handle_number_press(
                                this,
                                Button::Number(NumericButton::Two),
                                evt,
                                cx,
                            )
                        })),
                        three_btn.on_click(cx.listener(|this, evt, _, cx| {
                            Self::handle_number_press(
                                this,
                                Button::Number(NumericButton::Three),
                                evt,
                                cx,
                            )
                        })),
                        plus_btn.on_click(cx.listener(|this, evt, _, cx| {
                            Self::handle_operation_press(
                                this,
                                Button::Operation(OperationButton::Plus),
                                evt,
                                cx,
                            );
                        })),
                        // Row 5
                        calc_btn,
                        zero_btn.on_click(cx.listener(|this, evt, _, cx| {
                            Self::handle_number_press(
                                this,
                                Button::Number(NumericButton::Zero),
                                evt,
                                cx,
                            )
                        })),
                        comma_btn,
                        equals_btn.on_click(cx.listener(|this, evt, _, cx| {
                            Self::handle_operation_press(
                                this,
                                Button::Operation(OperationButton::Equals),
                                evt,
                                cx,
                            );
                        })),
                    ]),
            ])
    }
}

#[derive(Debug)]
pub enum Button {
    Number(NumericButton),
    Operation(OperationButton),
    Ac,
    PlusMinus,
    Percent,
    Calc,
    Comma,
}

#[derive(Debug)]
pub enum NumericButton {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Zero,
}

#[derive(Debug)]
pub enum OperationButton {
    Division,
    Times,
    Minus,
    Plus,
    Equals,
}

impl Into<f64> for NumericButton {
    fn into(self) -> f64 {
        match self {
            NumericButton::Zero => 0.,
            NumericButton::One => 1.,
            NumericButton::Two => 2.,
            NumericButton::Three => 3.,
            NumericButton::Four => 4.,
            NumericButton::Five => 5.,
            NumericButton::Six => 6.,
            NumericButton::Seven => 7.,
            NumericButton::Eight => 8.,
            NumericButton::Nine => 9.,
        }
    }
}

impl Into<i128> for NumericButton {
    fn into(self) -> i128 {
        match self {
            NumericButton::Zero => 0,
            NumericButton::One => 1,
            NumericButton::Two => 2,
            NumericButton::Three => 3,
            NumericButton::Four => 4,
            NumericButton::Five => 5,
            NumericButton::Six => 6,
            NumericButton::Seven => 7,
            NumericButton::Eight => 8,
            NumericButton::Nine => 9,
        }
    }
}

impl Display for OperationButton {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OperationButton::Plus => f.write_str("+"),
            OperationButton::Minus => f.write_str("-"),
            OperationButton::Times => f.write_str("⨉"),
            OperationButton::Division => f.write_str("÷"),
            OperationButton::Equals => f.write_str("＝"),
        }
    }
}

impl Into<SharedString> for OperationButton {
    fn into(self) -> SharedString {
        format!("{}", self).into()
    }
}

impl Display for OperandValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OperandValue::Integer(val) => f.write_str(&format!("{}", val)),
            OperandValue::Float(val) => f.write_str(&format!("{}", val)),
        }
    }
}

// impl OperandValue {
//     pub fn value(&self) {

//     }
// }
