//
//  Bech32Address.swift
//  TokenCore
//
//  Created by XM on 16/4/19.
//  Copyright © 2019 imToken PTE. LTD. All rights reserved.
//

import Foundation
import CoreBitcoin


public class Bech32Address {
  public static func pubToBech32Address(_ publicKey: Data) -> String {
    let stringToHash = publicKey.key_toHexString()
    let hash = BTCHash160(stringToHash.key_dataFromHexString())
    let outPt = UnsafeMutablePointer<UInt8>.allocate(capacity: 65) //@@XM 20byte+hrp *5 / 8 should be ok
    let outLenPt = UnsafeMutablePointer<Int>.allocate(capacity: 1)
    outLenPt.initialize(to: 0)
    let inPt = hash?.bytes.assumingMemoryBound(to: UInt8.self)
    convert_bits(outPt, outLenPt, 5, inPt, 20, 8, 1)
    let addressPt = UnsafeMutablePointer<Int8>.allocate(capacity: 65) //@@XM 20byte+hrp *5 / 8 should be ok
    let hrpPt = strdup(Hrp.accountAddress.rawValue)
    bech32_encode(addressPt, hrpPt, outPt, outLenPt[0])
    return String(cString: addressPt)
  }
}
