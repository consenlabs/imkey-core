//
//  ByteUtilTest.swift
//  imKeyConnector_Example
//
//  Created by joe on 11/23/18.
//  Copyright © 2018 CocoaPods. All rights reserved.
//

import XCTest
import imKeyConnector
import CoreBitcoin
import secp256k1


class KeyManagerTest: XCTestCase {
  func testGenFileKey(){
//    let key = KeyManager.shared().genFileKey(seid: "seid", sn: "sn")
//    XCTAssertEqual("CDABCDABCDAB", key)
  }
  
  func testECDH(){
    let btcKey1 = BTCKey()!
    let btcKey2 = BTCKey()!

    let context = secp256k1_context_create(UInt32(SECP256K1_CONTEXT_SIGN | SECP256K1_CONTEXT_VERIFY))!//witch part
    defer {
      secp256k1_context_destroy(context)
    }

    //bob
    var prvkey = [UInt8](btcKey1.privateKey as Data)

    var pubkey = [UInt8](btcKey2.publicKey as Data)
    var secp256k1Pubkey = secp256k1_pubkey()
    let paserResult = secp256k1_ec_pubkey_parse(context, &secp256k1Pubkey, &pubkey, pubkey.count)
    Log.d("paserResult:\(paserResult)")

    var secKey = [UInt8](repeating: 0, count: 32)
    let ecdhResult = secp256k1_ecdh(context, &secKey, &secp256k1Pubkey, &prvkey)
    Log.d("ecdhResult:\(ecdhResult)")

    //alice
    var prvkey2 = [UInt8](btcKey2.privateKey as Data)
    var pubkey2 = [UInt8](btcKey1.publicKey as Data)
    var secp256k1Pubkey2 = secp256k1_pubkey()
    let paserResult2 = secp256k1_ec_pubkey_parse(context, &secp256k1Pubkey2, &pubkey2, pubkey2.count)
    Log.d("paserResult2:\(paserResult2)")

    var secKey2 = [UInt8](repeating: 0, count: 32)
    let ecdhResult2 = secp256k1_ecdh(context, &secKey2, &secp256k1Pubkey2, &prvkey2)
    Log.d("ecdhResult2:\(ecdhResult2)")

    XCTAssertEqual(secKey, secKey2)
  }
  
  func testKey(){
    let appPrvkey = "44DD587E45A3B8936CF367E6B38E3FD40E5C4390B4C04B7BC7082A766B442AA4"
    let sePubkey = "04A609910703E61E5E924A0889468C8BFBB0F37751F786100ED31F593394A68FB4E6F8E31BC8A2CEDC9243FC96664512CEFD6A4378A751593BEFA1D8063D1183DC"
    
    var appPrv = ByteUtil.hexString2Uint8Array(data: appPrvkey)!
    var sePub = ByteUtil.hexString2Uint8Array(data: sePubkey)!
    
//    let context = secp256k1_context_create(UInt32(SECP256K1_CONTEXT_SIGN | SECP256K1_CONTEXT_VERIFY))!//witch part
    let context = secp256k1_context_create(UInt32(SECP256K1_CONTEXT_NONE))!//witch part
    defer {
      secp256k1_context_destroy(context)
    }
    
    var secp256k1Pubkey = secp256k1_pubkey()
    let paserResult = secp256k1_ec_pubkey_parse(context, &secp256k1Pubkey, &sePub, sePub.count)
    Log.d("paserResult:\(paserResult)")
    
    var ecdhKey = [UInt8](repeating: 0, count: 32)
    let ecdhResult = secp256k1_ecdh(context, &ecdhKey, &secp256k1Pubkey, &appPrv)
    Log.d("ecdhResult:\(ecdhResult)")
    Log.d("ecdhKey:\(ByteUtil.uint8Array2HexString(data: ecdhKey))")
  }
  
  func testAes(){
    let data = "7B5F9092ABD19BAF6B14CD4E4BEF298CC64FFA2C3B938AA53182B025BBC4C9B7"
//    try! KeyManager.shared().genFileKey(handle: 0)
//    let encryptStr = KeyManager.shared().encrypt(data: data)
//    //1ACC5ADC8F2B8C81E163E3DA3902F3D65B230414DBA9E53BCB9E0BE116EAD0906913BD663C7F35E5D2306AFEEED3BFA6
//    Log.d(encryptStr)
//    let decryptStr = KeyManager.shared().decrypt(data: encryptStr!)
//    Log.d(decryptStr)
    
    let key = "3D8A7AA2DD3D39E76E573B31409BE701"
    let iv = "9C0C30889CBCC5E01AB5B2BB88715799"
    let encryptStr = KeyManager.shared().sessionEncrypt(key: key, iv: iv, data: data)
    //4CB1B4A480951F5E2FF8C9041DB7A19FB57271D4DD3BABB62F0E8A1EE8CDBF81BCCF6CF175C42F03D58FA8630B8DF5A1
    Log.d(encryptStr)
  }
  
  func testOriAes(){
    let data = "7B5F9092ABD19BAF6B14CD4E4BEF298CC64FFA2C3B938AA53182B025BBC4C9B7"
        // use the test seid and sn
        let encryptStr = KeyManager.shared().encrypt(data: data)
        //1ACC5ADC8F2B8C81E163E3DA3902F3D65B230414DBA9E53BCB9E0BE116EAD0906913BD663C7F35E5D2306AFEEED3BFA6
        Log.d(encryptStr)
        let decryptStr = KeyManager.shared().decrypt(data: encryptStr!)
        Log.d(decryptStr)
  }
  
  func testECSign(){
    let prvKey = "C7C054D1D4EE0DF704273FCD93A025CAA12E06FCF93923EC97DBAE8051E4A753"
    let hash = "5646a94cd7d6b60a74082fe3b446ef9b8fc02d8c2bfb7ca8712c80833a1da623"
    let result = SigUtil.ecsign(with: prvKey, data: hash)
    Log.d("signature:\(result)")
    
    let signature = try! TLVUtil.encodeSignature(r: result["r"] as! String, s: result["s"] as! String)
   
    let expectSig = "304402202ACECDA07C17EAF83FC40EA3887267B5BB87E5C6286CFB5B5C0C1F6F8AFB17DE02207C3C7BD07E553BB5D19201D01CDA50388023F79593189330E0794D78AAAD0269"
    
    XCTAssertEqual(signature, expectSig)
  }
  
  func testRSA(){
    
  }
}
