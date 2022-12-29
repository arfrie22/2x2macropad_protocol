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
    CommandReleaseConsumer = 0x06,
    CommandSetLed = 0x07,
    CommandClearLed = 0x08,
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
    KeyPress = 0x01,
    ConsumerPress = 0x02,
    String = 0x03,
    Chord = 0x04,
    LoopBegin = 0x05,
    LoopIteration = 0x06,
    LoopEnd = 0x07,
}