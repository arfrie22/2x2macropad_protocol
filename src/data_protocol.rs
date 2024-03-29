use num_enum::{IntoPrimitive, FromPrimitive};
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
pub enum DataCommand {
    None = 0x00,
    GetBuildVersion = 0x01,
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
    GetBuildInfo = 0x0C,

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
#[cfg_attr(feature = "multi_versioned", derive(MultiVersion))]
pub enum BuildInfoElements {
    FirmwareVersion = 0x00,
    BuildDate = 0x01,
    BuildTimestamp = 0x02,
    BuildProfile = 0x03,
    GitHash = 0x04,
    GitBranch = 0x05,
    GitSemver = 0x06,

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
#[cfg_attr(feature = "multi_versioned", derive(MultiVersion))]
pub enum ConfigElements {
    TapSpeed = 0x00,
    HoldSpeed = 0x01,

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
#[cfg_attr(feature = "multi_versioned", derive(MultiVersion))]
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
#[cfg_attr(feature = "multi_versioned", derive(MultiVersion))]
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
#[cfg_attr(feature = "multi_versioned", derive(MultiVersion))]
pub enum LedCommand {
    None = 0x00,

    // Single LED Control
    BaseColor = 0x01,
    Effect = 0x02,
    Brightness = 0x03,
    EffectPeriod = 0x04,
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
#[cfg_attr(feature = "multi_versioned", derive(MultiVersion))]
pub enum LedEffect {
    #[num_enum(default)]
    None = 0x00,
    Static = 0x01,
    Breathing = 0x02,
    BreathingSpaced = 0x03,
    ColorCycle = 0x04,
    Rainbow = 0x05,
}

#[cfg(feature = "std")]
impl std::fmt::Display for LedEffect {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            LedEffect::None => write!(f, "None"),
            LedEffect::Static => write!(f, "Static"),
            LedEffect::Breathing => write!(f, "Breathing"),
            LedEffect::BreathingSpaced => write!(f, "Breathing Spaced"),
            LedEffect::ColorCycle => write!(f, "Color Cycle"),
            LedEffect::Rainbow => write!(f, "Rainbow"),
        }
    }
}