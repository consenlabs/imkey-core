//
//  BTCTransaction.swift
//  TokenCore
//
//  Created by James Chen on 2018/05/17.
//  Copyright © 2018 ConsenLabs. All rights reserved.
//

import Foundation
import CoreBitcoin

extension BTCTransaction {
  func imkeyAddInputs(from utxos: [UTXO], isSegWit: Bool = false) {
    for utxo in utxos {
      let input: BTCTransactionInput
      if isSegWit {
        input = SegWitInput()
      } else {
        input = BTCTransactionInput()
      }
      
      input.previousHash = BTCReversedData(BTCDataFromHex(utxo.txHash))
      input.previousIndex = UInt32(utxo.vout)
      input.signatureScript = BTCScript(hex: utxo.scriptPubKey)
      input.sequence = utxo.sequence
      input.value = utxo.amount
      
      addInput(input)
    }
  }
  
  func imkeyCalculateTotalSpend(utxos: [UTXO]) -> Int64 {
    return utxos.map { $0.amount }.reduce(0, +)
  }
  
  public func imkeySign(handle:UInt, utxos:[UTXO], fee: Int64, to:BTCAddress, network: Network, pathPrefix:String)throws{
    try Wallet.selectApplet(handle: handle, aid: Applet.btcAID);
    //apdu - get pubkeys
    let inSize = UInt8(truncatingIfNeeded: utxos.count)
    var pkArray = [Data]();
    
    var mainPath = pathPrefix
    mainPath.remove(at: pathPrefix.index(before: pathPrefix.endIndex))
    Log.d(mainPath)
    let xpubVersion = network.isMainnet ? 76067358 : 70617039
    let mainXpub = try Wallet.getBTCXpub(handle: handle, version: Int32(xpubVersion), path: mainPath)
    
    for (utxo, _) in zip(utxos, inputs) {
      let btckey = BTCKeychain(extendedKey: mainXpub).derivedKeychain(withPath: utxo.derivedPath).key!
      
      var btcAddress = network.isMainnet ? btckey.address.string : btckey.addressTestnet.string
      if btcAddress == utxo.address{
        pkArray.append(btckey.publicKey as Data)
      }else{
        btcAddress = network.isMainnet ? btckey.witnessAddress.string : btckey.witnessAddressTestnet.string
        if btcAddress == utxo.address{
          pkArray.append(btckey.compressedPublicKey as Data)
        }else{
          throw SDKError.addressVerifyFailed
        }
      }
    }
    
    var outputsRaw = serializeOutputs(hashType: .BTCSignatureHashTypeAll, inSize: inSize, outSize: UInt8(outputs.count), fee: fee).toHexString()
    let hexToAddress = BTCDataFromBase58(to.string).hex()!
    let version = hexToAddress.key_substring(to: 2)
    outputsRaw += version
    
    let outPutHex = try packSign(signHex: outputsRaw)
    
//    if let outputApdu = APDU.btcOutput(data: outPutHex) {
//      let result = try BLE.shared().sendApdu(handle: handle, apdu: outputApdu,timeout: Constants.sendSignPreAPDUTimeout)
//      try APDU.checkResponse(res: result)
//    }
    
    if let outputApdus = APDU.btcOutputs(data: outPutHex){
      try BLE.shared().sendPrepareApdus(handle: handle, apdus: outputApdus)
    }
    
    let eachRoundNumber = 5
    let round = inputs.count / eachRoundNumber + (inputs.count % eachRoundNumber == 0 ? 0 : 1)
    for r in 0..<round{
      Log.d("round \(r)........")
      var signIndexs = [Int]()
      
      //send inputs
      for (index,ele) in inputs.enumerated() {
        let payload = NSMutableData()
        let input = ele as! BTCTransactionInput
        let indexHex = String(format:"%02X", index)
        payload.append(indexHex.key_dataFromHexString()!)
        payload.append(input.outpoint.outpointData)
        if index >= r * eachRoundNumber && index < (r + 1) * eachRoundNumber{
          payload.append("19".key_dataFromHexString()!)   //need manually add '19'
          payload.append(input.signatureScript.data)
          //collect current round sign index
          signIndexs.append(index)
        }else{
          payload.append("00".key_dataFromHexString()!)
        }
        payload.append(&input.sequence, length:4)
        let data = payload as Data
        let inputApdu = APDU.btcInput(data: data.toHexString())!
        let inputResult = try BLE.shared().sendApdu(handle: handle, apdu: inputApdu)
        try APDU.checkResponse(res: inputResult)
      }
      
      //sign
      for (index, ele) in inputs.enumerated() {
        if signIndexs.contains(index){
          let input = ele as! BTCTransactionInput
          let uIndex = UInt8(index)
          guard let derPath = utxos[index].derivedPath else {
            throw GenericError.paramError
          }
          let apdu = APDU.btcSign(index: uIndex, hashType: 1, path: pathPrefix + derPath);
          let sign = try BLE.shared().sendApdu(handle: handle, apdu: apdu!);
          try APDU.checkResponse(res: sign)
          let signRes = sign

          let r = signRes.key_substring(from: 2).key_substring(to: 64)
          let s = signRes.key_substring(from: 66).key_substring(to: 64)

          Log.d("\n*****************")
          Log.d("\(index) r:\(r)")
          Log.d("\(index) s:\(s)")

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
          signDer.append(Data(hex: "01")) //need signtype

          input.signatureScript = BTCScript()!.appendData(signDer).appendData(pkArray[index] as Data)
        }
      }
    }
  }
  
