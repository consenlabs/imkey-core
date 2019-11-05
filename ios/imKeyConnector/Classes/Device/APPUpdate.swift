//
//  APPDownload.swift
//  BigInt
//
//  Created by joe on 2018/9/20.
//

import Foundation

public class APPUpdate:TSMReq{
  func update(handle:UInt,request:APPRequest)throws -> Response{
    var req = request
    req.commandID = Constants.appUpdate
    
    guard let data = try? JSONEncoder().encode(req) else {
      throw SDKError.illegalArgument
    }
    let res = try HTTP.syncRequest(action: Constants.appUpdate,from: data)
    guard let jsonData = res.data(using: String.Encoding.utf8),
      let response = try? JSONDecoder().decode(Response.self, from: jsonData) else {
        throw SDKError.jsonError
    }
    guard response._ReturnCode == "000000" else {
      throw TSMError.fromCode(code: response._ReturnCode)
    }
    if(response._ReturnData.nextStepKey == "end"){
      return response
    }else{
      guard let apdus = response._ReturnData.apduList else{
        throw SDKError.unwrapError
      }
      var cardList:[String] = []
      for (index,apdu) in apdus.enumerated() {
        let res = try BLE._shareManager.sendApdu(handle: handle, apdu: apdu)
        cardList.append(res.uppercased())
        let status = getStatus(apdu: res)
        if "03" == response._ReturnData.nextStepKey && index > 0 && status != "9000"{
          break
        }
      }
      let status = getStatus(apdu: cardList[cardList.count-1])
      let reRequest = APPRequest(stepKey: response._ReturnData.nextStepKey!,
                                 statusWord: status,
                                 cardRetDataList: cardList,
                                 seid: response._ReturnData.seid,
                                 deviceCert: "",
                                 instanceAid: request.instanceAid)
      return try update(handle: handle,request: reRequest)
    }
  }
}
