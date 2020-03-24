//
//  COSMOSTransaction.swift
//  TokenCore
//
//  Created by XM on 5/4/19.
//  Copyright © 2019 imToken PTE. LTD. All rights reserved.
//

import Foundation
import CoreBitcoin
import OrderedDictionary

public final class CosmosTransaction {
  private var accountNumber, sequence, chainId, memo: String
  private var fee: JSONObject
  private var msgs: [JSONObject]

  public init(raw: [String: Any]) throws {

    guard let chainId = raw["chainId"] as? String,
          let accountNumber = raw["accountNumber"] as? String,
          let sequence = raw["sequence"] as? String,
          let fee = raw["fee"] as? JSONObject,
          let msgs = raw["msgs"] as? [JSONObject] else {
            throw GenericError.paramError
    }

    self.chainId = chainId
    
    if raw["memo"] as? String == nil{
      self.memo = ""
    }else{
      self.memo = raw["memo"] as! String
    }
    
    self.accountNumber = accountNumber
    self.sequence = sequence
    self.fee = fee
    self.msgs = msgs
  }

  private func prepareSignBytes(Tx: Any) -> Any{
    if let arrayTx = Tx as? [JSONObject] {
      let sortedArrayTx = arrayTx.map{prepareSignBytes(Tx: $0)}
      return sortedArrayTx
    } else if let jsonTx = Tx as? JSONObject {
      let orderedJson = MutableOrderedDictionary<AnyObject, AnyObject>(dictionary: jsonTx)
      let orderedDictionary = jsonTx.sorted{$0.0.compare($1.0, options: .caseInsensitive) == .orderedAscending }
      orderedJson.removeAllObjects()
      for item in orderedDictionary {
        let sortedItem = prepareSignBytes(Tx: item.value)
        orderedJson.setObject(sortedItem as AnyObject, forKey: item.key as AnyObject)
      }
      return orderedJson
    } else {
      return Tx
    }
  }

  private func createSignMessage()throws -> String {
    let dictionary: JSONObject = [
      "sequence": sequence,
      "account_number": accountNumber,
      "chain_id": chainId,
      "msgs": msgs,
      "fee": fee,
      "memo": memo
    ]

    let orderedJson = prepareSignBytes(Tx: dictionary as Any)
    let data = try! JSONSerialization.data(withJSONObject: orderedJson)
    return String(data: data, encoding: .utf8)!
  }

  public func sign(handle:UInt,path:String,paymentDis:String?,toDis:String,feeDis:String)throws -> TransactionSignedResult {
    try Wallet.selectApplet(handle: handle, aid: Applet.cosmosAID)
    try sanityCheckMsg()

    let signMessage = try createSignMessage()
    let signMessageWithSlashRemoved = signMessage.replacingOccurrences(of: "\\/", with: "/")
    var payment:String
    if paymentDis == nil{
      payment = ""
    }else{
      payment = paymentDis!
    }
    
    let paymentDisBytes: [UInt8] = Array(payment.utf8)
    let toDisBytes: [UInt8] = Array(toDis.utf8)
    let feeDisBytes: [UInt8] = Array(feeDis.utf8)
    
    let json_hash = signMessageWithSlashRemoved.sha256()
    Log.d("json hash:\(json_hash)")
    let signPack = "0120" + signMessageWithSlashRemoved.sha256()
      + "07" + String(format:"%02x", paymentDisBytes.count) + paymentDisBytes.toHexString()
      + "08" + String(format:"%02x", toDisBytes.count) + toDisBytes.toHexString()
      + "09" + String(format:"%02x", feeDisBytes.count) + feeDisBytes.toHexString()
    Log.d("signPack:\(signPack)")
    
    guard let bytes = ByteUtil.hexString2Uint8Array(data: signPack)else{
      throw SDKError.unwrapError
    }
    let hash = bytes.sha256().sha256()
    Log.d("hash:\(ByteUtil.uint8Array2HexString(data: hash))")
    var sig = try SigUtil.ecSignHash(hash: hash)
    sig.insert(UInt8(sig.count), at: 0)
    sig.insert(0x00, at: 0)
    sig.append(contentsOf: bytes)
    let predata = ByteUtil.uint8Array2HexString(data: sig)
    
    let commands = APDU.cosmosPre(data: predata)
    if let apdus = commands {
      try BLE.shared().sendPrepareApdus(handle: handle, apdus: apdus)
    }
    
    let signApdu = APDU.cosmosSign(path: path)
    let res = try BLE.shared().sendApdu(handle: handle, apdu: signApdu!)
    try APDU.checkResponse(res: res)
    
    let r = res.key_substring(from: 2).key_substring(to: 64)
    let s = res.key_substring(from: 66).key_substring(to: 64)
    
    let sBig = BTCBigNumber.init(string: s, base: 16)
    guard var s_low = SigUtil.getLowS(s:sBig!).hexString else{
      throw SDKError.unwrapError
    }
    
    while s_low.count<64 {
      s_low = "0" + s_low
    }
    
    let signature = r + s_low.uppercased()
    
    let sigData = Data(ByteUtil.hexString2Uint8Array(data: signature)!)
    let base64Sig = sigData.base64EncodedString()
    
    let xPub = try CosmosKey.getCosmosXPub(handle: handle, path: path)
    let compresPK = SigUtil.getPubKeyComp(xPub: xPub)
    let base64PubKey = compresPK.base64EncodedString()

    let stdSignature = createSignature(signature: base64Sig, sequence: sequence, accountNumber: accountNumber, pubKey: base64PubKey)

    let signRes = createSignedTx(signature: stdSignature)
    let txHash = CreateHash(stdTx: signRes)
    return TransactionSignedResult(cosmosSignedTx: signRes, txHash: txHash)
  }

  private func createSignedTx(signature: JSONObject) -> JSONObject {
    let json: JSONObject = [
      "msg": msgs,
      "fee": fee,
      "signatures": [signature],
      "memo": memo
      ]

    return json
  }

  private func createSignature(signature: String, sequence: String, accountNumber: String, pubKey: String) -> JSONObject {
    let json: JSONObject = [
      "pub_key": ["type": "tendermint/PubKeySecp256k1", "value": pubKey],
      "signature": signature,
      "account_number": accountNumber,
      "sequence": sequence
    ]

    return json
  }

  private func CreateHash(stdTx: JSONObject) -> String {
    return "" 
  }

  private func sanityCheckMsg()throws {
    if self.accountNumber.isEmpty || self.chainId.isEmpty || self.sequence.isEmpty{
      throw GenericError.paramError
    }
  }

}