  func packSign(signHex:String)throws ->String{
    guard let oriSignBytes = ByteUtil.hexString2Uint8Array(data: signHex)else{
      throw SDKError.unwrapError
    }
    
    var data:[UInt8] = [UInt8](repeating: 0, count: oriSignBytes.count + 2)
    data[0] = 0x01
    data[1] = UInt8(oriSignBytes.count)
    data[2...] = oriSignBytes[0...]
    let hash = data.sha256().sha256()
    
    let nilValue:[UInt8] = [UInt8](repeating: 0, count: 32)
    if KeyManager.shared().prvKey == nilValue{
      throw SDKError.notBindCheck
    }
    let prvKey = ByteUtil.uint8Array2HexString(data: KeyManager.shared().prvKey)
    let result = SigUtil.ecsign(with: prvKey, data: ByteUtil.uint8Array2HexString(data: hash).lowercased())
    let sig = try TLVUtil.encodeSignature(r: result["r"] as! String, s: result["s"] as! String)
    guard let sigBytes = ByteUtil.hexString2Uint8Array(data: sig) else{
      throw SDKError.unwrapError
    }
    data.insert(contentsOf: sigBytes, at: 0)
    data.insert(UInt8(sigBytes.count), at: 0)
    data.insert(0x00,at: 0)
    let signHex = ByteUtil.uint8Array2HexString(data: data)
    return signHex
  }
  
  func serializeOutputs(hashType: BTCSignatureHashType, inSize: UInt8, outSize: UInt8, fee: Int64) -> Data {
    let payload = NSMutableData()
    var inputSize = inSize
    var outputSize = outSize
    var lock = lockTime
    let feeLE = UInt64(fee)
    var feeBE = CFSwapInt64HostToBig(feeLE)
    var hashTypeLe = UInt32(hashType.rawValue)
    payload.append(&version, length: 4)
    
    payload.append(&inputSize, length: 1)
    
    payload.append(&outputSize, length: 1)
    for ele in outputs {
      let output = ele as! BTCTransactionOutput
      payload.append(output.data)
    }
    
    payload.append(&lock, length:4)
    payload.append(&hashTypeLe, length:4)
    payload.append(&feeBE, length:8)
    
    return payload as Data
  }
  
  func sendInputsAndSign(handle:UInt) throws{
    let eachTimeNumber = 5
    
    let time = inputs.count / eachTimeNumber + (inputs.count % eachTimeNumber == 0 ? 0 : 1)
    for i in 0..<time{
      for (index,ele) in inputs.enumerated() {
        let payload = NSMutableData()
        let input = ele as! BTCTransactionInput
        payload.append(input.outpoint.outpointData)
        if index >= i * eachTimeNumber && index < (eachTimeNumber + 1) * time{
          payload.append("19".key_dataFromHexString()!)   //need manually add '19'
          payload.append(input.signatureScript.data)
        }
        payload.append(&input.sequence, length:4)
        let data = payload as Data
        let inputApdu = APDU.btcInput(data: data.toHexString())!
        let inputResult = try BLE.shared().sendApdu(handle: handle, apdu: inputApdu)
        try APDU.checkResponse(res: inputResult)
      }
    }
  }
  
