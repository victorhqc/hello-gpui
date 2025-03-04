use super::numeric_value::NumericValue;
use dashu_float::DBig;
use gpui::SharedString;
use std::{
    fmt::Display,
    ops::{Add, Div, Mul, Sub},
    str::FromStr,
};

#[derive(Clone, Debug, PartialEq, Deserialize, JsonSchema)]
pub enum Operation {
    Division,
    Multiplication,
    Subtraction,
    Addition,
    Equals,
}

#[derive(Debug, Clone, PartialEq)]
pub enum OperandValue {
    Number(NumericValue),
    Undefined,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Operand {
    operation: Option<Operation>,
    value: OperandValue,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Calculation {
    past_operands: Vec<Operand>,
    operands: Vec<Operand>,
}

impl Calculation {
    pub fn calculate(&mut self) {
        if self.operands.len() <= 1 {
            return;
        }

        self.past_operands = self.operands.clone();

        let (value, _) = calculate(&self.operands);

        self.operands = vec![Operand {
            operation: None,
            value,
        }]
    }

    pub fn is_empty(&self) -> bool {
        if self.operands.is_empty() || !self.past_operands.is_empty() {
            return true;
        }

        if self.operands.len() > 1 {
            return false;
        }

        if let Some(operand) = self.operands.first() {
            let no_operation = operand.operation.is_none();
            let equals_zero = operand
                .value
                .eq(&OperandValue::Number(NumericValue::default()));
            let is_undefined = operand.value.eq(&OperandValue::Undefined);

            return (no_operation && equals_zero) || is_undefined;
        }

        false
    }

    pub fn current_operation_string(&self) -> SharedString {
        let mut str = String::new();

        for operand in &self.operands {
            str.insert_str(str.len(), &operand.value.to_string());

            if let Some(operation) = &operand.operation {
                str.insert_str(str.len(), &operation.to_string());
            }
        }

        SharedString::new(str)
    }

    pub fn past_operations_string(&self) -> SharedString {
        let mut str = String::new();

        for operand in &self.past_operands {
            str.insert_str(str.len(), &operand.value.to_string());

            if let Some(operation) = &operand.operation {
                str.insert_str(str.len(), &operation.to_string());
            }
        }

        SharedString::new(str)
    }

    pub fn append_number(&mut self, num: usize) {
        let current_operand = self.operands.last_mut();

        if let Some(&mut ref mut operand) = current_operand {
            match operand.value.clone() {
                OperandValue::Number(val) => {
                    if operand.operation.is_some() {
                        let new_val = DBig::from(num);

                        self.operands.push(Operand {
                            value: OperandValue::Number(NumericValue::new(new_val)),
                            operation: None,
                        });

                        return;
                    }

                    if val.has_comma() {
                        let stringified = val.val().to_string();

                        let appended = format!("{}.{}", stringified, num);

                        operand.value = OperandValue::Number(NumericValue::new(
                            DBig::from_str(&appended).unwrap(),
                        ));

                        return;
                    }

                    let (trunc, fract) = val.val().clone().split_at_point();
                    let fract_precision = fract.precision();
                    let trunc_precision = trunc.precision();
                    let precision = 1 + trunc_precision + fract_precision;

                    let zero = dbig!(0);
                    let (trunc, fract) = if fract.gt(&zero) {
                        let stringified = fract.to_string();

                        let appended = format!("{}{}", stringified, num);

                        (trunc.to_string(), appended)
                    } else {
                        let stringified = trunc.to_string();

                        let appended = format!("{}{}", stringified, num);

                        (appended, fract.to_string())
                    };

                    let trunc = DBig::from_str(&trunc).unwrap();
                    let fract = DBig::from_str(&fract).unwrap();

                    operand.value = OperandValue::Number(NumericValue::new(
                        trunc
                            .with_precision(precision)
                            .value()
                            .add(fract)
                            .with_precision(precision)
                            .value(),
                    ));
                }
                OperandValue::Undefined => panic!("Undefined operand encountered"),
            }
        } else {
            self.operands.push(Operand {
                value: OperandValue::Number(NumericValue::new(DBig::from(num))),
                operation: None,
            })
        }
    }

    pub fn add_comma(&mut self) {
        let current_operand = self.operands.last_mut();

        if let Some(&mut ref mut operand) = current_operand {
            if operand.operation.is_some() {
                self.operands.push(Operand {
                    value: OperandValue::Number(NumericValue::new_with_comma(DBig::from(0))),
                    operation: None,
                });
                return;
            }

            match operand.value.clone() {
                OperandValue::Number(val) => {
                    operand.value =
                        OperandValue::Number(NumericValue::new_with_comma(val.val().clone()))
                }
                OperandValue::Undefined => panic!("Undefined operand encountered"),
            }
        } else {
            self.operands.push(Operand {
                value: OperandValue::Number(NumericValue::new_with_comma(DBig::from(0))),
                operation: None,
            })
        }
    }

    pub fn append_operation(&mut self, op: Operation) {
        let current_operand = self.operands.last_mut();
        if let Some(&mut ref mut operand) = current_operand {
            operand.operation = Some(op)
        }

        if !self.past_operands.is_empty() {
            self.past_operands = vec![];
        }
    }

    pub fn remove_last(&mut self) {
        let operands_len = self.operands.len();
        let current_operand = self.operands.last_mut();

        if let Some(&mut ref mut operand) = current_operand {
            if operand.operation.is_some() {
                operand.operation = None;

                return;
            }

            let val = operand.value.clone();

            match val {
                OperandValue::Number(ref num) => {
                    let was_float = num.is_float();

                    if num.eq(&NumericValue::new(dbig!(0))) {
                        if operands_len > 1 {
                            self.operands.pop();
                        }

                        return;
                    };

                    let mut new_value = format!("{}", num);
                    new_value.pop();

                    if new_value.is_empty() && operands_len > 1 {
                        self.operands.pop();
                        return;
                    }

                    let new_value = if new_value.is_empty() {
                        "0".to_string()
                    } else {
                        new_value
                    };

                    let new_value = DBig::from_str(&new_value).unwrap();
                    let num_value = NumericValue::new(new_value.clone());
                    let is_float = num_value.is_float();

                    if was_float && !is_float {
                        operand.value =
                            OperandValue::Number(NumericValue::new_with_comma(new_value));
                    } else {
                        operand.value = OperandValue::Number(num_value);
                    }
                }
                OperandValue::Undefined => {
                    operand.value = OperandValue::default();
                }
            }
        };
    }
}

fn calculate(operands: &[Operand]) -> (OperandValue, Option<Operation>) {
    operands.iter().fold(
        (OperandValue::default(), None),
        |(acc, operation), operand| match (acc, operand.value.clone()) {
            (OperandValue::Number(acc), OperandValue::Number(val)) => {
                if let Some(op) = operation {
                    let new_value: OperandValue = match op {
                        Operation::Addition => OperandValue::Number(NumericValue::new(
                            acc.val()
                                .clone()
                                .with_precision(30)
                                .value()
                                .add(val.val())
                                .with_precision(30)
                                .value(),
                        )),
                        Operation::Subtraction => OperandValue::Number(NumericValue::new(
                            acc.val()
                                .clone()
                                .with_precision(30)
                                .value()
                                .sub(val.val())
                                .with_precision(30)
                                .value(),
                        )),
                        Operation::Multiplication => OperandValue::Number(NumericValue::new(
                            acc.val()
                                .clone()
                                .with_precision(30)
                                .value()
                                .mul(val.val())
                                .with_precision(30)
                                .value(),
                        )),
                        Operation::Division => {
                            if val.val().eq(&dbig!(0)) {
                                OperandValue::Undefined
                            } else {
                                OperandValue::Number(NumericValue::new(
                                    acc.val()
                                        .clone()
                                        .with_precision(30)
                                        .value()
                                        .div(val.val())
                                        .with_precision(30)
                                        .value(),
                                ))
                            }
                        }
                        _ => panic!("Unsupported operation encountered"),
                    };

                    return (new_value, operand.operation.clone());
                };

                (OperandValue::Number(val), operand.operation.clone())
            }
            _ => (OperandValue::Undefined, None),
        },
    )
}

impl Default for Calculation {
    fn default() -> Self {
        Self {
            past_operands: vec![],
            operands: vec![Operand::default()],
        }
    }
}

impl Display for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operation::Addition => f.write_str("+"),
            Operation::Subtraction => f.write_str("-"),
            Operation::Multiplication => f.write_str("⨉"),
            Operation::Division => f.write_str("÷"),
            Operation::Equals => f.write_str("＝"),
        }
    }
}

