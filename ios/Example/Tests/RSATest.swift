//
//  RSATest.swift
//  imKeyConnector_Example
//
//  Created by joe on 1/18/19.
//  Copyright © 2019 CocoaPods. All rights reserved.
//

import XCTest
import imKeyConnector

class RSATest:XCTestCase{
  
  func testRSA(){
    let pubkey = """
MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEAwS+r/gaNAFAKGrrBa0Pv
YicgxLDfY6ZgYi4p1BiZvIjBrLjMkLLXMkvOA3N5SljKNRx1PKzQ7aC8SY4Rp1IF
45sFyN7ffb0X8hSu8vrZFePyl2IAGzEwYTcFYuz0q/OzTK+PZTcqPrw+VoWFCALi
1GFIJSFsbgA7M8K282fgSDrgRjcXDE4+B8uixRNgo4gUHSxChL9VAtkMs8+OUwiR
a1DnmZRsqy1Xz6sltSzQM9GA1ozvtaU0mgKWLA+iABq1/9K5aNpXakMXCYhyD7lN
oOOUnw1bafLlNoQDou8XlAI4FvrK5kYD8B74XWCeAtcF2dJdPM2YQk+TpDSK1PJo
GwIDAQAB
"""
    let result = try! RSAUtils.encryptWithRSAPublicKey(str: "AB8UX0R0", pubkeyBase64: pubkey)!
    //    Log.d(String(result))
    let bytes = [UInt8](result)
    let encryptStr = ByteUtil.uint8Array2HexString(data: bytes)
    Log.d(encryptStr)
  }
}
