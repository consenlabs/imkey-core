// Generated by the protocol buffer compiler.  DO NOT EDIT!
// source: tezos.proto

#ifndef PROTOBUF_tezos_2eproto__INCLUDED
#define PROTOBUF_tezos_2eproto__INCLUDED

#include <string>

#include <google/protobuf/stubs/common.h>

#if GOOGLE_PROTOBUF_VERSION < 3001000
#error This file was generated by a newer version of protoc which is
#error incompatible with your Protocol Buffer headers.  Please update
#error your headers.
#endif
#if 3001000 < GOOGLE_PROTOBUF_MIN_PROTOC_VERSION
#error This file was generated by an older version of protoc which is
#error incompatible with your Protocol Buffer headers.  Please
#error regenerate this file with a newer version of protoc.
#endif

#include <google/protobuf/arena.h>
#include <google/protobuf/arenastring.h>
#include <google/protobuf/generated_message_util.h>
#include <google/protobuf/metadata.h>
#include <google/protobuf/message.h>
#include <google/protobuf/repeated_field.h>
#include <google/protobuf/extension_set.h>
#include <google/protobuf/unknown_field_set.h>
// @@protoc_insertion_point(includes)

namespace tezosapi {

// Internal implementation detail -- do not call these.
void protobuf_AddDesc_tezos_2eproto();
void protobuf_InitDefaults_tezos_2eproto();
void protobuf_AssignDesc_tezos_2eproto();
void protobuf_ShutdownFile_tezos_2eproto();

class TezosTxInput;
class TezosTxOutput;

// ===================================================================

class TezosTxInput : public ::google::protobuf::Message /* @@protoc_insertion_point(class_definition:tezosapi.TezosTxInput) */ {
 public:
  TezosTxInput();
  virtual ~TezosTxInput();

  TezosTxInput(const TezosTxInput& from);

  inline TezosTxInput& operator=(const TezosTxInput& from) {
    CopyFrom(from);
    return *this;
  }

  static const ::google::protobuf::Descriptor* descriptor();
  static const TezosTxInput& default_instance();

  static const TezosTxInput* internal_default_instance();

  void Swap(TezosTxInput* other);

  // implements Message ----------------------------------------------

  inline TezosTxInput* New() const { return New(NULL); }

  TezosTxInput* New(::google::protobuf::Arena* arena) const;
  void CopyFrom(const ::google::protobuf::Message& from);
  void MergeFrom(const ::google::protobuf::Message& from);
  void CopyFrom(const TezosTxInput& from);
  void MergeFrom(const TezosTxInput& from);
  void Clear();
  bool IsInitialized() const;

  size_t ByteSizeLong() const;
  bool MergePartialFromCodedStream(
      ::google::protobuf::io::CodedInputStream* input);
  void SerializeWithCachedSizes(
      ::google::protobuf::io::CodedOutputStream* output) const;
  ::google::protobuf::uint8* InternalSerializeWithCachedSizesToArray(
      bool deterministic, ::google::protobuf::uint8* output) const;
  ::google::protobuf::uint8* SerializeWithCachedSizesToArray(::google::protobuf::uint8* output) const {
    return InternalSerializeWithCachedSizesToArray(false, output);
  }
  int GetCachedSize() const { return _cached_size_; }
  private:
  void SharedCtor();
  void SharedDtor();
  void SetCachedSize(int size) const;
  void InternalSwap(TezosTxInput* other);
  void UnsafeMergeFrom(const TezosTxInput& from);
  private:
  inline ::google::protobuf::Arena* GetArenaNoVirtual() const {
    return _internal_metadata_.arena();
  }
  inline void* MaybeArenaPtr() const {
    return _internal_metadata_.raw_arena_ptr();
  }
  public:

  ::google::protobuf::Metadata GetMetadata() const;

  // nested types ----------------------------------------------------

  // accessors -------------------------------------------------------