impl Default for OperandValue {
    fn default() -> Self {
        OperandValue::Number(NumericValue::default())
    }
}

impl Display for OperandValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OperandValue::Number(v) => f.write_fmt(format_args!("{}", v)),
            OperandValue::Undefined => f.write_str("Undefined"),
        }
    }
}

#[cfg(test)]
mod test_sum_calculation {
    use super::*;

    #[test]
    fn calculate_with_only_one_operand() {
        let mut calculation = Calculation {
            past_operands: vec![],
            operands: vec![Operand {
                operation: None,
                value: OperandValue::Number(NumericValue::new(dbig!(5))),
            }],
        };
        calculation.calculate();

        assert_eq!(calculation, calculation.clone());
    }

    #[test]
    fn calculate_with_only_one_operand_and_operation() {
        let mut calculation = Calculation {
            past_operands: vec![],
            operands: vec![Operand {
                operation: Some(Operation::Addition),
                value: OperandValue::Number(NumericValue::new(dbig!(5))),
            }],
        };
        calculation.calculate();

        assert_eq!(calculation, calculation.clone());
    }

    #[test]
    fn calculate_sum_two_values() {
        let mut calculation = Calculation {
            past_operands: vec![],
            operands: vec![
                Operand {
                    operation: Some(Operation::Addition),
                    value: OperandValue::Number(NumericValue::new(dbig!(5))),
                },
                Operand {
                    operation: None,
                    value: OperandValue::Number(NumericValue::new(dbig!(5))),
                },
            ],
        };
        calculation.calculate();

        assert_eq!(
            calculation,
            Calculation {
                past_operands: vec![
                    Operand {
                        operation: Some(Operation::Addition),
                        value: OperandValue::Number(NumericValue::new(dbig!(5))),
                    },
                    Operand {
                        operation: None,
                        value: OperandValue::Number(NumericValue::new(dbig!(5))),
                    }
                ],
                operands: vec![Operand {
                    operation: None,
                    value: OperandValue::Number(NumericValue::new(dbig!(10))),
                }]
            }
        );
    }

