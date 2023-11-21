use strum_macros::{Display, EnumString};

#[derive(EnumString, Display)]
pub enum Segment {
    #[strum(serialize = "argument")]
    Argument,
    #[strum(serialize = "local")]
    Local,
    #[strum(serialize = "static")]
    Static,
    #[strum(serialize = "constant")]
    Constant,
    #[strum(serialize = "this")]
    This,
    #[strum(serialize = "that")]
    That,
    #[strum(serialize = "pointer")]
    Pointer,
    #[strum(serialize = "temp")]
    Temp,
}
