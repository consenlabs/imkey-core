//
//  Wallet.swift
//  ImkeyLibrary
//
//  Created by joe on 2018/9/5.
//  Copyright © 2018年 joe. All rights reserved.
//

import Foundation
import CoreBitcoin

public class Wallet{
  public class func selectApplet(handle:UInt,aid:String)throws {
    guard let apdu = APDU.select(aid: aid) else{
      return
    }
    let resApdu = try BLE.shared().sendApdu(handle: handle, apdu: apdu)
    try APDU.checkResponse(res: resApdu)
  }
  
  // mainnet：76067358(0x0488B21E) testnet：70617039(0x043587CF)
  public class func getBTCXpub(handle:UInt,version:Int32,path:String)throws -> String{
    //serialize
    //1.version
    //2.depth
    //3.fingerprint
    //4.child number
    //5.chain code
    //6.pubkey or prvkey
    let payload = NSMutableData()
    var vVersion = version.bigEndian
    payload.append(&vVersion, length: 4)
    
    let subPaths = path.split(separator: "/")
    Log.d(subPaths)
    var depth = UInt8(subPaths.count-1)
    payload.append(&depth, length: 1)
    
    var parentPath:String = ""
    for (index,subPath) in subPaths.enumerated() {
      if(index == subPaths.count-1){
        continue
      }
      if(index == 0){
        parentPath += String(subPath)
      }else {
        parentPath += "/" + String(subPath)
      }
    }
    let parentXpubHex = try getBTCXPubHex(handle: handle, path: parentPath)
    let parentComprsPub = SigUtil.getPubKeyComp(xPub: parentXpubHex)
    let hash160 = BTCRIPEMD160(parentComprsPub.sha256()).hex()!
    let head = hash160.key_substring(to: 8)
    guard var fingerPrint = UInt32(head,radix:16) else{
      throw SDKError.unwrapError
    }
    fingerPrint = fingerPrint.bigEndian
    payload.append(&fingerPrint, length: 4)
    
    let child = String(subPaths[subPaths.count-1])
    var childNum:UInt32
    if(child.contains("\'")){
      childNum = 0x80000000|UInt32(String(child.dropLast()))!
    }else{
      childNum = UInt32(child)!
    }
    childNum = childNum.bigEndian
    payload.append(&childNum, length: 4)
    
    let xpubHex = try getBTCXPubHex(handle: handle, path: path, select: false)
    let chainCode =  ByteUtil.hexString2Uint8Array(data: xpubHex.key_substring(to: 194).key_substring(from: 130))!
    payload.append(Data(chainCode))
    
    let comprsPub = ByteUtil.hexString2Uint8Array(data: SigUtil.getPubKeyComp(xPub: xpubHex).toHexString())!
    payload.append(Data(comprsPub))
    
    let data = payload as Data
    return BTCBase58CheckStringWithData(data)
  }
  
  public class func getBTCXPubHex(handle:UInt,path:String,select:Bool = true,verifyKey:Bool = true)throws -> String{
    if select == true {
      try selectApplet(handle: handle, aid: Applet.btcAID)
    }
    let apdu = APDU.btcXpub(path: path,verifyKey: verifyKey)
    let res = try BLE.shared().sendApdu(handle: handle, apdu: apdu)
    try APDU.checkResponse(res: res)
    
    let xpubHex = res.key_substring(to: res.count - 4)
    if verifyKey{
//      try verifySEPubkey(xpubHex: res)
      try verifyDerSig(xpubHex: xpubHex)
    }
    return xpubHex
  }
  
  
  //this feature implement in applet now
  private class func verifyPubKey(data:String,xpubHex:String)throws {
    let sig = xpubHex.key_substring(from: 196).key_substring(to: 130)
    let (signature, recId) = try SigUtil.unpackSig(sig: sig)
    guard var recoverPubkey = SigUtil.ecrecover(signature: signature, recid: recId, forHash: data.sha256()) else{
      throw SDKError.unwrapError
    }
    recoverPubkey = recoverPubkey.uppercased()
    Log.d("recoverPubkey:\n")
    Log.d(recoverPubkey)
    let pubkey = xpubHex.key_substring(to: 130)
    Log.d("pubkey:\n")
    Log.d(pubkey)
    if recoverPubkey != pubkey {
      throw SDKError.pubkeyInvalid
    }
  }
  
