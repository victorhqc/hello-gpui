use dashu::Decimal;
use dashu_float::{round::mode::HalfAway, DBig, FBig};
use gpui::SharedString;
use std::{
    fmt::Display,
    ops::{Add, Sub},
    str::FromStr,
};

#[derive(Debug, Clone, PartialEq)]
pub enum Operation {
    Division,
    Times,
    Minus,
    Plus,
    Equals,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Operand {
    operation: Option<Operation>,
    value: Decimal,
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

        let (value, _): (FBig<HalfAway, 10>, Option<Operation>) =
            self.operands
                .iter()
                .fold((dbig!(0), None), |(acc, operation), operand| {
                    if let Some(op) = operation {
                        let new_value = match op {
                            Operation::Plus => acc.add(operand.value.clone()).with_precision(10),
                            Operation::Minus => acc.sub(operand.value.clone()).with_precision(10),
                            _ => todo!(),
                        };

                        return (new_value.value(), operand.operation.clone());
                    };

                    (operand.value.clone(), operand.operation.clone())
                });

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
            return operand.operation.is_none() && operand.value.eq(&dbig!(0));
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
            if operand.operation.is_some() {
                let new_val = DBig::from(num);

                self.operands.push(Operand {
                    value: new_val,
                    operation: None,
                });

                return;
            }

            let (trunc, fract) = operand.value.clone().split_at_point();

            let zero = dbig!(0);
            let (trunc, fract) = if fract.gt(&zero) && trunc.ne(&zero) {
                let stringified = fract.to_string();

                let appended = format!("{}{}", stringified, num);

                (trunc.to_string(), appended)
            } else {
                let stringified = trunc.to_string();

                let appended = format!("{}{}", stringified, num);

                (appended, fract.to_string())
            };

            operand.value = DBig::from_str(&format!("{}.{}", trunc, fract)).unwrap();
        } else {
            self.operands.push(Operand {
                value: DBig::from(num),
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

            if val.eq(&dbig!(0)) {
                if operands_len > 1 {
                    self.operands.pop();
                }

                return;
            };

            let mut new_value = format!("{}", val);
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
            operand.value = new_value;
        };
    }
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
            Operation::Plus => f.write_str("+"),
            Operation::Minus => f.write_str("-"),
            Operation::Times => f.write_str("⨉"),
            Operation::Division => f.write_str("÷"),
            Operation::Equals => f.write_str("＝"),
        }
    }
}

impl Default for Operand {
    fn default() -> Self {
        Operand {
            operation: None,
            value: dbig!(0),
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
                value: dbig!(5),
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
                operation: Some(Operation::Plus),
                value: dbig!(5),
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
                    operation: Some(Operation::Plus),
                    value: dbig!(5),
                },
                Operand {
                    operation: None,
                    value: dbig!(5),
                },
            ],
        };
        calculation.calculate();

        assert_eq!(
            calculation,
            Calculation {
                past_operands: vec![
                    Operand {
                        operation: Some(Operation::Plus),
                        value: dbig!(5),
                    },
                    Operand {
                        operation: None,
                        value: dbig!(5),
                    }
                ],
                operands: vec![Operand {
                    operation: None,
                    value: dbig!(10)
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
                    operation: Some(Operation::Plus),
                    value: dbig!(5),
                },
                Operand {
                    operation: Some(Operation::Plus),
                    value: dbig!(5),
                },
                Operand {
                    operation: None,
                    value: dbig!(5.5),
                },
            ],
        };
        calculation.calculate();

        assert_eq!(
            calculation,
            Calculation {
                past_operands: vec![
                    Operand {
                        operation: Some(Operation::Plus),
                        value: dbig!(5),
                    },
                    Operand {
                        operation: Some(Operation::Plus),
                        value: dbig!(5),
                    },
                    Operand {
                        operation: None,
                        value: dbig!(5.5),
                    },
                ],
                operands: vec![Operand {
                    operation: None,
                    value: dbig!(15.5)
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
                    operation: Some(Operation::Plus),
                    value: dbig!(-10),
                },
                Operand {
                    operation: None,
                    value: dbig!(5),
                },
            ],
        };
        calculation.calculate();

        assert_eq!(
            calculation,
            Calculation {
                past_operands: vec![
                    Operand {
                        operation: Some(Operation::Plus),
                        value: dbig!(-10),
                    },
                    Operand {
                        operation: None,
                        value: dbig!(5),
                    }
                ],
                operands: vec![Operand {
                    operation: None,
                    value: dbig!(-5)
                }]
            }
        );
    }
}

