//
//  TSMReq.swift
//  BigInt
//
//  Created by joe on 2018/9/20.
//

import Foundation

struct SERequest:Codable{
  var sdkVersion:String?
  let stepKey:String
  let statusWord:String
  let cardRetDataList: [String]
  let seid:String
  let deviceCert:String
  let sn:String
  var commandID:String?
  init(stepKey:String,statusWord:String,cardRetDataList: [String],
       seid:String,deviceCert:String,sn:String) {
    self.sdkVersion = Manager.getSDKInfo().sdkVersion
    self.stepKey = stepKey
    self.statusWord = statusWord
    self.cardRetDataList = cardRetDataList
    self.seid = seid
    self.deviceCert = deviceCert
    self.sn = sn
    self.commandID = ""
  }
}

struct APPRequest:Codable{
  let stepKey:String
  let statusWord:String
  let cardRetDataList: [String]
  let seid:String
  let deviceCert:String
  let instanceAid:String
  var commandID:String?
  
  init(stepKey:String,statusWord:String,cardRetDataList: [String],
       seid:String,deviceCert:String,instanceAid:String) {
    self.stepKey = stepKey
    self.statusWord = statusWord
    self.cardRetDataList = cardRetDataList
    self.seid = seid
    self.deviceCert = deviceCert
    self.instanceAid = instanceAid
    self.commandID = ""
  }
}

struct Response: Codable{
  let _ReturnCode:String
  let _ReturnMsg:String
  let _ReturnData:ReturnDataBean
  struct ReturnDataBean : Codable{
    let seid:String
    var nextStepKey:String?
    var apduList:[String]?
  }
}

public class TSMReq {
  let tsmSuccessCode = "000000"
  func getStatus(apdu:String) -> String{
    return APDU.getStatus(apdu: apdu)
  }
}
