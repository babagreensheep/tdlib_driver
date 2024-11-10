include!(concat!(env!("OUT_DIR"), "/td_api.rs"));

impl From<OptionValue> for String {
    fn from(option_value: OptionValue) -> Self {
        match option_value {
            OptionValue::optionValueBoolean(optionValueBoolean { value: bool }) => bool.to_string(),
            OptionValue::optionValueEmpty => "".into(),
            OptionValue::optionValueInteger(optionValueInteger { value: integer }) => {
                integer.to_string()
            }
            OptionValue::optionValueString(optionValueString { value: string }) => string,
        }
    }
}