  public class func verifyDerSig(xpubHex:String)throws{
    let nilValue:[UInt8] = [UInt8](repeating: 0, count: 65)
    if KeyManager.shared().sePubKey == nilValue{
      throw SDKError.notBindCheck
    }
    let pkData = Data(KeyManager.shared().sePubKey)
    let btckey = BTCKey(publicKey: pkData)!
    
    let data = xpubHex.key_substring(to: 194)
    let sig = xpubHex.key_substring(from: 194)
    
    let dataBytes = ByteUtil.hexString2Uint8Array(data: data)
    let sha256 = ByteUtil.uint8Array2HexString(data: dataBytes!.sha256())
    
    let isValid = btckey.isValidSignature(Data(hex: sig), hash: Data(hex: sha256))
    
    if !isValid {
      throw SDKError.signVerifyFail
    }
  }
  
  private class func verifySEPubkey(xpubHex:String)throws {
    let data = xpubHex.key_substring(to: 194)
    let sig = xpubHex.key_substring(from: 194)
//    try TLVUtil.findValue2(tlv: sig, byTag: "hhh")
    let tlvs = try TLVUtil.decodeTLV(tlv: sig)
    Log.d("tlvs:\(tlvs)")
    
    let childTlvs = tlvs[0].value as! [TlvObject]
    let r = childTlvs[0].length == 32 ? childTlvs[0].value as! String
      : (childTlvs[0].value as! String).key_substring(from: 2)
    let s = childTlvs[1].length == 32 ? childTlvs[1].value as! String
      : (childTlvs[1].value as! String).key_substring(from: 2)
    let signature = r + s
    
    let nilValue:[UInt8] = [UInt8](repeating: 0, count: 65)
    if KeyManager.shared().sePubKey == nilValue{
      throw SDKError.notBindCheck
    }
    let sePK = ByteUtil.uint8Array2HexString(data: KeyManager.shared().sePubKey)
    
    let dataBytes = ByteUtil.hexString2Uint8Array(data: data)
    let sha256 = ByteUtil.uint8Array2HexString(data: dataBytes!.sha256())
    
    var pass = false
    // recId in 0...3
    for i in 0...3 {
      var recoverPubkey = SigUtil.ecrecover(signature: signature, recid: Int32(i), forHash: sha256)
      recoverPubkey = recoverPubkey?.uppercased()
      if sePK == recoverPubkey{
        Log.d("rcid is \(i)")
        pass = true
        break
      }
    }
    
    if !pass {
      throw SDKError.signVerifyFail
    }
  }
  
  public class func getETHXPub(handle:UInt,path:String,select:Bool = true,verifyKey:Bool = true)throws -> String{
    if select == true {
      try selectApplet(handle: handle, aid: Applet.ethAID)
    }
    let apdu = APDU.ethXpub(path: path, verifyKey: verifyKey)
    let res = try BLE.shared().sendApdu(handle: handle, apdu: apdu)
    try APDU.checkResponse(res: res)
    
    if verifyKey{
//      try verifySEPubkey(xpubHex: res)
      try verifyDerSig(xpubHex: res.key_substring(to: res.count - 4))
    }
    return res
  }
  
  public class func getEOSPubkey(handle:UInt,path:String,select:Bool = true,verifyKey:Bool = true)throws -> String{
    if select == true {
      try selectApplet(handle: handle, aid: Applet.eosAID)
    }
    let apdu = APDU.eosXpub(path: path, verifyKey: verifyKey)
    let res = try BLE.shared().sendApdu(handle: handle, apdu: apdu)
    try APDU.checkResponse(res: res)
    if verifyKey{
      try verifyDerSig(xpubHex: res.key_substring(to: res.count - 4))
//      try verifySEPubkey(xpubHex: res.key_substring(to: res.count - 4))
    }
    let comprsPub = SigUtil.getPubKeyComp(xPub: res)
    let hash = BTCRIPEMD160(comprsPub) as Data
    let checkSum = hash.key_toHexString().key_substring(to: 8)
    let addChecksum = (comprsPub.key_toHexString() + checkSum).uppercased()
    let finalData = Data(ByteUtil.hexString2Uint8Array(data: addChecksum)!)
    let eosPub = "EOS" + BTCBase58StringWithData(finalData)
    return eosPub
  }
  
  //version mainnet:0, testnet:111
  public class func getBTCAddress(handle:UInt, version:Int32, path:String)throws -> String?{
    // get pubkey
    let xPub = try getBTCXPubHex(handle: handle, path: path)
    let pubKey = SigUtil.getPubKeyComp(xPub: xPub)
    
