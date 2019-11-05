//
//  CosmosKey.swift
//  imKeyConnector
//
//  Created by joe on 5/25/19.
//

import Foundation

public class CosmosKey {
  
  public class func getCosmosXPub(handle:UInt,path:String,select:Bool = true,verifyKey:Bool = true)throws -> String{
    if select == true {
      try Wallet.selectApplet(handle: handle, aid: Applet.cosmosAID)
    }
    let apdu = APDU.cosmosXpub(path: path, verifyKey: verifyKey)
    let res = try BLE.shared().sendApdu(handle: handle, apdu: apdu)
    try APDU.checkResponse(res: res)
    if verifyKey{
      try Wallet.verifyDerSig(xpubHex: res.key_substring(to: res.count - 4))
    }
    return res
  }
  
  public class func getCosmosAddress(handle:UInt,path:String)throws -> String{
    let xPub = try getCosmosXPub(handle: handle, path: path)
    let compresPK = SigUtil.getPubKeyComp(xPub: xPub)
    return Bech32Address.pubToBech32Address(compresPK)
  }
  
  public class func displayCosmosAddress(handle: UInt,path:String)throws ->String{
    let address = try getCosmosAddress(handle: handle, path: path)
    let apdu = APDU.setCosmosAddress(pubkey: address)
    let res = try BLE.shared().sendApdu(handle: handle, apdu: apdu)
    try APDU.checkResponse(res: res)
    return address
  }
}
