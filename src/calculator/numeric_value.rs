use dashu::Decimal;
use std::fmt::Display;

#[derive(Debug, Clone, PartialEq)]
pub struct NumericValue {
    value: Decimal,
    comma: bool,
}

impl NumericValue {
    pub fn new(value: Decimal) -> Self {
        NumericValue {
            value,
            comma: false,
        }
    }

    pub fn new_with_comma(value: Decimal) -> Self {
        NumericValue { value, comma: true }
    }

    pub fn is_float(&self) -> bool {
        self.value.fract() != Decimal::ZERO
    }

    pub fn val(&self) -> &Decimal {
        &self.value
    }

    pub fn has_comma(&self) -> bool {
        self.comma
    }
}

impl Display for NumericValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.comma && !self.is_float() {
            return write!(f, "{}.", self.value);
        }

        write!(f, "{}", self.value)
    }
}

impl Default for NumericValue {
    fn default() -> Self {
        NumericValue {
            value: Decimal::ZERO,
            comma: false,
        }
    }
}
