//
//  CosmosViewController.swift
//  imKeyConnector_Example
//
//  Created by joe on 7/17/19.
//  Copyright © 2019 CocoaPods. All rights reserved.
//

import UIKit
import imKeyBleLib

class TronViewController: UIViewController {

  var handle:UInt = 0
  override func viewDidLoad() {
    super.viewDidLoad()
  }
  @IBOutlet weak var txtResult: UITextView!
  
  @IBAction func backClick(_ sender: Any) {
    self.dismiss(animated: true, completion: nil)
  }
  
  @IBAction func autoSignClick(_ sender: Any) {
    txtResult.text = "autosign"
//    DispatchQueue.global().async {
//      let result = CosmosTest.testCosmosSign(handle: self.handle)
//      Log.d(result)
//      self.appendResult(msg: result.description)
//    }
  }
  
  @IBAction func signClick(_ sender: Any) {
    txtResult.text = ""
//    let cosmosTx = createCosmosTX()
//    do {
////      let cosmosSigner = try CosmosTransaction(raw: cosmosTx)
////
////      let to = "cosmos1yeckxz7tapz34kjwnjxvmxzurerquhtrmxmuxt";
////      let fee = "0.00075 atom";
////      let signResult = try cosmosSigner.sign(handle: handle, path: BIP44.cosmos, paymentDis: nil, toDis: to, feeDis: fee)
////      Log.d(signResult)
//      let comsosInput = createCosmosInput()
//      let comsosOutput = API.cosmosSignTx(cosmosInput: comsosInput)
//      txtResult.text = "cosmos sign result:\n\(comsosOutput)"
//    } catch let e as ImkeyError {
//      Log.d("!!!error:\(e.message)")
//      toastMsg(message: e.message)
//    }catch{
//      Log.d(error)
//    }
  }
  
  @IBAction func addressBtnClick(_ sender: Any) {
    txtResult.text = ""
    do {
//      let address = try CosmosKey.getCosmosAddress(handle: handle, path: BIP44.cosmos)
      let address = API.TronAddress(path: BIP44.tron)
      Log.d(address)
      txtResult.text = address
    } catch let e as ImkeyError {
      Log.d("!!!error:\(e.message)")
      toastMsg(message: e.message)
    }catch{
      Log.d(error)
    }
  }
  
    @IBAction func reginAddressBtnClick(_ sender: Any) {
      txtResult.text = ""
      do {
  //      let address = try CosmosKey.getCosmosAddress(handle: handle, path: BIP44.cosmos)
        let address = API.cosmosReginAddress(path: BIP44.cosmos)
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
