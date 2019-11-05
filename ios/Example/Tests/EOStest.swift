//
//  EOStest.swift
//  imKeyConnector_Tests
//
//  Created by XM on 26/9/18.
//  Copyright © 2018 CocoaPods. All rights reserved.
//

import XCTest
import imKeyConnector
import CoreBitcoin


class EOStest: XCTestCase {
  
  override func setUp() {
    // Put setup code here. This method is called before the invocation of each test method in the class.
  }
  
  override func tearDown() {
    // Put teardown code here. This method is called after the invocation of each test method in the class.
  }
  
  func testExample() {
    // This is an example of a functional test case.
    // Use XCTAssert and related functions to verify your tests produce the correct results.
  }
  
  func testPerformanceExample() {
    // This is an example of a performance test case.
    self.measure {
      // Put the code you want to measure the time of here.
    }
  }
  
  func testSignTransacton() {
    /*
    //checksum(first 4 bytes of sha256) + preview + hashedTx + path in TLV format
    let signRaw = "00044cabb9db0704786d746f0806786d66726f6d09063132333435360120b998c88d8478e87e6dee727adecec067a3201da03ec8f8e8861c946559be635505116d2f3434272f313934272f30272f302f30"
    let txs = [
      EOSTransaction(
        data: "c578065b93aec6a7c811000000000100a6823403ea3055000000572d3ccdcd01000000602a48b37400000000a8ed323225000000602a48b374208410425c95b1ca80969800000000000453595300000000046d656d6f00",
        //hash: 6af5b3ae9871c25e2de195168ed7423f455a68330955701e327f02276bb34088   //@XM by code calculated
        publicKeys: ["EOS5SxZMjhKiXsmjxac8HBx56wWdZV1sCLZESh3ys1rzbMn4FUumU"],
        chainID: "aca376f206b8fc25a6ed44dbdc66547c36c6c33e3a119ffbeaef943642f0e906",
        /* @XM preview info now designed to be to + from + amount in TLV format
         to:     786d746f --- "xmto" / EOS_TAG_TO = 0x07;
         from:   786d66726f6d --- "xmfrom" / EOS_TAG_FROM = 0x08;
         amount: 123456  /   EOS_TAG_AMOUNT = 0x09
         */
        //preview: "0704786d746f0806786d66726f6d0906313233343536"
        to: "786d746f",
        memo: "testmemo",
        amount: "2.00000",
        symbol: "LXMEOS"
      )
    ]
    let result = try! EOSTransactionSigner(txs: txs, handle: 0,path:BIP44.EOS_LEDGER).sign()
    /* @XM can only check serializaiton result now
     XCTAssertEqual(1, result.count)
     XCTAssertEqual(
     result[0],
     EOSSignResult(hash: "6af5b3ae9871c25e2de195168ed7423f455a68330955701e327f02276bb34088", signs: ["SIG_K1_KkCTdqnTztAPnYeB2TWhrqcDhnnLvFJJdXnFCE3g8jRyz2heCggDQt5bMABu4LawHaDy4taHwJR3XMKV2ZXnBWqyiBnQ9J"])
     )
     */
 */
  }
}
