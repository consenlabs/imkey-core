//
//  Apdu.swift
//  Demo
//
//  Created by joe on 2018/7/3.
//  Copyright © 2018年 joe. All rights reserved.
//

import Foundation

public class APDU{
  public static let APDU_RSP_SUCCESS = "9000";
  public static let APDU_RSP_USER_NOT_CONFIRMED = "6940";
  public static let APDU_CONDITIONS_NOT_SATISFIED = "6985";
  public static let APDU_RSP_APPLET_NOT_EXIST = "6A82";
  public static let APDU_RSP_INCORRECT_P1P2 = "6A86";
  public static let APDU_RSP_CLA_NOT_SUPPORTED = "6E00";
  public static let APDU_RSP_APPLET_WRONG_DATA = "6A80";
  public static let APDU_RSP_WRONG_LENGTH = "6700";
  public static let APDU_RSP_SIGNATURE_VERIFY_FAILED = "6942";
  public static let APDU_RSP_EXCEEDED_MAX_UTXO_NUMBER = "6941";
  
  public static let btcSegwitPreType_output:UInt8 = 0x00
  public static let btcSegwitPreType_utxoHashVout:UInt8 = 0x40
  public static let btcSegwitPreType_utxoSequence:UInt8 = 0x80
  
  static let len = 245//data length in apdu
  public class func select(aid:String) -> String?{
    let byteAid:[UInt8] = ByteUtil.hexString2Uint8Array(data:aid)!
    var header:[UInt8] = [0x00,0xA4,0x04,0x00,UInt8(byteAid.count)]
    header.append(contentsOf: byteAid)
    return ByteUtil.uint8Array2HexString(data: header)
  }
  
  class func battery() -> String{
    return "00D6FEED01"
  }
  
  class func checkResponse(res:String)throws {
    if res.hasSuffix(APDU_RSP_SUCCESS){//9000 means success
      return
    }
    
    //exception
    switch res {
    case APDU_RSP_USER_NOT_CONFIRMED:
      throw APDUError.userNotConfirm
    case APDU_CONDITIONS_NOT_SATISFIED:
      throw APDUError.conditionsNotStatisfied
    case APDU_RSP_INCORRECT_P1P2,               //incorrect p1p2
    APDU_RSP_CLA_NOT_SUPPORTED:            //CLA not support
      throw APDUError.cmdFormatError
    case APDU_RSP_APPLET_WRONG_DATA:            //wrong data
      throw APDUError.cmdDataError
    case APDU_RSP_APPLET_NOT_EXIST:            //applet does not exist
      throw APDUError.appletNotExist
    case APDU_RSP_WRONG_LENGTH:
      throw APDUError.wrongLength
    case APDU_RSP_SIGNATURE_VERIFY_FAILED:
      throw SDKError.signVerifyFail
    case APDU_RSP_EXCEEDED_MAX_UTXO_NUMBER:
      throw SDKError.exceededMaxUtxoNum
    default:
      throw SDKError.unknownError
    }
  }
}
