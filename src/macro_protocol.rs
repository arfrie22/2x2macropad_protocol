use num_enum::{FromPrimitive, IntoPrimitive};
use packed_struct::prelude::PrimitiveEnum;

#[cfg(feature = "multi_versioned")]
use multi_version::MultiVersion;

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
#[cfg_attr(feature = "multi_versioned", derive(MultiVersion))]
pub enum MacroCommand {
    #[num_enum(default)]
    Empty = 0x00,
    LoopBegin = 0x01,
    LoopEnd = 0x02,
    SetLed = 0x03,
    ClearLed = 0x04,
    KeyDown = 0x05,
    KeyUp = 0x06,
    KeyPress = 0x07,
    ConsumerPress = 0x08,
    TypeString = 0x09,
    Chord = 0x0A,
}