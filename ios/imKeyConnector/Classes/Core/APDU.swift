//
//  Apdu.swift
//  Demo
//
//  Created by joe on 2018/7/3.
//  Copyright © 2018年 joe. All rights reserved.
//

import Foundation

public class APDU{
  public static let APDU_RSP_SUCCESS = "9000";
  public static let APDU_RSP_USER_NOT_CONFIRMED = "6940";
  public static let APDU_CONDITIONS_NOT_SATISFIED = "6985";
  public static let APDU_RSP_APPLET_NOT_EXIST = "6A82";
  public static let APDU_RSP_INCORRECT_P1P2 = "6A86";
  public static let APDU_RSP_CLA_NOT_SUPPORTED = "6E00";
  public static let APDU_RSP_APPLET_WRONG_DATA = "6A80";
  public static let APDU_RSP_WRONG_LENGTH = "6700";
  public static let APDU_RSP_SIGNATURE_VERIFY_FAILED = "6942";
  public static let APDU_RSP_EXCEEDED_MAX_UTXO_NUMBER = "6941";
  
  public static let btcSegwitPreType_output:UInt8 = 0x00
  public static let btcSegwitPreType_utxoHashVout:UInt8 = 0x40
  public static let btcSegwitPreType_utxoSequence:UInt8 = 0x80
  
  static let len = 245//data length in apdu
  public class func select(aid:String) -> String?{
    let byteAid:[UInt8] = ByteUtil.hexString2Uint8Array(data:aid)!
    var header:[UInt8] = [0x00,0xA4,0x04,0x00,UInt8(byteAid.count)]
    header.append(contentsOf: byteAid)
    return ByteUtil.uint8Array2HexString(data: header)
  }
  
  class func prepare(coinType:UInt8,data:String) -> [String]? {
    var apdus:[String] = []
    guard let bytes = ByteUtil.hexString2Uint8Array(data: data) else{
      return nil
    }
    let size = bytes.count / len + (bytes.count % len == 0 ? 0 : 1)
    for i in 0..<size{
      var apdu:[UInt8] = []
      if i == size - 1{
        apdu = [UInt8](repeating: 0, count: bytes.count - len*(size-1) + 6)
      }else{
        apdu = [UInt8](repeating: 0, count: len + 6)
      }
      apdu[0] = 0x80//CLA
      apdu[1] = coinType//INS
      //P1
      if i==0 {
        apdu[2] = 0x00
      }else{
        apdu[2] = 0x80
      }
      if i==size-1 {
        apdu[3] = 0x80//P2
        apdu[4] = UInt8(bytes.count - len*(size-1))//LC
        apdu[5..<apdu.count] = bytes[i*len..<bytes.count]
        //                apdu.append(bytes2[i*len..<bytes2.count-len*i])
      }else {
        apdu[3] = 0x00//P2
        apdu[4] = 0xF5//LC
        apdu[5..<apdu.count] = bytes[i*len..<i*len + len]
      }
      apdu.append(0x00)
      apdus.append(ByteUtil.uint8Array2HexString(data: apdu))
    }
    return apdus
  }
  
  class func sign(coinType:UInt8,index:UInt8,hashType:UInt8,path:String) -> String?{
    let bytes:[UInt8] = Array(path.utf8)
    var apdu:[UInt8] = [UInt8](repeating: 0, count: bytes.count + 6)
    apdu[0] = 0x80//CLA
    apdu[1] = coinType//INS
    apdu[2] = index//P1
    apdu[3] = hashType//P2
    apdu[4] = UInt8(bytes.count)//Lc
    apdu[5..<apdu.count] = bytes[0..<bytes.count]
    apdu.append(0x00)
    return ByteUtil.uint8Array2HexString(data: apdu)
  }
  