    let key = BTCKey.init(publicKey: pubKey)
    let address : BTCPublicKeyAddress!
    if version == 0 {
      address = key?.address
    } else {
      address = key?.addressTestnet
    }
    return address?.string
    
    /*@XM@20181114 DON'T DELETE: below are xiangyun's implementation
     //ripemd-160
     guard let hash160 = BTCRIPEMD160(pubKey.sha256()).hex() else {
     return nil
     }
     let hexVersion = String(format: "%02x", version)
     let addVersion = hexVersion + hash160
     let data = BTCDataFromHex(addVersion)
     return BTCBase58CheckStringWithData(data)
     */
  }
  
  //version mainnet:5, testnet:196
  public class func getBTCSegwitAddress(handle:UInt, version:Int32, path:String)throws -> String?{
    // get pubkey
    let xPub = try getBTCXPubHex(handle: handle, path: path)
    let pubKey = SigUtil.getPubKeyComp(xPub: xPub)
    
    let key = BTCKey.init(publicKey: pubKey)
    let address : BTCScriptHashAddress!
    if version == 5 {
      address = key?.witnessAddress
    } else {
      address = key?.witnessAddressTestnet
    }
    return address?.string
    
    /*@XM@20181114 DON'T DELETE: below are xiangyun's implementation
     //ripemd-160
     guard let hash160 = BTCRIPEMD160(pubKey.sha256()).hex() else {
     return nil
     }
     
     let addPrefix = "0x0014" + hash160
     let redeemScript = BTCDataFromHex(addPrefix)!
     guard let redeemScriptHash = BTCRIPEMD160(redeemScript.sha256()).hex() else {
     return nil
     }
     
     let hexVersion = String(format: "%02x", version)
     let addVersion = hexVersion + redeemScriptHash
     let data = BTCDataFromHex(addVersion)
     return BTCBase58CheckStringWithData(data)
     */
  }
  
  public class func getETHAddress(handle:UInt,path:String)throws -> String{
    let xPub = try getETHXPub(handle: handle, path: path)
    let stringToEncrypt = xPub.key_substring(from: 2).key_substring(to: 128)
    let sha3Keccak = Encryptor.Keccak256_ik().encrypt(hex: stringToEncrypt)
    let address = sha3Keccak.key_substring(from: sha3Keccak.count-40)
    return address
  }
  
  //@XM@20181113 TODO: please remove this function
  public class func btcSignMessage(handle:UInt,path:String,data:String)throws -> String{
    try selectApplet(handle: handle, aid: Applet.btcAID)
    guard let prepareApdus  = APDU.btcMessagePrepare(data: data) else{
      //throw ex
      return ""
    }
    
    try BLE.shared().sendPrepareApdus(handle:handle,apdus:prepareApdus)
    let apdu = APDU.btcMessageSign(path: path)
    let res = try BLE.shared().sendApdu(handle: handle, apdu: apdu)
    return res
  }
  
  public class func ethSignPersonalMessage(handle:UInt,path:String,data:String,sender:String)throws -> ECSignature{
    //add prefix and hash it for later recId caculation
    let dataBytes = data.key_isHex() ? data.key_dataFromHexString()! : data.data(using: .utf8)!
    let msgLen = dataBytes.count
    let headerMsg = "\u{0019}Ethereum Signed Message:\n\(msgLen)"
    var data2sign = headerMsg.data(using: .utf8)!
    data2sign.append(dataBytes)
    let msgHash = Encryptor.Keccak256_ik().encrypt(data: data2sign)

    var lenBytes = [UInt8](repeating: 0, count: 2)
    lenBytes[0] = UInt8((data2sign.count & 0xFF00) >> 8)
    lenBytes[1] = UInt8(data2sign.count & 0x00FF)
    data2sign.insert(contentsOf: lenBytes, at: 0)
    
    data2sign.insert(0x01, at: 0)
    
    let hash = data2sign.sha256().sha256()
    var signResult = try SigUtil.ecSignHash(hash: hash.bytes)
    signResult.insert(UInt8(signResult.count), at: 0)
    signResult.insert(0x00, at: 0)
    signResult.append(contentsOf: data2sign)
    let predata = ByteUtil.uint8Array2HexString(data: signResult)
    
