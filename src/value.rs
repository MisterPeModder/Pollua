pub use crate::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ValueType {
    Nil,
    Boolean,
    Number,
    String,
    Function,
    LightUserdata,
    Userdata,
    Thread,
    Table,
}
