#[derive(Fail, Debug, PartialOrd, PartialEq)]
pub enum HidError {
    #[fail(display = "imkey_device_not_connect")]
    DeviceIsNotConnectOrNoVerifyPin,
    #[fail(display = "device_connect_interface_not_called")]
    DeviceConnectInterfaceNotCalled,
    #[fail(display = "device_pin_code_no_verify")]
    DevicePinCodeNoVerify,
}