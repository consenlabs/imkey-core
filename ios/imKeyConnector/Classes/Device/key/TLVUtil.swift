//
//  TLVUtil.swift
//  imKeyConnector
//
//  Created by joe on 1/5/19.
//

import Foundation

public struct TlvObject{
  init() {
    tag = "tag"
    length = 0
    value = "value"
  }
  public var tag:String
  public var length:Int
  public var value:Any
}

public class TLVUtil{
  
  private class func biteOneByte(bytes:[UInt8]) ->(oneByte:UInt8,leftBytes:[UInt8]){
    let byte = bytes[0]
    var left = [UInt8](repeating: 0, count: bytes.count-1)
    left[0...] = bytes[1...]
    return (byte,left)
  }
  
  private class func cutBytes(bytes:[UInt8],num:Int)throws -> (cutBytes:[UInt8],leftBytes:[UInt8]){
    if num < bytes.count{
      var cut = [UInt8](repeating: 0, count: num)
      var left = [UInt8](repeating: 0, count: bytes.count-num)
      cut[0...] = bytes[0..<num]
      left[0...] = bytes[num...]
      return (cut,left)
    }else if num == bytes.count{
      let left:[UInt8] = []
      return (bytes,left)
    }else{
      throw SDKError.illegalArgument//todo: throw more info
    }
  }
  
  
  //do not delete this method，it can use for common tlv
  public class func findValue2(tlv:String, byTag:String)throws -> String{
    guard var leftBytes = ByteUtil.hexString2Uint8Array(data: tlv) else{
      throw SDKError.unwrapError
    }
    var oneByte:UInt8 = 0
//    var leftBytes:[UInt8] = []
    repeat{
      Log.d("\ntlv---------")
      var tagBytes:[UInt8] = []
      var lengthBytes:[UInt8] = []
      var valueBytes:[UInt8] = []
      (oneByte,leftBytes) = biteOneByte(bytes: leftBytes)
      tagBytes.append(oneByte)
      
      //tag
      if oneByte & 0x1F == 0x1F{//expend
        Log.d("expend")
        repeat{
          (oneByte,leftBytes) = biteOneByte(bytes: leftBytes)
          tagBytes.append(oneByte)
        }while oneByte & 0x80 == 0x80
      }
      
      let tag = ByteUtil.uint8Array2HexString(data: tagBytes)
      Log.d("tag:\(tag)")
      
      //length
      (oneByte,leftBytes) = biteOneByte(bytes: leftBytes)
      if oneByte & 0x80 == 0x80{// multi bytes for length
        let llLength = oneByte & 0x7F
        (lengthBytes,leftBytes) = try cutBytes(bytes: leftBytes, num: Int(llLength))
      }else{//one byte for length
        lengthBytes.append(oneByte & 0x7F)
      }
      let lengthStr = ByteUtil.uint8Array2HexString(data: lengthBytes)
      guard let length = Int(lengthStr,radix:16) else{
        throw SDKError.unwrapError
      }
      Log.d("length:\(length)")
      (valueBytes,leftBytes) = try cutBytes(bytes: leftBytes, num: length)
      let value = ByteUtil.uint8Array2HexString(data: valueBytes)
      Log.d("value:\(value)")
      
      if(tag == byTag){
        return value
      }
      
      if tagBytes[0] & 0x20 == 0x20 {//complex tlv
        Log.d("complex")
        let valueStr = ByteUtil.uint8Array2HexString(data: valueBytes)
        return try findValue2(tlv: valueStr, byTag: byTag)
      }
      Log.d("---------tlv")
    } while leftBytes.count > 0

    return ""
  }
  
  
  public class func findValue(tlv:String, byTag:String)throws -> String?{
    guard var leftBytes = ByteUtil.hexString2Uint8Array(data: tlv) else{
      throw SDKError.unwrapError
    }
    var oneByte:UInt8 = 0
    //    var leftBytes:[UInt8] = []
    repeat{
//      Log.d("\ntlv---------")
      var tagBytes:[UInt8] = []
      var lengthBytes:[UInt8] = []
      var valueBytes:[UInt8] = []
      (oneByte,leftBytes) = biteOneByte(bytes: leftBytes)
      tagBytes.append(oneByte)
      
      //tag
      if oneByte & 0x1F == 0x1F{//expend
//        Log.d("expend")
        repeat{
          (oneByte,leftBytes) = biteOneByte(bytes: leftBytes)
          tagBytes.append(oneByte)
        }while oneByte & 0x80 == 0x80
      }
      
      let tag = ByteUtil.uint8Array2HexString(data: tagBytes)
//      Log.d("tag:\(tag)")
      
      //length
      (oneByte,leftBytes) = biteOneByte(bytes: leftBytes)
      if oneByte & 0x80 == 0x80{// multi bytes for length
        let llLength = oneByte & 0x7F
        (lengthBytes,leftBytes) = try cutBytes(bytes: leftBytes, num: Int(llLength))
      }else{//one byte for length
        lengthBytes.append(oneByte & 0x7F)
      }
      let lengthStr = ByteUtil.uint8Array2HexString(data: lengthBytes)
      guard let length = Int(lengthStr,radix:16) else{
        throw SDKError.unwrapError
      }
//      Log.d("length:\(length)")
      (valueBytes,leftBytes) = try cutBytes(bytes: leftBytes, num: length)
      let value = ByteUtil.uint8Array2HexString(data: valueBytes)
//      Log.d("value:\(value)")
      
      if(tag == byTag){
        return value
      }
//      Log.d("---------tlv")
    } while leftBytes.count > 0
    
