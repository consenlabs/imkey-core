//
//  Manage.swift
//  ImkeyLibrary
//
//  Created by joe on 2018/8/28.
//  Copyright © 2018年 joe. All rights reserved.
//

import Foundation

public class Manager:TSMReq{
  class func selectMainSE(handle:UInt)throws {
    let apdu = APDU.selectMainSE()
    let resApdu = try BLE.shared().sendApdu(handle: handle, apdu: apdu)
    try APDU.checkResponse(res: resApdu)
    // check select result?
  }
  
  public class func getSeid(handle:UInt)throws -> String{
    Log.d("get seid: \n")
    try selectMainSE(handle: handle)
    let seidApdu = APDU.seID()
    let res = try BLE.shared().sendApdu(handle: handle, apdu:seidApdu)
    try APDU.checkResponse(res: res)
    return res.key_substring(to: res.count-4)
  }
  
  public class func getSn(handle:UInt)throws -> String{
    Log.d("get sn: \n")
    let apdu = APDU.sn()
    let res = try BLE.shared().sendApdu(handle: handle, apdu:apdu)
    try APDU.checkResponse(res: res)
    let snbytes = ByteUtil.hexString2Uint8Array(data: res.key_substring(to: res.count-4))!
    let sn = String(bytes: snbytes, encoding: .utf8)!
    Log.d("sn:" + sn)
    return sn
  }
  
  class func getCert(handle:UInt)throws -> String{
    Log.d("get cert: \n")
    try selectMainSE(handle: handle)
    let certApdu = APDU.cert()
    let certRes = try BLE.shared().sendApdu(handle: handle, apdu:certApdu)
    try APDU.checkResponse(res: certRes)
    let cert = certRes.key_substring(to: certRes.count-4)
    return cert
  }
  
  public class func activeDevice(handle:UInt)throws {
    let seActive = SEActive()
    let cardList:[String] = []
    let request = SERequest(stepKey: "01",
                            statusWord: "",
                            cardRetDataList: cardList,
                            seid: try getSeid(handle: handle),
                            deviceCert: try getCert(handle: handle),
                            sn: try getSn(handle: handle))
    let res = try seActive.activeSe(handle:handle, request: request)
    Log.d("active se : \n\(res)")
  }
  
  public class func checkDevice(handle:UInt)throws {
    let seCheck = SECheck()
    let cardList:[String] = []
    let request = SERequest(stepKey: "01",
                            statusWord: "",
                            cardRetDataList: cardList,
                            seid: try getSeid(handle: handle),
                            deviceCert: try getCert(handle: handle),
                            sn: try getSn(handle: handle))
    let res = try seCheck.checkSE(handle:handle, request: request)
    Log.d("check se : \n\(res)")
  }
  
  public class func checkDeviceCert(handle:UInt,seCert:String)throws ->Bool{
    let seCertCheck = SECertCheck()
    let cardList:[String] = []
    let request = SERequest(stepKey: "01",
                            statusWord: "",
                            cardRetDataList: cardList,
                            seid: try getSeid(handle: handle),
                            deviceCert: seCert,
                            sn: try getSn(handle: handle))
    return try seCertCheck.checkCert(handle: handle, request: request)
  }
  
  public class func checkUpdate(handle:UInt)throws -> ImkeyDevice {
    let seQuery = SEQuery()
    let cardList:[String] = []
    let request = SERequest(stepKey: "01",
                            statusWord: "",
                            cardRetDataList: cardList,
                            seid: try getSeid(handle: handle),
                            deviceCert: try getCert(handle: handle),
                            sn: try getSn(handle: handle))
    return try seQuery.query(handle:handle, request: request)
  }
  
  public class func downloadAPP(handle:UInt,appletName:String)throws{
    let appDownload = APPDownload()
    let cardList:[String] = []
    let request = APPRequest(stepKey: "01",
                             statusWord: "",
                             cardRetDataList: cardList,
                             seid: try getSeid(handle: handle),
                             deviceCert: try getCert(handle: handle),
                             instanceAid: Applet.appletName2Aid(appletName: appletName))
    let res = try appDownload.download(handle:handle, request: request)
    Log.d("download applet : \n\(res)")
  }
  
  public class func deleteAPP(handle:UInt,appletName:String)throws{
    let appDelete = APPDelete()
    let cardList:[String] = []
    let request = APPRequest(stepKey: "01",
                             statusWord: "",
                             cardRetDataList: cardList,
                             seid: try getSeid(handle: handle),
                             deviceCert: try getCert(handle: handle),
                             instanceAid: Applet.appletName2Aid(appletName: appletName))
    let res = try appDelete.delete(handle:handle, request: request)
    Log.d("delete applet : \n\(res)")
  }
  
