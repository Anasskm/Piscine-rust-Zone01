use std::{collections::HashMap, num::ParseFloatError};

pub struct Flag {
    // expected public fields
    pub short_hand: String,
    pub long_hand: String,
    pub desc: String,
}

impl Flag {
    pub fn opt_flag(l_h: &str, d: &str) -> Flag {
        Flag {
            desc: d.to_string(),
            short_hand: format!("-{}", l_h.chars().next().unwrap()),
            long_hand: format!("--{}", l_h),
        }
    }
}

pub type Callback = fn(&str, &str) -> Result<String, ParseFloatError>;

pub struct FlagsHandler {
    pub flags: HashMap<(String, String), Callback>,
}

impl FlagsHandler {
    pub fn add_flag(&mut self, flag: (String, String), func: Callback) {
        self.flags.insert(flag, func);
    }
    pub fn exec_func(&mut self, flag: (String, String), argv: &[&str]) -> String {
        match self.flags[&flag](argv[0], argv[1]) {
            Ok(r) => r,
            Err(e) => e.to_string(),
        }
    }
}

pub fn div(a: &str, b: &str) -> Result<String, ParseFloatError> {
    let x: f32 = a.parse()?;
    let y: f32 = b.parse()?;
    let res = x / y;
    Ok(res.to_string())
}
pub fn rem(a: &str, b: &str) -> Result<String, ParseFloatError> {
    let x: f32 = a.parse()?;
    let y: f32 = b.parse()?;
    let res = x % y;
    Ok(res.to_string())
}
