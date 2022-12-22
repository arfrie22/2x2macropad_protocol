use num_enum::{IntoPrimitive, TryFromPrimitive};
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
    TryFromPrimitive,
)]
pub enum MacroCommand {
    CommandTerminator = 0x00,
    CommandDelay = 0x01,
    CommandPressKey = 0x02,
    CommandReleaseKey = 0x03,
    CommandConsumer = 0x04,
    CommandSetLed = 0x05,
    CommandClearLed = 0x06,
}