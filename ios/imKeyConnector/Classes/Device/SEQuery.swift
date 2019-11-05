//
//  SEQuery.swift
//  BigInt
//
//  Created by joe on 2018/9/21.
//

import Foundation

public struct SEQueryResponse: Codable{
  let _ReturnCode:String
  let _ReturnMsg:String
  public let _ReturnData:ReturnDataBean
  public struct ReturnDataBean : Codable{
    public let seid:String
    public var nextStepKey:String?
    public var sdkMode:String?
    public let availableAppBeanList:[AvailableAppBeanList]?
    public struct AvailableAppBeanList : Codable{
      public let instanceAid:String
      public let appLogo:String
      public let lastUpdated:String
      public let installMode:String
      public let latestVersion:String
      public let installedVersion:String?
    }
  }
}

public struct ImkeyDevice{
  public var seid:String
  public var sn:String
  public var status:String
  public var sdkMode:String
  public var availableAppList:[AppInfo]
  
  public struct AppInfo : Codable{
    public var appletName:String
    public var appletLogo:String
    public var lastUpdated:String
    public var installMode:String
    public var latestVersion:String
    public var installedVersion:String
  }
}

public class SEQuery:TSMReq{
  func query(handle:UInt,request:SERequest)throws -> ImkeyDevice{
    var req = request
    req.commandID = Constants.seInfoQuery
    
    guard let data = try? JSONEncoder().encode(req) else {
      throw SDKError.illegalArgument
    }
    let res = try HTTP.syncRequest(action: Constants.seInfoQuery,from: data)
    guard let jsonData = res.data(using: String.Encoding.utf8),
      let response = try? JSONDecoder().decode(SEQueryResponse.self, from: jsonData) else {
        throw SDKError.jsonError
    }
    
    var status:String
    if response._ReturnCode == "000000"{
      status = "latest"
    }else if response._ReturnCode == "BSE0007"{
      status = "inactivated"
    }else{
      throw TSMError.fromCode(code: response._ReturnCode)
    }
    
    var sdkMode = ""
    var availableApps = [ImkeyDevice.AppInfo]()
    
    if let availableList =  response._ReturnData.availableAppBeanList{
      for item in availableList {
        let appletName = Applet.aid2AppletName(aid: item.instanceAid)
        var installedVersion = ""
        if item.installedVersion != nil{
          installedVersion = item.installedVersion!
        }
        
        sdkMode = response._ReturnData.sdkMode!
        
        availableApps.append(ImkeyDevice.AppInfo(appletName: appletName, appletLogo: item.appLogo, lastUpdated: item.lastUpdated, installMode: item.installMode, latestVersion: item.latestVersion, installedVersion: installedVersion))
      }
    }
    
    let imkeyDevice = ImkeyDevice(seid: request.seid,
                                  sn: request.sn,
                                  status: status,
                                  sdkMode:sdkMode,
                                  availableAppList: availableApps)
    
    return imkeyDevice
  }
  
  private func updatetype(flag:String) ->String{
    if flag  == "01"{
      return "forceUpdate"
    }
    return "normal"
  }
}
