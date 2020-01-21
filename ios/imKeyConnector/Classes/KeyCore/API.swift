//
//  API.swift
//  imKeyConnector
//
//  Created by joe on 12/9/18.
//

import Foundation
import SwiftProtobuf

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
    Log.d("check device ......")
    var deviceParam = Api_DeviceParam()
    deviceParam.action = "se_secure_check"
    deviceParam.param = Google_Protobuf_Any()
    
    var action = Api_TcxAction()
    action.method = "device_manage"
    action.param = Google_Protobuf_Any()
    action.param.value = try! deviceParam.serializedData()
    
    let paramHex = try! action.serializedData().key_toHexString()
    call_tcx_api(paramHex)
  }
  
  public class func activeDevice(){
    var deviceParam = Api_DeviceParam()
    deviceParam.action = "se_activate"
    deviceParam.param = Google_Protobuf_Any()
    
    var action = Api_TcxAction()
    action.method = "device_manage"
    action.param = Google_Protobuf_Any()
    action.param.value = try! deviceParam.serializedData()
    
    let paramHex = try! action.serializedData().key_toHexString()
    call_tcx_api(paramHex)
  }
  
  public class func checkUpdate(){
    var deviceParam = Api_DeviceParam()
    deviceParam.action = "se_query"
    deviceParam.param = Google_Protobuf_Any()
    
    var action = Api_TcxAction()
    action.method = "device_manage"
    action.param = Google_Protobuf_Any()
    action.param.value = try! deviceParam.serializedData()
    
    let paramHex = try! action.serializedData().key_toHexString()
    call_tcx_api(paramHex)
  }
  
  public class func downloadApp(){
    var deviceParam = Api_DeviceParam()
    deviceParam.action = "app_download"
    deviceParam.param = Google_Protobuf_Any()
    
    var action = Api_TcxAction()
    action.method = "device_manage"
    action.param = Google_Protobuf_Any()
    action.param.value = try! deviceParam.serializedData()
    
    let paramHex = try! action.serializedData().key_toHexString()
    call_tcx_api(paramHex)
  }
  
  public class func updateApp(){
    var deviceParam = Api_DeviceParam()
    deviceParam.action = "app_update"
    deviceParam.param = Google_Protobuf_Any()
    
    var action = Api_TcxAction()
    action.method = "device_manage"
    action.param = Google_Protobuf_Any()
    action.param.value = try! deviceParam.serializedData()
    
    let paramHex = try! action.serializedData().key_toHexString()
    call_tcx_api(paramHex)
  }
  
  public class func deleteApp(){
    var deviceParam = Api_DeviceParam()
    deviceParam.action = "app_delete"
    deviceParam.param = Google_Protobuf_Any()
    
    var action = Api_TcxAction()
    action.method = "device_manage"
    action.param = Google_Protobuf_Any()
    action.param.value = try! deviceParam.serializedData()
    
    let paramHex = try! action.serializedData().key_toHexString()
    call_tcx_api(paramHex)
  }
  
  public class func getAddress(){
    return get_address()
  }
  
  public class func signTransaction(){
    return sign_transaction()
  }
  
  public class func eosSignTx(){
    Log.d("eos sign ...")
    var eosSignData = Eosapi_EosSignData()
    eosSignData.txHash = "c578065b93aec6a7c811000000000100a6823403ea3055000000572d3ccdcd01000000602a48b37400000000a8ed323225000000602a48b374208410425c95b1ca80969800000000000453595300000000046d656d6f00"
    eosSignData.pubKeys = ["EOS88XhiiP7Cu5TmAUJqHbyuhyYgd6sei68AU266PyetDDAtjmYWF"]
    eosSignData.chainID = "aca376f206b8fc25a6ed44dbdc66547c36c6c33e3a119ffbeaef943642f0e906"
    eosSignData.to = "liujianmin12"
    eosSignData.from = "liujianmin13"
    eosSignData.payment = "sellram 0.0739 EOS"

    var eosInput = Eosapi_EosTxInput()
    eosInput.path = BIP44.EOS_LEDGER
    eosInput.signDatas = [eosSignData]

    var signParam = Api_SignParam()
    signParam.chainType = "EOS"
    signParam.input = Google_Protobuf_Any()
    signParam.input.value = try! eosInput.serializedData()

    var action = Api_TcxAction()
    action.method = "sign_tx"
    action.param = Google_Protobuf_Any()
    action.param.value = try! signParam.serializedData()

    let paramHex = try! action.serializedData().key_toHexString()
    
    Log.d("eos param ready..")
    call_tcx_api(paramHex)
  }
  
  public class func eosPubkey(path:String) -> String{
    var addressParam = Api_AddressParam()
    addressParam.chainType = "EOS"
    addressParam.path = path
    
    var action = Api_TcxAction()
    action.method = "get_address"
    action.param = Google_Protobuf_Any()
    action.param.value = try! addressParam.serializedData()
    
    let paramHex = try! action.serializedData().key_toHexString()
    let res = call_tcx_api(paramHex)
    
    let strRes = String(cString:res!)
    let dataRes = strRes.key_dataFromHexString()!
    let eosPubkeyResponse = try! Eosapi_EosPubkeyResponse(serializedData: dataRes)
    return eosPubkeyResponse.pubkey
  }
  
  public class func cosmosAddress(path:String) -> String{
    var addressParam = Api_AddressParam()
    addressParam.chainType = "COSMOS"
    addressParam.path = path
    
    var action = Api_TcxAction()
    action.method = "get_address"
    action.param = Google_Protobuf_Any()
    action.param.value = try! addressParam.serializedData()
    
    let paramHex = try! action.serializedData().key_toHexString()
    let res = call_tcx_api(paramHex)
    
    let strRes = String(cString:res!)
    let dataRes = strRes.key_dataFromHexString()!
    let eosPubkeyResponse = try! Cosmosapi_CosmosAddressResponse(serializedData: dataRes)
    return eosPubkeyResponse.address
  }
  
  public class func ethAddress(path:String){

  }
}
