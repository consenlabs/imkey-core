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
  
  public class func eosSignTx(eosInput:Eosapi_EosTxInput) -> Eosapi_EosTxOutput{
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
    let res = call_tcx_api(paramHex)
    let strRes = String(cString:res!)
    let dataRes = strRes.key_dataFromHexString()!
    let eosOutput = try! Eosapi_EosTxOutput(serializedData: dataRes)
    return eosOutput
  }
  
  public class func cosmosSignTx(cosmosInput:Cosmosapi_CosmosTxInput) -> Cosmosapi_CosmosTxOutput{
    //call api
    var signParam = Api_SignParam()
    signParam.chainType = "COSMOS"
    signParam.input = Google_Protobuf_Any()
    signParam.input.value = try! cosmosInput.serializedData()

    var action = Api_TcxAction()
    action.method = "sign_tx"
    action.param = Google_Protobuf_Any()
    action.param.value = try! signParam.serializedData()

    let paramHex = try! action.serializedData().key_toHexString()
    
    //response
    Log.d("cosmos param ready..")
    let res = call_tcx_api(paramHex)
    let strRes = String(cString:res!)
    let dataRes = strRes.key_dataFromHexString()!
    let cosmosOutput = try! Cosmosapi_CosmosTxOutput(serializedData: dataRes)
    return cosmosOutput
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
  
  public class func eosReginPubkey(path:String) -> String{
    var addressParam = Api_AddressParam()
    addressParam.chainType = "EOS"
    addressParam.path = path
    
    var action = Api_TcxAction()
    action.method = "register_coin"
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
  
  public class func cosmosReginAddress(path:String) -> String{
    var addressParam = Api_AddressParam()
    addressParam.chainType = "COSMOS"
    addressParam.path = path
    
    var action = Api_TcxAction()
    action.method = "register_coin"
    action.param = Google_Protobuf_Any()
    action.param.value = try! addressParam.serializedData()
    
    let paramHex = try! action.serializedData().key_toHexString()
    let res = call_tcx_api(paramHex)
    
    let strRes = String(cString:res!)
    let dataRes = strRes.key_dataFromHexString()!
    let eosPubkeyResponse = try! Cosmosapi_CosmosAddressResponse(serializedData: dataRes)
    return eosPubkeyResponse.address
  }
  
  public class func ethAddress(path:String) -> String{
    var addressParam = Api_AddressParam()
    addressParam.chainType = "ETH"
    addressParam.path = path
    
    var action = Api_TcxAction()
    action.method = "get_address"
    action.param = Google_Protobuf_Any()
    action.param.value = try! addressParam.serializedData()
    
    let paramHex = try! action.serializedData().key_toHexString()
    let res = call_tcx_api(paramHex)
    
    let strRes = String(cString:res!)
    let dataRes = strRes.key_dataFromHexString()!
    let ethAddressResponse = try! Ethapi_EthAddressResponse(serializedData: dataRes)
    return ethAddressResponse.address
  }
  
  public class func ethReginAddress(path:String) -> String{
    var addressParam = Api_AddressParam()
    addressParam.chainType = "ETH"
    addressParam.path = path
    
    var action = Api_TcxAction()
    action.method = "register_coin"
    action.param = Google_Protobuf_Any()
    action.param.value = try! addressParam.serializedData()
    
    let paramHex = try! action.serializedData().key_toHexString()
    let res = call_tcx_api(paramHex)
    
    let strRes = String(cString:res!)
    let dataRes = strRes.key_dataFromHexString()!
    let ethAddressResponse = try! Ethapi_EthAddressResponse(serializedData: dataRes)
    return ethAddressResponse.address
  }
  
  public class func bindCheck() -> String{
    let storage = KeyFileStorage()
    let path = storage.getPath()
    
    var bindCheckParam = Deviceapi_BindCheck()
    bindCheckParam.filePath = path
    
    var deviceParam = Api_DeviceParam()
    deviceParam.action = "bind_check"
    deviceParam.param = Google_Protobuf_Any()
    deviceParam.param.value = try! bindCheckParam.serializedData()
    
    var action = Api_TcxAction()
    action.method = "device_manage"
    action.param = Google_Protobuf_Any()
    action.param.value = try! deviceParam.serializedData()
    
//    clear_err()
    let paramHex = try! action.serializedData().key_toHexString()
    let res = call_tcx_api(paramHex)
//    let error = get_last_err_message()
//    print(error)
    
    let strRes = String(cString:res!)
    let dataRes = strRes.key_dataFromHexString()!
    let bindResponse = try! Deviceapi_BindCheckResponse(serializedData: dataRes)
    return bindResponse.bindStatus
  }
  
  public class func bindAcquire(bindCode:String) -> String{
    var bindAcquireParam = Deviceapi_BindAcquire()
    bindAcquireParam.bindCode = bindCode
    
    var deviceParam = Api_DeviceParam()
    deviceParam.action = "bind_acquire"
    deviceParam.param = Google_Protobuf_Any()
    deviceParam.param.value = try! bindAcquireParam.serializedData()
    
    var action = Api_TcxAction()
    action.method = "device_manage"
    action.param = Google_Protobuf_Any()
    action.param.value = try! deviceParam.serializedData()
    
    //    clear_err()
    let paramHex = try! action.serializedData().key_toHexString()
    let res = call_tcx_api(paramHex)
    //    let error = get_last_err_message()
    //    print(error)
    
    let strRes = String(cString:res!)
    let dataRes = strRes.key_dataFromHexString()!
    let bindResponse = try! Deviceapi_BindAcquireResponse(serializedData: dataRes)
    return bindResponse.bindResult
  }
  
  public class func btcSignTX(){
    Log.d("btc sign ...")
    var btcInput = Btcapi_BtcTxInput()
    btcInput.amount = Int64(799988000)
    btcInput.to = "moLK3tBG86ifpDDTqAQzs4a9cUoNjVLRE3"
    btcInput.fee = Int64(10000)
    btcInput.payment = "0.0001 BT"
    btcInput.toDis = "3CVD68V71no5jn2UZpLLq6hASpXu1jrByt"
    btcInput.from = "3GrvKsZWbb9ocBaNF7XosFZEKuCVBRSoiy"
    btcInput.feeDis = "0.00007945 BTC"
    btcInput.pathPrefix = BIP44.btcMainnet
    btcInput.changeAddressIndex = 1
    btcInput.network = "TESTNET"
    
    let utxos = [
    [
      "txHash": "983adf9d813a2b8057454cc6f36c6081948af849966f9b9a33e5b653b02f227a",
      "vout": "0",
      "amount": "200000000",
      "address": "mh7jj2ELSQUvRQELbn9qyA4q5nADhmJmUC",
      "scriptPubKey": "76a914118c3123196e030a8a607c22bafc1577af61497d88ac",
      "derivedPath": "0/22"
    ],
    [
      "txHash": "45ef8ac7f78b3d7d5ce71ae7934aea02f4ece1af458773f12af8ca4d79a9b531",
      "vout": "1",
      "amount": "200000000",
      "address": "mkeNU5nVnozJiaACDELLCsVUc8Wxoh1rQN",
      "scriptPubKey": "76a914383fb81cb0a3fc724b5e08cf8bbd404336d711f688ac",
      "derivedPath": "0/0"
    ],
    [
      "txHash": "14c67e92611dc33df31887bbc468fbbb6df4b77f551071d888a195d1df402ca9",
      "vout": "0",
      "amount": "200000000",
      "address": "mkeNU5nVnozJiaACDELLCsVUc8Wxoh1rQN",
      "scriptPubKey": "76a914383fb81cb0a3fc724b5e08cf8bbd404336d711f688ac",
      "derivedPath": "0/0"
    ],
    [
      "txHash": "117fb6b85ded92e87ee3b599fb0468f13aa0c24b4a442a0d334fb184883e9ab9",
      "vout": "1",
      "amount": "200000000",
      "address": "mkeNU5nVnozJiaACDELLCsVUc8Wxoh1rQN",
      "scriptPubKey": "76a914383fb81cb0a3fc724b5e08cf8bbd404336d711f688ac",
      "derivedPath": "0/0"
    ]
      ].map { _ in Btcapi_Utxo() }
    btcInput.unspents = utxos
    
    var signParam = Api_SignParam()
    signParam.chainType = "BTC"
    signParam.input = Google_Protobuf_Any()
    signParam.input.value = try! btcInput.serializedData()

    var action = Api_TcxAction()
    action.method = "sign_tx"
    action.param = Google_Protobuf_Any()
    action.param.value = try! signParam.serializedData()

    let paramHex = try! action.serializedData().key_toHexString()
    
    Log.d("eos param ready..")
    let res = call_tcx_api(paramHex)
    let hexRes = String(cString:res!).key_toHexString()
    Log.d(hexRes)
  }
  
//  public class func cosmosSignxTX() -> TransactionSignedResult{
//    Cosmosapi_CosmosTxInput
//  }
}
