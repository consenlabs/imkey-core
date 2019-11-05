//
//  OmniTransaction.swift
//  imKeyConnector
//
//  Created by joe on 5/26/19.
//

import Foundation

public class OmniTransaction{
  public class func omniSign(utxos: [UTXO], amount: Int64, fee: Int64, toAddress: BTCAddress, propertyId: Int, handle: UInt,network:Network,pathPrefix:String, payment: String, receiver: String, sender: String, feeDis: String) throws -> TransactionSignedResult{
    guard let hexToAddress = BTCDataFromBase58(toAddress.string).hex() else{
      throw SDKError.illegalArgument
    }
    try ValidatorAddress.checkAddress(network:network,to: hexToAddress)
    
    let signer = try OmniTransactionSigner(utxos: utxos, amount: amount, fee: fee, toAddress: toAddress, propertyId: propertyId, handle: handle,payment: payment, to: receiver, from: sender, feeDis: feeDis)
    return try signer.sign(network: network, pathPrefix: pathPrefix)
  }
  
  public class func omniSignSegwit(utxos: [UTXO], amount: Int64, fee: Int64, toAddress: BTCAddress, propertyId: Int, handle: UInt,network:Network,pathPrefix:String, payment: String, receiver: String, sender: String, feeDis: String) throws -> TransactionSignedResult{
    guard let hexToAddress = BTCDataFromBase58(toAddress.string).hex() else{
      throw SDKError.illegalArgument
    }
    try ValidatorAddress.checkAddress(network:network,to: hexToAddress)
    
    let signer = try OmniTransactionSigner(utxos: utxos, amount: amount, fee: fee, toAddress: toAddress, propertyId: propertyId, handle: handle,payment: payment, to: receiver, from: sender, feeDis: feeDis)
    return try signer.signSegWit(network: network, pathPrefix: pathPrefix)
  }
}

extension BTCTransaction {
  public func imkeyOmniSign(handle:UInt, utxos:[UTXO], fee: Int64, to:BTCAddress, network: Network, pathPrefix:String)throws{
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
    
    if let outputApdu = APDU.omniOutput(data: outPutHex) {
      let result = try BLE.shared().sendApdu(handle: handle, apdu: outputApdu,timeout: Constants.sendSignPreAPDUTimeout)
      try APDU.checkResponse(res: result)
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
        let inputApdu = APDU.omniInput(data: data.toHexString())!
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
          let apdu = APDU.omniSign(index: uIndex, hashType: 1, path: pathPrefix + derPath);
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
          let sLow = SigUtil.getLowS(s:sBig!)
          sPoint.initialize(from: sLow.bignum, count: 1)
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
  
  public func imkeyOmniSegwitSign(handle:UInt, utxos:[UTXO], fee: Int64,to:BTCAddress, network: Network, pathPrefix:String) throws {
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
    if let apdus = APDU.omniSegwitTransPre(data: signHex, type: APDU.btcSegwitPreType_output) {
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
    
    if let hashVoutApdus = APDU.omniSegwitTransPre(data: hashVoutHex, type: APDU.btcSegwitPreType_utxoHashVout),let sequenceApdus = APDU.btcSegwitTransPre(data: sequenceHex, type: APDU.btcSegwitPreType_utxoSequence){
      
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
      //      payload.append(input.outpoint.outpointData)
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
      let sLow = SigUtil.getLowS(s:sBig!)
      sPoint.initialize(from: sLow.bignum, count: 1)
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
