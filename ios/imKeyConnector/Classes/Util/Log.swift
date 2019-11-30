//
//  File.swift
//  imKeyConnector
//
//  Created by joe on 1/24/19.
//

import Foundation
public class Log{
  public class func d(_ items:Any){
    #if DEBUG
    print(items)
    #endif
  }
  
  class func makeCString(from str: String) -> UnsafeMutablePointer<Int8> {
      let count = str.utf8CString.count
      let result: UnsafeMutableBufferPointer<Int8> = UnsafeMutableBufferPointer<Int8>.allocate(capacity: count)
      // func initialize<S>(from: S) -> (S.Iterator, UnsafeMutableBufferPointer<Element>.Index)
      _ = result.initialize(from: str.utf8CString)
      return result.baseAddress!
  }
  
  public class func test_ffi(){
//    let swiftCallback : @convention(c) (UnsafePointer<Int8>?) -> UnsafePointer<Int8>? = {
//      (apdu) -> UnsafePointer<Int8>? in
//      print("callback miaomiao v v")
//      let swiftApdu = String(cString:apdu!)
//      let r = try! BLE.shared().sendApdu(handle: 0, apdu: swiftApdu)
//      return apdu
//    }
//
//    let result = rust_hello("world",swiftCallback)
//    let swift_result = String(cString: result!)
//    rust_hello_free(UnsafeMutablePointer(mutating: result))
//    print(swift_result)
    
    let swiftCallback2 : @convention(c) (UnsafePointer<Int8>?) -> UnsafePointer<Int8>? = {
      (apdu) -> UnsafePointer<Int8>? in
      print("callback miaomiao v v")
      let swiftApdu = String(cString:apdu!)
      let resApdu = try! BLE.shared().sendApdu(handle: 0, apdu: swiftApdu)
      let count = resApdu.utf8CString.count
      let result: UnsafeMutableBufferPointer<Int8> = UnsafeMutableBufferPointer<Int8>.allocate(capacity: count)
      _ = result.initialize(from: resApdu.utf8CString)
      let p = UnsafePointer(result.baseAddress!)
      return p
    }
    
    let result2 = get_se_id(swiftCallback2)
    let swift_result2 = String(cString: result2!)
    rust_hello_free(UnsafeMutablePointer(mutating: result2))
    print(swift_result2)
  }
}
