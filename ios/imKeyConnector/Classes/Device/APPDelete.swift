//
//  APPDownload.swift
//  BigInt
//
//  Created by joe on 2018/9/20.
//

import Foundation

public class APPDelete:TSMReq{
  func delete(handle:UInt,request:APPRequest)throws -> Response{
    var req = request
    req.commandID = Constants.appDelete
    
    guard let data = try? JSONEncoder().encode(req) else {
      throw SDKError.illegalArgument
    }
    
    let res = try HTTP.syncRequest(action: Constants.appDelete,from: data)
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
      for apdu in apdus {
        let res = try BLE._shareManager.sendApdu(handle: handle, apdu: apdu)
        cardList.append(res)
      }
      let status = getStatus(apdu: cardList[cardList.count-1])
      let reRequest = APPRequest(stepKey: response._ReturnData.nextStepKey!,
                                 statusWord: status,
                                 cardRetDataList: cardList,
                                 seid: response._ReturnData.seid,
                                 deviceCert: "",
                                 instanceAid: request.instanceAid)
      return try delete(handle: handle,request: reRequest)
    }
  }
}