    try selectApplet(handle: handle, aid: Applet.ethAID)
    
    // check address
    let apduPub = APDU.ethXpub(path: path)
    let pubApdu = try BLE.shared().sendApdu(handle: handle, apdu: apduPub)
    try APDU.checkResponse(res: pubApdu)
    let pubkey = pubApdu.key_substring(to: 130).key_substring(from:2)

    guard let pubkeyData = BTCDataFromHex(pubkey) else{
      throw SDKError.unwrapError
    }

    //pubkey to address
//    let pkHash = pubkey.ik_keccak256()
    let pkHash = pubkeyData.key_keccak256()
    let mainAddr = pkHash.key_substring(from: pkHash.count - 40)
    let checkMainAddr = try toETHChecksumAddress(address: mainAddr)
    guard checkMainAddr == sender else{
      throw SDKError.addressVerifyFailed
    }

    guard let prepareApdus  = APDU.ethMessagePrepare(data: predata) else {
      throw GenericError.paramError
    }
    try BLE.shared().sendPrepareApdus(handle: handle, apdus: prepareApdus)
    let apdu = APDU.ethMessageSign(path: path)
    let res = try BLE.shared().sendApdu(handle: handle, apdu: apdu)
    try APDU.checkResponse(res: res)
    let resApdu = res
    
    let r = resApdu.key_substring(from: 2).key_substring(to: 64)
    var s = resApdu.key_substring(from: 66).key_substring(to: 64)
    
    Log.d("\n*****************")
    Log.d("r: \(r)");
    Log.d("s: \(s)");
    
    let rBig = BTCBigNumber.init(string: r, base: 16)
    let sBig = BTCBigNumber.init(string: s, base: 16)
    let rPoint = UnsafeMutablePointer<BIGNUM>.allocate(capacity: 1)
    let sPoint = UnsafeMutablePointer<BIGNUM>.allocate(capacity: 1)
    rPoint.initialize(from: rBig!.bignum, count: 1)
    let s_low = SigUtil.getLowS(s:sBig!)
    sPoint.initialize(from: s_low.bignum, count: 1)
    var ecSig: ECDSA_SIG = ECDSA_SIG(r:rPoint,s:sPoint)
    
    
    let pkCompress = SigUtil.getPubKeyComp(xPub: pubApdu)
    let pubkey2 = pubApdu.key_substring(from: 0).key_substring(to: 130)
    let pkData = BTCDataFromHex(pubkey2)
    let btcKey = BTCKey.init(publicKey: pkData)
    
    let vInt = btcKey!.imKey_ECDSA_SIG_recover(&ecSig, forHash: msgHash.key_dataFromHexString()!, pubKey: pkCompress)
    let v = vInt + 27
    s = s_low.hexString
    
    return ["v": v, "r": r, "s": s]
  }
  
  public static func eosEcSign(handle:UInt, path:String, data: String, isHex: Bool, publicKey: String?) throws -> String {
    guard let hashedData = (isHex ? data.key_dataFromHexString(): data.data(using: .utf8)?.sha256()) else {
      throw SDKError.notHexOrString
    }
 
    let key = EOSKey(pubKey: publicKey!)
    let txPack = NSMutableData()
    // data
    txPack.append(("0120" + hashedData.toHexString()).key_dataFromHexString()!)    //TL
    // path
    let pathString = path.key_toHexString()
    txPack.append(("0211" + pathString).key_dataFromHexString()!)    //TL
    let txPackString = (txPack as Data).key_toHexString()
    guard let bytes = ByteUtil.hexString2Uint8Array(data: txPackString) else{
      throw SDKError.unwrapError
    }
    let hash = bytes.sha256().sha256()
    var sig = try SigUtil.ecSignHash(hash: hash)
    sig.insert(UInt8(sig.count), at: 0)
    sig.insert(0x00, at: 0)
    sig.append(contentsOf: bytes)
    
    return EOSTransaction.signatureBase58(data: try key.eosPersonalSign(handle: handle, forhash: Data(sig), hash: hashedData))
  }
  
  public class func btcSignTransaction(utxos: [UTXO], amount: Int64, fee: Int64, toAddress: BTCAddress, changeAddress: BTCAddress, extra: [AnyHashable: Any]? = nil, handle: UInt,network:Network,pathPrefix:String, payment: String, receiver: String, sender: String, feeDis: String) throws -> TransactionSignedResult{
    guard let hexToAddress = BTCDataFromBase58(toAddress.string).hex() else{
      throw SDKError.illegalArgument
    }
    try ValidatorAddress.checkAddress(network:network,to: hexToAddress)
    