  public class func btcOutput(data:String)-> String?{
    guard let bytes = ByteUtil.hexString2Uint8Array(data: data) else{
      return nil
    }
    var apdu = [UInt8]()
    apdu.append(0x80)//CLA
    apdu.append(0x41)//INS
    apdu.append(0x00)//P1
    apdu.append(0x00)//P2
    apdu.append(UInt8(bytes.count))//Lc
    apdu.append(contentsOf: bytes)
    apdu.append(0x00)
    return ByteUtil.uint8Array2HexString(data: apdu)
  }
  
  public class func omniOutput(data:String)-> String?{
    guard let bytes = ByteUtil.hexString2Uint8Array(data: data) else{
      return nil
    }
    var apdu = [UInt8]()
    apdu.append(0x80)//CLA
    apdu.append(0x44)//INS
    apdu.append(0x00)//P1
    apdu.append(0x00)//P2
    apdu.append(UInt8(bytes.count))//Lc
    apdu.append(contentsOf: bytes)
    apdu.append(0x00)
    return ByteUtil.uint8Array2HexString(data: apdu)
  }
  
  public class func btcOutputs(data:String) -> [String]?{
    var apdus:[String] = []
    guard let bytes = ByteUtil.hexString2Uint8Array(data: data) else{
      return nil
    }
    let size = bytes.count / len + (bytes.count % len == 0 ? 0 : 1)
    for i in 0..<size{
      var apdu:[UInt8] = []
      if i == size - 1{
        apdu = [UInt8](repeating: 0, count: bytes.count - len*(size-1) + 6)
      }else{
        apdu = [UInt8](repeating: 0, count: len + 6)
      }
      apdu[0] = 0x80//CLA
      apdu[1] = 0x41//INS
      //P1
      apdu[2] = 0x00

      if i==size-1 {
        apdu[3] = 0x80//P2
        apdu[4] = UInt8(bytes.count - len*(size-1))//LC
        apdu[5..<apdu.count] = bytes[i*len..<bytes.count]
        //                apdu.append(bytes2[i*len..<bytes2.count-len*i])
      }else {
        apdu[3] = 0x00//P2
        apdu[4] = 0xF5//LC
        apdu[5..<apdu.count] = bytes[i*len..<i*len + len]
      }
      apdu.append(0x00)
      apdus.append(ByteUtil.uint8Array2HexString(data: apdu))
    }
    return apdus
  }
  
  
  public class func btcInput(data:String)-> String?{
    guard let bytes = ByteUtil.hexString2Uint8Array(data: data) else{
      return nil
    }
    var apdu = [UInt8]()
    apdu.append(0x80)//CLA
    apdu.append(0x41)//INS
    apdu.append(0x80)//P1
    apdu.append(0x00)//P2
    apdu.append(UInt8(bytes.count))//Lc
    apdu.append(contentsOf: bytes)
    apdu.append(0x00)
    return ByteUtil.uint8Array2HexString(data: apdu)
  }
  
  public class func omniInput(data:String)-> String?{
    guard let bytes = ByteUtil.hexString2Uint8Array(data: data) else{
      return nil
    }
    var apdu = [UInt8]()
    apdu.append(0x80)//CLA
    apdu.append(0x44)//INS
    apdu.append(0x80)//P1
    apdu.append(0x00)//P2
    apdu.append(UInt8(bytes.count))//Lc
    apdu.append(contentsOf: bytes)
    apdu.append(0x00)
    return ByteUtil.uint8Array2HexString(data: apdu)
  }
  
