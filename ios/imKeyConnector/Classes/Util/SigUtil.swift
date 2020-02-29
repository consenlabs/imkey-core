//
//  SigUtil.swift
//  token
//
//  Created by Kai Chen on 16/11/2017.
//  Copyright © 2017 ConsenLabs. All rights reserved.
//

import Foundation
import CoreBitcoin

public typealias ECSignature = [String: Any] // -> { v: integer, r: string, s: string }

public struct SigUtil {
  //below are added for imKey
  public static func getPubKeyComp(xPub:String) -> Data{
    var xPubBytes = Hex.toBytes(xPub)
    var pubKey = ""
    if xPubBytes[64] % 2 != 0 { //@XM@20181113 TODO: add protection or exception
      pubKey = "03" + xPub.key_substring(from: 2).key_substring(to: 64)
    } else {
      pubKey = "02" + xPub.key_substring(from: 2).key_substring(to: 64)
    }
    let pkData = BTCDataFromHex(pubKey)
    return pkData!
  }
  
  public static func getLowS(s: BTCBigNumber) ->  BTCBigNumber {
    let halfCurveOrder = BTCBigNumber.init(string: "7FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF5D576E7357A4501DDFE92F46681B20A0", base: 16)
    let curveN = BTCBigNumber.init(string: "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEBAAEDCE6AF48A03BBFD25E8CD0364141", base: 16)
    if s.greater(halfCurveOrder) {
      let res = curveN!.mutableCopy()!.subtract(s)!;
      return res
    }
    return s;
  }
  
  public static func unpackSig(sig: String) throws -> (String, Int32) {
    guard sig.count == 130 else {
      throw GenericError.paramError
    }
    let signature = sig.key_substring(to: 128)
    let recIdStr = sig.key_substring(from: 128)
    
    guard let recId = Int32(recIdStr, radix: 16) else {
      throw GenericError.paramError
    }
//    return (signature, recId - 27) //???
    return (signature, recId)
  }
  
  public static func ecrecover(signature: String, recid: Int32, forHash msgHash: String) -> String? {
    return Encryptor.Secp256k1().recover(signature: signature, message: msgHash, recid: recid)
  }
  
  public static func ecsign(with privateKey: String, data: String) -> ECSignature {
    let result = Encryptor.Secp256k1().sign(key: privateKey, message: data)
    let v = result.recid + 27
    let r = result.signature.key_substring(to: 64)
    let s = result.signature.key_substring(from: 64)
    return ["v": v, "r": r, "s": s]
  }
  
  public static func ecSignHash(hash:[UInt8])throws -> [UInt8]{
    let nilValue:[UInt8] = [UInt8](repeating: 0, count: 32)
    if KeyManager.shared().prvKey == nilValue{
      throw SDKError.notBindCheck
    }
    let prvKey = ByteUtil.uint8Array2HexString(data: KeyManager.shared().prvKey)
    Log.d("prvKey:\(prvKey)")
    let result = ecsign(with: prvKey, data: ByteUtil.uint8Array2HexString(data: hash).lowercased())
    Log.d("result:\(result)")
    let sig = try TLVUtil.encodeSignature(r: result["r"] as! String, s: result["s"] as! String)
    guard let sigBytes = ByteUtil.hexString2Uint8Array(data: sig) else{
      throw SDKError.unwrapError
    }
    return sigBytes
  }
}
