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
    do {
      var input = Tronapi_TronTxReq()
      input.path = BIP44.tron
      input.rawData = "0a0208312208b02efdc02638b61e40f083c3a7c92d5a65080112610a2d747970652e676f6f676c65617069732e636f6d2f70726f746f636f6c2e5472616e73666572436f6e747261637412300a1541a1e81654258bf14f63feb2e8d1380075d45b0dac1215410b3e84ec677b3e63c99affcadb91a6b4e086798f186470a0bfbfa7c92d"
      input.address = "TY2uroBeZ5trA9QT96aEWj32XLkAAhQ9R2"
      input.payment = "100 TRX"
      input.to = "TDQqJsFsStSy5fjG52KuiWW7HhJGAKGJLb"
      let output = API.tronSignTx(input: input)
      txtResult.text = "tron sign result:\n\(output)"
    } catch let e as ImkeyError {
      Log.d("!!!error:\(e.message)")
      toastMsg(message: e.message)
    }catch{
      Log.d(error)
    }
  }
  
  @IBAction func signMessageClick(_ sender: Any) {
    txtResult.text = ""
    do {
      var input = Tronapi_TronMessageSignReq()
      input.path = BIP44.tron
      input.message = "0a0208312208b02efdc02638b61e40f083c3a7c92d5a65080112610a2d747970652e676f6f676c65617069732e636f6d2f70726f746f636f6c2e5472616e73666572436f6e747261637412300a1541a1e81654258bf14f63feb2e8d1380075d45b0dac1215410b3e84ec677b3e63c99affcadb91a6b4e086798f186470a0bfbfa7c92d"
      input.address = "TY2uroBeZ5trA9QT96aEWj32XLkAAhQ9R2"
      let comsosOutput = API.tronSignMessage(input: input)
      txtResult.text = "tron sign message result:\n\(comsosOutput)"
    } catch let e as ImkeyError {
      Log.d("!!!error:\(e.message)")
      toastMsg(message: e.message)
    }catch{
      Log.d(error)
    }
  }
  
  @IBAction func addressBtnClick(_ sender: Any) {
    txtResult.text = ""
    do {
//      let address = try CosmosKey.getCosmosAddress(handle: handle, path: BIP44.cosmos)
      let address = API.tronAddress(path: BIP44.tron)
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
        let address = API.tronReginAddress(path: BIP44.tron)
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
