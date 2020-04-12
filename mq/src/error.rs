#[derive(Fail, Debug, PartialOrd, PartialEq)]
pub enum HidError {
    #[fail(display = "imkey_device_not_connect")]
    ImkeyDeviceIsNotConnect,
    #[fail(display = "imkey_device_model_not_exist")]
    ImkeyDeviceModelNotExist,
}