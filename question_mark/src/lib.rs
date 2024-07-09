pub struct One {
    // expected public fields
    first_layer: Option<Two>,
}
pub struct Two {
    // expected public fields
    second_layer: Option<Three>,
}
pub struct Three {
    // expected public fields
    third_layer: Option<Four>,
}
pub struct Four {
    // expected public fields
    fourth_layer: Option<u16>,
}

impl One {
    pub fn get_fourth_layer(&self) -> Option<u16> {
        self.first_layer.as_ref()?.second_layer.as_ref()?.third_layer.as_ref()?.fourth_layer
    }
}