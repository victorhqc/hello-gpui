use std::fmt::Display;

use super::{
    button::{Button as CalculatorButton, ButtonLabel, Event as ButtonEvent},
    calculation::{Calculation, Operation},
};
use gpui::{
    div, impl_actions, prelude::*, px, rgb, rgba, App, Context, Entity, KeyBinding, SharedString,
    Window,
};

#[derive(Debug)]
pub struct Calculator {
    calculation: Calculation,
    ac_btn: Entity<CalculatorButton>,
    plus_minus_btn: Entity<CalculatorButton>,
    percent_btn: Entity<CalculatorButton>,
    division_btn: Entity<CalculatorButton>,
    multiplication_btn: Entity<CalculatorButton>,
    subtraction_btn: Entity<CalculatorButton>,
    addition_btn: Entity<CalculatorButton>,
    equals_btn: Entity<CalculatorButton>,
    zero_btn: Entity<CalculatorButton>,
    one_btn: Entity<CalculatorButton>,
    two_btn: Entity<CalculatorButton>,
    three_btn: Entity<CalculatorButton>,
    four_btn: Entity<CalculatorButton>,
    five_btn: Entity<CalculatorButton>,
    six_btn: Entity<CalculatorButton>,
    seven_btn: Entity<CalculatorButton>,
    eight_btn: Entity<CalculatorButton>,
    nine_btn: Entity<CalculatorButton>,
    calc_btn: Entity<CalculatorButton>,
    comma_btn: Entity<CalculatorButton>,
}

impl Calculator {
    pub fn new(_window: &mut Window, cx: &mut Context<Self>) -> Self {
        let light_gray = rgb(0x707070);
        let dark_gray = rgb(0x515251);
        let orange = rgb(0xff9600);

        let ac_btn = cx.new(|_| {
            CalculatorButton::new(
                ButtonLabel::String("AC".into()),
                light_gray,
                ButtonEvent::Clear,
            )
        });
        Self::subscribe_btn(&ac_btn, cx);

        let plus_minus_btn = cx.new(|_| {
            CalculatorButton::new(
                ButtonLabel::String("±".into()),
                light_gray,
                ButtonEvent::PlusMinus,
            )
        });
        Self::subscribe_btn(&plus_minus_btn, cx);

        let percent_btn = cx.new(|_| {
            CalculatorButton::new(
                ButtonLabel::String("%".into()),
                light_gray,
                ButtonEvent::Percent,
            )
        });
        Self::subscribe_btn(&percent_btn, cx);

        let division_btn = cx.new(|_| {
            CalculatorButton::new(
                ButtonLabel::String(OperationButton::Division.into()),
                orange,
                ButtonEvent::Operation(Operation::Division),
            )
        });
        Self::subscribe_btn(&division_btn, cx);

        let seven_btn = cx.new(|_| {
            CalculatorButton::new(
                ButtonLabel::String("7".into()),
                dark_gray,
                ButtonEvent::Number(7),
            )
        });
        Self::subscribe_btn(&seven_btn, cx);

        let eight_btn = cx.new(|_| {
            CalculatorButton::new(
                ButtonLabel::String("8".into()),
                dark_gray,
                ButtonEvent::Number(8),
            )
        });
        Self::subscribe_btn(&eight_btn, cx);

        let nine_btn = cx.new(|_| {
            CalculatorButton::new(
                ButtonLabel::String("9".into()),
                dark_gray,
                ButtonEvent::Number(9),
            )
        });
        Self::subscribe_btn(&nine_btn, cx);

        let multiplication_btn = cx.new(|_| {
            CalculatorButton::new(
                ButtonLabel::String(OperationButton::Times.into()),
                orange,
                ButtonEvent::Operation(Operation::Multiplication),
            )
        });
        Self::subscribe_btn(&multiplication_btn, cx);

        let four_btn = cx.new(|_| {
            CalculatorButton::new(
                ButtonLabel::String("4".into()),
                dark_gray,
                ButtonEvent::Number(4),
            )
        });
        Self::subscribe_btn(&four_btn, cx);

        let five_btn = cx.new(|_| {
            CalculatorButton::new(
                ButtonLabel::String("5".into()),
                dark_gray,
                ButtonEvent::Number(5),
            )
        });
        Self::subscribe_btn(&five_btn, cx);

        let six_btn = cx.new(|_| {
            CalculatorButton::new(
                ButtonLabel::String("6".into()),
                dark_gray,
                ButtonEvent::Number(6),
            )
        });
        Self::subscribe_btn(&six_btn, cx);

        let subtraction_btn = cx.new(|_| {
            CalculatorButton::new(
                ButtonLabel::String("－".into()),
                orange,
                ButtonEvent::Operation(Operation::Subtraction),
            )
        });
        Self::subscribe_btn(&subtraction_btn, cx);

        let one_btn = cx.new(|_| {
            CalculatorButton::new(
                ButtonLabel::String("1".into()),
                dark_gray,
                ButtonEvent::Number(1),
            )
        });
        Self::subscribe_btn(&one_btn, cx);

        let two_btn = cx.new(|_| {
            CalculatorButton::new(
                ButtonLabel::String("2".into()),
                dark_gray,
                ButtonEvent::Number(2),
            )
        });
        Self::subscribe_btn(&two_btn, cx);

        let three_btn = cx.new(|_| {
            CalculatorButton::new(
                ButtonLabel::String("3".into()),
                dark_gray,
                ButtonEvent::Number(3),
            )
        });
        Self::subscribe_btn(&three_btn, cx);

        let addition_btn = cx.new(|_| {
            CalculatorButton::new(
                ButtonLabel::String("＋".into()),
                orange,
                ButtonEvent::Operation(Operation::Addition),
            )
        });
        Self::subscribe_btn(&addition_btn, cx);

        let calc_btn = cx.new(|_| {
            CalculatorButton::new(
                ButtonLabel::Svg("rocket.svg".to_string()),
                dark_gray,
                ButtonEvent::Noop,
            )
        });
        Self::subscribe_btn(&calc_btn, cx);

        let zero_btn = cx.new(|_| {
            CalculatorButton::new(
                ButtonLabel::String("0".into()),
                dark_gray,
                ButtonEvent::Number(0),
            )
        });
        Self::subscribe_btn(&zero_btn, cx);

        let comma_btn = cx.new(|_| {
            CalculatorButton::new(
                ButtonLabel::String(",".into()),
                dark_gray,
                ButtonEvent::Comma,
            )
        });
        Self::subscribe_btn(&comma_btn, cx);

        let equals_btn = cx.new(|_| {
            CalculatorButton::new(
                ButtonLabel::String(OperationButton::Equals.into()),
                orange,
                ButtonEvent::Operation(Operation::Equals),
            )
        });
        Self::subscribe_btn(&equals_btn, cx);

        Calculator {
            calculation: Calculation::default(),
            // row 1
            ac_btn,
            plus_minus_btn,
            percent_btn,
            division_btn,
            // row 2
            seven_btn,
            eight_btn,
            nine_btn,
            multiplication_btn,
            // row 3
            four_btn,
            five_btn,
            six_btn,
            subtraction_btn,
            // row 4
            one_btn,
            two_btn,
            three_btn,
            addition_btn,
            // row 5
            calc_btn,
            zero_btn,
            comma_btn,
            equals_btn,
        }
    }