    return nil
  }
  
  public class func find7F49(cert:String)throws -> String?{
    guard let tlv7F21 = try findValue(tlv: cert, byTag: "7F21") else{
      return nil
    }
    Log.d("7f21.......\(tlv7F21)")
    let cert = try findValue(tlv: tlv7F21, byTag: "7F49")
    Log.d("7f49.......\(cert)")
    return cert
  }
  

  public class func decodeTLV(tlv:String)throws -> [TlvObject]{
    guard var leftBytes = ByteUtil.hexString2Uint8Array(data: tlv) else{
      throw SDKError.unwrapError
    }
    
    var tlvObjs:[TlvObject] = []
    
    var oneByte:UInt8 = 0
    //    var leftBytes:[UInt8] = []

    repeat{
//      Log.d("\ntlv---------")
      var tagBytes:[UInt8] = []
      var lengthBytes:[UInt8] = []
      var valueBytes:[UInt8] = []
      (oneByte,leftBytes) = biteOneByte(bytes: leftBytes)
      tagBytes.append(oneByte)
      
      var tlvObj = TlvObject()
      
      //tag
      if oneByte & 0x1F == 0x1F{//expend
//        Log.d("expend")
        repeat{
          (oneByte,leftBytes) = biteOneByte(bytes: leftBytes)
          tagBytes.append(oneByte)
        }while oneByte & 0x80 == 0x80
      }
      
      let tag = ByteUtil.uint8Array2HexString(data: tagBytes)
//      Log.d("tag:\(tag)")
      tlvObj.tag = tag
      
      //length
      (oneByte,leftBytes) = biteOneByte(bytes: leftBytes)
      if oneByte & 0x80 == 0x80{// multi bytes for length
        let llLength = oneByte & 0x7F
        (lengthBytes,leftBytes) = try cutBytes(bytes: leftBytes, num: Int(llLength))
      }else{//one byte for length
        lengthBytes.append(oneByte & 0x7F)
      }
      let lengthStr = ByteUtil.uint8Array2HexString(data: lengthBytes)
      guard let length = Int(lengthStr,radix:16) else{
        throw SDKError.unwrapError
      }
//      Log.d("length:\(length)")
      tlvObj.length = length
      
      //value
      (valueBytes,leftBytes) = try cutBytes(bytes: leftBytes, num: length)
      let value = ByteUtil.uint8Array2HexString(data: valueBytes)
//      Log.d("value:\(value)")
      tlvObj.value = value
      
      if tagBytes[0] & 0x20 == 0x20 {//complex tlv
//        Log.d("complex")
        let valueStr = ByteUtil.uint8Array2HexString(data: valueBytes)
        let childTlv =  try decodeTLV(tlv: valueStr)
        tlvObj.value = childTlv
      }
//      Log.d("---------tlv")
      
      tlvObjs.append(tlvObj)
    } while leftBytes.count > 0
    
    return tlvObjs
  }
  
  // encode r  s value to tlv String (DER)
  public class func encodeSignature(r: String, s: String)throws -> String{
    guard var rValue = ByteUtil.hexString2Uint8Array(data: r),
      var sValue = ByteUtil.hexString2Uint8Array(data: s) else{
        throw SDKError.illegalArgument
    }
    if rValue[0] > 0x7f {
      rValue.insert(0x00, at: 0)
    }
    if sValue[0] > 0x7f {
      sValue.insert(0x00, at: 0)
    }
    
    //insert length
    rValue.insert(UInt8(rValue.count), at: 0)
    sValue.insert(UInt8(sValue.count), at: 0)
    
    //insert tag
    rValue.insert(0x02, at: 0)
    sValue.insert(0x02, at: 0)
    
    //concat signature
    var tlvBytes:[UInt8] = []
    tlvBytes.append(contentsOf: rValue)
    tlvBytes.append(contentsOf: sValue)
    tlvBytes.insert(UInt8(tlvBytes.count), at: 0)
    tlvBytes.insert(0x30, at: 0)
    
    let tlv = ByteUtil.uint8Array2HexString(data: tlvBytes)
    Log.d(tlv)
    return tlv
  }
}