    #[test]
    fn calculate_sum_three_values() {
        let mut calculation = Calculation {
            past_operands: vec![],
            operands: vec![
                Operand {
                    operation: Some(Operation::Addition),
                    value: OperandValue::Number(NumericValue::new(dbig!(5))),
                },
                Operand {
                    operation: Some(Operation::Addition),
                    value: OperandValue::Number(NumericValue::new(dbig!(5))),
                },
                Operand {
                    operation: None,
                    value: OperandValue::Number(NumericValue::new(dbig!(5.5))),
                },
            ],
        };
        calculation.calculate();

        assert_eq!(
            calculation,
            Calculation {
                past_operands: vec![
                    Operand {
                        operation: Some(Operation::Addition),
                        value: OperandValue::Number(NumericValue::new(dbig!(5))),
                    },
                    Operand {
                        operation: Some(Operation::Addition),
                        value: OperandValue::Number(NumericValue::new(dbig!(5))),
                    },
                    Operand {
                        operation: None,
                        value: OperandValue::Number(NumericValue::new(dbig!(5.5))),
                    },
                ],
                operands: vec![Operand {
                    operation: None,
                    value: OperandValue::Number(NumericValue::new(dbig!(15.5))),
                }]
            }
        );
    }

    #[test]
    fn calculate_with_negative_value() {
        let mut calculation = Calculation {
            past_operands: vec![],
            operands: vec![
                Operand {
                    operation: Some(Operation::Addition),
                    value: OperandValue::Number(NumericValue::new(dbig!(-10))),
                },
                Operand {
                    operation: None,
                    value: OperandValue::Number(NumericValue::new(dbig!(5))),
                },
            ],
        };
        calculation.calculate();

        assert_eq!(
            calculation,
            Calculation {
                past_operands: vec![
                    Operand {
                        operation: Some(Operation::Addition),
                        value: OperandValue::Number(NumericValue::new(dbig!(-10))),
                    },
                    Operand {
                        operation: None,
                        value: OperandValue::Number(NumericValue::new(dbig!(5))),
                    }
                ],
                operands: vec![Operand {
                    operation: None,
                    value: OperandValue::Number(NumericValue::new(dbig!(-5))),
                }]
            }
        );
    }
}

#[cfg(test)]
mod test_append_number {
    use super::*;

    #[test]
    fn append_when_empty() {
        let mut calculation = Calculation {
            past_operands: vec![],
            operands: Vec::new(),
        };

        calculation.append_number(5);
        assert_eq!(
            calculation,
            Calculation {
                past_operands: vec![],
                operands: vec![Operand {
                    operation: None,
                    value: OperandValue::Number(NumericValue::new(dbig!(5))),
                }],
            }
        );
    }

