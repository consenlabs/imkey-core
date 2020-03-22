//
//  Transaction.swift
//  token
//
//  Created by James Chen on 2016/11/03.
//  Copyright © 2016 imToken PTE. LTD. All rights reserved.
//

import Foundation
import CoreBitcoin

public final class ETHTransaction: NSObject {
  private var raw: [String: Any]
  private var chainID: Int
  let nonce, gasPrice, gasLimit, to, value, data, payment, receiver, sender, fee: String
  var v, r, s: String
  let handle: UInt
  
  /**
   Construct a transaction with raw data
   - Parameters:
   - raw: Raw data
   - chainID: Chain ID, 1 by default after [EIP 155](https://github.com/ethereum/EIPs/issues/155) fork.
   */
  public init(raw: [String: Any], chainID: Int, handle: UInt) throws {
    self.raw = raw
    self.chainID = chainID
    self.handle = handle
    
    // Make sure every property at least has empty string value
    nonce       = ETHTransaction.parse(raw, "nonce") as! String
    gasPrice    = ETHTransaction.parse(raw, "gasPrice") as! String
    gasLimit    = ETHTransaction.parse(raw, "gasLimit") as! String
    to          = ETHTransaction.parse(raw, "to") as! String
    value       = ETHTransaction.parse(raw, "value") as! String
    data        = ETHTransaction.parse(raw, "data") as! String
    v           = ETHTransaction.parse(raw, "v") as! String
    r           = ETHTransaction.parse(raw, "r") as! String
    s           = ETHTransaction.parse(raw, "s") as! String
    guard let preview = ETHTransaction.parse(raw, "preview") as? [String: Any] else {
      throw GenericError.paramError
    }
    payment     = ETHTransaction.parse(preview, "payment") as! String
    receiver    = ETHTransaction.parse(preview, "receiver") as! String
    sender      = ETHTransaction.parse(preview, "sender") as! String
    fee         = ETHTransaction.parse(preview, "fee") as! String
    
    if v.isEmpty && chainID > 0 {
      v = String(chainID)
      r = "0"
      s = "0"
    }
    
    super.init()
  }
  
  convenience init(raw: [String: String], handle: UInt) throws {
    try self.init(raw: raw, chainID: -4, handle: handle) // -4 is to support old ecoding without chain id.
  }
  
  /// - Returns: Signed TX, always prefixed with 0x
  public var signedTx: String {
    return RLP.encode(serialize())
  }
  
  /// Should only called after signing
  public var signedResult: TransactionSignedResult {
    return TransactionSignedResult(signedTx: signedTx, txHash: signingHash)
  }
  
  private var signingData: String {
    return RLP.encode(serialize())
  }
  
  var signingHash: String {
    return Encryptor.Keccak256_ik().encrypt(hex: signingData).ik_add0xIfNeeded()
  }
  
  private func encodeV(_ v: Int32) -> String {
    let intValue: Int32 = v + Int32(chainID) * 2 + 35
    return String(intValue)
  }
  
  private var isSigned: Bool {
    return !(v.isEmpty || r.isEmpty || s.isEmpty)
  }
  
