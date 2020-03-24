//
//  EOSKey.swift
//  TokenCore
//
//  Created by James Chen on 2018/06/21.
//  Copyright © 2018 ConsenLabs. All rights reserved.
//

import Foundation
import CoreBitcoin

class EOSKey {
  private let pubKey: String
  
  init(pubKey: String) {
    self.pubKey = pubKey
  }
  
  func sign(handle:UInt, data: Data, hash: Data)throws -> Data {
    return try eosSignHW(handle: handle, forhash: data, hash: hash)
  }
  
  func pubKeyEos(pubKeyRaw: Data) -> String {
    let checksum = (BTCRIPEMD160(pubKeyRaw) as Data).bytes[0..<4]
    let base58 = BTCBase58StringWithData(pubKeyRaw + checksum)!
    return "EOS" + base58
  }
  
  func eosSignHW(handle:UInt, forhash: Data, hash: Data)throws -> Data{
    //apdu-prepare data
    var resApdu = ""
    var resCode: Int
    var sigResult = ""
    try Wallet.selectApplet(handle: handle, aid: Applet.eosAID)
    Log.d("pre data:\(forhash.bytes.toHexString())")
    let commands = APDU.eosPre(data: forhash.bytes.toHexString())   /* @XM@20180923 TODO: check this conversion */
    if let apdus = commands {
      resApdu = try BLE.shared().sendPrepareApdus(handle: handle, apdus: apdus)
    }
    let pkCompress = SigUtil.getPubKeyComp(xPub: resApdu)
    let pubKeyEos = self.pubKeyEos(pubKeyRaw: pkCompress)
    
    //send adpu-sign if compare result is ok：interate nonce value to sign the package
    if pubKeyEos == self.pubKey {
      var nonce = 0   //@XM@20180923 align with eosjs-ecc line 209@https://github.com/EOSIO/eosjs-ecc/blob/master/src/signature.js
      var v = 0
      var r = ""
      var s = ""
      while (true) {
        let apdu = APDU.eosSign(nonce: nonce)
        let sign = try BLE.shared().sendApdu(handle: handle, apdu: apdu!);
        try APDU.checkResponse(res: sign)
        let signRes = sign
        
        r = signRes.key_substring(from: 2).key_substring(to: 64)
        s = signRes.key_substring(from: 66).key_substring(to: 64)
        
        Log.d("\n*****************")
        Log.d("\(index) r:\(r)")
        Log.d("\(index) r:\(s)")
        
        let rBig = BTCBigNumber.init(string: r, base: 16)
        let sBig = BTCBigNumber.init(string: s, base: 16)
        let rPoint = UnsafeMutablePointer<BIGNUM>.allocate(capacity: 1)
        let sPoint = UnsafeMutablePointer<BIGNUM>.allocate(capacity: 1)
        rPoint.initialize(from: rBig!.bignum, count: 1)
        let s_low = SigUtil.getLowS(s:sBig!)
        sPoint.initialize(from: s_low.bignum, count: 1)
        var ecSig: ECDSA_SIG = ECDSA_SIG(r:rPoint,s:sPoint)
        var signature: UnsafeMutablePointer<UInt8>?
        let lenDer = i2d_ECDSA_SIG(&ecSig, &signature)
        var signDer = Data.init(bytes: signature!, count: Int(lenDer))
        
        let lenR = signDer[3];
        let lenS = signDer[5 + Int(lenR)];
        if (lenR == 32 && lenS == 32) {
          let btcKey = BTCKey.init(publicKey: pkCompress)
          v = Int(btcKey!.imKey_ECDSA_SIG_recover(&ecSig, forHash: hash, pubKey: pkCompress))
          break;
        }
        nonce += 1;
      }
      let headerByte = String(format: "%02X", v + 27 + 4)
      sigResult = headerByte + r + s
    }else{
      throw SDKError.pubKeyVerifyFailed
    }
    let sigData = sigResult.key_dataFromHexString()!
    return sigData
  }
  
  func eosPersonalSign(handle:UInt, forhash: Data, hash: Data)throws -> Data{
    //apdu-prepare data
    var resApdu = ""
    var resCode: Int
    var sigResult = ""
    try Wallet.selectApplet(handle: handle, aid: Applet.eosAID)
    let commands = APDU.eosMessagePre(data: forhash.bytes.toHexString())
    if let apdus = commands {
      resApdu = try BLE.shared().sendPrepareApdus(handle: handle, apdus: apdus)
    }
    let pkCompress = SigUtil.getPubKeyComp(xPub: resApdu)
    let pubKeyEos = self.pubKeyEos(pubKeyRaw: pkCompress)
    
    //send adpu-sign if compare result is ok：interate nonce value to sign the package
    if pubKeyEos == self.pubKey {
      var nonce = 0   //@XM@20180923 align with eosjs-ecc line 209@https://github.com/EOSIO/eosjs-ecc/blob/master/src/signature.js
      var v = 0
      var r = ""
      var s = ""
      while (true) {
        let apdu = APDU.eosMessageSign(nonce: nonce)
        let sign = try BLE.shared().sendApdu(handle: handle, apdu: apdu!);
        try APDU.checkResponse(res: sign)
        let signRes = sign
        
        r = signRes.key_substring(from: 2).key_substring(to: 64)
        s = signRes.key_substring(from: 66).key_substring(to: 64)
        
        Log.d("\n*****************")
        Log.d("\(index) r:\(r)")
        Log.d("\(index) r:\(s)")
        
        let rBig = BTCBigNumber.init(string: r, base: 16)
        let sBig = BTCBigNumber.init(string: s, base: 16)
        let rPoint = UnsafeMutablePointer<BIGNUM>.allocate(capacity: 1)
        let sPoint = UnsafeMutablePointer<BIGNUM>.allocate(capacity: 1)
        rPoint.initialize(from: rBig!.bignum, count: 1)
        let s_low = SigUtil.getLowS(s:sBig!)
        sPoint.initialize(from: s_low.bignum, count: 1)
        var ecSig: ECDSA_SIG = ECDSA_SIG(r:rPoint,s:sPoint)
        var signature: UnsafeMutablePointer<UInt8>?
        let lenDer = i2d_ECDSA_SIG(&ecSig, &signature)
        var signDer = Data.init(bytes: signature!, count: Int(lenDer))
        
        let lenR = signDer[3];
        let lenS = signDer[5 + Int(lenR)];
        if (lenR == 32 && lenS == 32) {
          let btcKey = BTCKey.init(publicKey: pkCompress)
          v = Int(btcKey!.imKey_ECDSA_SIG_recover(&ecSig, forHash: hash, pubKey: pkCompress))
          break;
        }
        nonce += 1;
      }
      let headerByte = String(format: "%02X", v + 27 + 4)
      sigResult = headerByte + r + s
    }else{
      throw SDKError.pubKeyVerifyFailed
    }
    let sigData = sigResult.key_dataFromHexString()!
    return sigData
  }
}
