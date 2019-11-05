//
//  ByteUtilTest.swift
//  imKeyConnector_Example
//
//  Created by joe on 11/23/18.
//  Copyright © 2018 CocoaPods. All rights reserved.
//

import XCTest
import imKeyConnector


class ByteUtilTest: XCTestCase {
  func testHexString2Uint8Array(){
    let bytes = ByteUtil.hexString2Uint8Array(data: "00A40400")
    let result:[UInt8] = [0x0,0xA4,0x4,0x0]
    XCTAssertEqual(result, bytes)
  }
  
  func testUint8Array2HexString(){
    let bytes:[UInt8] = [0x0,0xA4,0x4,0x0]
    let result = ByteUtil.uint8Array2HexString(data: bytes)
    XCTAssertEqual("00A40400", result)
  }
  
  func testReserve(){
    let result = ByteUtil.reserve(hexString: "ABCDABCDABCD")
    XCTAssertEqual("CDABCDABCDAB", result)
  }
}
