//
//  KeyManager.swift
//  imKeyConnector
//
//  Created by joe on 12/27/18.
//

import CryptoSwift
import secp256k1
import CoreBitcoin



public class KeyManager{
  //bind status
  public static let status_unbound = "unbound"
  public static let status_bound_this = "bound_this"
  public static let status_bound_other = "bound_other"
  
  //bind result
  public static let result_success = "success"
  public static let result_authcode_error = "authcode_error"
  
  var fileKey:[UInt8] = [UInt8](repeating: 0, count: 16)
  var fileIV:[UInt8] = [UInt8](repeating: 0, count: 16)
  
  //keys
  var prvKey:[UInt8] = [UInt8](repeating: 0, count: 32)
  var pubKey:[UInt8] = [UInt8](repeating: 0, count: 65)
  var sePubKey:[UInt8] = [UInt8](repeating: 0, count: 65)
  var sessionKey:[UInt8] = [UInt8](repeating: 0, count: 16)
  var checksum:[UInt8] = [UInt8](repeating: 0, count: 4)
  
  
  static let _shared = KeyManager()
  
  public class func shared() -> KeyManager {
    return _shared
  }
  
  private init() {
    
  }
  
  private func computeFileKey(seid:String,sn:String)throws {
    guard let seidHashBytes = ByteUtil.hexString2Uint8Array(data: seid.sha256()),
      let snHashBytes = ByteUtil.hexString2Uint8Array(data: sn.sha256())else{
        throw SDKError.unwrapError
    }
    var bytes = seidHashBytes
    for (index,byte) in seidHashBytes.enumerated(){
      let newByte = byte ^ snHashBytes[index]
      bytes[index] = newByte
    }
    fileKey[0..<16] = bytes[0..<16]
    fileIV[0...] = bytes[16...]
  }
  
  private func genECkey(){
    let btcKey = BTCKey()!
    prvKey = [UInt8](btcKey.privateKey as Data)
    pubKey =  [UInt8](btcKey.publicKey as Data)
  }
  
  public func bindCheck(handle:UInt)throws ->String{
    let seid = try Manager.getSeid(handle: handle)
    let sn = try Manager.getSn(handle: handle)
    try computeFileKey(seid: seid, sn: sn)
    //    try computeFileKey(seid: "18090000000000860001010000000113", sn: "696D4B65793031313930313030303131")//for test
    
    let storage = KeyFileStorage()
    let content = storage.readFrom(KeyFileStorage.defaultFileName + seid)
    
    if content == nil || content?.count == 0 {
      //gen keys
      genECkey()
    }else{
      let decryptContent = decrypt(data: content!)
      let valide = decodeKeys(from: decryptContent)
      if !valide {
        //gen keys
        genECkey()
      }
    }
    
    try selectApplet(handle: handle)
    
    let checkApdu = APDU.bindCheck(appPubkey: pubKey)
    let res = try BLE.shared().sendApdu(handle: handle, apdu: checkApdu)
    try APDU.checkResponse(res: res)
    
    let status = res.key_substring(to: 2)
    let seCert = res.key_substring(from: 2).key_substring(to: res.count - 6)
    
    //check device cert
    let checkResult = try Manager.checkDeviceCert(handle: handle,seCert: seCert)
    if !checkResult{
      throw SDKError.secertInvalid
    }
    
    if status == "00" || status == "AA"{
      guard let pk = try TLVUtil.find7F49(cert: seCert) else {
        throw SDKError.secertInvalid // can't happen
      }
      
      guard let pkBytes = ByteUtil.hexString2Uint8Array(data: pk) else {
        throw SDKError.unwrapError
      }
      sePubKey[0...] = pkBytes[2..<67]
      
      let context = secp256k1_context_create(UInt32(SECP256K1_CONTEXT_SIGN | SECP256K1_CONTEXT_VERIFY))!//witch part
      defer {
        secp256k1_context_destroy(context)
      }
      
      var secp256k1Pubkey = secp256k1_pubkey()
      let paserResult = secp256k1_ec_pubkey_parse(context, &secp256k1Pubkey, &sePubKey, sePubKey.count)
      Log.d("paserResult:\(paserResult)")
      
      var ecdhKey = [UInt8](repeating: 0, count: 32)
      let ecdhResult = secp256k1_ecdh(context, &ecdhKey, &secp256k1Pubkey, &prvKey)
      Log.d("ecdhResult:\(ecdhResult)")
      
      let sha1 = ecdhKey.sha1()
      sessionKey[0...] = sha1[0..<16]
      
      let encodeStr = encodeKeys()
      guard let encryptStr = encrypt(data: encodeStr) else{
        throw SDKError.unwrapError
      }
      _ = storage.writeContent(encryptStr, to: KeyFileStorage.defaultFileName + seid)
    }
    
    if status == "55" {
      return KeyManager.status_bound_this
    }else if status == "AA"{
      return KeyManager.status_bound_other
    }else {
      return KeyManager.status_unbound
    }
  }
  
