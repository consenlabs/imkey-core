//
//  EOSTest.swift
//  imKeyConnector_Example
//
//  Created by joe on 7/17/19.
//  Copyright © 2019 CocoaPods. All rights reserved.
//

import Foundation
import imKeyConnector

class EOSTest: FeatTest {
  class func testEOSign(handle:UInt) -> TestResult{
    var sucessCount = 0
    var failCount = 0
    var failCaseInfo = [String]()
    let jsonRoot = readJson(resource: "eostransactiontest.json")
    for (key, value) in jsonRoot {
      if let jsonObject = value as? [String: Any], let preview = jsonObject["preview"] as? [String: Any],
        let publicKeys = jsonObject["publicKeys"] as? [[String: Any]]{
        let txHash = jsonObject["txHash"] as! String
        let txHex = jsonObject["txHex"] as! String
        let chainId = jsonObject["chainId"] as! String
        
        var pks = [String]()
        for item in publicKeys{
          pks.append(item["publicKey"] as! String)
        }
        
        var eosSignData = Eosapi_EosSignData()
        eosSignData.txData = txHex
        eosSignData.pubKeys = pks
        eosSignData.chainID = chainId
        eosSignData.to = preview["receiver"] as! String
        eosSignData.from = preview["sender"] as! String
        eosSignData.payment = preview["payment"] as! String
        
        var eosInput = Eosapi_EosTxInput()
        eosInput.path = BIP44.EOS_LEDGER
        eosInput.signDatas = [eosSignData]
        
        for i in 0...3{
          clear_err()
          let eosOutput = API.eosSignTx(eosInput: eosInput)
          let err = get_last_err_message()
          
          if err != nil{
            if eosOutput.transMultiSigns[0].hash == txHash{
              sucessCount += 1
              break
            }else{
              failCount += 1
              failCaseInfo.append("\(key)  \(i) time: Assert fail")
            }
          }else{
            failCount += 1
            failCaseInfo.append("\(key)  \(i) time: \(err)")
          }
          
        }
      }
    }
    return TestResult(totalCaseCount: jsonRoot.count, successCaseCount: sucessCount, failCaseCount: failCount, failCaseInfo: failCaseInfo)
  }
}