  public class func btcSegwitTransPre(data:String,type:UInt8) -> [String]?{
    var apdus:[String] = []
    guard let bytes = ByteUtil.hexString2Uint8Array(data: data) else{
      return nil
    }
    let size = bytes.count / len + (bytes.count % len == 0 ? 0 : 1)
    for i in 0..<size{
      var apdu:[UInt8] = []
      if i == size - 1{
        apdu = [UInt8](repeating: 0, count: bytes.count - len*(size-1) + 6)
      }else{
        apdu = [UInt8](repeating: 0, count: len + 6)
      }
      apdu[0] = 0x80//CLA
      apdu[1] = 0x31//INS
      apdu[2] = type//P1
      
      if i==size-1 {
        apdu[3] = 0x80//P2
        apdu[4] = UInt8(bytes.count - len*(size-1))//LC
        apdu[5..<apdu.count] = bytes[i*len..<bytes.count]
      }else {
        apdu[3] = 0x00//P2
        apdu[4] = 0xF5//LC
        apdu[5..<apdu.count] = bytes[i*len..<i*len + len]
      }
      apdu.append(0x00)
      apdus.append(ByteUtil.uint8Array2HexString(data: apdu))
    }
    return apdus
  }
  
  public class func omniSegwitTransPre(data:String,type:UInt8) -> [String]?{
    var apdus:[String] = []
    guard let bytes = ByteUtil.hexString2Uint8Array(data: data) else{
      return nil
    }
    let size = bytes.count / len + (bytes.count % len == 0 ? 0 : 1)
    for i in 0..<size{
      var apdu:[UInt8] = []
      if i == size - 1{
        apdu = [UInt8](repeating: 0, count: bytes.count - len*(size-1) + 6)
      }else{
        apdu = [UInt8](repeating: 0, count: len + 6)
      }
      apdu[0] = 0x80//CLA
      apdu[1] = 0x34//INS
      apdu[2] = type//P1
      
      if i==size-1 {
        apdu[3] = 0x80//P2
        apdu[4] = UInt8(bytes.count - len*(size-1))//LC
        apdu[5..<apdu.count] = bytes[i*len..<bytes.count]
      }else {
        apdu[3] = 0x00//P2
        apdu[4] = 0xF5//LC
        apdu[5..<apdu.count] = bytes[i*len..<i*len + len]
      }
      apdu.append(0x00)
      apdus.append(ByteUtil.uint8Array2HexString(data: apdu))
    }
    return apdus
  }
  
  public class func btcSegwitTransSign(utxo:String,path:String,isLastUtxo:Bool) -> String?{
    guard let utxoBytes = ByteUtil.hexString2Uint8Array(data: utxo) else{
      return nil
    }
    let pathBytes:[UInt8] = Array(path.utf8)
    
    var bytes = [UInt8]()
    bytes.append(UInt8(utxoBytes.count))
    bytes.append(contentsOf: utxoBytes)
    bytes.append(UInt8(pathBytes.count))
    bytes.append(contentsOf: pathBytes)
    
    var apdu = [UInt8]()
    apdu.append(0x80)//CLA
    apdu.append(0x32)//INS
    apdu.append(isLastUtxo ? 0x80 : 0x00)//P1
    apdu.append(0x01)//P2   hash type, 0x01 = all
    apdu.append(UInt8(bytes.count))//Lc
    apdu.append(contentsOf: bytes)
    apdu.append(0x00)
    return ByteUtil.uint8Array2HexString(data: apdu)
  }
  
  public class func ethPre(data:String) -> [String]?{
    return prepare(coinType: 0x51, data: data)
  }
  
  public class func cosmosPre(data:String) -> [String]?{
    return prepare(coinType: 0x71, data: data)
  }
  
  public class func eosPre(data:String) -> [String]?{
    return prepare(coinType: 0x61, data: data)
  }
  
  public class func cosmosSign(path:String) -> String? {
    return sign(coinType: 0x72, index: 0x00, hashType: 0x00, path: path)
  }
  
  public class func btcSign(index:UInt8,hashType:UInt8,path:String) -> String? {
    return sign(coinType: 0x42, index: index, hashType: hashType, path: path)
  }
  
  public class func omniSign(index:UInt8,hashType:UInt8,path:String) -> String? {
    return sign(coinType: 0x42, index: index, hashType: hashType, path: path)
  }
  
