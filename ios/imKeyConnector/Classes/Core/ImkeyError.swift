//
//  AppError.swift
//  token
//
//  Created by James Chen on 2016/09/20.
//  Copyright © 2016 imToken PTE. LTD. All rights reserved.
//

import Foundation

// Base protocol of errors that could be thrown from imToken.
public protocol ImkeyError: Error {
  var message: String { get }
}

public protocol ImkeyDescError: ImkeyError { }

public protocol ImkeyCodeError: ImkeyError { }

public extension ImkeyDescError where Self: RawRepresentable, Self.RawValue == String {
  var message: String {
    return rawValue
  }
}

public enum AddressError: String, ImkeyDescError {
  case invalid = "address_invalid"
  case alreadyExist = "address_already_exist"
}

public enum GenericError: String, ImkeyDescError {
  case importFailed = "import_failed"
  case generateFailed = "generate_failed"
  case deleteWalletFailed = "delete_wallet_failed"
  case walletNotFound = "wallet_not_found"
  case operationUnsupported = "operation_unsupported"
  case unknownError = "unknown_error"
  case unsupportedChain = "unsupported_chain"
  case storeWalletFailed = "store_wallet_failed"
  case paramError = "param_error"
  case wifWrongNetwork = "wif_wrong_network"
  case insufficientFunds = "insufficient_funds"
  case amountLessThanMinimum = "imkey_amount_less_than_minimum"
  case enableBluetoothFailed = "enable_bluetooth_failed"
}

public enum EOSError: String, ImkeyDescError {
  case accountNameAlreadySet = "Only can set accountName once in eos wallet"
  case privatePublicNotMatch = "eos_private_public_not_match"
  case publicKeyNotFound = "eos_public_key_not_found"
  case accountNameInvalid = "eos_account_name_invalid"
}

public enum IdentityError: String, ImkeyDescError {
  case invalidIdentity = "invalid_identity"
  case unsupportEncryptionDataVersion = "unsupport_encryption_data_version"
  case invalidEncryptionDataSignature = "invalid_encryption_data_signature"
}

public enum NetError: String, ImkeyDescError {
  case connectFail = "connect_fail"
  case serverError = "server_error"
}

public enum InternalError: String, ImkeyDescError {
  case parseFail = "parse_fail"
  case serverReport = "server_return_exception"
  case unknown = "unknown"
  case imkeyNotConnected = "imkey_not_connected"
}

public enum TSMError: String, ImkeyDescError {
  case systemError = "imkey_tsm_system_error"
  case deviceIllegal = "imkey_tsm_device_illegal"
  case deviceStopUsing = "imkey_tsm_device_stop_using"
  case deviceUpdateCheckFail = "imkey_tsm_device_update_check_fail"
  case deviceActiveFail = "imkey_tsm_device_active_fail"
  case deviceAuthCheckFail = "imkey_tsm_device_authenticity_check_fail"
  case receiptCheckFail = "imkey_tsm_receipt_check_fail"
  case appDownloadFail = "imkey_tsm_app_download_fail"
  case appUpdateFail = "imkey_tsm_app_update_fail"
  case appDeleteFail = "imkey_tsm_app_delete_fail"
  case internalServerError = "imkey_tsm_internal_server_error"
  case networkError = "imkey_tsm_network_error"
  case deviceNotActive = "imkey_tsm_device_not_activated"
  case oceCertCheckFail = "imkey_tsm_oce_cert_check_fail"
  
  
  public static func fromCode(code:String) -> TSMError{
    Log.d("error code :\(code)")
    switch code {
    case "BSE0017":
      return .deviceIllegal
    case "BSE0019":
      return .deviceStopUsing
    case "BSE0018":
      return .deviceUpdateCheckFail
    case "BSE0015":
      return .deviceActiveFail
    case "BSE0009":
      return .deviceAuthCheckFail
    case "BSE0012":
      return .receiptCheckFail
    case "BAPP0006":
      return .appDownloadFail
    case "BAPP0008":
      return .appUpdateFail
    case "BAPP0011":
      return .appDeleteFail
    case "BSE0008":
      return .deviceIllegal
    case "BSE0007":
      return .deviceNotActive
    case "BSE0010":
      return .oceCertCheckFail
    default:
      return .systemError
    }
  }
}

public enum SDKError: String, ImkeyDescError {
  case illegalArgument = "imkey_sdk_illegal_argument"
  case unknownError = "imkey_unknow_error"
  case pubKeyVerifyFailed = "imkey_publickey_mismatch_with_path"
  case jsonError = "imkey_sdk_json_parse_error"
  case unwrapError = "imkey_unwrap_error"//should not happen
  case pubkeyInvalid = "imkey_public_key_invalid"
  case signVerifyFail = "imkey_signature_verify_fail"
  case secertInvalid = "imkey_se_cert_invalid"
  case notBindCheck = "imkey_not_bind_check"
  case notHexOrString = "Data shoud be string or hex"
  case addressVerifyFailed = "imkey_address_mismatch_with_path"
  case connectFail = "imkey_connect_fail"
  case authCodeError = "authcode_error"
  case pathIllegal = "imkey_path_illegal"
  case exceededMaxUtxoNum = "imkey_exceeded_max_utxo_number"
}

