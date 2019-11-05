//
//  EOSTransactionSigner.swift
//  TokenCore
//
//  Created by xyz on 2018/5/26.
//  Copyright © 2018 ConsenLabs. All rights reserved.
//

import Foundation

public class EOSTransactionSigner {    /* @XM@TODO: check public */
  private let txs: [EOSTransaction]
  private let path: String
  private let handle: UInt
  
  public init(txs: [EOSTransaction], handle: UInt, path: String) {  /* @XM@TODO: check public */
    self.txs = txs
    self.path = path
    self.handle = handle
  }
  
  public func sign() throws -> [EOSSignResult] {
    return try txs.map { tx -> EOSSignResult in
      return try tx.sign(handle: handle, path: path);
    }
  }
}