  public class func btcSegWitSign(index:UInt8,hashType:UInt8,path:String) -> String? {
    return sign(coinType: 0x32, index: index, hashType: hashType, path: path)
  }
  
  public class func ethSign(index:UInt8,hashType:UInt8,path:String) -> String? {
    return sign(coinType: 0x52, index: index, hashType: hashType, path: path)
  }
  
  public class func eosSign(nonce: Int) -> String? {
    var apdu:[UInt8] = [UInt8](repeating: 0, count: 8)
    apdu[0] = 0x80//CLA
    apdu[1] = 0x62//INS
    apdu[2] = 0x00//P1
    apdu[3] = 0x00//P2
    apdu[4] = 2//Lc
    apdu[5] = UInt8((nonce & 0xFF00) >> 8)
    apdu[6] = UInt8((nonce & 0x00FF))
    apdu[7] = 0x00 //le
    return ByteUtil.uint8Array2HexString(data: apdu)
  }
  
  public class func btcXpub(path:String,verifyKey:Bool = false) -> String{
    let uint8Array = Array(path.utf8)
    var apdu:[UInt8] = [UInt8](repeating: 0, count: 6 + uint8Array.count)
    Log.d(apdu.count)
    apdu[0] = 0x80//CLA
    apdu[1] = 0x43//INS
    apdu[2] = verifyKey ? 0x01 : 0x00//P1
    apdu[3] = 0x00//P2
    apdu[4] = UInt8(uint8Array.count)//LC
    apdu[5...] = uint8Array[0..<uint8Array.count]
    apdu.append(0x00)//LE
    return ByteUtil.uint8Array2HexString(data: apdu)
  }
  
  public class func ethXpub(path:String,verifyKey:Bool = false) -> String{
    let uint8Array = Array(path.utf8)
    var apdu:[UInt8] = [UInt8](repeating: 0, count: 6 + uint8Array.count)
    Log.d(apdu.count)
    apdu[0] = 0x80//CLA
    apdu[1] = 0x53//INS
    apdu[2] = verifyKey ? 0x01 : 0x00//P1
    apdu[3] = 0x00//P2
    apdu[4] = UInt8(uint8Array.count)//LC
    apdu[5...] = uint8Array[0..<uint8Array.count]
    apdu.append(0x00)//LE
    return ByteUtil.uint8Array2HexString(data: apdu)
  }
  
  public class func eosXpub(path:String,verifyKey:Bool = false) -> String{
    let uint8Array = Array(path.utf8)
    var apdu:[UInt8] = [UInt8](repeating: 0, count: 6 + uint8Array.count)
    Log.d(apdu.count)
    apdu[0] = 0x80//CLA
    apdu[1] = 0x63//INS
    apdu[2] = verifyKey ? 0x01 : 0x00//P1
    apdu[3] = 0x00//P2
    apdu[4] = UInt8(uint8Array.count)//LC
    apdu[5...] = uint8Array[0..<uint8Array.count]
    apdu.append(0x00)//LE
    return ByteUtil.uint8Array2HexString(data: apdu)
  }
  
  public class func cosmosXpub(path:String,verifyKey:Bool = false) -> String{
    let uint8Array = Array(path.utf8)
    var apdu:[UInt8] = [UInt8](repeating: 0, count: 6 + uint8Array.count)
    Log.d(apdu.count)
    apdu[0] = 0x80//CLA
    apdu[1] = 0x73//INS
    apdu[2] = verifyKey ? 0x01 : 0x00//P1
    apdu[3] = 0x00//P2
    apdu[4] = UInt8(uint8Array.count)//LC
    apdu[5...] = uint8Array[0..<uint8Array.count]
    apdu.append(0x00)//LE
    return ByteUtil.uint8Array2HexString(data: apdu)
  }
  
  //@XM@20181114 TODO: please delete these two functions
  class func btcMessagePrepare(data:String) -> [String]?{
    return prepare(coinType: 0x44, data: data)
  }
  
