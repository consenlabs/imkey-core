//
//  CosmosViewController.swift
//  imKeyConnector_Example
//
//  Created by joe on 7/17/19.
//  Copyright © 2019 CocoaPods. All rights reserved.
//

import UIKit
import imKeyConnector

class CosmosViewController: UIViewController {

  var handle:UInt = 0
  override func viewDidLoad() {
    super.viewDidLoad()
  }
  @IBOutlet weak var txtResult: UITextView!
  
  @IBAction func backClick(_ sender: Any) {
    self.dismiss(animated: true, completion: nil)
  }
  
  @IBAction func autoSignClick(_ sender: Any) {
    txtResult.text = ""
    DispatchQueue.global().async {
      let result = CosmosTest.testCosmosSign(handle: self.handle)
      Log.d(result)
      self.appendResult(msg: result.description)
    }
  }
  
  @IBAction func signClick(_ sender: Any) {
    txtResult.text = ""
    let cosmosTx = createCosmosTX()
    do {
      let cosmosSigner = try CosmosTransaction(raw: cosmosTx)
      
      let to = "cosmos1yeckxz7tapz34kjwnjxvmxzurerquhtrmxmuxt";
      let fee = "0.00075 atom";
      let signResult = try cosmosSigner.sign(handle: handle, path: BIP44.cosmos, paymentDis: nil, toDis: to, feeDis: fee)
      Log.d(signResult)
      txtResult.text = "cosmos sign result:\n\(signResult)"
    } catch let e as ImkeyError {
      Log.d("!!!error:\(e.message)")
      toastMsg(message: e.message)
    }catch{
      Log.d(error)
    }
  }
  
  
  private func createCosmosTX() -> [String: Any]{
    let cosmosTx: [String: Any] = [
      "accountNumber": "1",
      "sequence": "0",
      "chainId": "tendermint_test",
      "msgs": [[
        "type": "cosmos-sdk/MsgDelegate",
        "value": [
          "amount": [
            [
              "denom": "atom",
              "amount": "10"
            ]
          ],
          "delegator_address": "cosmos1y0a8sc5ayv52f2fm5t7hr2g88qgljzk4jcz78f",
          "validator_address": "cosmosvaloper1zkupr83hrzkn3up5elktzcq3tuft8nxsmwdqgp"
        ]
        ]
      ],
      "fee": [
        "amount": [
          [
            "denom": "",
            "amount": "0"
          ]
        ],
        "gas": "21906"
      ],
      "signatures": Optional<Int>.none,
      "memo": Optional<String>.none,
    ]
    return cosmosTx
  }
  
  @IBAction func addressBtnClick(_ sender: Any) {
    txtResult.text = ""
    do {
      let address = try CosmosKey.getCosmosAddress(handle: handle, path: BIP44.cosmos)
      Log.d(address)
      txtResult.text = address
    } catch let e as ImkeyError {
      Log.d("!!!error:\(e.message)")
      toastMsg(message: e.message)
    }catch{
      Log.d(error)
    }
  }
  
  func appendResult(msg:String){
    DispatchQueue.main.async {
      self.txtResult.text += msg
      let bottom = NSMakeRange(self.txtResult.text.count - 1, 1)
      self.txtResult.scrollRangeToVisible(bottom)
    }
  }
}