  // optional string raw_data = 1;
  void clear_raw_data();
  static const int kRawDataFieldNumber = 1;
  const ::std::string& raw_data() const;
  void set_raw_data(const ::std::string& value);
  void set_raw_data(const char* value);
  void set_raw_data(const char* value, size_t size);
  ::std::string* mutable_raw_data();
  ::std::string* release_raw_data();
  void set_allocated_raw_data(::std::string* raw_data);

  // @@protoc_insertion_point(class_scope:tezosapi.TezosTxInput)
 private:

  ::google::protobuf::internal::InternalMetadataWithArena _internal_metadata_;
  ::google::protobuf::internal::ArenaStringPtr raw_data_;
  mutable int _cached_size_;
  friend void  protobuf_InitDefaults_tezos_2eproto_impl();
  friend void  protobuf_AddDesc_tezos_2eproto_impl();
  friend void protobuf_AssignDesc_tezos_2eproto();
  friend void protobuf_ShutdownFile_tezos_2eproto();

  void InitAsDefaultInstance();
};
extern ::google::protobuf::internal::ExplicitlyConstructed<TezosTxInput> TezosTxInput_default_instance_;

// -------------------------------------------------------------------

class TezosTxOutput : public ::google::protobuf::Message /* @@protoc_insertion_point(class_definition:tezosapi.TezosTxOutput) */ {
 public:
  TezosTxOutput();
  virtual ~TezosTxOutput();

  TezosTxOutput(const TezosTxOutput& from);

  inline TezosTxOutput& operator=(const TezosTxOutput& from) {
    CopyFrom(from);
    return *this;
  }

  static const ::google::protobuf::Descriptor* descriptor();
  static const TezosTxOutput& default_instance();

  static const TezosTxOutput* internal_default_instance();

  void Swap(TezosTxOutput* other);

  // implements Message ----------------------------------------------

  inline TezosTxOutput* New() const { return New(NULL); }

  TezosTxOutput* New(::google::protobuf::Arena* arena) const;
  void CopyFrom(const ::google::protobuf::Message& from);
  void MergeFrom(const ::google::protobuf::Message& from);
  void CopyFrom(const TezosTxOutput& from);
  void MergeFrom(const TezosTxOutput& from);
  void Clear();
  bool IsInitialized() const;

  size_t ByteSizeLong() const;
  bool MergePartialFromCodedStream(
      ::google::protobuf::io::CodedInputStream* input);
  void SerializeWithCachedSizes(
      ::google::protobuf::io::CodedOutputStream* output) const;
  ::google::protobuf::uint8* InternalSerializeWithCachedSizesToArray(
      bool deterministic, ::google::protobuf::uint8* output) const;
  ::google::protobuf::uint8* SerializeWithCachedSizesToArray(::google::protobuf::uint8* output) const {
    return InternalSerializeWithCachedSizesToArray(false, output);
  }
  int GetCachedSize() const { return _cached_size_; }
  private:
  void SharedCtor();
  void SharedDtor();
  void SetCachedSize(int size) const;
  void InternalSwap(TezosTxOutput* other);
  void UnsafeMergeFrom(const TezosTxOutput& from);
  private:
  inline ::google::protobuf::Arena* GetArenaNoVirtual() const {
    return _internal_metadata_.arena();
  }
  inline void* MaybeArenaPtr() const {
    return _internal_metadata_.raw_arena_ptr();
  }
  public:

  ::google::protobuf::Metadata GetMetadata() const;

  // nested types ----------------------------------------------------

  // accessors -------------------------------------------------------

  // optional string signature = 1;
  void clear_signature();
  static const int kSignatureFieldNumber = 1;
  const ::std::string& signature() const;
  void set_signature(const ::std::string& value);
  void set_signature(const char* value);
  void set_signature(const char* value, size_t size);
  ::std::string* mutable_signature();
  ::std::string* release_signature();
  void set_allocated_signature(::std::string* signature);