  public func sign(path:String)throws -> [String: String] {
    try Wallet.selectApplet(handle: handle, aid: Applet.ethAID)
    //apdu-prepare data
    let msgHash = Encryptor.Keccak256_ik().encrypt(hex: signingData).ik_add0xIfNeeded()
    //payment
    let payArray: [UInt8] = Array(payment.utf8)
    //receiver
    let rcvArray: [UInt8] = Array(receiver.utf8)
    //fee
    let feeArray: [UInt8] = Array(fee.utf8)
    
    let sigDatBytes = ByteUtil.hexString2Uint8Array(data: signingData)!
    
    var lenBytes = [UInt8](repeating: 0, count: 2)
    lenBytes[0] = UInt8((sigDatBytes.count & 0xFF00) >> 8)
    lenBytes[1] = UInt8(sigDatBytes.count & 0x00FF)
    let lenHex = ByteUtil.uint8Array2HexString(data: lenBytes)
    
    //package
    let signPack = "01" + lenHex + signingData
                + "07" + String(format:"%02x", payArray.count) + payArray.toHexString()
                + "08" + String(format:"%02x", rcvArray.count) + rcvArray.toHexString()
                + "09" + String(format:"%02x", feeArray.count) + feeArray.toHexString()
    
//    //let chkSum = (BTCSHA256(signPack.key_dataFromHexString()) as Data).bytes[0..<4]
//    let metaLen = 2 + payArray.count + 2 + rcvArray.count + 2 + feeArray.count
//    //signPack += Data(chkSum).toHexString() + String(format:"%02x", metaLen)
//    signPack += String(format:"%02x", metaLen)
    
    guard let bytes = ByteUtil.hexString2Uint8Array(data: signPack)else{
      throw SDKError.unwrapError
    }
    let hash = bytes.sha256().sha256()
    var sig = try SigUtil.ecSignHash(hash: hash)
    sig.insert(UInt8(sig.count), at: 0)
    sig.insert(0x00, at: 0)
    sig.append(contentsOf: bytes)
    let predata = ByteUtil.uint8Array2HexString(data: sig)
    
    //check address
    let apduPub = APDU.ethXpub(path: path)
    let pubApdu = try BLE.shared().sendApdu(handle: handle, apdu: apduPub)
    try APDU.checkResponse(res: pubApdu)
    let pubkey = pubApdu.key_substring(to: 130).key_substring(from:2)
    guard let pubkeyData = BTCDataFromHex(pubkey) else{
      throw SDKError.unwrapError
    }
    let pkHash = pubkeyData.key_keccak256()
    let mainAddr = pkHash.key_substring(from: pkHash.count - 40)
    let checkMainAddr = try Wallet.toETHChecksumAddress(address: mainAddr)
    guard checkMainAddr == sender else{
      throw SDKError.addressVerifyFailed
    }
    
    let commands = APDU.ethPre(data: predata)
    if let apdus = commands {
      try BLE.shared().sendPrepareApdus(handle: handle, apdus: apdus)
    }
    //apdu-sign
    let apdu = APDU.ethSign(index: 0, hashType: 0, path: path)
    let resApdu = try BLE.shared().sendApdu(handle: handle, apdu: apdu!) /*!<@XM@20180911 TODO: force unwrap,,need to change */
    try APDU.checkResponse(res: resApdu)
    
    r = resApdu.key_substring(from: 2).key_substring(to: 64)
    s = resApdu.key_substring(from: 66).key_substring(to: 64)
    
    Log.d("\n*****************")
    Log.d("r:\(r)")
    Log.d("s:\(s)")
    
    let rBiBTCBigNumberg = .init(string: r, base: 16)
    let sBig = BTCBigNumber.init(string: s, base: 16)
    let rPoint = UnsafeMutablePointer<BIGNUM>.allocate(capacity: 1)
    let sPoint = UnsafeMutablePointer<BIGNUM>.allocate(capacity: 1)
    rPoint.initialize(from: rBig!.bignum, count: 1)
    let sLow = SigUtil.getLowS(s:sBig!)
    sPoint.initialize(from: sLow.bignum, count: 1)
    var ecSig: ECDSA_SIG = ECDSA_SIG(r:rPoint,s:sPoint)
    
    //apdu-getpub
    let pkCompress = SigUtil.getPubKeyComp(xPub: pubApdu)
    let pubkey2 = pubApdu.key_substring(from: 0).key_substring(to: 130)
    let pkData = BTCDataFromHex(pubkey2)
    let btcKey = BTCKey.init(publicKey: pkData)
    
    let vInt = btcKey!.imKey_ECDSA_SIG_recover(&ecSig, forHash: msgHash.key_dataFromHexString()!, pubKey: pkCompress)
    v = encodeV(vInt)
    s = sLow.hexString
    
    return ["v": v, "r": r, "s": s]
  }
}

// Parse and construct values
private extension ETHTransaction {
  func serialize() -> [BigNumber] {
    let base: [BigNumber] = [
      BigNumber.parse(nonce),
      BigNumber.parse(gasPrice),
      BigNumber.parse(gasLimit),
      BigNumber.parse(to, padding: true),  // Address
      BigNumber.parse(value),
      BigNumber.parse(data, padding: true) // Binary
    ]
    
    if isSigned {
      return base + [BigNumber.parse(v), BigNumber.parse(r), BigNumber.parse(s)]
    } else {
      return base
    }
  }
  
  static func parse(_ data: [String: Any], _ key: String) -> Any {
    return data[key] ?? ""
  }
}
