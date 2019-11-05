//
//  BTCTransaction.swift
//  token
//
//  Created by xyz on 2018/1/4.
//  Copyright © 2018 ConsenLabs. All rights reserved.
//

import Foundation
import CoreBitcoin

public struct UTXO {
  let txHash: String
  let vout: Int
  let amount: Int64
  let address: String
  let scriptPubKey: String
  let derivedPath: String?
  let sequence: UInt32
  
  public init(txHash: String, vout: Int, amount: Int64, address: String, scriptPubKey: String, derivedPath: String?, sequence: UInt32 = 4294967295) {
    self.txHash = txHash
    self.vout = vout
    self.amount = amount
    self.address = address
    self.scriptPubKey = scriptPubKey
    self.derivedPath = derivedPath
    self.sequence = sequence
  }
  
  public init?(raw: [String: Any]) {
    guard let txHash = raw["txHash"] as? String,
      let vout = raw["vout"] as? Int,
      let amount = Int64(raw["amount"] as? String ?? "badamount"),
      let address = raw["address"] as? String,
      let scriptPubKey = raw["scriptPubKey"] as? String else {
        return nil
    }
    let derivedPath = raw["derivedPath"] as? String
    
    self.init(
      txHash: txHash,
      vout: vout,
      amount: amount,
      address: address,
      scriptPubKey: scriptPubKey,
      derivedPath: derivedPath
    )
  }
}

public class BTCTransactionSigner {
  let utxos: [UTXO]
  let amount: Int64
  let fee: Int64
  let toAddress: BTCAddress
  let changeAddress: BTCAddress
  let dustThreshold: Int64 = 2730
  let handle: UInt
  let payment: String
  let to: String
  let from: String
  let feeDis: String  // this is fee used for display
  let extra: [AnyHashable: Any]?
  
  public init(utxos: [UTXO], amount: Int64, fee: Int64, toAddress: BTCAddress, changeAddress: BTCAddress, extra: [AnyHashable: Any]? = nil, handle: UInt, payment: String, to: String, from: String, feeDis: String) throws {
    guard amount >= dustThreshold else {
      throw GenericError.amountLessThanMinimum
    }
    
    self.utxos = utxos
    self.amount = amount
    self.fee = fee
    self.toAddress = toAddress
    self.changeAddress = changeAddress
    self.handle = handle
    self.payment =  payment
    self.to = to
    self.from = from
    self.feeDis = feeDis
    self.extra = extra
  }
  
  func sign(network:Network,pathPrefix:String) throws -> TransactionSignedResult {
    try BIP44.checkPath(path: pathPrefix)
    if utxos.count > Constants.maxUtxoNumber{
      throw SDKError.exceededMaxUtxoNum
    }
    
    let rawTx = BTCTransaction()
    rawTx.version = 1
    
    let totalAmount = rawTx.imkeyCalculateTotalSpend(utxos: utxos)
    if totalAmount < amount {
      throw GenericError.insufficientFunds
    }
    
    rawTx.imkeyAddInputs(from: utxos)
    
    rawTx.addOutput(BTCTransactionOutput(value: amount, address: toAddress))
    
    let changeAmount = totalAmount - amount - fee
    if changeAmount >= dustThreshold {
      rawTx.addOutput(BTCTransactionOutput(value: changeAmount, address: changeAddress))
    }
    
    if let extra = self.extra {
      if let opReturnHex = extra["opReturn"] as? String {
        guard let payload = opReturnHex.key_dataFromHexString() else {
          throw "opReturn must be a valid hex"
        }
        let bytes = Hex.toBytes(opReturnHex);
        guard bytes.count <= Constants.maxOPReturnSize else{
            throw SDKError.illegalArgument
        }
        
        let opReturnScript = BTCScript()?.append(BTCOpcode.OP_RETURN)?.appendData(payload)
        rawTx.addOutput(BTCTransactionOutput(value: 0, script: opReturnScript))
      }
    }
    
    try rawTx.imkeySign(handle: handle, utxos: utxos, fee: fee,to: toAddress, network: network, pathPrefix: pathPrefix)
    
    let signedTx = rawTx.hex!
    let txHash = rawTx.transactionID!
    return TransactionSignedResult(signedTx: signedTx, txHash: txHash)
  }
  
  public func signSegWit(network:Network,pathPrefix: String) throws -> TransactionSignedResult {
    try BIP44.checkPath(path: pathPrefix)
    
    let rawTx = BTCTransaction()
    rawTx.version = 2
    
    let totalAmount = rawTx.imkeyCalculateTotalSpend(utxos: utxos)
    if totalAmount < amount {
      throw GenericError.insufficientFunds
    }
    
    rawTx.imkeyAddInputs(from: utxos, isSegWit: true)
    
    rawTx.addOutput(BTCTransactionOutput(value: amount, address: toAddress))
    
    let changeAmount = rawTx.imkeyCalculateTotalSpend(utxos: utxos) - amount - fee
    if changeAmount >= dustThreshold {
      rawTx.addOutput(BTCTransactionOutput(value: changeAmount, address: changeAddress))
    }
    
    if let extra = self.extra {
      if let opReturnHex = extra["opReturn"] as? String {
        guard let payload = opReturnHex.key_dataFromHexString() else {
          throw "opReturn must be a valid hex"
        }
        let bytes = Hex.toBytes(opReturnHex);
        guard bytes.count <= Constants.maxOPReturnSize else{
            throw SDKError.illegalArgument
        }
        
        let opReturnScript = BTCScript()?.append(BTCOpcode.OP_RETURN)?.appendData(payload)
        rawTx.addOutput(BTCTransactionOutput(value: 0, script: opReturnScript))
      }
    }
    
    try rawTx.imkeySignSegWit(handle:handle, utxos:utxos, fee: fee,to:toAddress, network: network, pathPrefix: pathPrefix);
    
    let signedTx = rawTx.hexWithWitness!
    let txHash = rawTx.transactionID!
    let wtxID = rawTx.witnessTransactionID!
    return TransactionSignedResult(signedTx: signedTx, txHash: txHash, wtxID: wtxID)
  }
}