  //6rl35cmk
  public func bindAcquire(handle:UInt,authCode:String)throws -> String{
    let authCodeUpper = authCode.uppercased()
    // authCode 校验  0,1,I,O,排除
    let pattern = "^[A-HJ-NP-Z2-9]{8}$";
    let regex = try NSRegularExpression(pattern: pattern)
    let matchNumber = regex.numberOfMatches(in: authCodeUpper, range: NSMakeRange(0, authCodeUpper.utf8.count))
    if matchNumber != 1 {
      throw SDKError.illegalArgument
    }
    
    guard let result = try RSAUtils.encryptWithRSAPublicKey(str: authCodeUpper, pubkeyBase64: Constants.rsaPublicKey)else{
      throw SDKError.unwrapError
    }
    let bytes = [UInt8](result)
    let encryptStr = ByteUtil.uint8Array2HexString(data: bytes)
    try Manager.saveAuthCode(handle: handle, authCode: encryptStr)
    
    try selectApplet(handle: handle)
    var data = [UInt8](repeating: 0, count: 8 + 65 + 65)
    let authCodeBytes = Array(authCodeUpper.utf8)
    
    data[0..<8] = authCodeBytes[0...]
    data[8..<73] = pubKey[0...]
    data[73..<138] = sePubKey[0...]
    
    let hash = data.sha256()
    let hashStr = ByteUtil.uint8Array2HexString(data: hash)
    
    let aesKey = ByteUtil.uint8Array2HexString(data: sessionKey)
    let ivBytes = try computeIV(authcode: authCode.uppercased())
    let sessionIV = ByteUtil.uint8Array2HexString(data: ivBytes)
    let cipher = sessionEncrypt(key: aesKey, iv: sessionIV, data: hashStr)
    
    
    guard let cipherBytes = ByteUtil.hexString2Uint8Array(data: cipher) else{
      throw SDKError.unwrapError
    }
    var verifyData = [UInt8](repeating: 0, count: pubKey.count + cipherBytes.count)
    verifyData[0..<pubKey.count] = pubKey[0...]
    verifyData[pubKey.count...] = cipherBytes[0...]
    
    let apdu = APDU.identyVerify(data: verifyData, bind: true)
    let res = try BLE.shared().sendApdu(handle: handle, apdu: apdu)
    try APDU.checkResponse(res: res)
    let status = res.key_substring(to: 2)
    
    if status == "5A" {
      return KeyManager.result_success
    }else {
      return KeyManager.result_authcode_error
    }
  }
  
  
  private func selectApplet(handle:UInt)throws{
    guard let selectApdu = APDU.select(aid: Applet.sioAID) else {
      throw SDKError.unwrapError
    }
    let selectRes = try BLE.shared().sendApdu(handle: handle, apdu: selectApdu)
    try APDU.checkResponse(res: selectRes)
  }
  
  private func encodeKeys() -> String{
    var data = [UInt8](repeating: 0, count: 178)
    data[0..<32] = prvKey[0...]
    data[32..<97] = pubKey[0...]
    data[97..<162] = sePubKey[0...]
    data[162..<178] = sessionKey[0...]
    
    let hash = data.sha256()
    checksum[0...] = hash[0..<4]
    
    var bytes = [UInt8](repeating: 0, count: 182)
    bytes[0..<178] = data[0...]
    bytes[178..<182] = checksum[0...]
    
    let encodeStr = ByteUtil.uint8Array2HexString(data: bytes)
    return encodeStr
  }
  
  private func decodeKeys(from:String?) -> Bool{
    guard let data = from else{
      return false
    }
    guard let bytes = ByteUtil.hexString2Uint8Array(data: data) else{
      return false
    }
    
    prvKey[0...] = bytes[0..<32]
    pubKey[0...] = bytes[32..<97]
    sePubKey[0...] = bytes[97..<162]
    sessionKey[0...] = bytes[162..<178]
    checksum[0...] = bytes[178..<182]
    
    var keys = [UInt8](repeating: 0, count: 178)
    keys[0...]  = bytes[0..<178]
    let hash = keys.sha256()
    
    for (index,byte) in checksum.enumerated() {
      if hash[index] != byte{
        return false
      }
    }
    return true
  }
  
  public func displayBindingCode(handle:UInt) throws {
    try selectApplet(handle: handle)
    let apdu = APDU.genAuthCode()
    let res = try BLE.shared().sendApdu(handle: handle, apdu: apdu)
    try APDU.checkResponse(res: res)
  }
  
  public func decrypt(data:String, base64:Bool = false) -> String?{
    let key = ByteUtil.uint8Array2HexString(data: fileKey)
    let iv = ByteUtil.uint8Array2HexString(data: fileIV)
    let aes = Encryptor.AES128_ik(key: key, iv: iv, mode: .cbc, padding: .pkcs5)
    if base64{
      guard let decodeBase64 = data.ik_fromBase64() else{
        return nil
      }
      return aes.decrypt(hex: decodeBase64)
    }else{
      return aes.decrypt(hex: data)
    }
  }
  
  public func encrypt(data:String,base64:Bool = false) -> String? {
    let key = ByteUtil.uint8Array2HexString(data: fileKey)
    let iv = ByteUtil.uint8Array2HexString(data: fileIV)
    let aes = Encryptor.AES128_ik(key: key, iv: iv, mode: .cbc, padding: .pkcs5)
    if base64{
      return aes.encrypt(hex:data).ik_toBase64()
    }else{
      return aes.encrypt(hex:data)
    }
  }
  
  public func sessionEncrypt(key:String,iv:String,data:String) ->String{
    let aes = Encryptor.AES128_ik(key: key, iv: iv, mode: .cbc, padding: .pkcs5)
    return aes.encrypt(hex:data)
  }
  
  public func computeIV(authcode:String)throws -> [UInt8]{
    let salt = "bindingCode"
    guard let bindingCodeHash = ByteUtil.hexString2Uint8Array(data: authcode.sha256()),
      let saltHash = ByteUtil.hexString2Uint8Array(data: salt.sha256())else{
        throw SDKError.unwrapError
    }
    var bytes = bindingCodeHash
    for (index,byte) in bindingCodeHash.enumerated(){
      let newByte = byte ^ saltHash[index]
      bytes[index] = newByte
    }
    var iv = [UInt8](repeating: 0, count: 16)
    iv[0...] = bytes[0..<16]
    return iv
  }
}
