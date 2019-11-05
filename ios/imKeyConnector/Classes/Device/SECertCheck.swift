//
//  File.swift
//  imKeyConnector
//
//  Created by joe on 1/17/19.
//

import Foundation

struct SECertCheckResponse: Codable{
  let _ReturnCode:String
  let _ReturnMsg:String
  public let _ReturnData:ReturnDataBean
  public struct ReturnDataBean : Codable{
    public let seid:String
    public let verifyResult:Bool
  }
}

public class SECertCheck{
  func checkCert(handle:UInt,request:SERequest)throws -> Bool{
    var req = request
    req.commandID = Constants.deviceCertCheck
    
    guard let data = try? JSONEncoder().encode(req) else {
      throw SDKError.illegalArgument
    }
    let res = try HTTP.syncRequest(action: Constants.deviceCertCheck,from: data)
    
    guard let jsonData = res.data(using: String.Encoding.utf8),
      let response = try? JSONDecoder().decode(SECertCheckResponse.self, from: jsonData) else {
        throw SDKError.jsonError
    }
    
    guard response._ReturnCode == "000000" else {
      throw TSMError.fromCode(code: response._ReturnCode)
    }
    
   return response._ReturnData.verifyResult
  }
}
