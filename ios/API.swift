//
//  API.swift
//  imKeyConnector
//
//  Created by joe on 12/9/18.
//

import Foundation

public class API{
  public class func startMessageDeamon(){
    DispatchQueue.global().async {
        while true{
          Log.d("start while...")
          
          //get apdu
          var apdu = ""
          while true{
            apdu = String(cString:get_apdu())
            if apdu != ""{
              let count = "".utf8CString.count
              let result: UnsafeMutableBufferPointer<Int8> = UnsafeMutableBufferPointer<Int8>.allocate(capacity: count)
              _ = result.initialize(from: "".utf8CString)
              let p = UnsafePointer(result.baseAddress!)
              set_apdu(p)
              break
            }
            sleep(1)
          }
          
          //send apdu
          let res = try! BLE.shared().sendApdu(handle: 0, apdu: apdu)
          
          //set return
          var apduRet = ""
          while true{
            apduRet = String(cString:get_apdu_return())
            if apduRet == ""{
              let count = res.utf8CString.count
              let result: UnsafeMutableBufferPointer<Int8> = UnsafeMutableBufferPointer<Int8>.allocate(capacity: count)
              _ = result.initialize(from: res.utf8CString)
              let p = UnsafePointer(result.baseAddress!)
              set_apdu_return(p)
              break
            }
            sleep(1)
          }
        }
      }
    }
  
  public class func getSEID() ->String{
    return String(cString:get_seid())
  }
  
  public class func checkDevice(){
    return check_device()
  }
  
  public class func activeDevice(){
    return active_device()
  }
  
  public class func checkUpdate(){
    return check_update()
  }
  
  public class func downloadApp(){
    return app_download()
  }
  
  public class func updateApp(){
    return app_update()
  }
  
  public class func deleteApp(){
    return app_delete()
  }
}
