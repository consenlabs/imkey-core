//
//  HTTP.swift
//  ImkeyLibrary
//
//  Created by joe on 2018/9/16.
//  Copyright © 2018年 joe. All rights reserved.
//

import Foundation
import Security

public class HTTP{
  
  public class func syncRequest(action:String,from data:Data)throws -> String {
    let semaphore = DispatchSemaphore(value: 0)
    var res:String = ""
    let url = URL(string: Constants.host + action)!
    var request = URLRequest(url: url)
    request.httpMethod = "POST"
    request.setValue("application/json", forHTTPHeaderField: "Content-Type")
    Log.d("\nhttp >>>> \(url)")
    let reqStr = String(data: data, encoding: .utf8)!
    Log.d("http >>>> \(reqStr)")
    var exception:ImkeyError? = nil
    
    let session = URLSession(
      configuration: URLSessionConfiguration.ephemeral,
      delegate: nil,
      delegateQueue: nil)
//    let session = URLSession.shared //for test

    let task = session.uploadTask(with: request, from: data) { data, response, error in
      if let error = error {
        Log.d ("error: \(error)")
        exception = TSMError.networkError
        semaphore.signal()
        return
      }
      
      guard let response = response as? HTTPURLResponse else{
        exception = TSMError.networkError
        semaphore.signal()
        return
      }
      
      if(500...599).contains(response.statusCode){
        exception = TSMError.internalServerError
        semaphore.signal()
        return
      }
      guard response.statusCode == 200 else {
        exception = SDKError.unknownError
        return
      }
      if let mimeType = response.mimeType,
        mimeType == "application/json",
        let data = data,
        let datString = String(data: data, encoding: .utf8) {
        Log.d ("http <<<< \(datString)")
        res = datString
        semaphore.signal()
      }else{
        exception = SDKError.unknownError
      }
    }
    task.resume()
    _ = semaphore.wait(timeout: DispatchTime.distantFuture)
    if let e = exception{
      throw e
    }
    return res
  }
}


class URLSessionPinningDelegate: NSObject, URLSessionDelegate {
  
  func urlSession(_ session: URLSession, didReceive challenge: URLAuthenticationChallenge, completionHandler: @escaping (URLSession.AuthChallengeDisposition, URLCredential?) -> Swift.Void) {
    
    if (challenge.protectionSpace.authenticationMethod == NSURLAuthenticationMethodServerTrust) {
      if let serverTrust = challenge.protectionSpace.serverTrust {
        var secresult = SecTrustResultType.invalid
        let status = SecTrustEvaluate(serverTrust, &secresult)
        
        if(errSecSuccess == status) {
          Log.d(SecTrustGetCertificateCount(serverTrust))
          if let serverCertificate = SecTrustGetCertificateAtIndex(serverTrust, 0) {
            
            let serverCertificateData:NSData = SecCertificateCopyData(serverCertificate)
            let certHashData = (serverCertificateData as Data).sha256().base64EncodedData()
            
            let certHash = String(bytes: certHashData, encoding: .ascii)
            Log.d("certHash:" + certHash!)
            if (certHash == Constants.pinnedCertificateHash) {
              // Success! This is our server
              completionHandler(.useCredential, URLCredential(trust:serverTrust))
              return
            }
            
          }
        }
      }
    }
    
    // Pinning failed
    completionHandler(.cancelAuthenticationChallenge, nil)
  }
}
