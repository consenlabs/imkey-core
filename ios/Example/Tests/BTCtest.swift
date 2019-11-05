//
//  BTCtest.swift
//  imKeyConnector_Tests
//
//  Created by XM on 25/9/18.
//  Copyright © 2018 CocoaPods. All rights reserved.
//

import XCTest
import imKeyConnector
import CoreBitcoin

class BTCtest: XCTestCase {
  
  override func setUp() {
    // Put setup code here. This method is called before the invocation of each test method in the class.
  }
  
  override func tearDown() {
    // Put teardown code here. This method is called after the invocation of each test method in the class.
  }
  
  func testExample() {
    // This is an example of a functional test case.
    // Use XCTAssert and related functions to verify your tests produce the correct results.
    Log.d(String(16, radix: 76067358))
  }
  
  func testPerformanceExample() {
    // This is an example of a performance test case.
    self.measure {
      // Put the code you want to measure the time of here.
    }
  }
  
  func testSegWitSignTx() {
    /*
    let pathPrefix = "m/49'/0'/0'"
    let singleUTXOfixture: [String: Any] = [
      "tx": "020000000177541aeb3c4dac9260b68f74f44c973081a9d4cb2ebe8038b2d70faa201b6bdb010000001976a91479091972186c449eb1ded22b78e40d009bdf008988acffffffff00ca9a3b0000000002b8b4eb0b000000001976a914a457b684d7f0d539a46a45bbc043f35b59d0d96388ac0008af2f000000001976a914fd270b1ee6abcaea97fea7ad0402e8bd8ad6d77c88ac0000000001000000480d000000000000",
      "amount": Int64(199996600),
      "fee": Int64(3400),
      "to": "1Fyxts6r24DpEieygQiNnWxUdb18ANa5p7",
      "changeIdx": 0,
      "utxos": [
        [
          "txHash": "db6b1b20aa0fd7b23880be2ecbd4a98130974cf4748fb66092ac4d3ceb1a5477",
          "vout": 1,
          "amount": "1000000000",
          "address": "mh7jj2ELSQUvRQELbn9qyA4q5nADhmJmUC",
          "scriptPubKey": "a9144733f37cf4db86fbc2efed2500b4f4e49f31202387",
          "derivedPath": "0/12"
        ],
      ]
    ]
    
    do {
      let changeAddr =  BTCAddress(string: "1Q5YjKVj5yQWHBBsyEBamkfph3cA6G9KK8");
      let utxos: [UTXO] = (singleUTXOfixture["utxos"] as! [[String: Any]]).map {UTXO(raw: $0)! }
      
      let signer = try BTCTransactionSigner(
        utxos: utxos,
        amount: singleUTXOfixture["amount"] as! Int64,
        fee: singleUTXOfixture["fee"] as! Int64,
        toAddress: BTCAddress(string: singleUTXOfixture["to"] as? String)!,
        changeAddress: changeAddr!,
        handle: 0   //@XM@20180925 TODO: pass in real handle here
      )
      let result = try signer.signSegWit(network:Network.testnet,pathPrefix: pathPrefix)
      //@XM@20180925 compare tx with signedHexRaw
      //XCTAssertEqual(result.signedTx, singleUTXOfixture["sign"] as! String)
    } catch {
      XCTFail("Create wallet failed \(error)")
    }
 */
  }
  
  func testParentPath(){
    sayHi()
  }
  
  func sayHi(name:String = "everyone"){
    Log.d(name)
  }
  
  func testDepth(){
    //        let depth = Wallet.getDepth(path: "m/44'/0'/0'/0/")
    //        Log.d(depth)
    //        XCTAssertEqual(depth, 5)
  }
  
  func testCheckPath(){
    let path1 = "m/44'"
    let path2 = "m/44'/0'/0'/0'/0'/0'/0'/0'/0'/0'/0'/0'/0'/0'/0'"
    let path3 = "s/44'/0'/0'"
    let path4 = "ms/44'/0'/0'"
    let path5 = "m/44'/0'/0'ssssssss"
    
    do {
//      try BIP44.checkPath(path: path1)
//      try BIP44.checkPath(path: path2)
//      try BIP44.checkPath(path: path3)
//      try BIP44.checkPath(path: path4)
//      try BIP44.checkPath(path: path5)
      try BIP44.checkPath(path: BIP44.btcMainnet)
      try BIP44.checkPath(path: BIP44.btcSegwitMainnet)
    } catch let e as ImkeyError {
      Log.d("!!!error:\(e.message)")
    }catch{
      Log.d(error)
    }
  }
}
