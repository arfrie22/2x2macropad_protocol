use num_enum::{FromPrimitive, IntoPrimitive};
use packed_struct::prelude::PrimitiveEnum;

#[repr(u8)]
#[derive(
    Debug,
    Copy,
    Clone,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    PrimitiveEnum,
    Hash,
    IntoPrimitive,
    FromPrimitive,
)]
pub enum MacroCommand {
    #[num_enum(default)]
    CommandTerminator = 0x00,
    CommandSectionAnnotation = 0x01,
    CommandDelay = 0x02,
    CommandPressKey = 0x03,
    CommandReleaseKey = 0x04,
    CommandConsumer = 0x05,
    CommandSetLed = 0x06,
    CommandClearLed = 0x07,
}

#[repr(u8)]
#[derive(
    Debug,
    Copy,
    Clone,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    PrimitiveEnum,
    Hash,
    IntoPrimitive,
    FromPrimitive,
)]
pub enum MacroSectionAnnotation {
    #[num_enum(default)]
    None = 0x00,
    String = 0x01,
    Chord = 0x02,
    LoopBegin = 0x03,
    LoopIteration = 0x04,
    LoopEnd = 0x05,
}