// #[cfg(test)]
// mod test_append_number {
//     use super::*;

//     #[test]
//     fn append_when_empty() {
//         let mut calculation = Calculation {
//             past_operands: vec![],
//             operands: Vec::new(),
//         };

//         calculation.append_number(5);
//         assert_eq!(
//             calculation,
//             Calculation {
//                 past_operands: vec![],
//                 operands: vec![Operand {
//                     operation: None,
//                     value: OperandValue::Decimal(dbig!(5)),
//                 }],
//             }
//         );
//     }
// }

#[cfg(test)]
mod append_operation {
    use super::*;

    #[test]
    fn appends_when_empty() {
        let mut calculation = Calculation::default();
        calculation.append_operation(Operation::Plus);

        assert_eq!(
            calculation,
            Calculation {
                past_operands: vec![],
                operands: vec![Operand {
                    operation: Some(Operation::Plus),
                    value: dbig!(0),
                }],
            }
        );
    }

    #[test]
    fn appends_when_having_value() {
        let mut calculation = Calculation::default();
        calculation.append_number(5);
        calculation.append_operation(Operation::Plus);

        assert_eq!(
            calculation,
            Calculation {
                past_operands: vec![],
                operands: vec![Operand {
                    operation: Some(Operation::Plus),
                    value: dbig!(5),
                }],
            }
        );
    }

    #[test]
    fn overrides_operation_on_multiple_appends() {
        let mut calculation = Calculation::default();
        calculation.append_number(5);
        calculation.append_operation(Operation::Plus);
        calculation.append_operation(Operation::Minus);
        calculation.append_operation(Operation::Times);

        assert_eq!(
            calculation,
            Calculation {
                past_operands: vec![],
                operands: vec![Operand {
                    operation: Some(Operation::Times),
                    value: dbig!(5),
                }],
            }
        );
    }

    #[test]
    fn appends_operation_on_multiple_operands() {
        let mut calculation = Calculation::default();
        calculation.append_number(5);
        calculation.append_operation(Operation::Times);
        calculation.append_number(10);
        calculation.append_operation(Operation::Minus);

        assert_eq!(
            calculation,
            Calculation {
                past_operands: vec![],
                operands: vec![
                    Operand {
                        operation: Some(Operation::Times),
                        value: dbig!(5),
                    },
                    Operand {
                        operation: Some(Operation::Minus),
                        value: dbig!(10),
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
                    operation: Some(Operation::Times),
                    value: dbig!(2),
                },
                Operand {
                    operation: None,
                    value: dbig!(2),
                },
            ],
            operands: vec![Operand {
                operation: None,
                value: dbig!(5),
            }],
        };

        calculation.append_operation(Operation::Plus);

        assert_eq!(
            calculation,
            Calculation {
                past_operands: vec![],
                operands: vec![Operand {
                    operation: Some(Operation::Plus),
                    value: dbig!(5),
                }],
            }
        )
    }
}

#[cfg(test)]
mod test_is_empty {
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
                value: dbig!(1),
            }],
            operands: vec![Operand {
                operation: None,
                value: dbig!(1),
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
                value: dbig!(0),
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
                value: dbig!(1),
            }],
        };

        assert!(!calculation.is_empty());
    }

    #[test]
    fn is_not_empty_when_having_symbol() {
        let calculation = Calculation {
            past_operands: vec![],
            operands: vec![Operand {
                operation: Some(Operation::Times),
                value: dbig!(0),
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