  public func imkeySignSegWit(handle:UInt, utxos:[UTXO], fee: Int64,to:BTCAddress, network: Network, pathPrefix:String) throws {
    try Wallet.selectApplet(handle: handle, aid: Applet.btcAID);
    //apdu-get pubkeys
    let inSize = UInt8(truncatingIfNeeded: utxos.count)
    var pkArray = [Data]();
    
    var mainPath = pathPrefix
    mainPath.remove(at: pathPrefix.index(before: pathPrefix.endIndex))
    Log.d(mainPath)
    let xpubVersion = network.isMainnet ? 76067358 : 70617039
    let mainXpub = try Wallet.getBTCXpub(handle: handle, version: Int32(xpubVersion), path: mainPath)
    
    for (utxo, ele) in zip(utxos, inputs) {
      let btckey = BTCKeychain(extendedKey: mainXpub).derivedKeychain(withPath: utxo.derivedPath).key!
      let btcAddress = network.isMainnet ? btckey.witnessAddress.string : btckey.witnessAddressTestnet.string
      if btcAddress != utxo.address{
        throw SDKError.addressVerifyFailed
      }
      
      pkArray.append(btckey.compressedPublicKey as Data);
      let scriptCode = BTCScript(hex: "1976a914\((BTCHash160(btckey.compressedPublicKey as Data) as Data).toHexString())88ac")
      let input = ele as! BTCTransactionInput
      input.signatureScript = scriptCode;
    }
    //orgnaize output
    //serialize
    var signedHexRaw = serializeOutputs(hashType: .BTCSignatureHashTypeAll, inSize: inSize, outSize: UInt8(outputs.count), fee: fee).toHexString()
    
    let hexToAddress = BTCDataFromBase58(to.string).hex()!
    let version = hexToAddress.key_substring(to: 2)
    signedHexRaw += version
    
    let signHex = try packSign(signHex: signedHexRaw)
    
    //apdu-prepare data
    if let apdus = APDU.btcSegwitTransPre(data: signHex, type: APDU.btcSegwitPreType_output) {
      let preRes = try BLE.shared().sendPrepareApdus(handle: handle, apdus: apdus)
      try APDU.checkResponse(res: preRes)
    }
    
    let hashPayload = NSMutableData()
    let sequencePayload = NSMutableData()
    for ele in inputs{
      let input = ele as! BTCTransactionInput
      hashPayload.append(input.previousHash)
      hashPayload.append(&input.previousIndex,length: 4)
      sequencePayload.append(&input.sequence,length: 4)
    }
    let hashVoutHex = (hashPayload as Data).toHexString()
    let sequenceHex = (sequencePayload as Data).toHexString()
    
    if let hashVoutApdus = APDU.btcSegwitTransPre(data: hashVoutHex, type: APDU.btcSegwitPreType_utxoHashVout),let sequenceApdus = APDU.btcSegwitTransPre(data: sequenceHex, type: APDU.btcSegwitPreType_utxoSequence){
      
      for apdu in hashVoutApdus {
        let hashRes = try BLE.shared().sendApdu(handle: handle, apdu: apdu)
        try APDU.checkResponse(res: hashRes)
      }
      
      for apdu in sequenceApdus{
        let seqRes = try BLE.shared().sendApdu(handle: handle, apdu: apdu)
        try APDU.checkResponse(res: seqRes)
      }
    }
    
    
    
    //apdu-sign
    for (index, ele) in inputs.enumerated() {
      //redeem script
      let redeemScript = BTCScript(hex: "0014\((BTCHash160(pkArray[index]) as Data).toHexString())")
      let input = ele as! BTCTransactionInput
      
      // utxo
      let payload = NSMutableData()
      payload.append(input.previousHash)
      payload.append(&input.previousIndex,length: 4)
      payload.append(input.signatureScript.data)
      var amount = input.value
      payload.append(&amount, length:MemoryLayout<Int64>.size)
      payload.append(&input.sequence, length:4)
      let utxo = (payload as Data).toHexString()

      
      guard let derPath = utxos[index].derivedPath else {
        throw GenericError.paramError
      }
      
      let apdu = APDU.btcSegwitTransSign(utxo: utxo, path: pathPrefix + derPath,isLastUtxo: index == inputs.count-1)
      let sign = try BLE.shared().sendApdu(handle: handle, apdu: apdu!);
      try APDU.checkResponse(res: sign)
      let signRes = sign
      
      let r = signRes.key_substring(from: 2).key_substring(to: 64)
      let s = signRes.key_substring(from: 66).key_substring(to: 64)
      
      Log.d("\n*****************")
      Log.d("\(index) r:\(r)")
      Log.d("\(index) s:\(s)")
      
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
      signDer.append(Data(hex: "01")) //need signtype
      
      input.witnessData = BTCScript()!.appendData(signDer).appendData(pkArray[index] as Data)
      input.signatureScript = BTCScript()!.append(redeemScript)
    }
  }

}

class SegWitInput: BTCTransactionInput {
  override var data: Data! {
    let payload = NSMutableData()
    
    payload.append(previousHash)
    payload.append(&previousIndex, length: 4)
    
    if isCoinbase {
      payload.append(BTCProtocolSerialization.data(forVarInt: UInt64(coinbaseData.count)))
      payload.append(coinbaseData)
    } else {
      payload.append(BTCProtocolSerialization.data(forVarInt: 23))
      payload.append(BTCProtocolSerialization.data(forVarInt: 22))
      payload.append(signatureScript.data)
    }
    
    payload.append(&sequence, length: 4)
    
    return payload as Data
  }
}
