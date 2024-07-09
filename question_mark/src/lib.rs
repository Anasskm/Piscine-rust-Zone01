pub struct One {
    // expected public fields
    pub first_layer: Option<Two>,
}
pub struct Two {
    // expected public fields
    pub second_layer: Option<Three>,
}
pub struct Three {
    // expected public fields
    pub third_layer: Option<Four>,
}
pub struct Four {
    // expected public fields
    pub fourth_layer: Option<u16>,
}

impl One {
    pub fn get_fourth_layer(&self) -> Option<u16> {
        self.first_layer
            .as_ref()?
            .second_layer
            .as_ref()?
            .third_layer
            .as_ref()?
            .fourth_layer
    }
}