    let signer = try BTCTransactionSigner(utxos: utxos, amount: amount, fee: fee, toAddress: toAddress, changeAddress: changeAddress, extra: extra, handle: handle,
                                          payment: payment, to: receiver, from: sender, feeDis: feeDis)
    return try signer.sign(network: network, pathPrefix: pathPrefix)
  }
  
  public class func ethSignTransaction(handle:UInt,raw: [String: Any], chainID: Int, path:String)throws -> TransactionSignedResult {
    let trans:ETHTransaction = try ETHTransaction(raw:raw,chainID:chainID,handle:handle)
    let signRes = try trans.sign(path: path)
    let signedResult = trans.signedResult
    return signedResult
  }
  
  public class func btcSignSegwitTransaction(utxos: [UTXO], amount: Int64, fee: Int64, toAddress: BTCAddress, changeAddress: BTCAddress, extra: [AnyHashable: Any]? = nil, handle: UInt,network:Network,pathPrefix:String, payment: String, receiver: String, sender: String, feeDis: String)throws -> TransactionSignedResult{
    guard let hexToAddress = BTCDataFromBase58(toAddress.string).hex() else{
      throw SDKError.illegalArgument
    }
    try ValidatorAddress.checkAddress(network:network,to: hexToAddress)
    
    let signer = try BTCTransactionSigner(utxos: utxos, amount: amount, fee: fee, toAddress: toAddress, changeAddress: changeAddress, extra:extra, handle: handle,
                                          payment: payment, to: receiver, from: sender, feeDis: feeDis)
    return try signer.signSegWit(network:network,pathPrefix: pathPrefix)
  }
  
  public class func eosSignTransaction(txs: [EOSTransaction], handle: UInt, path: String)throws -> [EOSSignResult]{
    let trans:EOSTransactionSigner = EOSTransactionSigner(txs: txs, handle: handle, path: path)
    return try trans.sign()
  }
  public class func displayBTCddress(handle: UInt,version:Int32,path:String)throws ->String{
    let mainAddress = try getBTCAddress(handle: handle, version: version, path: path)
    let mAddr = mainAddress!
    let apdu = APDU.setBTCAddress(address: mAddr)
    let res = try BLE.shared().sendApdu(handle: handle, apdu: apdu)
    try APDU.checkResponse(res: res)
    return mAddr
  }
  
  public class func displayBTCSegwitAddress(handle: UInt,version:Int32,path:String)throws ->String{
    let mainAddress = try getBTCSegwitAddress(handle: handle, version: version, path: path)
    let mAddr = mainAddress!
    let apdu = APDU.setBTCAddress(address: mAddr)
    let res = try BLE.shared().sendApdu(handle: handle, apdu: apdu)
    try APDU.checkResponse(res: res)
    return mAddr
  }
  
  public class func displayETHAddress(handle: UInt,path:String)throws ->String{
    let mainAddress = try getETHAddress(handle: handle, path: path)
    let checksumAddress = try toETHChecksumAddress(address: mainAddress)
    let apdu = APDU.setETHAddress(address: checksumAddress)
    let res = try BLE.shared().sendApdu(handle: handle, apdu: apdu)
    try APDU.checkResponse(res: res)
    return mainAddress
  }
  
  public class func toETHChecksumAddress(address:String)throws ->String{
    let addr = address.lowercased().replacingOccurrences(of: "0x", with: "")
    let hash = addr.ik_keccak256()
    var checksumAddress = "0x"
    for (index,c) in addr.enumerated() {
      let i = addr.index(addr.startIndex, offsetBy: index)
      guard let intC = Int(String(hash[i]),radix:16) else{
        throw SDKError.unwrapError
      }
      if intC >= 8{
        checksumAddress += String(c).uppercased()
      }else{
        checksumAddress += String(c)
      }
    }
    return checksumAddress
  }
  
  public class func displayEOSPubKey(handle: UInt,path:String)throws ->String{
    let mainPub = try getEOSPubkey(handle: handle, path: path)
    let apdu = APDU.setEOSPubkey(pubkey: mainPub)
    let res = try BLE.shared().sendApdu(handle: handle, apdu: apdu)
    try APDU.checkResponse(res: res)
    return mainPub
  }
}