    #[test]
    fn append_with_comma() {
        let mut calculation = Calculation {
            past_operands: vec![],
            operands: vec![Operand {
                operation: None,
                value: OperandValue::Number(NumericValue::new_with_comma(dbig!(5))),
            }],
        };

        calculation.append_number(5);
        assert_eq!(
            calculation,
            Calculation {
                past_operands: vec![],
                operands: vec![Operand {
                    operation: None,
                    value: OperandValue::Number(NumericValue::new(dbig!(5.5))),
                }],
            }
        );
    }

    #[test]
    fn append_zero_with_comma() {
        let mut calculation = Calculation {
            past_operands: vec![],
            operands: vec![Operand {
                operation: None,
                value: OperandValue::Number(NumericValue::new_with_comma(dbig!(0))),
            }],
        };

        calculation.append_number(5);
        calculation.append_number(5);
        assert_eq!(
            calculation,
            Calculation {
                past_operands: vec![],
                operands: vec![Operand {
                    operation: None,
                    value: OperandValue::Number(NumericValue::new(dbig!(0.55))),
                }],
            }
        );
    }
}

#[cfg(test)]
mod append_operation {
    use super::*;

    #[test]
    fn appends_when_empty() {
        let mut calculation = Calculation::default();
        calculation.append_operation(Operation::Addition);

        assert_eq!(
            calculation,
            Calculation {
                past_operands: vec![],
                operands: vec![Operand {
                    operation: Some(Operation::Addition),
                    value: OperandValue::default(),
                }],
            }
        );
    }

    #[test]
    fn appends_when_having_value() {
        let mut calculation = Calculation::default();
        calculation.append_number(5);
        calculation.append_operation(Operation::Addition);

        assert_eq!(
            calculation,
            Calculation {
                past_operands: vec![],
                operands: vec![Operand {
                    operation: Some(Operation::Addition),
                    value: OperandValue::Number(NumericValue::new(dbig!(5))),
                }],
            }
        );
    }

    #[test]
    fn overrides_operation_on_multiple_appends() {
        let mut calculation = Calculation::default();
        calculation.append_number(5);
        calculation.append_operation(Operation::Addition);
        calculation.append_operation(Operation::Subtraction);
        calculation.append_operation(Operation::Multiplication);

        assert_eq!(
            calculation,
            Calculation {
                past_operands: vec![],
                operands: vec![Operand {
                    operation: Some(Operation::Multiplication),
                    value: OperandValue::Number(NumericValue::new(dbig!(5))),
                }],
            }
        );
    }

    #[test]
    fn appends_operation_on_multiple_operands() {
        let mut calculation = Calculation::default();
        calculation.append_number(5);
        calculation.append_operation(Operation::Multiplication);
        calculation.append_number(10);
        calculation.append_operation(Operation::Subtraction);

        assert_eq!(
            calculation,
            Calculation {
                past_operands: vec![],
                operands: vec![
                    Operand {
                        operation: Some(Operation::Multiplication),
                        value: OperandValue::Number(NumericValue::new(dbig!(5))),
                    },
                    Operand {
                        operation: Some(Operation::Subtraction),
                        value: OperandValue::Number(NumericValue::new(dbig!(10))),
                    },
                ],
            }
        );
    }

    #[test]
    fn clears_previous_operands() {
        let mut calculation = Calculation {
            past_operands: vec![
                Operand {
                    operation: Some(Operation::Multiplication),
                    value: OperandValue::Number(NumericValue::new(dbig!(2))),
                },
                Operand {
                    operation: None,
                    value: OperandValue::Number(NumericValue::new(dbig!(2))),
                },
            ],
            operands: vec![Operand {
                operation: None,
                value: OperandValue::Number(NumericValue::new(dbig!(5))),
            }],
        };

        calculation.append_operation(Operation::Addition);

        assert_eq!(
            calculation,
            Calculation {
                past_operands: vec![],
                operands: vec![Operand {
                    operation: Some(Operation::Addition),
                    value: OperandValue::Number(NumericValue::new(dbig!(5))),
                }],
            }
        )
    }
}

#[cfg(test)]
mod test_is_empty {
    use dashu::dbig;

    use super::*;

    #[test]
    fn is_empty_when_no_operands() {
        let calculation = Calculation::default();

        assert!(calculation.is_empty());
    }

