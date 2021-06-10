#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum ObjectType {
    Struct,
    Enum,
    Function,
    Trait,
    Mod,
    Impl,
    RePublish,
    Use,
    ImplicitUse
}