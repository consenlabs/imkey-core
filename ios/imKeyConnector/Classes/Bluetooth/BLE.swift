//
//  Ble.swift
//  ImkeyLibrary
//
//  Created by joe on 2018/8/25.
//  Copyright © 2018年 joe. All rights reserved.
//

import Foundation

public class BLE:NSObject,FTBLEDelegate{
  var mHandle:UInt = 0
  
  public func fTdidTheDeviceDisconnected(_ uuid: String!, error Error: Int) {
    Log.d("fTdidTheDeviceDisconnected...")
    _bleDelegate?.deviceDidDisconnect(uuid, error: Error)
  }
  
  public func ftDidFindBLEDevice(_ deviceName: String!, uuid: String!) {
    _bleDelegate?.deviceDidFind(deviceName, address: uuid)
  }
  
  public func fTdidTheDeviceConnected(_ uuid: String!, handler: Int, errorCode Error: Int) {
    Log.d("fTdidTheDeviceConnected...")
    _bleDelegate?.deviceDidConnect(uuid, handler: handler, errorCode: Error)
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
  

  public func connect(address:String,handle:UnsafeMutablePointer<UInt>,timeout:UInt)throws -> Int{
    var tempHandle:UInt = 0
    let result =  BLEKeyInterface.sharedInstance().connect(address, handle: &tempHandle, timeout: timeout)
    Log.d("result：\(result)")
    
    if result == 0 {
      handle.pointee = tempHandle
      mHandle = handle.pointee
      Log.d("connect handle:\(handle.pointee)")
      Log.d("send keep connect apdu...")
      do {
        let apduResponse = try sendApdu(handle: handle.pointee, apdu: APDU.battery(),timeout: 3 * 1000)
        try APDU.checkResponse(res: apduResponse)
        Log.d("keep connect sucess...")
      } catch let e as ImkeyError {
        Log.d("original error:\(e.message)")
        throw SDKError.connectFail
      }
    }
    return result
  }
  
  public func disConnect(handle:UInt) -> Int{
    Log.d("disconnect handle:\(handle)")
    return BLEKeyInterface.sharedInstance().disconnect(handle)
  }
  
  public func sendApdu(handle:UInt,apdu:String,timeout:UInt32 = Constants.sendAPDUTimeout)throws ->String{
    Log.d("\nble >>>> \(apdu)")
    let apduData = ByteUtil.hexString2Uint8Array(data: apdu)
    let apduLen = apdu.count/2
    var rcvData:[UInt8] = [UInt8](repeating: 0, count: 260)
    var len:UInt32 = 260 // apdu max length is 256
    let resCode = BLEKeyInterface.sharedInstance().sendData(mHandle, data: apduData, length: UInt32(apduLen), rcvData: &rcvData, rcvDataLen: &len, timeout: timeout)
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
  
  public func sendPrepareApdus(handle:UInt,apdus:[String])throws -> String {
    var res:String = ""
    for (index,apdu) in apdus.enumerated(){
      if index == apdus.count-1{
        Log.d("final prepare apdu:")
        res = try sendApdu(handle: handle, apdu: apdu,timeout:Constants.sendSignPreAPDUTimeout)
      }else{
        res = try sendApdu(handle: handle, apdu: apdu)
      }
      try APDU.checkResponse(res: res)
    }
    return res
  }
  
  public func setDelegate(bleDelegate:BLEDelegate){
    BLEKeyInterface.sharedInstance().setDelegate(self)
    _bleDelegate = bleDelegate
  }
  
  public func removeDelegate(){
    _bleDelegate = nil
    BLEKeyInterface.sharedInstance().removeDelegate()
  }
}