    #[test]
    fn is_empty_when_having_past_operands_and_result() {
        let calculation = Calculation {
            past_operands: vec![Operand {
                operation: None,
                value: OperandValue::Number(NumericValue::new(dbig!(1))),
            }],
            operands: vec![Operand {
                operation: None,
                value: OperandValue::Number(NumericValue::new(dbig!(1))),
            }],
        };

        assert!(calculation.is_empty());
    }

    #[test]
    fn is_empty_when_operand_value_is_zero() {
        let calculation = Calculation {
            past_operands: vec![],
            operands: vec![Operand {
                operation: None,
                value: OperandValue::default(),
            }],
        };

        assert!(calculation.is_empty());
    }

    #[test]
    fn is_not_empty_when_having_operands() {
        let calculation = Calculation {
            past_operands: vec![],
            operands: vec![Operand {
                operation: None,
                value: OperandValue::Number(NumericValue::new(dbig!(1))),
            }],
        };

        assert!(!calculation.is_empty());
    }

    #[test]
    fn is_not_empty_when_having_symbol() {
        let calculation = Calculation {
            past_operands: vec![],
            operands: vec![Operand {
                operation: Some(Operation::Multiplication),
                value: OperandValue::default(),
            }],
        };

        assert!(!calculation.is_empty());
    }

    #[test]
    fn is_not_empty_when_having_multiple_operands() {
        let calculation = Calculation {
            past_operands: vec![],
            operands: vec![Operand::default(), Operand::default()],
        };

        assert!(!calculation.is_empty());
    }
}

#[cfg(test)]
mod test_remove_last {
    use super::*;

    #[test]
    fn remove_when_having_number() {
        let mut calculation = Calculation {
            past_operands: vec![],
            operands: vec![Operand {
                operation: None,
                value: OperandValue::Number(NumericValue::new(dbig!(155))),
            }],
        };

        calculation.remove_last();

        assert_eq!(
            calculation,
            Calculation {
                past_operands: vec![],
                operands: vec![Operand {
                    operation: None,
                    value: OperandValue::Number(NumericValue::new(dbig!(15))),
                }],
            }
        );
    }

    #[test]
    fn remove_to_reach_zero() {
        let mut calculation = Calculation {
            past_operands: vec![],
            operands: vec![Operand {
                operation: None,
                value: OperandValue::Number(NumericValue::new(dbig!(155))),
            }],
        };

        calculation.remove_last();
        calculation.remove_last();
        calculation.remove_last();

        assert_eq!(
            calculation,
            Calculation {
                past_operands: vec![],
                operands: vec![Operand {
                    operation: None,
                    value: OperandValue::Number(NumericValue::new(dbig!(0))),
                }],
            }
        );
    }

    #[test]
    fn remove_does_not_get_further_than_zero() {
        let mut calculation = Calculation {
            past_operands: vec![],
            operands: vec![Operand {
                operation: None,
                value: OperandValue::Number(NumericValue::new(dbig!(155))),
            }],
        };

        calculation.remove_last();
        calculation.remove_last();
        calculation.remove_last();
        calculation.remove_last();
        calculation.remove_last();

        assert_eq!(
            calculation,
            Calculation {
                past_operands: vec![],
                operands: vec![Operand {
                    operation: None,
                    value: OperandValue::Number(NumericValue::new(dbig!(0))),
                }],
            }
        );
    }

    #[test]
    fn removes_operation_before_number() {
        let mut calculation = Calculation {
            past_operands: vec![],
            operands: vec![Operand {
                operation: Some(Operation::Addition),
                value: OperandValue::Number(NumericValue::new(dbig!(155))),
            }],
        };

        calculation.remove_last();

        assert_eq!(
            calculation,
            Calculation {
                past_operands: vec![],
                operands: vec![Operand {
                    operation: None,
                    value: OperandValue::Number(NumericValue::new(dbig!(155))),
                }],
            }
        );
    }

    #[test]
    fn remove_when_float_leaves_the_comma() {
        let mut calculation = Calculation {
            past_operands: vec![],
            operands: vec![Operand {
                operation: None,
                value: OperandValue::Number(NumericValue::new(dbig!(155.5))),
            }],
        };

        calculation.remove_last();

        assert_eq!(
            calculation,
            Calculation {
                past_operands: vec![],
                operands: vec![Operand {
                    operation: None,
                    value: OperandValue::Number(NumericValue::new_with_comma(dbig!(155))),
                }],
            }
        );
    }
}