//#define CKR_OK                                  0x00000000  // 正确
//#define CKR_ARGUMENTS_BAD                       0x00000007  // 参数错误
//#define CKR_DEVICE_ERROR                        0x00000030  // 设备异常
//#define CKR_BUFFER_TOO_SMALL                    0x00000150  // 参数空间太小
//#define CKR_CRYPTOKI_NOT_INITIALIZED            0x00000190  // SDK尚未初始化
//#define CKR_CRYPTOKI_ALREADY_INITIALIZED        0x00000191  // SDK已经初始化
//
//#define CKR_VENDOR_DEFINED                      0x80000000
//#define CKR_DEVICE_IS_BUSY                      (CKR_VENDOR_DEFINED + 1)   //设备正忙
//#define CKR_TIMEOUT                             (CKR_VENDOR_DEFINED + 8)   //超时
//#define CKR_USER_CANCEL                         (CKR_VENDOR_DEFINED + 9)   //用户取消操作
//#define CKR_NO_DEVICE                           (CKR_VENDOR_DEFINED + 11)  //设备未找到
//
////蓝牙通讯库定义错误码
//#define CKR_VENDOR_BLE_DEFINED                  0x81000000
//#define CKR_BLE_POWEROFF                        (CKR_VENDOR_BLE_DEFINED + 1)  //蓝牙未打开
//#define CKR_BLE_NOT_SUPPORT                     (CKR_VENDOR_BLE_DEFINED + 2)  //不支持蓝牙4.0
//#define CKR_BLE_CONNECT_FAIL                    (CKR_VENDOR_BLE_DEFINED + 3)  //连接失败
//#define CKR_BLE_BOND_FAIL                       (CKR_VENDOR_BLE_DEFINED + 4)  //绑定失败
//#define CKR_BLE_NOT_AUTHORIZE                   (CKR_VENDOR_BLE_DEFINED + 5)  //蓝牙设备未授权
//#define CKR_BLE_UNKNOW                          (CKR_VENDOR_BLE_DEFINED + 6)  //未知蓝牙错误

public enum DeviceError: Int64, ImkeyCodeError {
  case ok = 0x00000000
  case argumentsBad = 0x00000007
  case deviceError = 0x00000030
  case bufferTooSmall = 0x00000150
  case cryptokiNotInitialized = 0x00000190
  case cryptokiAlreadyInitialized = 0x00000191
  
  case deviceIsBusy = 0x80000001
  case timeout = 0x80000008
  case userCancel = 0x80000009
  case userEnd = 0x8000000A
  case noDevice = 0x8000000B
  case deviceAlreadyConnect = 0x8000000C
  
  case blePowerOff = 0x81000001
  case bleNotSupport = 0x81000002
  case bleConnectFail = 0x81000003
  case bleBondFail = 0x81000004
  case bleNotAuthorize = 0x81000005
  case bleUnknow = 0x81000006
  
  public var message: String {
    switch self {
    case .ok:
      return "ok"
    case .argumentsBad:
      return "imkey_ckr_argument_bad"
    case .deviceError:
      return "imkey_ckr_device_error"
    case .bufferTooSmall:
      return "imkey_ckr_buffer_too_small"
    case .cryptokiNotInitialized:
      return "imkey_ckr_cryptoki_not_initilized"
    case .cryptokiAlreadyInitialized:
      return "imkey_ckr_cryptoki_already_initilized"
    case .deviceIsBusy:
      return "imkey_ckr_device_is_busy"
    case .timeout:
      return "imkey_ckr_timeout"
    case .userCancel:
      return "imkey_ckr_user_cancel"
    case .userEnd:
      return "imkey_ckr_user_end"
    case .noDevice:
      return "imkey_ckr_no_device"
    case .deviceAlreadyConnect:
      return "imkey_ckr_device_already_connected"
    case .blePowerOff:
      return "imkey_ckr_ble_poweroff"
    case .bleNotSupport:
      return "imkey_ckr_ble_not_support"
    case .bleConnectFail:
      return "imkey_ckr_ble_connect_fail"
    case .bleBondFail:
      return "imkey_ckr_ble_bound_fail"
    case .bleNotAuthorize:
      return "imkey_ckr_ble_not_authorize"
    case .bleUnknow:
      return "imkey_ckr_ble_unkown"
    default:
      return "imkey_ckr_ble_unkown"
    }
  }
}

public enum APDUError: String, ImkeyDescError {
  case userNotConfirm = "imkey_user_not_confirmed"
  case conditionsNotStatisfied = "imkey_conditions_not_satisfied"
  case cmdFormatError = "imkey_command_format_error"
  case cmdDataError = "imkey_command_data_error"
  case appletNotExist = "imkey_applet_not_exist"
  case wrongLength = "imkey_apdu_wrong_length"
  case walletNotCreated = "imkey_wallet_not_created"
  case inMenuPage = "imkey_in_menu_page"
  case pinNotVerified = "imkey_pin_not_verified"
  case bluetoothChannelError = "imkey_bluetooth_channel_error"
  case appletFunctionNotSupported = "imkey_applet_function_not_supported"
}

extension String : ImkeyError {
  public var message: String {
    return self
  }
}
