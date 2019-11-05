//
//  LocalFileStorage.swift
//  token
//
//  Created by James Chen on 2016/09/16.
//  Copyright © 2016 imToken PTE. LTD. All rights reserved.
//

import Foundation

public final class KeyFileStorage {
  public static let defaultFileName = "keys"
  public init() {}
  
  public func cleanStorage() -> Bool {
    do {
      try FileManager.default.removeItem(at: walletsDirectory)
    } catch {
      return false
    }
    return true
  }
  
  func readFrom(_ filename: String) -> String? {
    do {
      let filePath = walletsDirectory.appendingPathComponent(filename).path
      return try String(contentsOfFile: filePath, encoding: .utf8)
    } catch {
      return nil
    }
  }
  
  func writeContent(_ content: String, to filename: String) -> Bool {
    do {
      let filePath = walletsDirectory.appendingPathComponent(filename).path
      try content.write(toFile: filePath, atomically: true, encoding: .utf8)
      return true
    } catch {
      Log.d("Error: \(error)")
      return false
    }
  }
  
  func deleteFile(_ filename: String) -> Bool {
    do {
      let filePath = walletsDirectory.appendingPathComponent(filename).path
      try FileManager.default.removeItem(atPath: filePath)
      return true
    } catch {
      return false
    }
  }
  
  var walletsDirectory: URL {
    let walletsPath = "\(NSHomeDirectory())/Documents/wallets/imkey"
    var walletsDirectory = URL(fileURLWithPath: walletsPath)
    
    do {
      if !FileManager.default.fileExists(atPath: walletsPath) {
        try FileManager.default.createDirectory(atPath: walletsDirectory.path, withIntermediateDirectories: true, attributes: nil)
        var resourceValues = URLResourceValues()
        resourceValues.isExcludedFromBackup = true
        try walletsDirectory.setResourceValues(resourceValues)
      }
    } catch let err {
      Log.d(err)
    }
    
    return walletsDirectory
  }
}
