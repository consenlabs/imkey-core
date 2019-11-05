//
//  ValidatorAddress.swift
//  BigInt
//
//  Created by XM on 13/11/18.
//

import Foundation
public class ValidatorAddress {
  public static func checkAddress(network:Network,to:String)throws {
    let version = to.key_substring(to: 2)
    if network.isMainnet {
      if version.uppercased() == "C4" || version.uppercased() == "6F" {
        throw SDKError.illegalArgument
      }
    }else{
      if version.uppercased() == "00" || version.uppercased() == "05" {
        throw SDKError.illegalArgument
      }
    }
  }
}
