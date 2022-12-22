use num_enum::{IntoPrimitive, FromPrimitive};
use packed_struct::prelude::PrimitiveEnum;

pub const PROTOCOL_VERSION: u16 = 1;

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
pub enum DataCommand {
    None = 0x00,
    GetProtocolVersion = 0x01,
    ReadMacro = 0x02,
    WriteMacro = 0x03,
    ClearMacro = 0x04,
    ValidateMacro = 0x05,
    ReadConfig = 0x06,
    WriteConfig = 0x07,
    GetLed = 0x08,
    SetLed = 0x09,
    ReadKeyConfig = 0x0A,
    WriteKeyConfig = 0x0B,


    // Extra commands not included in the count
    EnterBootloader = 0xFE,
    #[num_enum(default)]
    Error = 0xFF
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
pub enum ConfigElements {
    Version = 0x00,
    TapSpeed = 0x01,
    HoldSpeed = 0x02,
    DefaultDelay = 0x03,

    //...
    #[num_enum(default)]
    Error = 0xFF
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
pub enum KeyConfigElements {
    KeyMode = 0x00,
    KeyboardData = 0x01,
    ConsumerData = 0x02,
    KeyColor = 0x03,

    //...
    #[num_enum(default)]
    Error = 0xFF
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
pub enum KeyMode {
    #[num_enum(default)]
    MacroMode = 0x00,
    SingleTapMode = 0x01,
    KeyboardMode = 0x02,
    ConsumerMode = 0x03,
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
pub enum LedCommand {
    None = 0x00,

    // Single LED Control
    BaseColor = 0x01,
    Effect = 0x02,
    Brightness = 0x03,
    EffectSpeed = 0x04,
    EffectOffset = 0x05,

    //...
    #[num_enum(default)]
    Error = 0xFF
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
pub enum LedEffect {
    #[num_enum(default)]
    None = 0x00,
    Static = 0x01,
    Breathing = 0x02,
    BreathingSpaced = 0x03,
    ColorCycle = 0x04,
    Rainbow = 0x05,
}