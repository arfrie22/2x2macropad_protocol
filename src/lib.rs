pub mod data_protocol;
pub mod macro_protocol;

#[cfg(test)]
mod tests {
    use strum::EnumCount;

    use crate::data_protocol;
    use crate::macro_protocol;

    #[test]
    fn test_data_command() {
        assert_eq!(data_protocol::DataCommand::None as u8, 0x00);
        assert_eq!(data_protocol::DataCommand::GetProtocolVersion as u8, 0x01);
        assert_eq!(data_protocol::DataCommand::ReadMacro as u8, 0x02);
        assert_eq!(data_protocol::DataCommand::WriteMacro as u8, 0x03);
        assert_eq!(data_protocol::DataCommand::ClearMacro as u8, 0x04);
        assert_eq!(data_protocol::DataCommand::ValidateMacro as u8, 0x05);
        assert_eq!(data_protocol::DataCommand::ReadConfig as u8, 0x06);
        assert_eq!(data_protocol::DataCommand::WriteConfig as u8, 0x07);
        assert_eq!(data_protocol::DataCommand::GetLed as u8, 0x08);
        assert_eq!(data_protocol::DataCommand::SetLed as u8, 0x09);
        assert_eq!(data_protocol::DataCommand::ReadKeyConfig as u8, 0x0A);
        assert_eq!(data_protocol::DataCommand::WriteKeyConfig as u8, 0x0B);

        assert_eq!(data_protocol::DataCommand::EnterBootloader as u8, 0xFE);
        assert_eq!(data_protocol::DataCommand::Error as u8, 0xFF);

        assert_eq!(data_protocol::DataCommand::COUNT as u8, 0x0C);
    }

    #[test]
    fn test_config_elements() {
        assert_eq!(data_protocol::ConfigElements::Version as u8, 0x00);
        assert_eq!(data_protocol::ConfigElements::TapSpeed as u8, 0x01);
        assert_eq!(data_protocol::ConfigElements::HoldSpeed as u8, 0x02);
        assert_eq!(data_protocol::ConfigElements::DefaultDelay as u8, 0x03);
        assert_eq!(data_protocol::ConfigElements::Error as u8, 0xFF);
    }

    #[test]
    fn test_key_mode() {
        assert_eq!(data_protocol::KeyMode::MacroMode as u8, 0x00);
        assert_eq!(data_protocol::KeyMode::SingleTapMode as u8, 0x01);
        assert_eq!(data_protocol::KeyMode::KeyboardMode as u8, 0x02);
        assert_eq!(data_protocol::KeyMode::ConsumerMode as u8, 0x03);
    }

    #[test]
    fn test_macro_protocol() {
        assert_eq!(macro_protocol::MacroCommand::CommandTerminator as u8, 0x00);
        assert_eq!(macro_protocol::MacroCommand::CommandDelay as u8, 0x01);
        assert_eq!(macro_protocol::MacroCommand::CommandPressKey as u8, 0x02);
        assert_eq!(macro_protocol::MacroCommand::CommandReleaseKey as u8, 0x03);
        assert_eq!(macro_protocol::MacroCommand::CommandConsumer as u8, 0x04);
        assert_eq!(macro_protocol::MacroCommand::CommandSetLed as u8, 0x05);
        assert_eq!(macro_protocol::MacroCommand::CommandClearLed as u8, 0x06);
    }
}