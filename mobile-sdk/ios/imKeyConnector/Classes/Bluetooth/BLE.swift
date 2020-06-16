//
//  Ble.swift
//  ImkeyLibrary
//
//  Created by joe on 2018/8/25.
//  Copyright © 2018年 joe. All rights reserved.
//

import Foundation

public class BLE:NSObject,FTBLEDelegate{
  public func fTdidTheDeviceDisconnected(_ uuid: String!, error Error: Int) {
    Log.d("fTdidTheDeviceDisconnected...")
    _bleDelegate?.deviceDidDisconnect(address:uuid, errorCode: Error)
  }
  
  public func ftDidFindBLEDevice(_ deviceName: String!, uuid: String!) {
    _bleDelegate?.deviceDidFind(deviceName: deviceName, address: uuid)
  }
  
  public func fTdidTheDeviceConnected(_ uuid: String!, handler: Int, errorCode Error: Int) {
    Log.d("fTdidTheDeviceConnected...")
    _bleDelegate?.deviceDidConnect(address:uuid, errorCode: Error)
  }
  private override init(){
    Log.d("init...")
  }
  static let _shareManager = BLE()
  var _bleDelegate: BLEDelegate?
  
  public class func shared() -> BLE {
    return _shareManager
  }
  
  public func initialize() -> Int{
    Log.d("initialize...")
    setCallback()
    return BLEKeyInterface.sharedInstance().initialize()
  }
  
  public func finalize() -> Int{
    Log.d("fialize...")
    return BLEKeyInterface.sharedInstance().finalize()
  }
  
  public func startScan() -> Int{
    return BLEKeyInterface.sharedInstance().startScan()
  }
  
  public func stopScan() -> Int{
    return BLEKeyInterface.sharedInstance().stopScan()
  }
  
  var handle:UInt = 0
  public func connect(address:String,timeout:UInt)throws -> Int{
    let result =  BLEKeyInterface.sharedInstance().connect(address, handle: &handle, timeout: timeout)
    Log.d("result：\(result)")
    
    if result == 0 {
      Log.d("send keep connect apdu...")
      do {
        let apduResponse = try sendApdu(apdu: APDU.battery(),timeout: 3 * 1000)
        try APDU.checkResponse(res: apduResponse)
        Log.d("keep connect sucess...")
      } catch let e as ImkeyError {
        Log.d("original error:\(e.message)")
        throw SDKError.connectFail
      }
    }
    return result
  }
  
  public func disConnect() -> Int{
    Log.d("disconnect handle:\(handle)")
    return BLEKeyInterface.sharedInstance().disconnect(handle)
  }
  
  public func sendApdu(apdu:String,timeout:UInt32 = Constants.sendAPDUTimeout)throws ->String{
    Log.d("\nble >>>> \(apdu)")
    let apduData = ByteUtil.hexString2Uint8Array(data: apdu)
    let apduLen = apdu.count/2
    var rcvData:[UInt8] = [UInt8](repeating: 0, count: 260)
    var len:UInt32 = 260 // apdu max length is 256
    let resCode = BLEKeyInterface.sharedInstance().sendData(handle, data: apduData, length: UInt32(apduLen), rcvData: &rcvData, rcvDataLen: &len, timeout: timeout)
    if(resCode != 0){
//      throw BLEError.fromCode(code: Int64(resCode))
      throw DeviceError(rawValue: Int64(resCode))!
    }
    
    let resApdu = ByteUtil.uint8Array2HexString(data: rcvData)
    let bound = resApdu.index(resApdu.startIndex,offsetBy: len*2)
    let trimApdu = String(resApdu[..<bound])
    Log.d("ble <<<< \(trimApdu)")
    if trimApdu == "F000"{
      throw APDUError.walletNotCreated
    }
    if trimApdu == "F080"{
      throw APDUError.inMenuPage
    }
    if trimApdu == "F081"{
      throw APDUError.pinNotVerified
    }
    if trimApdu == "6F01"{
      throw APDUError.bluetoothChannelError
    }
    if trimApdu == "6D00"{
      throw APDUError.appletFunctionNotSupported
    }
    return trimApdu
  }
  
//  public func sendPrepareApdus(apdus:[String])throws -> String {
//    var res:String = ""
//    for (index,apdu) in apdus.enumerated(){
//      if index == apdus.count-1{
//        Log.d("final prepare apdu:")
//        res = try sendApdu(apdu: apdu,timeout:Constants.sendSignPreAPDUTimeout)
//      }else{
//        res = try sendApdu(apdu: apdu)
//      }
//      try APDU.checkResponse(res: res)
//    }
//    return res
//  }
  
  public func setDelegate(bleDelegate:BLEDelegate){
    BLEKeyInterface.sharedInstance().setDelegate(self)
    _bleDelegate = bleDelegate
  }
  
  public func removeDelegate(){
    _bleDelegate = nil
    BLEKeyInterface.sharedInstance().removeDelegate()
  }
  
  public func setCallback(){
    set_callback(swiftCallback)
  }
  
  let swiftCallback : @convention(c) (UnsafePointer<Int8>?,Int32) -> UnsafePointer<Int8>? = {
    (apdu,timeout) -> UnsafePointer<Int8>? in
    print("callback miaomiao v v timeout\(timeout)")
    let swiftApdu = String(cString:apdu!)
    
    var response = "";
    do {
      response = try BLE.shared().sendApdu(apdu: swiftApdu,timeout: UInt32(timeout * 1000))
    }catch let e as ImkeyError {
      response = "communication_error_" + e.message
    }catch{
      Log.d(error)
    }
    let count = response.utf8CString.count
    let result: UnsafeMutableBufferPointer<Int8> = UnsafeMutableBufferPointer<Int8>.allocate(capacity: count)
    _ = result.initialize(from: response.utf8CString)
    let p = UnsafePointer(result.baseAddress!)
    return p
  }
}
