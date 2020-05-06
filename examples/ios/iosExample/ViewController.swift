//
//  ViewController.swift
//  iosExample
//
//  Created by joe on 4/17/20.
//  Copyright © 2020 joe. All rights reserved.
//

import UIKit
import imKeyConnector


class ViewController: UIViewController,BLEDelegate {
  @IBOutlet weak var tbDevices: UITableView!
  @IBOutlet weak var tvDeviceInfo: UITextView!
  @IBOutlet weak var indictorDeviceInfo: UIActivityIndicatorView!
  
  override func viewDidLoad() {
    super.viewDidLoad()
    
    let initRes = BLE.shared().initialize()
    Log.d("initRes \(initRes)")
    BLE.shared().setDelegate(bleDelegate: self)
    tbDevices.dataSource = self
    tbDevices.delegate = self
    
    self.hideKeyboardWhenTappedAround()
    
    API.setCallback()
  }
  
  // bind ble devices
  struct Device {
    var name:String = ""
    var address:String = ""
  }
  
  var devices:[Device] = [Device]()
  var currentDevice:Device? = nil
  
  func deviceDidFind(deviceName: String!, address: String!) {
    Log.d("deviceName:\(String(describing: deviceName)) address:\(String(describing: address))")
    var device = Device.init()
    device.name = deviceName
    device.address = address
    let isContain = devices.contains(where: { (value) -> Bool in
      return value.address == device.address
    })
    if !isContain {
      devices.append(device)
    }
    DispatchQueue.main.sync() {
      tbDevices.reloadData()
    }
  }
  
  func deviceDidConnect(address: String!, errorCode: Int) {
    Log.d("deviceDidConnect.. \(String(describing: address)) \(errorCode)")
  }
  
  func deviceDidDisconnect(address: String!, errorCode: Int) {
    Log.d("deviceDidDisconnect.. \(String(describing: address)) \(errorCode)")
  }
  
  
  @IBAction func scan(_ sender: Any) {
    let res:Int = BLE.shared().startScan()
    if res != 0 {
      toastMsg(message: "搜索设备失败 \(res)")
    }
  }
  
  @IBAction func stopScan(_ sender: Any) {
    let res = BLE.shared().stopScan()
    if res != 0 {
      toastMsg(message: "停止失败 \(res)")
    }
    devices.removeAll()
    tbDevices.reloadData()
  }
  
  @IBAction func disconnect(_ sender: Any) {
    let res = BLE.shared().disConnect()
    if(res != 0){
      let err = DeviceError(rawValue: Int64(res))!
      toastMsg(message: "断开连接失败\(err.message)")
    }
  }
  
  @IBAction func bindCheck(_ sender: Any) {
    let status = API.bindCheck()
    print("status:\(status)")
    let bindResult = API.bindAcquire(bindCode: "YDSGQPKX")
    print("bind result:\(bindResult)")
  }
  
  @IBAction func deviceManageClick(_ sender: Any) {
    let storyboard = UIStoryboard(name: "Main", bundle: nil)
    guard let deviceManageVC = storyboard.instantiateViewController(withIdentifier: "DeviceManageViewController") as? DeviceManageViewController else {  return }
//    deviceManageVC.handle= handle
    self.present(deviceManageVC, animated: true, completion: nil)
  }
}



extension UIViewController{
  func toastMsg(message:String){
    toast(title: "提示", message: message)
  }
  
  func toast(title:String,message:String){
    let alertToast = UIAlertController(title: title, message: message, preferredStyle: .alert)
    DispatchQueue.main.async {
      self.present(alertToast, animated: true, completion: nil)
    }
    
    //一秒钟后自动消失
    
    DispatchQueue.main.asyncAfter(deadline: DispatchTime.now() + 1.5) {
      
      alertToast.dismiss(animated: false, completion: nil)
    }
  }
}

extension ViewController:UITableViewDataSource,UITableViewDelegate{
  func tableView(_ tableView: UITableView, numberOfRowsInSection section: Int) -> Int {
    return devices.count
  }
  
  func tableView(_ tableView: UITableView, cellForRowAt indexPath: IndexPath) -> UITableViewCell {
    let cell = tbDevices.dequeueReusableCell(withIdentifier: "NormalCell", for: indexPath)
    cell.textLabel?.text = devices[indexPath.row].name
    return cell
  }
  
  func tableView(_ tableView: UITableView, didSelectRowAt indexPath: IndexPath) {
    Log.d("select \(indexPath)")
    let device = devices[indexPath.row]
    BLE.shared().stopScan()
    do {
      //      handle = 0
      let result = try BLE.shared().connect(address: device.address, timeout: 12*1000)
      let err = DeviceError(rawValue: Int64(result))!
      Log.d("connect result:\(err.message)")
      if(result == 0){
        currentDevice = device
        tvDeviceInfo.text = ""
        tvDeviceInfo.text += "蓝牙名称：" + device.name
        tvDeviceInfo.text += "\n蓝牙地址：" + device.address
      }else{
        toastMsg(message: "connect fail \(err.message)")
      }
    }catch let e as ImkeyError {
      Log.d("!!!error:\(e.message)")
      toastMsg(message: e.message)
    }catch{
      Log.d(error)
    }
  }
}

// Put this piece of code anywhere you like
extension UIViewController {
  func hideKeyboardWhenTappedAround() {
    let tap: UITapGestureRecognizer = UITapGestureRecognizer(target: self, action: #selector(UIViewController.dismissKeyboard))
    tap.cancelsTouchesInView = false
    view.addGestureRecognizer(tap)
  }
  
  @objc func dismissKeyboard() {
    view.endEditing(true)
  }
}