  // optional string edsig = 2;
  void clear_edsig();
  static const int kEdsigFieldNumber = 2;
  const ::std::string& edsig() const;
  void set_edsig(const ::std::string& value);
  void set_edsig(const char* value);
  void set_edsig(const char* value, size_t size);
  ::std::string* mutable_edsig();
  ::std::string* release_edsig();
  void set_allocated_edsig(::std::string* edsig);

  // optional string sbytes = 3;
  void clear_sbytes();
  static const int kSbytesFieldNumber = 3;
  const ::std::string& sbytes() const;
  void set_sbytes(const ::std::string& value);
  void set_sbytes(const char* value);
  void set_sbytes(const char* value, size_t size);
  ::std::string* mutable_sbytes();
  ::std::string* release_sbytes();
  void set_allocated_sbytes(::std::string* sbytes);

  // @@protoc_insertion_point(class_scope:tezosapi.TezosTxOutput)
 private:

  ::google::protobuf::internal::InternalMetadataWithArena _internal_metadata_;
  ::google::protobuf::internal::ArenaStringPtr signature_;
  ::google::protobuf::internal::ArenaStringPtr edsig_;
  ::google::protobuf::internal::ArenaStringPtr sbytes_;
  mutable int _cached_size_;
  friend void  protobuf_InitDefaults_tezos_2eproto_impl();
  friend void  protobuf_AddDesc_tezos_2eproto_impl();
  friend void protobuf_AssignDesc_tezos_2eproto();
  friend void protobuf_ShutdownFile_tezos_2eproto();