  public class func updateAPP(handle:UInt,appletName:String)throws{
    let appUpdate = APPUpdate()
    let cardList:[String] = []
    let request = APPRequest(stepKey: "01",
                             statusWord: "",
                             cardRetDataList: cardList,
                             seid: try getSeid(handle: handle),
                             deviceCert: try getCert(handle: handle),
                             instanceAid: Applet.appletName2Aid(appletName: appletName))
    let res = try appUpdate.update(handle:handle, request: request)
    Log.d("update applet : \n\(res)")
  }
  
  public class func saveAuthCode(handle:UInt,authCode:String)throws{
    let authCodeStorage = AuthCodeStorage()
    let request = AuthCodeStorageRequest(seid: try getSeid(handle: handle),
                                         authCode: authCode)
    try authCodeStorage.saveAuthCode(request: request)
  }
  
  public class func getBatteryPower(handle:UInt)throws -> String{
    let apdu = APDU.battery()
    let batteryApdu = try BLE.shared().sendApdu(handle: handle, apdu:apdu)
    try APDU.checkResponse(res: batteryApdu)
    var battery = APDU.removeStatus(apdu: batteryApdu)
    if battery != "FF"{
      battery = String(Int(battery,radix:16)!)
    }
    return battery
  }
  
  public class func getFirmwareVersion(handle:UInt)throws -> String{
    try selectMainSE(handle: handle)
    let apdu = APDU.firmwareVersion()
    let result = try BLE.shared().sendApdu(handle: handle, apdu:apdu)
    try APDU.checkResponse(res: result)
    let array = Array(result.key_substring(to: 4))
    let version = "\(array[0]).\(array[1]).\(array[2])\(array[3])"
    return version
  }
  
  public class func reset(handle:UInt)throws {
    try selectMainSE(handle: handle)
    let apdu = APDU.reset()
    let res = try BLE.shared().sendApdu(handle: handle, apdu: apdu)
    try APDU.checkResponse(res: res)
  }
  
  public class func getLifeTime(handle:UInt)throws ->String{
    //        try selectMainSE(handle: handle)
    let apdu = APDU.lifeTime()
    let result = try BLE.shared().sendApdu(handle: handle, apdu:apdu)
    try APDU.checkResponse(res: result)
    let lifeTime = APDU.removeStatus(apdu: result)
    switch lifeTime {
    case "80":
      return "life_time_device_inited";
    case "89":
      return "life_time_device_activated";
    case "81":
      return "life_time_unset_pin";
    case "83":
      return "life_time_wallet_unready";
    case "84":
      return "life_time_wallet_creatting";
    case "85":
      return "life_time_wallet_recovering";
    case "86":
      return "life_time_wallet_ready";
    default:
      return "life_time_unknown";
    }
  }
  
  public class func getBLEName(handle:UInt)throws ->String{
    let apdu = APDU.getBLEName();
    let result = try BLE.shared().sendApdu(handle: handle, apdu:apdu)
    guard let uint8s = ByteUtil.hexString2Uint8Array(data: result) else{
      throw SDKError.unwrapError;
    }
    
    let name = String(bytes: uint8s, encoding: .utf8)!
    return name
  }
  
  public class func setBLEName(handle:UInt,bleName:String)throws {
    let pattern = "^[a-zA-Z0-9\\-]{1,12}$"
    let regex = try NSRegularExpression(pattern: pattern)
    let matchNumber = regex.numberOfMatches(in: bleName, range: NSMakeRange(0, bleName.utf8.count))
    if matchNumber != 1 {
      throw SDKError.illegalArgument
    }
    
    let apdu = APDU.setBLEName(bleName: bleName);
    let result = try BLE.shared().sendApdu(handle: handle, apdu:apdu)
    try APDU.checkResponse(res: result)
  }
  
  public class func getBLEVersion(handle:UInt)throws ->String{
    try selectMainSE(handle: handle)
    let apdu = APDU.getBLEVersion()
    let result = try BLE.shared().sendApdu(handle: handle, apdu:apdu)
    try APDU.checkResponse(res: result)
    let array = Array(result.key_substring(to: 4))
    let version = "\(array[0]).\(array[1]).\(array[2])\(array[3])"
    return version
  }
  
  public class func getSDKInfo()->SDKInfo{
    return SDKInfo(sdkVersion: Constants.sdkVersion)
  }
}

public struct SDKInfo: Codable{
  public let sdkVersion:String
}
