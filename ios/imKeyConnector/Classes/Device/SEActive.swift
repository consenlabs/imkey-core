//
//  SEActive.swift
//  ImkeyLibrary
//
//  Created by joe on 2018/9/16.
//  Copyright © 2018年 joe. All rights reserved.
//

import Foundation

public class SEActive:TSMReq{
  func activeSe(handle:UInt,request:SERequest)throws -> Response{
    var req = request
    req.commandID = Constants.seActivate
    
    guard let data = try? JSONEncoder().encode(req) else {
      throw SDKError.illegalArgument
    }
    let res = try HTTP.syncRequest(action: Constants.seActivate,from: data)
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
      let reRequest = SERequest(stepKey: response._ReturnData.nextStepKey!,
                                statusWord: status,
                                cardRetDataList: cardList,
                                seid: response._ReturnData.seid,
                                deviceCert: "",
                                sn: "")
      return try activeSe(handle: handle,request: reRequest)
    }
  }
}
