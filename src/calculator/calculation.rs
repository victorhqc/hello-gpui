use dashu::Decimal;
use dashu_float::DBig;
use gpui::SharedString;
use std::{fmt::Display, str::FromStr};

#[derive(Debug)]
pub enum Operation {
    Division,
    Times,
    Minus,
    Plus,
    Equals,
}

#[derive(Debug, Clone)]
pub enum OperandValue {
    Decimal(Decimal),
}

#[derive(Debug)]
pub struct Operand {
    operation: Option<Operation>,
    value: OperandValue,
}

#[derive(Debug)]
pub struct Calculation {
    result: Option<OperandValue>,
    operands: Vec<Operand>,
}

impl Calculation {
    pub fn is_empty(&self) -> bool {
        if self.operands.is_empty() || self.result.is_some() {
            return true;
        }

        if self.operands.len() > 1 {
            return false;
        }

        if let Some(operand) = self.operands.first() {
            return operand.operation.is_none() && operand.value.is_empty();
        }

        false
    }

    pub fn to_shared_string(&self) -> SharedString {
        if let Some(result) = &self.result {
            return SharedString::new(result.to_string());
        };

        let mut str = String::new();

        for operand in &self.operands {
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
                    value: OperandValue::Decimal(new_val),
                    operation: None,
                });

                return;
            }

            match operand.value.clone() {
                OperandValue::Decimal(v) => {
                    let (trunc, fract) = v.split_at_point();

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

                    operand.value = OperandValue::Decimal(
                        DBig::from_str(&format!("{}.{}", trunc, fract)).unwrap(),
                    );
                }
            }
        } else {
            self.operands.push(Operand {
                value: OperandValue::Decimal(DBig::from(num)),
                operation: None,
            })
        }
    }

    pub fn append_operation(&mut self, op: Operation) {
        let current_operand = self.operands.last_mut();
        if let Some(&mut ref mut operand) = current_operand {
            operand.operation = Some(op)
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

            match operand.value.clone() {
                OperandValue::Decimal(val) => {
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

                    operand.value = OperandValue::Decimal(new_value);
                }
            }
        };
    }
}

impl Default for Calculation {
    fn default() -> Self {
        Self {
            result: None,
            operands: vec![Operand {
                operation: None,
                value: OperandValue::Decimal(dbig!(0)),
            }],
        }
    }
}

impl Display for OperandValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OperandValue::Decimal(v) => f.write_str(&v.to_string()),
        }
    }
}

impl OperandValue {
    pub fn is_empty(&self) -> bool {
        match self {
            OperandValue::Decimal(v) => v.eq(&dbig!(0)),
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

#[cfg(test)]
mod test_is_empty {
    use super::*;

    #[test]
    fn is_empty_when_no_operands() {
        let calculation = Calculation {
            result: None,
            operands: Vec::new(),
        };

        assert!(calculation.is_empty());
    }

    #[test]
    fn is_empty_when_having_result() {
        let calculation = Calculation {
            result: Some(OperandValue::Decimal(dbig!(1))),
            operands: vec![Operand {
                operation: None,
                value: OperandValue::Decimal(dbig!(1)),
            }],
        };

        assert!(calculation.is_empty());
    }

    #[test]
    fn is_empty_when_operand_value_is_zero() {
        let calculation = Calculation {
            result: None,
            operands: vec![Operand {
                operation: None,
                value: OperandValue::Decimal(dbig!(0)),
            }],
        };

        assert!(calculation.is_empty());
    }

    #[test]
    fn is_not_empty_when_having_operands() {
        let calculation = Calculation {
            result: None,
            operands: vec![Operand {
                operation: None,
                value: OperandValue::Decimal(dbig!(1)),
            }],
        };

        assert!(!calculation.is_empty());
    }

    #[test]
    fn is_not_empty_when_having_symbol() {
        let calculation = Calculation {
            result: None,
            operands: vec![Operand {
                operation: Some(Operation::Times),
                value: OperandValue::Decimal(dbig!(0)),
            }],
        };

        assert!(!calculation.is_empty());
    }

    #[test]
    fn is_not_empty_when_having_multiple_operands() {
        let calculation = Calculation {
            result: None,
            operands: vec![
                Operand {
                    operation: None,
                    value: OperandValue::Decimal(dbig!(0)),
                },
                Operand {
                    operation: None,
                    value: OperandValue::Decimal(dbig!(0)),
                },
            ],
        };

        assert!(!calculation.is_empty());
    }
}
