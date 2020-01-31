//
//  Constants.swift
//  imKeyConnector
//
//  Created by joe on 1/24/19.
//

public struct Constants {
  public static let sdkVersion = "1.2.10"
  
  #if false
  //test environment
  public static let host = "https://imkeyserver.com:10443/imkey/"
  public static let pinnedCertificateHash = "PmHzekO6Q7O4fM1eXXq4g9bNkVVks2arM2UVkXNqvuQ="
  #else
  //prod environment
  public static let host = "https://imkey.online:1000/imkey/"
  public static let pinnedCertificateHash = "JRCN3S5togsp2X1wyfSOxxWlxAxt3/TG9B2uhUIG5UA="
  #endif

  //tsm request
  public static let seActivate = "seActivate"
  public static let seSecureCheck = "seSecureCheck"
  public static let seInfoQuery = "seInfoQuery"
  public static let deviceCertCheck = "deviceCertCheck"
  public static let appDelete = "appDelete"
  public static let appDownload = "appDownload"
  public static let appUpdate = "appUpdate"
  public static let authCodeStorage = "authCodeStorage"

  //encrypt
  public static let rsaPublicKey = """
MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEAxmJ6bwSFsz3cHKfgYsZO
iEETO5JGpB9A0HZ7rkTqsu9FPQCP+we42f380hiCSH7MTakzyX5JQkKto84CxaBR
iapJQQ53GmboEA5Dyxr2zGELWe5OuyNv84xirXsdEd+9TgVNGeM0k5GjH16JynIS
krc4ApV0XYlozFwtIjrGdQuwrKJ3c2h+nNdgZeR/QvSuAFRZvOV0a9dgZGpb0Rm6
NGmpNfSOuJjLq3LLOUw/7J5BY16ulUEHoXrHuMYyHY8XVa05FanSOY2yaKP2Qs7p
y+n4Ls1a1k6+3d5mYB3CuJHi/t33La9if6j6FvfGQNtmG+Fdy0J02VdtmNvrIMJT
CQIDAQAB
"""

  //send apdu timeout
  public static let sendAPDUTimeout:UInt32 = 20 * 1000
  public static let sendSignPreAPDUTimeout:UInt32 = 120 * 1000
  
  public static let maxUtxoNumber = 252
  
  public static let maxOPReturnSize = 80
}
