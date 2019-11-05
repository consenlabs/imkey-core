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
  
  public class func test_ffi(){
    let result = rust_hello("world")
    let swift_result = String(cString: result!)
    rust_hello_free(UnsafeMutablePointer(mutating: result))
    print(swift_result)
  }
}
