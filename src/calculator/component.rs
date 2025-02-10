use std::fmt::Display;

use super::calculation::{Calculation, Operation};
use crate::round_button::RoundButton;
use gpui::{
    actions, div, impl_actions, prelude::*, px, rgb, rgba, Action, App, ClickEvent, Context,
    KeyBinding, SharedString, Window,
};

#[derive(Default, Debug)]
pub struct Calculator {
    calculation: Calculation,
}

impl Calculator {
    fn handle_number_press(&mut self, value: Button, _event: &ClickEvent, cx: &mut Context<Self>) {
        if let Button::Number(num) = value {
            self.append_number(num.into(), cx);
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
                    self.calculation.calculate();
                }
                _ => self.calculation.append_operation(op.into()),
            };

            cx.notify();
        }
    }

    fn handle_ac_press(&mut self, value: Button, _event: &ClickEvent, cx: &mut Context<Self>) {
        if let Button::Ac = value {
            self.remove_or_clear(cx);
        };
    }

    fn append_number(&mut self, num: usize, cx: &mut Context<Self>) {
        if self.calculation.is_empty() {
            self.calculation = Calculation::default();
        }

        self.calculation.append_number(num);

        cx.notify();
    }

    fn remove_or_clear(&mut self, cx: &mut Context<Self>) {
        if self.calculation.is_empty() {
            self.calculation = Calculation::default();
        } else {
            self.calculation.remove_last();
        }
        cx.notify();
    }

    fn render_result(&self) -> impl IntoElement {
        div()
            .w_full()
            .flex()
            .flex_row()
            .justify_end()
            .child(self.calculation.current_operation_string())
    }

    fn render_past_operations(&self) -> impl IntoElement {
        div()
            .w_full()
            .flex()
            .flex_row()
            .justify_end()
            .child(self.calculation.past_operations_string())
    }

    fn render_ac_label(&self) -> SharedString {
        if self.calculation.is_empty() {
            return "AC".into();
        }

        "<-".into()
    }
}

impl Calculator {
    fn backspace(&mut self, _: &Backspace, _: &mut Window, cx: &mut Context<Self>) {
        self.remove_or_clear(cx);
    }

    fn enter(&mut self, _: &Calculate, _: &mut Window, cx: &mut Context<Self>) {
        self.calculation.calculate();
        cx.notify();
    }

    fn addition(&mut self, _: &Addition, _: &mut Window, cx: &mut Context<Self>) {
        self.calculation
            .append_operation(OperationButton::Plus.into());
        cx.notify();
    }

    fn subtraction(&mut self, _: &Subtraction, _: &mut Window, cx: &mut Context<Self>) {
        self.calculation
            .append_operation(OperationButton::Minus.into());
        cx.notify();
    }

    fn multiplication(&mut self, _: &Multiplication, _: &mut Window, cx: &mut Context<Self>) {
        self.calculation
            .append_operation(OperationButton::Times.into());
        cx.notify();
    }

    fn division(&mut self, _: &Division, _: &mut Window, cx: &mut Context<Self>) {
        self.calculation
            .append_operation(OperationButton::Division.into());
        cx.notify();
    }

    fn press_number(&mut self, a: &NumberAction, _: &mut Window, cx: &mut Context<Self>) {
        self.append_number(a.val, cx);
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
            .id("calculator")
            .focusable()
            .key_context(CONTEXT)
            .on_action(cx.listener(Self::backspace))
            .on_action(cx.listener(Self::enter))
            .on_action(cx.listener(Self::addition))
            .on_action(cx.listener(Self::subtraction))
            .on_action(cx.listener(Self::multiplication))
            .on_action(cx.listener(Self::division))
            .on_action(cx.listener(Self::press_number))
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
                    .text_lg()
                    .text_color(rgb(0xcccccc))
                    .child(self.render_past_operations()),
                div()
                    .w_full()
                    .text_2xl()
                    .px(px(5.))
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
                        ac_btn.label(self.render_ac_label()).on_click(cx.listener(
                            |this, evt, _, cx| Self::handle_ac_press(this, Button::Ac, evt, cx),
                        )),
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

const CONTEXT: &str = "Calculator";

pub fn init(cx: &mut App) {
    cx.bind_keys([
        KeyBinding::new("backspace", Backspace, Some(CONTEXT)),
        KeyBinding::new("enter", Calculate, Some(CONTEXT)),
        KeyBinding::new("+", Addition, Some(CONTEXT)),
        KeyBinding::new("-", Subtraction, Some(CONTEXT)),
        KeyBinding::new("*", Multiplication, Some(CONTEXT)),
        KeyBinding::new("/", Division, Some(CONTEXT)),
        KeyBinding::new("0", NumberAction { val: 0 }, Some(CONTEXT)),
        KeyBinding::new("1", NumberAction { val: 1 }, Some(CONTEXT)),
        KeyBinding::new("2", NumberAction { val: 2 }, Some(CONTEXT)),
        KeyBinding::new("3", NumberAction { val: 3 }, Some(CONTEXT)),
        KeyBinding::new("4", NumberAction { val: 4 }, Some(CONTEXT)),
        KeyBinding::new("5", NumberAction { val: 5 }, Some(CONTEXT)),
        KeyBinding::new("6", NumberAction { val: 6 }, Some(CONTEXT)),
        KeyBinding::new("7", NumberAction { val: 7 }, Some(CONTEXT)),
        KeyBinding::new("8", NumberAction { val: 8 }, Some(CONTEXT)),
        KeyBinding::new("9", NumberAction { val: 9 }, Some(CONTEXT)),
    ]);
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

impl From<NumericButton> for usize {
    fn from(value: NumericButton) -> Self {
        match value {
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

impl From<OperationButton> for SharedString {
    fn from(value: OperationButton) -> Self {
        format!("{}", value).into()
    }
}

impl From<OperationButton> for Operation {
    fn from(value: OperationButton) -> Self {
        match value {
            OperationButton::Plus => Operation::Addition,
            OperationButton::Minus => Operation::Subtraction,
            OperationButton::Times => Operation::Multiplication,
            OperationButton::Division => Operation::Division,
            OperationButton::Equals => Operation::Equals,
        }
    }
}

#[derive(Clone, PartialEq, Deserialize, JsonSchema)]
struct NumberAction {
    val: usize,
}

impl_actions!(calculator1, [NumberAction]);

actions!(
    calculator,
    [
        Backspace,
        PlusMinus,
        Percent,
        Addition,
        Subtraction,
        Multiplication,
        Division,
        Calculate
    ]
);