  class func btcMessageSign(path:String) -> String{
    let uint8Array = Array(path.utf8)
    var apdu:[UInt8] = [UInt8](repeating: 0, count: 6 + uint8Array.count)
    apdu[0] = 0x80//CLA
    apdu[1] = 0x45//INS
    apdu[2] = 0x00//P1
    apdu[3] = 0x00//P2
    apdu[4] = UInt8(uint8Array.count)//LC
    apdu[5...] = uint8Array[0..<uint8Array.count]
    apdu.append(0x00)//LE
    return ByteUtil.uint8Array2HexString(data: apdu)
  }
  
  public class func ethMessagePrepare(data:String) -> [String]?{
    return prepare(coinType: 0x54, data: data)
  }
  
  public class func ethMessageSign(path:String) -> String{
    let uint8Array = Array(path.utf8)
    var apdu:[UInt8] = [UInt8](repeating: 0, count: 6 + uint8Array.count)
    apdu[0] = 0x80//CLA
    apdu[1] = 0x55//INS
    apdu[2] = 0x00//P1
    apdu[3] = 0x00//P2
    apdu[4] = UInt8(uint8Array.count)//LC
    apdu[5...] = uint8Array[0..<uint8Array.count]
    apdu.append(0x00)//LE
    return ByteUtil.uint8Array2HexString(data: apdu)
  }
  
  public class func ftSign(path:String,data:String) -> String{
    let uint8Array = Array(path.utf8)
    guard let bytes = ByteUtil.hexString2Uint8Array(data: data) else{
      return ""
    }
    
//    let dataBytes = bytes[0...31]
    let dataBytes = bytes
    
    var apdu = [UInt8]()
    apdu.append(0x80)//CLA
    apdu.append(0x81)//INS
    apdu.append(0x01)//P1
    apdu.append(0x80)//P2
    apdu.append(UInt8(uint8Array.count + dataBytes.count + 2))//LC
    apdu.append(UInt8(dataBytes.count))//data len
    apdu.append(contentsOf: dataBytes)//sign data
    apdu.append(UInt8(uint8Array.count))//path len
    apdu.append(contentsOf: uint8Array)//path
    apdu.append(0x00)//LE
    return ByteUtil.uint8Array2HexString(data: apdu)
  }
  
  public class func mvcSign(path:String,bytes:[UInt8]) -> String{
    let uint8Array = Array(path.utf8)
    
    //    let dataBytes = bytes[0...31]
    let dataBytes = bytes
    
    var apdu = [UInt8]()
    apdu.append(0x80)//CLA
    apdu.append(0x81)//INS
    apdu.append(0x01)//P1
    apdu.append(0x80)//P2
    apdu.append(UInt8(uint8Array.count + dataBytes.count + 2))//LC
    apdu.append(UInt8(dataBytes.count))//data len
    apdu.append(contentsOf: dataBytes)//sign data
    apdu.append(UInt8(uint8Array.count))//path len
    apdu.append(contentsOf: uint8Array)//path
    apdu.append(0x00)//LE
    return ByteUtil.uint8Array2HexString(data: apdu)
  }
  
  public class func eosMessagePre(data:String) -> [String]?{
    return prepare(coinType: 0x64, data: data)
  }
  
  public class func eosMessageSign(nonce: Int) -> String? {
    var apdu:[UInt8] = [UInt8](repeating: 0, count: 8)
    apdu[0] = 0x80//CLA
    apdu[1] = 0x65//INS
    apdu[2] = 0x00//P1
    apdu[3] = 0x00//P2
    apdu[4] = 2//Lc
    apdu[5] = UInt8((nonce & 0xFF00) >> 8)
    apdu[6] = UInt8((nonce & 0x00FF))
    apdu[7] = 0x00 //le
    return ByteUtil.uint8Array2HexString(data: apdu)
  }
  
