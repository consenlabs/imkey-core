// DO NOT EDIT.
//
// Generated by the Swift generator plugin for the protocol buffer compiler.
// Source: api.proto
//
// For information on using the generated types, please see the documentation:
//   https://github.com/apple/swift-protobuf/

import Foundation
import SwiftProtobuf

// If the compiler emits an error on this type, it is because this file
// was generated by a version of the `protoc` Swift plug-in that is
// incompatible with the version of SwiftProtobuf to which you are linking.
// Please ensure that your are building against the same version of the API
// that was used to generate this file.
fileprivate struct _GeneratedWithProtocGenSwiftVersion: SwiftProtobuf.ProtobufAPIVersionCheck {
  struct _2: SwiftProtobuf.ProtobufAPIVersion_2 {}
  typealias Version = _2
}

/// Action Wrapper
/// There is a `call_imkey_api` method in tcx which act as a endpoint like RPC. It accepts a `ImkeyAction` param which method field is
/// the real action and param field is the real param of that method.
/// When an error occurred, the `call_imkey_api` will return a `Response` which isSuccess field be false and error field is the reason
/// which cause the error.
public struct Api_ImkeyAction {
  // SwiftProtobuf.Message conformance is added in an extension below. See the
  // `Message` and `Message+*Additions` files in the SwiftProtobuf library for
  // methods supported on all messages.

  public var method: String {
    get {return _storage._method}
    set {_uniqueStorage()._method = newValue}
  }

  public var param: SwiftProtobuf.Google_Protobuf_Any {
    get {return _storage._param ?? SwiftProtobuf.Google_Protobuf_Any()}
    set {_uniqueStorage()._param = newValue}
  }
  /// Returns true if `param` has been explicitly set.
  public var hasParam: Bool {return _storage._param != nil}
  /// Clears the value of `param`. Subsequent reads from it will return its default value.
  public mutating func clearParam() {_uniqueStorage()._param = nil}

  public var unknownFields = SwiftProtobuf.UnknownStorage()

  public init() {}

  fileprivate var _storage = _StorageClass.defaultInstance
}

/// A common response when error occurred.
public struct Api_Response {
  // SwiftProtobuf.Message conformance is added in an extension below. See the
  // `Message` and `Message+*Additions` files in the SwiftProtobuf library for
  // methods supported on all messages.

  public var isSuccess: Bool = false

  public var error: String = String()

  public var unknownFields = SwiftProtobuf.UnknownStorage()

  public init() {}
}

// MARK: - Code below here is support for the SwiftProtobuf runtime.

fileprivate let _protobuf_package = "api"

extension Api_ImkeyAction: SwiftProtobuf.Message, SwiftProtobuf._MessageImplementationBase, SwiftProtobuf._ProtoNameProviding {
  public static let protoMessageName: String = _protobuf_package + ".ImkeyAction"
  public static let _protobuf_nameMap: SwiftProtobuf._NameMap = [
    1: .same(proto: "method"),
    2: .same(proto: "param"),
  ]

  fileprivate class _StorageClass {
    var _method: String = String()
    var _param: SwiftProtobuf.Google_Protobuf_Any? = nil

    static let defaultInstance = _StorageClass()

    private init() {}

    init(copying source: _StorageClass) {
      _method = source._method
      _param = source._param
    }
  }

  fileprivate mutating func _uniqueStorage() -> _StorageClass {
    if !isKnownUniquelyReferenced(&_storage) {
      _storage = _StorageClass(copying: _storage)
    }
    return _storage
  }

  public mutating func decodeMessage<D: SwiftProtobuf.Decoder>(decoder: inout D) throws {
    _ = _uniqueStorage()
    try withExtendedLifetime(_storage) { (_storage: _StorageClass) in
      while let fieldNumber = try decoder.nextFieldNumber() {
        switch fieldNumber {
        case 1: try decoder.decodeSingularStringField(value: &_storage._method)
        case 2: try decoder.decodeSingularMessageField(value: &_storage._param)
        default: break
        }
      }
    }
  }

  public func traverse<V: SwiftProtobuf.Visitor>(visitor: inout V) throws {
    try withExtendedLifetime(_storage) { (_storage: _StorageClass) in
      if !_storage._method.isEmpty {
        try visitor.visitSingularStringField(value: _storage._method, fieldNumber: 1)
      }
      if let v = _storage._param {
        try visitor.visitSingularMessageField(value: v, fieldNumber: 2)
      }
    }
    try unknownFields.traverse(visitor: &visitor)
  }

  public static func ==(lhs: Api_ImkeyAction, rhs: Api_ImkeyAction) -> Bool {
    if lhs._storage !== rhs._storage {
      let storagesAreEqual: Bool = withExtendedLifetime((lhs._storage, rhs._storage)) { (_args: (_StorageClass, _StorageClass)) in
        let _storage = _args.0
        let rhs_storage = _args.1
        if _storage._method != rhs_storage._method {return false}
        if _storage._param != rhs_storage._param {return false}
        return true
      }
      if !storagesAreEqual {return false}
    }
    if lhs.unknownFields != rhs.unknownFields {return false}
    return true
  }
}

extension Api_Response: SwiftProtobuf.Message, SwiftProtobuf._MessageImplementationBase, SwiftProtobuf._ProtoNameProviding {
  public static let protoMessageName: String = _protobuf_package + ".Response"
  public static let _protobuf_nameMap: SwiftProtobuf._NameMap = [
    1: .same(proto: "isSuccess"),
    2: .same(proto: "error"),
  ]

  public mutating func decodeMessage<D: SwiftProtobuf.Decoder>(decoder: inout D) throws {
    while let fieldNumber = try decoder.nextFieldNumber() {
      switch fieldNumber {
      case 1: try decoder.decodeSingularBoolField(value: &self.isSuccess)
      case 2: try decoder.decodeSingularStringField(value: &self.error)
      default: break
      }
    }
  }

  public func traverse<V: SwiftProtobuf.Visitor>(visitor: inout V) throws {
    if self.isSuccess != false {
      try visitor.visitSingularBoolField(value: self.isSuccess, fieldNumber: 1)
    }
    if !self.error.isEmpty {
      try visitor.visitSingularStringField(value: self.error, fieldNumber: 2)
    }
    try unknownFields.traverse(visitor: &visitor)
  }

  public static func ==(lhs: Api_Response, rhs: Api_Response) -> Bool {
    if lhs.isSuccess != rhs.isSuccess {return false}
    if lhs.error != rhs.error {return false}
    if lhs.unknownFields != rhs.unknownFields {return false}
    return true
  }
}