    fn subscribe_btn(entity: &Entity<CalculatorButton>, cx: &mut Context<Self>) {
        cx.subscribe(entity, |this, _, event, cx| {
            Self::on_event(this, event, cx);
        })
        .detach();
    }
}

impl Calculator {
    fn append_number(&mut self, num: usize, cx: &mut Context<Self>) {
        if self.calculation.is_empty() {
            self.calculation = Calculation::default();
        }

        self.calculation.append_number(num);

        cx.notify();
    }

    fn add_comma(&mut self, cx: &mut Context<Self>) {
        if self.calculation.is_empty() {
            self.calculation = Calculation::default();
        }

        self.calculation.add_comma();

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

    fn render_ac_label(&self) -> ButtonLabel {
        if self.calculation.is_empty() {
            return ButtonLabel::String("AC".into());
        }

        ButtonLabel::Svg("undo.svg".to_string())
    }
}

impl Calculator {
    fn on_event(&mut self, evt: &ButtonEvent, cx: &mut Context<Self>) {
        match evt {
            ButtonEvent::Number(val) => {
                self.append_number(*val, cx);
            }
            ButtonEvent::Operation(op) => match op {
                Operation::Equals => {
                    self.calculation.calculate();
                    cx.notify();
                }
                _ => {
                    self.calculation.append_operation(op.clone());
                    cx.notify();
                }
            },
            ButtonEvent::Clear => {
                self.remove_or_clear(cx);
            }
            ButtonEvent::Comma => {
                self.add_comma(cx);
            }
            _ => {}
        }
    }