  public class func setBTCAddress(address: String) -> String{
    let uint8Array = Array(address.utf8)
    var apdu:[UInt8] = [UInt8](repeating: 0, count: 6 + uint8Array.count)
    apdu[0] = 0x80//CLA
    apdu[1] = 0x36//INS
    apdu[2] = 0x00//P1
    apdu[3] = 0x00//P2
    apdu[4] = UInt8(uint8Array.count)//LC
    apdu[5...] = uint8Array[0..<uint8Array.count]
    apdu.append(0x00)//LE
    return ByteUtil.uint8Array2HexString(data: apdu)
  }
  
  public class func setBTCSegWitAddress(address: String) -> String{
    let uint8Array = Array(address.utf8)
    var apdu:[UInt8] = [UInt8](repeating: 0, count: 6 + uint8Array.count)
    apdu[0] = 0x80//CLA
    apdu[1] = 0x36//INS
    apdu[2] = 0x01//P1
    apdu[3] = 0x00//P2
    apdu[4] = UInt8(uint8Array.count)//LC
    apdu[5...] = uint8Array[0..<uint8Array.count]
    apdu.append(0x00)//LE
    return ByteUtil.uint8Array2HexString(data: apdu)
  }
  
  public class func setETHAddress(address: String) -> String{
    let uint8Array = Array(address.utf8)
    var apdu:[UInt8] = [UInt8](repeating: 0, count: 6 + uint8Array.count)
    apdu[0] = 0x80//CLA
    apdu[1] = 0x56//INS
    apdu[2] = 0x00//P1
    apdu[3] = 0x00//P2
    apdu[4] = UInt8(uint8Array.count)//LC
    apdu[5...] = uint8Array[0..<uint8Array.count]
    apdu.append(0x00)//LE
    return ByteUtil.uint8Array2HexString(data: apdu)
  }
  
  public class func setEOSPubkey(pubkey: String) -> String{
    let uint8Array = Array(pubkey.utf8)
    var apdu:[UInt8] = [UInt8](repeating: 0, count: 6 + uint8Array.count)
    apdu[0] = 0x80//CLA
    apdu[1] = 0x66//INS
    apdu[2] = 0x00//P1
    apdu[3] = 0x00//P2
    apdu[4] = UInt8(uint8Array.count)//LC
    apdu[5...] = uint8Array[0..<uint8Array.count]
    apdu.append(0x00)//LE
    return ByteUtil.uint8Array2HexString(data: apdu)
  }
  
  public class func setCosmosAddress(pubkey: String) -> String{
    let uint8Array = Array(pubkey.utf8)
    var apdu:[UInt8] = [UInt8](repeating: 0, count: 6 + uint8Array.count)
    apdu[0] = 0x80//CLA
    apdu[1] = 0x76//INS
    apdu[2] = 0x00//P1
    apdu[3] = 0x00//P2
    apdu[4] = UInt8(uint8Array.count)//LC
    apdu[5...] = uint8Array[0..<uint8Array.count]
    apdu.append(0x00)//LE
    return ByteUtil.uint8Array2HexString(data: apdu)
  }
  
  class func seID() -> String{
    return "80CB800005DFFF028101"
  }
  
  class func sn() -> String{
    return "80CA004400"
  }
  
  class func selectMainSE() -> String{
    return "00A4040000"
  }
  
  class func cert() -> String{
    return "80CABF2106A6048302151800"
  }
  
  class func battery() -> String{
    return "00D6FEED01"
  }
  
  class func firmwareVersion() -> String{
    return "80CB800005DFFF02800300"
  }
  
  class func lifeTime() -> String{
    return "FFDCFEED00"
  }
  
  class func getBLEName() -> String{
    return "FFDB465400"
  }
  
  class func getBLEVersion() -> String{
    return "80CB800005DFFF02810000"
  }
  
