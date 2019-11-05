//
//  BTCTransaction.swift
//  token
//
//  Created by xyz on 2018/1/4.
//  Copyright © 2018 ConsenLabs. All rights reserved.
//

import Foundation
import CoreBitcoin

public class OmniTransactionSigner {
  let utxos: [UTXO]
  let amount: Int64
  let fee: Int64
  let toAddress: BTCAddress
  let propertyId: Int
  let handle: UInt
  let payment: String
  let to: String
  let from: String
  let feeDis: String
  let minNonDustOutPut: Int64 = 546
  
  public init(utxos: [UTXO], amount: Int64, fee: Int64, toAddress: BTCAddress, propertyId: Int, handle: UInt, payment: String, to: String, from: String, feeDis: String) throws {
    
    self.utxos = utxos
    self.amount = amount
    self.fee = fee
    self.toAddress = toAddress
    self.propertyId = propertyId
    self.handle = handle
    self.payment =  payment
    self.to = to
    self.from = from
    self.feeDis = feeDis
  }
  
  func sign(network:Network,pathPrefix:String) throws -> TransactionSignedResult {
    try BIP44.checkPath(path: pathPrefix)
    if utxos.count > Constants.maxUtxoNumber{
      throw SDKError.exceededMaxUtxoNum
    }
    
    let rawTx = BTCTransaction()
    rawTx.version = 1
    
    let totalAmount = rawTx.imkeyCalculateTotalSpend(utxos: utxos)
    let changeAmount = totalAmount - fee - minNonDustOutPut
    if changeAmount < minNonDustOutPut{
      throw GenericError.amountLessThanMinimum
    }
    
    rawTx.imkeyAddInputs(from: utxos)
    
    rawTx.addOutput(BTCTransactionOutput(value: changeAmount, address: BTCAddress(string: utxos[0].address)))
    rawTx.addOutput(BTCTransactionOutput(value: minNonDustOutPut, address: toAddress))
    // add op_return data
    let payloadOut = try createPayloadUSDT(propertyId: propertyId, amount: amount)
    rawTx.addOutput(payloadOut)
    
    try rawTx.imkeyOmniSign(handle: handle, utxos: utxos, fee: fee,to: toAddress, network: network, pathPrefix: pathPrefix)
    
    let signedTx = rawTx.hex!
    let txHash = rawTx.transactionID!
    return TransactionSignedResult(signedTx: signedTx, txHash: txHash)
  }
  
  public func signSegWit(network:Network,pathPrefix: String) throws -> TransactionSignedResult {
    try BIP44.checkPath(path: pathPrefix)

    let rawTx = BTCTransaction()
    rawTx.version = 2
    
    let totalAmount = rawTx.imkeyCalculateTotalSpend(utxos: utxos)
    let changeAmount = totalAmount - fee - minNonDustOutPut
    if changeAmount < minNonDustOutPut{
      throw GenericError.amountLessThanMinimum
    }
    
    rawTx.imkeyAddInputs(from: utxos, isSegWit: true)
    
    
    rawTx.addOutput(BTCTransactionOutput(value: changeAmount, address: BTCScriptHashAddress(string: utxos[0].address)))
    
    rawTx.addOutput(BTCTransactionOutput(value: minNonDustOutPut, address: toAddress))
    
    let payloadOut = try createPayloadUSDT(propertyId: propertyId, amount: amount)
    rawTx.addOutput(payloadOut)
    
    try rawTx.imkeyOmniSegwitSign(handle:handle, utxos:utxos, fee: fee,to:toAddress, network: network, pathPrefix: pathPrefix)
    let signedTx = rawTx.hexWithWitness!
    let txHash = rawTx.transactionID!.lowercased()
    let wtxID = rawTx.witnessTransactionID!.lowercased()
    return TransactionSignedResult(signedTx: signedTx, txHash: txHash, wtxID: wtxID)
  }
  
  func createPayloadUSDT(propertyId: Int, amount: Int64)throws -> BTCTransactionOutput {
    let payload = NSMutableData()
    payload.append("6f6d6e6900000000\(String(format: "%08x", propertyId))".key_dataFromHexString()!)
    var amountBEPay = CFSwapInt64HostToBig(UInt64(amount))
    payload.append(&amountBEPay, length: 8)
    let payloadScriptPay = BTCScript()?.append(BTCOpcode.OP_RETURN)?.appendData(payload as Data)
    return BTCTransactionOutput(value: 0, script: payloadScriptPay)
  }
}
