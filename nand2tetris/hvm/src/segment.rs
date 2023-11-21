use strum_macros::{Display, EnumString};

#[derive(EnumString, Display)]
#[strum(serialize_all = "lowercase")]
pub enum Segment {
    Argument,
    Local,
    Static,
    Constant,
    This,
    That,
    Pointer,
    Temp,
}