    fn keyboard(&mut self, a: &CalculatorAction, _: &mut Window, cx: &mut Context<Self>) {
        match a {
            CalculatorAction::Backspace => {
                self.remove_or_clear(cx);

                self.ac_btn.update(cx, |btn, cx| btn.set_clicked(cx));
            }
            CalculatorAction::Calculate => {
                self.calculation.calculate();
                cx.notify();

                self.equals_btn.update(cx, |btn, cx| btn.set_clicked(cx));
            }
            CalculatorAction::Op(Operation::Addition) => {
                self.calculation
                    .append_operation(OperationButton::Plus.into());
                cx.notify();

                self.addition_btn.update(cx, |btn, cx| btn.set_clicked(cx));
            }
            CalculatorAction::Op(Operation::Subtraction) => {
                self.calculation
                    .append_operation(OperationButton::Minus.into());
                cx.notify();

                self.subtraction_btn
                    .update(cx, |btn, cx| btn.set_clicked(cx));
            }
            CalculatorAction::Op(Operation::Multiplication) => {
                self.calculation
                    .append_operation(OperationButton::Times.into());
                cx.notify();

                self.multiplication_btn
                    .update(cx, |btn, cx| btn.set_clicked(cx));
            }
            CalculatorAction::Op(Operation::Division) => {
                self.calculation
                    .append_operation(OperationButton::Division.into());
                cx.notify();
            }
            &CalculatorAction::Numeric(val) => {
                self.append_number(val, cx);
                match val {
                    0 => self.zero_btn.update(cx, |btn, cx| btn.set_clicked(cx)),
                    1 => self.one_btn.update(cx, |btn, cx| btn.set_clicked(cx)),
                    2 => self.two_btn.update(cx, |btn, cx| btn.set_clicked(cx)),
                    3 => self.three_btn.update(cx, |btn, cx| btn.set_clicked(cx)),
                    4 => self.four_btn.update(cx, |btn, cx| btn.set_clicked(cx)),
                    5 => self.five_btn.update(cx, |btn, cx| btn.set_clicked(cx)),
                    6 => self.six_btn.update(cx, |btn, cx| btn.set_clicked(cx)),
                    7 => self.seven_btn.update(cx, |btn, cx| btn.set_clicked(cx)),
                    8 => self.eight_btn.update(cx, |btn, cx| btn.set_clicked(cx)),
                    9 => self.nine_btn.update(cx, |btn, cx| btn.set_clicked(cx)),
                    _ => {}
                };
            }
            CalculatorAction::Comma => {
                self.add_comma(cx);
                self.comma_btn.update(cx, |btn, cx| btn.set_clicked(cx));
            }
            _ => {}
        }
    }
}

impl Render for Calculator {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        self.ac_btn
            .update(cx, |btn, _| btn.label(self.render_ac_label()));

        let btns: Vec<Entity<CalculatorButton>> = vec![
            // Row 1
            self.ac_btn.clone(),
            self.plus_minus_btn.clone(),
            self.percent_btn.clone(),
            self.division_btn.clone(),
            // Row 2
            self.seven_btn.clone(),
            self.eight_btn.clone(),
            self.nine_btn.clone(),
            self.multiplication_btn.clone(),
            // Row 3
            self.four_btn.clone(),
            self.five_btn.clone(),
            self.six_btn.clone(),
            self.subtraction_btn.clone(),
            // Row 4
            self.one_btn.clone(),
            self.two_btn.clone(),
            self.three_btn.clone(),
            self.addition_btn.clone(),
            // Row 5,
            self.calc_btn.clone(),
            self.zero_btn.clone(),
            self.comma_btn.clone(),
            self.equals_btn.clone(),
        ];

        div()
            .id("calculator")
            .focusable()
            .key_context(CONTEXT)
            .on_action(cx.listener(Self::keyboard))
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
                    .children(btns),
            ])
    }
}

const CONTEXT: &str = "Calculator";

pub fn init(cx: &mut App) {
    cx.bind_keys([
        KeyBinding::new("backspace", CalculatorAction::Backspace, Some(CONTEXT)),
        KeyBinding::new("enter", CalculatorAction::Calculate, Some(CONTEXT)),
        KeyBinding::new(
            "+",
            CalculatorAction::Op(Operation::Addition),
            Some(CONTEXT),
        ),
        KeyBinding::new(
            "-",
            CalculatorAction::Op(Operation::Subtraction),
            Some(CONTEXT),
        ),
        KeyBinding::new(
            "*",
            CalculatorAction::Op(Operation::Multiplication),
            Some(CONTEXT),
        ),
        KeyBinding::new(
            "/",
            CalculatorAction::Op(Operation::Division),
            Some(CONTEXT),
        ),
        KeyBinding::new("0", CalculatorAction::Numeric(0), Some(CONTEXT)),
        KeyBinding::new("1", CalculatorAction::Numeric(1), Some(CONTEXT)),
        KeyBinding::new("2", CalculatorAction::Numeric(2), Some(CONTEXT)),
        KeyBinding::new("3", CalculatorAction::Numeric(3), Some(CONTEXT)),
        KeyBinding::new("4", CalculatorAction::Numeric(4), Some(CONTEXT)),
        KeyBinding::new("5", CalculatorAction::Numeric(5), Some(CONTEXT)),
        KeyBinding::new("6", CalculatorAction::Numeric(6), Some(CONTEXT)),
        KeyBinding::new("7", CalculatorAction::Numeric(7), Some(CONTEXT)),
        KeyBinding::new("8", CalculatorAction::Numeric(8), Some(CONTEXT)),
        KeyBinding::new("9", CalculatorAction::Numeric(9), Some(CONTEXT)),
        KeyBinding::new(",", CalculatorAction::Comma, Some(CONTEXT)),
        KeyBinding::new(".", CalculatorAction::Comma, Some(CONTEXT)),
    ]);
}

#[derive(Debug)]
pub enum OperationButton {
    Division,
    Times,
    Minus,
    Plus,
    Equals,
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

#[derive(Clone, Debug, PartialEq, Deserialize, JsonSchema)]
enum CalculatorAction {
    Numeric(usize),
    Op(Operation),
    Calculate,
    Backspace,
    Comma,
    NoAction,
}

impl_actions!(calculator, [CalculatorAction]);