  public class func setBLEName(bleName:String) -> String{
    let uint8Array = Array(bleName.utf8)
    var apdu:[UInt8] = [UInt8](repeating: 0, count: 6 + uint8Array.count)
    apdu[0] = 0xFF//CLA
    apdu[1] = 0xDA//INS
    apdu[2] = 0x46//P1
    apdu[3] = 0x54//P2
    apdu[4] = UInt8(uint8Array.count)//LC
    apdu[5...] = uint8Array[0..<uint8Array.count]
    apdu.append(0x00)//LE
    return ByteUtil.uint8Array2HexString(data: apdu)
  }
  
  class func reset()->String{
    return "80CB800005DFFE02814700"
  }
  
  class func getStatus(apdu:String) -> String{
    let lowerBounds = String.Index(encodedOffset: apdu.count-4)
    let upperBounds = String.Index(encodedOffset: apdu.count)
    return String(apdu[lowerBounds..<upperBounds])
  }
  
  class func removeStatus(apdu:String) -> String{
    let lowerBounds = String.Index(encodedOffset: 0)
    let upperBounds = String.Index(encodedOffset: apdu.count-4)
    return String(apdu[lowerBounds..<upperBounds])
  }
  
  class func checkResponse(res:String)throws {
    if res.hasSuffix(APDU_RSP_SUCCESS){//9000 means success
      return
    }
    
    //exception
    switch res {
    case APDU_RSP_USER_NOT_CONFIRMED:
      throw APDUError.userNotConfirm
    case APDU_CONDITIONS_NOT_SATISFIED:
      throw APDUError.conditionsNotStatisfied
    case APDU_RSP_INCORRECT_P1P2,               //incorrect p1p2
    APDU_RSP_CLA_NOT_SUPPORTED:            //CLA not support
      throw APDUError.cmdFormatError
    case APDU_RSP_APPLET_WRONG_DATA:            //wrong data
      throw APDUError.cmdDataError
    case APDU_RSP_APPLET_NOT_EXIST:            //applet does not exist
      throw APDUError.appletNotExist
    case APDU_RSP_WRONG_LENGTH:
      throw APDUError.wrongLength
    case APDU_RSP_SIGNATURE_VERIFY_FAILED:
      throw SDKError.signVerifyFail
    case APDU_RSP_EXCEEDED_MAX_UTXO_NUMBER:
      throw SDKError.exceededMaxUtxoNum
    default:
      throw SDKError.unknownError
    }
  }
  
  public class func bindCheck(appPubkey: [UInt8]) -> String{
//    let uint8Array = Array(appPubkey.utf8)
    var apdu:[UInt8] = [UInt8](repeating: 0, count: 6 + appPubkey.count)
    apdu[0] = 0x80//CLA
    apdu[1] = 0x71//INS
    apdu[2] = 0x00//P1
    apdu[3] = 0x00//P2
    apdu[4] = UInt8(appPubkey.count)//LC
    apdu[5...] = appPubkey[0..<appPubkey.count]
    apdu.append(0x00)//LE
    return ByteUtil.uint8Array2HexString(data: apdu)
  }
  
  public class func genAuthCode() -> String{
    return "8072000000"
  }
  
  
  //data: appPk(65 bytes) |L| hash encrypted using aes
  public class func identyVerify(data: [UInt8],bind:Bool) -> String{
    var apdu:[UInt8] = [UInt8](repeating: 0, count: 6 + data.count)
    apdu[0] = 0x80//CLA
    apdu[1] = 0x73//INS
    //P1
    if bind {
      apdu[2] = 0x80
    }else {
      apdu[2] = 0x00
    }
    apdu[3] = 0x00//P2
    apdu[4] = UInt8(data.count)//LC
    apdu[5...] = data[0..<data.count]
    apdu.append(0x00)//LE
    return ByteUtil.uint8Array2HexString(data: apdu)
  }
}