  void InitAsDefaultInstance();
};
extern ::google::protobuf::internal::ExplicitlyConstructed<TezosTxOutput> TezosTxOutput_default_instance_;

// ===================================================================


// ===================================================================

#if !PROTOBUF_INLINE_NOT_IN_HEADERS
// TezosTxInput

// optional string raw_data = 1;
inline void TezosTxInput::clear_raw_data() {
  raw_data_.ClearToEmptyNoArena(&::google::protobuf::internal::GetEmptyStringAlreadyInited());
}
inline const ::std::string& TezosTxInput::raw_data() const {
  // @@protoc_insertion_point(field_get:tezosapi.TezosTxInput.raw_data)
  return raw_data_.GetNoArena(&::google::protobuf::internal::GetEmptyStringAlreadyInited());
}
inline void TezosTxInput::set_raw_data(const ::std::string& value) {
  
  raw_data_.SetNoArena(&::google::protobuf::internal::GetEmptyStringAlreadyInited(), value);
  // @@protoc_insertion_point(field_set:tezosapi.TezosTxInput.raw_data)
}
inline void TezosTxInput::set_raw_data(const char* value) {
  
  raw_data_.SetNoArena(&::google::protobuf::internal::GetEmptyStringAlreadyInited(), ::std::string(value));
  // @@protoc_insertion_point(field_set_char:tezosapi.TezosTxInput.raw_data)
}
inline void TezosTxInput::set_raw_data(const char* value, size_t size) {
  
  raw_data_.SetNoArena(&::google::protobuf::internal::GetEmptyStringAlreadyInited(),
      ::std::string(reinterpret_cast<const char*>(value), size));
  // @@protoc_insertion_point(field_set_pointer:tezosapi.TezosTxInput.raw_data)
}
inline ::std::string* TezosTxInput::mutable_raw_data() {
  
  // @@protoc_insertion_point(field_mutable:tezosapi.TezosTxInput.raw_data)
  return raw_data_.MutableNoArena(&::google::protobuf::internal::GetEmptyStringAlreadyInited());
}
inline ::std::string* TezosTxInput::release_raw_data() {
  // @@protoc_insertion_point(field_release:tezosapi.TezosTxInput.raw_data)
  
  return raw_data_.ReleaseNoArena(&::google::protobuf::internal::GetEmptyStringAlreadyInited());
}
inline void TezosTxInput::set_allocated_raw_data(::std::string* raw_data) {
  if (raw_data != NULL) {
    
  } else {
    
  }
  raw_data_.SetAllocatedNoArena(&::google::protobuf::internal::GetEmptyStringAlreadyInited(), raw_data);
  // @@protoc_insertion_point(field_set_allocated:tezosapi.TezosTxInput.raw_data)
}

inline const TezosTxInput* TezosTxInput::internal_default_instance() {
  return &TezosTxInput_default_instance_.get();
}
// -------------------------------------------------------------------

// TezosTxOutput

// optional string signature = 1;
inline void TezosTxOutput::clear_signature() {
  signature_.ClearToEmptyNoArena(&::google::protobuf::internal::GetEmptyStringAlreadyInited());
}
inline const ::std::string& TezosTxOutput::signature() const {
  // @@protoc_insertion_point(field_get:tezosapi.TezosTxOutput.signature)
  return signature_.GetNoArena(&::google::protobuf::internal::GetEmptyStringAlreadyInited());
}
inline void TezosTxOutput::set_signature(const ::std::string& value) {
  
  signature_.SetNoArena(&::google::protobuf::internal::GetEmptyStringAlreadyInited(), value);
  // @@protoc_insertion_point(field_set:tezosapi.TezosTxOutput.signature)
}
inline void TezosTxOutput::set_signature(const char* value) {
  
  signature_.SetNoArena(&::google::protobuf::internal::GetEmptyStringAlreadyInited(), ::std::string(value));
  // @@protoc_insertion_point(field_set_char:tezosapi.TezosTxOutput.signature)
}
inline void TezosTxOutput::set_signature(const char* value, size_t size) {
  
  signature_.SetNoArena(&::google::protobuf::internal::GetEmptyStringAlreadyInited(),
      ::std::string(reinterpret_cast<const char*>(value), size));
  // @@protoc_insertion_point(field_set_pointer:tezosapi.TezosTxOutput.signature)
}
inline ::std::string* TezosTxOutput::mutable_signature() {
  
  // @@protoc_insertion_point(field_mutable:tezosapi.TezosTxOutput.signature)
  return signature_.MutableNoArena(&::google::protobuf::internal::GetEmptyStringAlreadyInited());
}
inline ::std::string* TezosTxOutput::release_signature() {
  // @@protoc_insertion_point(field_release:tezosapi.TezosTxOutput.signature)
  
  return signature_.ReleaseNoArena(&::google::protobuf::internal::GetEmptyStringAlreadyInited());
}
inline void TezosTxOutput::set_allocated_signature(::std::string* signature) {
  if (signature != NULL) {
    
  } else {
    
  }
  signature_.SetAllocatedNoArena(&::google::protobuf::internal::GetEmptyStringAlreadyInited(), signature);
  // @@protoc_insertion_point(field_set_allocated:tezosapi.TezosTxOutput.signature)
}

// optional string edsig = 2;
inline void TezosTxOutput::clear_edsig() {
  edsig_.ClearToEmptyNoArena(&::google::protobuf::internal::GetEmptyStringAlreadyInited());
}
inline const ::std::string& TezosTxOutput::edsig() const {
  // @@protoc_insertion_point(field_get:tezosapi.TezosTxOutput.edsig)
  return edsig_.GetNoArena(&::google::protobuf::internal::GetEmptyStringAlreadyInited());
}
inline void TezosTxOutput::set_edsig(const ::std::string& value) {
  
  edsig_.SetNoArena(&::google::protobuf::internal::GetEmptyStringAlreadyInited(), value);
  // @@protoc_insertion_point(field_set:tezosapi.TezosTxOutput.edsig)
}
inline void TezosTxOutput::set_edsig(const char* value) {
  
  edsig_.SetNoArena(&::google::protobuf::internal::GetEmptyStringAlreadyInited(), ::std::string(value));
  // @@protoc_insertion_point(field_set_char:tezosapi.TezosTxOutput.edsig)
}
inline void TezosTxOutput::set_edsig(const char* value, size_t size) {
  
  edsig_.SetNoArena(&::google::protobuf::internal::GetEmptyStringAlreadyInited(),
      ::std::string(reinterpret_cast<const char*>(value), size));
  // @@protoc_insertion_point(field_set_pointer:tezosapi.TezosTxOutput.edsig)
}
inline ::std::string* TezosTxOutput::mutable_edsig() {
  
  // @@protoc_insertion_point(field_mutable:tezosapi.TezosTxOutput.edsig)
  return edsig_.MutableNoArena(&::google::protobuf::internal::GetEmptyStringAlreadyInited());
}
inline ::std::string* TezosTxOutput::release_edsig() {
  // @@protoc_insertion_point(field_release:tezosapi.TezosTxOutput.edsig)
  
  return edsig_.ReleaseNoArena(&::google::protobuf::internal::GetEmptyStringAlreadyInited());
}
inline void TezosTxOutput::set_allocated_edsig(::std::string* edsig) {
  if (edsig != NULL) {
    
  } else {
    
  }
  edsig_.SetAllocatedNoArena(&::google::protobuf::internal::GetEmptyStringAlreadyInited(), edsig);
  // @@protoc_insertion_point(field_set_allocated:tezosapi.TezosTxOutput.edsig)
}

// optional string sbytes = 3;
inline void TezosTxOutput::clear_sbytes() {
  sbytes_.ClearToEmptyNoArena(&::google::protobuf::internal::GetEmptyStringAlreadyInited());
}
inline const ::std::string& TezosTxOutput::sbytes() const {
  // @@protoc_insertion_point(field_get:tezosapi.TezosTxOutput.sbytes)
  return sbytes_.GetNoArena(&::google::protobuf::internal::GetEmptyStringAlreadyInited());
}
inline void TezosTxOutput::set_sbytes(const ::std::string& value) {
  
  sbytes_.SetNoArena(&::google::protobuf::internal::GetEmptyStringAlreadyInited(), value);
  // @@protoc_insertion_point(field_set:tezosapi.TezosTxOutput.sbytes)
}
inline void TezosTxOutput::set_sbytes(const char* value) {
  
  sbytes_.SetNoArena(&::google::protobuf::internal::GetEmptyStringAlreadyInited(), ::std::string(value));
  // @@protoc_insertion_point(field_set_char:tezosapi.TezosTxOutput.sbytes)
}
inline void TezosTxOutput::set_sbytes(const char* value, size_t size) {
  
  sbytes_.SetNoArena(&::google::protobuf::internal::GetEmptyStringAlreadyInited(),
      ::std::string(reinterpret_cast<const char*>(value), size));
  // @@protoc_insertion_point(field_set_pointer:tezosapi.TezosTxOutput.sbytes)
}
inline ::std::string* TezosTxOutput::mutable_sbytes() {
  
  // @@protoc_insertion_point(field_mutable:tezosapi.TezosTxOutput.sbytes)
  return sbytes_.MutableNoArena(&::google::protobuf::internal::GetEmptyStringAlreadyInited());
}
inline ::std::string* TezosTxOutput::release_sbytes() {
  // @@protoc_insertion_point(field_release:tezosapi.TezosTxOutput.sbytes)
  
  return sbytes_.ReleaseNoArena(&::google::protobuf::internal::GetEmptyStringAlreadyInited());
}
inline void TezosTxOutput::set_allocated_sbytes(::std::string* sbytes) {
  if (sbytes != NULL) {
    
  } else {
    
  }
  sbytes_.SetAllocatedNoArena(&::google::protobuf::internal::GetEmptyStringAlreadyInited(), sbytes);
  // @@protoc_insertion_point(field_set_allocated:tezosapi.TezosTxOutput.sbytes)
}

inline const TezosTxOutput* TezosTxOutput::internal_default_instance() {
  return &TezosTxOutput_default_instance_.get();
}
#endif  // !PROTOBUF_INLINE_NOT_IN_HEADERS
// -------------------------------------------------------------------


// @@protoc_insertion_point(namespace_scope)

}  // namespace tezosapi

// @@protoc_insertion_point(global_scope)

#endif  // PROTOBUF_tezos_2eproto__INCLUDED