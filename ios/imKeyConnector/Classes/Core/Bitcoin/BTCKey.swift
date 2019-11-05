//
//  BTCKey.swift
//  imKeyConnector
//
//  Created by joe on 2018/10/16.
//

import Foundation
import CoreBitcoin

public extension BTCKey {
  public func imkeyAddress(on network: Network?, segWit: SegWit) -> BTCAddress {
    if segWit.isSegWit {
      if imkeyIsMainnet(network) {
        return witnessAddress
      } else {
        return witnessAddressTestnet
      }
    } else {
      if imkeyIsMainnet(network) {
        return address
      } else {
        return addressTestnet
      }
    }
  }
  
  private func imkeyIsMainnet(_ network: Network?) -> Bool {
    if let network = network {
      return network.isMainnet
    }
    return true
  }
}
