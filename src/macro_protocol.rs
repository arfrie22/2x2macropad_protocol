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
    CommandDelay = 0x01,
    CommandPressKey = 0x02,
    CommandReleaseKey = 0x03,
    CommandConsumer = 0x04,
    CommandSetLed = 0x05,
    CommandClearLed = 0x06,
}