//
//  AuthCodeStorage.swift
//  imKeyConnector
//
//  Created by joe on 1/17/19.
//

import Foundation

struct AuthCodeStorageRequest:Codable{
  let seid:String
  let authCode:String
  var commandID:String?
  
  init(seid:String,authCode:String) {
    self.seid = seid
    self.authCode = authCode
    self.commandID = ""
  }
}

class AuthCodeStorage {
  func saveAuthCode(request:AuthCodeStorageRequest)throws {
    var req = request
    req.commandID = Constants.authCodeStorage
    
    guard let data = try? JSONEncoder().encode(req) else {
      throw SDKError.illegalArgument
    }
    let res = try HTTP.syncRequest(action: Constants.authCodeStorage,from: data)
    
    guard let jsonData = res.data(using: String.Encoding.utf8),
      let response = try? JSONDecoder().decode(Response.self, from: jsonData) else {
        throw SDKError.jsonError
    }
    
    guard response._ReturnCode == "000000" else {
      throw TSMError.fromCode(code: response._ReturnCode)
    }
  }
}
