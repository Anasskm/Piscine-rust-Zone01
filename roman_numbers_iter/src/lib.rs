// Define the RomanDigit enum with the specified variants
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum RomanDigit {
    Nulla,
    I,
    V,
    X,
    L,
    C,
    D,
    M,
}
impl RomanDigit {
    pub fn to(&self) -> u32 {
        match self {
            RomanDigit::Nulla => 0,
            RomanDigit::I => 1,
            RomanDigit::V => 5,
            RomanDigit::X => 10,
            RomanDigit::L => 50,
            RomanDigit::C => 100,
            RomanDigit::D => 500,
            RomanDigit::M => 1000,
        }
    }
}
// Define the RomanNumber struct as a wrapper for a vector of RomanDigit
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RomanNumber(pub Vec<RomanDigit>);

// Define an enum to represent either a single RomanDigit or a pair for subtractive notation
enum RomanValue {
    Single(RomanDigit),
    Pair(RomanDigit, RomanDigit),
}

impl From<u32> for RomanNumber {
    fn from(mut num: u32) -> Self {
        let mut digits = Vec::new();
        // Update the values to use RomanValue
        let values = vec![
            (1000, RomanValue::Single(RomanDigit::M)),
            (900, RomanValue::Pair(RomanDigit::C, RomanDigit::M)),
            (500, RomanValue::Single(RomanDigit::D)),
            (400, RomanValue::Pair(RomanDigit::C, RomanDigit::D)),
            (100, RomanValue::Single(RomanDigit::C)),
            (90, RomanValue::Pair(RomanDigit::X, RomanDigit::C)),
            (50, RomanValue::Single(RomanDigit::L)),
            (40, RomanValue::Pair(RomanDigit::X, RomanDigit::L)),
            (10, RomanValue::Single(RomanDigit::X)),
            (9, RomanValue::Pair(RomanDigit::I, RomanDigit::X)),
            (5, RomanValue::Single(RomanDigit::V)),
            (4, RomanValue::Pair(RomanDigit::I, RomanDigit::V)),
            (1, RomanValue::Single(RomanDigit::I)),
        ];

        // Iterate over the values and match them to the input number
        for (value, roman_value) in values {
            while num >= value {
                num -= value;
                match roman_value {
                    RomanValue::Single(digit) => digits.push(digit),
                    RomanValue::Pair(digit1, digit2) => {
                        digits.push(digit1);
                        digits.push(digit2);
                    }
                }
            }
        }

        // Handle the special case for 0 (Nulla)
        if digits.is_empty() {
            digits.push(RomanDigit::Nulla);
        }

        RomanNumber(digits)
    }
}

impl Iterator for RomanNumber {
    type Item = RomanNumber;

    fn next(&mut self) -> Option<Self::Item> {
        Some(RomanNumber::from(self.to_u32() + 1))
    }
}

impl RomanNumber {
    pub fn to_u32(&self) -> u32 {
        if self.0.len() == 0 {
            return 0;
        }
        let mut result = 0;
        let mut last = self.0[0];
        for i in 1..self.0.len() {
            if self.0[i].to() as i32 <= last.to() as i32 {
                result += last.to() as i32;
            } else {
                result -= last.to() as i32;
            }
            last = self.0[i];
        }
        result += last.to() as i32;
        result as u32
    }
}
