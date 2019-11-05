//
//  TransactionSignedResult.swift
//  token
//
//  Created by xyz on 2018/1/22.
//  Copyright © 2018 ConsenLabs. All rights reserved.
//

import Foundation

public struct TransactionSignedResult {
  public let signedTx: String
  public let txHash: String
  public let wtxID: String
  public let cosmosSignedTx: JSONObject
  
  init(signedTx: String, txHash: String, wtxID: String, cosmosSignedTx: JSONObject) {
    self.signedTx = signedTx
    self.txHash = txHash
    self.wtxID = wtxID
    self.cosmosSignedTx = cosmosSignedTx
  }
  
  init(signedTx: String, txHash: String, wtxID: String) {
    self.init(signedTx: signedTx, txHash: txHash, wtxID: wtxID, cosmosSignedTx: [:])
  }
  
  init(signedTx: String, txHash: String) {
    self.init(signedTx: signedTx, txHash: txHash, wtxID: "", cosmosSignedTx: [:])
  }
  
  init(cosmosSignedTx: JSONObject, txHash: String) {
    self.init(signedTx: "", txHash: txHash, wtxID: "", cosmosSignedTx: cosmosSignedTx)
  }
}
