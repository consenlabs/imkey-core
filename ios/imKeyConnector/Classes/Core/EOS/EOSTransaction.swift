//
//  EOSTransaction.swift
//  TokenCore
//
//  Created by James Chen on 2018/06/25.
//  Copyright © 2018 ConsenLabs. All rights reserved.
//

import Foundation
import CoreBitcoin

public final class EOSTransaction {
  private let data: String // Hex tx data
  private let publicKeys: [String]
  private let chainID: String
  private let to: String
  private let from: String
  private let payment: String?
  
  public init(data: String, publicKeys: [String], chainID: String, to: String, from: String, payment: String?) {
    self.data = data
    self.publicKeys = publicKeys
    self.chainID = chainID
    self.to = to
    self.from = from
    self.payment = payment
  }
  
  public func sign(handle: UInt, path: String) throws -> EOSSignResult {
    let hash = Hex.toBytes(data).sha256().toHexString()
    
    let toSign = NSMutableData()
    toSign.append(chainID.key_dataFromHexString()!)
    toSign.append(data.key_dataFromHexString()!)
    toSign.append(Data(bytes: [UInt8](repeating: 0, count: 32)))
    let hashedTx = BTCSHA256(toSign as Data) as Data
    
    let signs = try publicKeys.map { publicKey -> String in
      let key = EOSKey(pubKey: publicKey)
      //construct package: preview + hashedTx + path (all in TLV format) + checksum
      //preview: payment + to (all in TLV format)
      let txPack = NSMutableData()
      // hashedtx
      txPack.append(("0120" + hashedTx.toHexString()).key_dataFromHexString()!)    //TL
      // path
      let pathString = path.key_toHexString()
      txPack.append(("0211" + pathString).key_dataFromHexString()!)    //TL
      if payment == nil {
        txPack.append(("07" + String(format:"%02x", 0)).key_dataFromHexString()!)
        txPack.append(("08" + String(format:"%02x", 0)).key_dataFromHexString()!)
      } else {
        //payment
        let payArray: [UInt8] = Array(payment!.utf8)
        txPack.append(("07" + String(format:"%02x", payArray.count)).key_dataFromHexString()!)
        txPack.append(Data(bytes: payArray))
        //to (receiver)
        let recArray: [UInt8] = Array(to.utf8)
        txPack.append(("08" + String(format:"%02x", recArray.count)).key_dataFromHexString()!)
        txPack.append(Data(bytes: recArray))
      }
      //let chkSum = (BTCSHA256(txPack as Data) as Data).bytes[0..<4]
      
      //let txPackChk = (txPack as Data) + Data(chkSum)
      let txPackString = (txPack as Data).key_toHexString()
      
      guard let bytes = ByteUtil.hexString2Uint8Array(data: txPackString) else{
        throw SDKError.unwrapError
      }
      let hash = bytes.sha256().sha256()
      var sig = try SigUtil.ecSignHash(hash: hash)
      sig.insert(UInt8(sig.count), at: 0)
      sig.insert(0x00, at: 0)
      sig.append(contentsOf: bytes)
      
      return EOSTransaction.signatureBase58(data: try key.sign(handle: handle, data: Data(sig), hash: hashedTx))
    }
    
    return EOSSignResult(hash: hash, signs: signs)
  }
  
  static func signatureBase58(data: Data) -> String {
    let toHash = NSMutableData()
    toHash.append(data)
    toHash.append("K1".data(using: .ascii)!)
    let checksum = (BTCRIPEMD160(toHash as Data) as Data).bytes[0..<4]
    
    let ret = NSMutableData()
    ret.append(data)
    ret.append(Data(bytes: checksum))
    
    return "SIG_K1_\(BTCBase58StringWithData(ret as Data)!)"
  }
}
