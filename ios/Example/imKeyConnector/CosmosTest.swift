//
//  CosmosTest.swift
//  imKeyConnector_Example
//
//  Created by joe on 7/17/19.
//  Copyright © 2019 CocoaPods. All rights reserved.
//

import Foundation
import imKeyConnector

class CosmosTest:FeatTest{
  class func testCosmosSign(handle:UInt) -> TestResult{
    var sucessCount = 0
    var failCount = 0
    var failCaseInfo = [String]()
    let jsonRoot = readJson(resource: "cosmostransactiontest.json")
    for (key, value) in jsonRoot {
      if let dict = value as? [String: Any],let preview = dict["preview"] as? [String: Any],
        let sigs = dict["signatures"] as? [[String: Any]]{
        let raw = createCosmosRaw(dict: dict)
        print(raw)
        for i in 0...3{
          do {
            let cosmosSigner = try CosmosTransaction(raw: raw)
            let signResult = try cosmosSigner.sign(handle: handle, path: BIP44.cosmos, paymentDis: preview["payment"] as? String, toDis: preview["receiver"] as! String, feeDis: preview["fee"] as! String)
            let expSig = sigs[0]["signature"] as! String
            let sigTx = signResult.cosmosSignedTx as!  [String: Any]
            let resSigs = sigTx["signatures"] as! [[String: Any]]
            let resSig = resSigs[0]["signature"] as! String
            
            if expSig == resSig{
              sucessCount += 1
              break
            }else{
              failCount += 1
              failCaseInfo.append("\(key)  \(i) time: Assert fail")
            }
          } catch let e as ImkeyError {
            failCount += 1
            failCaseInfo.append("\(key)  \(i) time: \(e.message)")
          }catch{
            failCount += 1
            failCaseInfo.append("\(key)  \(i) time: \(error)")
          }
        }
      }
    }
    return TestResult(totalCaseCount: jsonRoot.count, successCaseCount: sucessCount, failCaseCount: failCount, failCaseInfo: failCaseInfo)
  }
  
  class func createCosmosRaw(dict:[String: Any]) -> [String: Any]{
    let cosmosTx: [String: Any] = [
      "accountNumber": dict["accountNumber"]!,
      "sequence": dict["sequence"]!,
      "chainId": dict["chainId"]!,
      "msgs":dict["msg"]!,
      "fee":dict["fee"]!,
      "memo":dict["memo"]!
    ]
    return cosmosTx
  }
}
