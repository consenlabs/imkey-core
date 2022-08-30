// Generated by the protocol buffer compiler.  DO NOT EDIT!
// source: tezos.proto

#define INTERNAL_SUPPRESS_PROTOBUF_FIELD_DEPRECATION
#include "tezos.pb.h"

#include <algorithm>

#include <google/protobuf/stubs/common.h>
#include <google/protobuf/stubs/port.h>
#include <google/protobuf/stubs/once.h>
#include <google/protobuf/io/coded_stream.h>
#include <google/protobuf/wire_format_lite_inl.h>
#include <google/protobuf/descriptor.h>
#include <google/protobuf/generated_message_reflection.h>
#include <google/protobuf/reflection_ops.h>
#include <google/protobuf/wire_format.h>
// @@protoc_insertion_point(includes)

namespace tezosapi {

namespace {

const ::google::protobuf::Descriptor* TezosTxInput_descriptor_ = NULL;
const ::google::protobuf::internal::GeneratedMessageReflection*
  TezosTxInput_reflection_ = NULL;
const ::google::protobuf::Descriptor* TezosTxOutput_descriptor_ = NULL;
const ::google::protobuf::internal::GeneratedMessageReflection*
  TezosTxOutput_reflection_ = NULL;

}  // namespace


void protobuf_AssignDesc_tezos_2eproto() GOOGLE_ATTRIBUTE_COLD;
void protobuf_AssignDesc_tezos_2eproto() {
  protobuf_AddDesc_tezos_2eproto();
  const ::google::protobuf::FileDescriptor* file =
    ::google::protobuf::DescriptorPool::generated_pool()->FindFileByName(
      "tezos.proto");
  GOOGLE_CHECK(file != NULL);
  TezosTxInput_descriptor_ = file->message_type(0);
  static const int TezosTxInput_offsets_[1] = {
    GOOGLE_PROTOBUF_GENERATED_MESSAGE_FIELD_OFFSET(TezosTxInput, raw_data_),
  };
  TezosTxInput_reflection_ =
    ::google::protobuf::internal::GeneratedMessageReflection::NewGeneratedMessageReflection(
      TezosTxInput_descriptor_,
      TezosTxInput::internal_default_instance(),
      TezosTxInput_offsets_,
      -1,
      -1,
      -1,
      sizeof(TezosTxInput),
      GOOGLE_PROTOBUF_GENERATED_MESSAGE_FIELD_OFFSET(TezosTxInput, _internal_metadata_));
  TezosTxOutput_descriptor_ = file->message_type(1);
  static const int TezosTxOutput_offsets_[3] = {
    GOOGLE_PROTOBUF_GENERATED_MESSAGE_FIELD_OFFSET(TezosTxOutput, signature_),
    GOOGLE_PROTOBUF_GENERATED_MESSAGE_FIELD_OFFSET(TezosTxOutput, edsig_),
    GOOGLE_PROTOBUF_GENERATED_MESSAGE_FIELD_OFFSET(TezosTxOutput, sbytes_),
  };
  TezosTxOutput_reflection_ =
    ::google::protobuf::internal::GeneratedMessageReflection::NewGeneratedMessageReflection(
      TezosTxOutput_descriptor_,
      TezosTxOutput::internal_default_instance(),
      TezosTxOutput_offsets_,
      -1,
      -1,
      -1,
      sizeof(TezosTxOutput),
      GOOGLE_PROTOBUF_GENERATED_MESSAGE_FIELD_OFFSET(TezosTxOutput, _internal_metadata_));
}

namespace {

GOOGLE_PROTOBUF_DECLARE_ONCE(protobuf_AssignDescriptors_once_);
void protobuf_AssignDescriptorsOnce() {
  ::google::protobuf::GoogleOnceInit(&protobuf_AssignDescriptors_once_,
                 &protobuf_AssignDesc_tezos_2eproto);
}

void protobuf_RegisterTypes(const ::std::string&) GOOGLE_ATTRIBUTE_COLD;
void protobuf_RegisterTypes(const ::std::string&) {
  protobuf_AssignDescriptorsOnce();
  ::google::protobuf::MessageFactory::InternalRegisterGeneratedMessage(
      TezosTxInput_descriptor_, TezosTxInput::internal_default_instance());
  ::google::protobuf::MessageFactory::InternalRegisterGeneratedMessage(
      TezosTxOutput_descriptor_, TezosTxOutput::internal_default_instance());
}

}  // namespace

void protobuf_ShutdownFile_tezos_2eproto() {
  TezosTxInput_default_instance_.Shutdown();
  delete TezosTxInput_reflection_;
  TezosTxOutput_default_instance_.Shutdown();
  delete TezosTxOutput_reflection_;
}

void protobuf_InitDefaults_tezos_2eproto_impl() {
  GOOGLE_PROTOBUF_VERIFY_VERSION;

  ::google::protobuf::internal::GetEmptyString();
  TezosTxInput_default_instance_.DefaultConstruct();
  ::google::protobuf::internal::GetEmptyString();
  TezosTxOutput_default_instance_.DefaultConstruct();
  TezosTxInput_default_instance_.get_mutable()->InitAsDefaultInstance();
  TezosTxOutput_default_instance_.get_mutable()->InitAsDefaultInstance();
}

GOOGLE_PROTOBUF_DECLARE_ONCE(protobuf_InitDefaults_tezos_2eproto_once_);
void protobuf_InitDefaults_tezos_2eproto() {
  ::google::protobuf::GoogleOnceInit(&protobuf_InitDefaults_tezos_2eproto_once_,
                 &protobuf_InitDefaults_tezos_2eproto_impl);
}
void protobuf_AddDesc_tezos_2eproto_impl() {
  GOOGLE_PROTOBUF_VERIFY_VERSION;

  protobuf_InitDefaults_tezos_2eproto();
  ::google::protobuf::DescriptorPool::InternalAddGeneratedFile(
    "\n\013tezos.proto\022\010tezosapi\" \n\014TezosTxInput\022"
    "\020\n\010raw_data\030\001 \001(\t\"A\n\rTezosTxOutput\022\021\n\tsi"
    "gnature\030\001 \001(\t\022\r\n\005edsig\030\002 \001(\t\022\016\n\006sbytes\030\003"
    " \001(\tb\006proto3", 132);
  ::google::protobuf::MessageFactory::InternalRegisterGeneratedFile(
    "tezos.proto", &protobuf_RegisterTypes);
  ::google::protobuf::internal::OnShutdown(&protobuf_ShutdownFile_tezos_2eproto);
}

GOOGLE_PROTOBUF_DECLARE_ONCE(protobuf_AddDesc_tezos_2eproto_once_);
void protobuf_AddDesc_tezos_2eproto() {
  ::google::protobuf::GoogleOnceInit(&protobuf_AddDesc_tezos_2eproto_once_,
                 &protobuf_AddDesc_tezos_2eproto_impl);
}
// Force AddDescriptors() to be called at static initialization time.
struct StaticDescriptorInitializer_tezos_2eproto {
  StaticDescriptorInitializer_tezos_2eproto() {
    protobuf_AddDesc_tezos_2eproto();
  }
} static_descriptor_initializer_tezos_2eproto_;

namespace {

static void MergeFromFail(int line) GOOGLE_ATTRIBUTE_COLD GOOGLE_ATTRIBUTE_NORETURN;
static void MergeFromFail(int line) {
  ::google::protobuf::internal::MergeFromFail(__FILE__, line);
}

}  // namespace


// ===================================================================

#if !defined(_MSC_VER) || _MSC_VER >= 1900
const int TezosTxInput::kRawDataFieldNumber;
#endif  // !defined(_MSC_VER) || _MSC_VER >= 1900

TezosTxInput::TezosTxInput()
  : ::google::protobuf::Message(), _internal_metadata_(NULL) {
  if (this != internal_default_instance()) protobuf_InitDefaults_tezos_2eproto();
  SharedCtor();
  // @@protoc_insertion_point(constructor:tezosapi.TezosTxInput)
}

void TezosTxInput::InitAsDefaultInstance() {
}

TezosTxInput::TezosTxInput(const TezosTxInput& from)
  : ::google::protobuf::Message(),
    _internal_metadata_(NULL) {
  SharedCtor();
  UnsafeMergeFrom(from);
  // @@protoc_insertion_point(copy_constructor:tezosapi.TezosTxInput)
}

void TezosTxInput::SharedCtor() {
  raw_data_.UnsafeSetDefault(&::google::protobuf::internal::GetEmptyStringAlreadyInited());
  _cached_size_ = 0;
}

TezosTxInput::~TezosTxInput() {
  // @@protoc_insertion_point(destructor:tezosapi.TezosTxInput)
  SharedDtor();
}

void TezosTxInput::SharedDtor() {
  raw_data_.DestroyNoArena(&::google::protobuf::internal::GetEmptyStringAlreadyInited());
}

void TezosTxInput::SetCachedSize(int size) const {
  GOOGLE_SAFE_CONCURRENT_WRITES_BEGIN();
  _cached_size_ = size;
  GOOGLE_SAFE_CONCURRENT_WRITES_END();
}
const ::google::protobuf::Descriptor* TezosTxInput::descriptor() {
  protobuf_AssignDescriptorsOnce();
  return TezosTxInput_descriptor_;
}

const TezosTxInput& TezosTxInput::default_instance() {
  protobuf_InitDefaults_tezos_2eproto();
  return *internal_default_instance();
}

::google::protobuf::internal::ExplicitlyConstructed<TezosTxInput> TezosTxInput_default_instance_;

TezosTxInput* TezosTxInput::New(::google::protobuf::Arena* arena) const {
  TezosTxInput* n = new TezosTxInput;
  if (arena != NULL) {
    arena->Own(n);
  }
  return n;
}

void TezosTxInput::Clear() {
// @@protoc_insertion_point(message_clear_start:tezosapi.TezosTxInput)
  raw_data_.ClearToEmptyNoArena(&::google::protobuf::internal::GetEmptyStringAlreadyInited());
}

bool TezosTxInput::MergePartialFromCodedStream(
    ::google::protobuf::io::CodedInputStream* input) {
#define DO_(EXPRESSION) if (!GOOGLE_PREDICT_TRUE(EXPRESSION)) goto failure
  ::google::protobuf::uint32 tag;
  // @@protoc_insertion_point(parse_start:tezosapi.TezosTxInput)
  for (;;) {
    ::std::pair< ::google::protobuf::uint32, bool> p = input->ReadTagWithCutoff(127);
    tag = p.first;
    if (!p.second) goto handle_unusual;
    switch (::google::protobuf::internal::WireFormatLite::GetTagFieldNumber(tag)) {
      // optional string raw_data = 1;
      case 1: {
        if (tag == 10) {
          DO_(::google::protobuf::internal::WireFormatLite::ReadString(
                input, this->mutable_raw_data()));
          DO_(::google::protobuf::internal::WireFormatLite::VerifyUtf8String(
            this->raw_data().data(), this->raw_data().length(),
            ::google::protobuf::internal::WireFormatLite::PARSE,
            "tezosapi.TezosTxInput.raw_data"));
        } else {
          goto handle_unusual;
        }
        if (input->ExpectAtEnd()) goto success;
        break;
      }

      default: {
      handle_unusual:
        if (tag == 0 ||
            ::google::protobuf::internal::WireFormatLite::GetTagWireType(tag) ==
            ::google::protobuf::internal::WireFormatLite::WIRETYPE_END_GROUP) {
          goto success;
        }
        DO_(::google::protobuf::internal::WireFormatLite::SkipField(input, tag));
        break;
      }
    }
  }
success:
  // @@protoc_insertion_point(parse_success:tezosapi.TezosTxInput)
  return true;
failure:
  // @@protoc_insertion_point(parse_failure:tezosapi.TezosTxInput)
  return false;
#undef DO_
}

void TezosTxInput::SerializeWithCachedSizes(
    ::google::protobuf::io::CodedOutputStream* output) const {
  // @@protoc_insertion_point(serialize_start:tezosapi.TezosTxInput)
  // optional string raw_data = 1;
  if (this->raw_data().size() > 0) {
    ::google::protobuf::internal::WireFormatLite::VerifyUtf8String(
      this->raw_data().data(), this->raw_data().length(),
      ::google::protobuf::internal::WireFormatLite::SERIALIZE,
      "tezosapi.TezosTxInput.raw_data");
    ::google::protobuf::internal::WireFormatLite::WriteStringMaybeAliased(
      1, this->raw_data(), output);
  }

  // @@protoc_insertion_point(serialize_end:tezosapi.TezosTxInput)
}

::google::protobuf::uint8* TezosTxInput::InternalSerializeWithCachedSizesToArray(
    bool deterministic, ::google::protobuf::uint8* target) const {
  (void)deterministic; // Unused
  // @@protoc_insertion_point(serialize_to_array_start:tezosapi.TezosTxInput)
  // optional string raw_data = 1;
  if (this->raw_data().size() > 0) {
    ::google::protobuf::internal::WireFormatLite::VerifyUtf8String(
      this->raw_data().data(), this->raw_data().length(),
      ::google::protobuf::internal::WireFormatLite::SERIALIZE,
      "tezosapi.TezosTxInput.raw_data");
    target =
      ::google::protobuf::internal::WireFormatLite::WriteStringToArray(
        1, this->raw_data(), target);
  }

  // @@protoc_insertion_point(serialize_to_array_end:tezosapi.TezosTxInput)
  return target;
}

size_t TezosTxInput::ByteSizeLong() const {
// @@protoc_insertion_point(message_byte_size_start:tezosapi.TezosTxInput)
  size_t total_size = 0;

  // optional string raw_data = 1;
  if (this->raw_data().size() > 0) {
    total_size += 1 +
      ::google::protobuf::internal::WireFormatLite::StringSize(
        this->raw_data());
  }

  int cached_size = ::google::protobuf::internal::ToCachedSize(total_size);
  GOOGLE_SAFE_CONCURRENT_WRITES_BEGIN();
  _cached_size_ = cached_size;
  GOOGLE_SAFE_CONCURRENT_WRITES_END();
  return total_size;
}

void TezosTxInput::MergeFrom(const ::google::protobuf::Message& from) {
// @@protoc_insertion_point(generalized_merge_from_start:tezosapi.TezosTxInput)
  if (GOOGLE_PREDICT_FALSE(&from == this)) MergeFromFail(__LINE__);
  const TezosTxInput* source =
      ::google::protobuf::internal::DynamicCastToGenerated<const TezosTxInput>(
          &from);
  if (source == NULL) {
  // @@protoc_insertion_point(generalized_merge_from_cast_fail:tezosapi.TezosTxInput)
    ::google::protobuf::internal::ReflectionOps::Merge(from, this);
  } else {
  // @@protoc_insertion_point(generalized_merge_from_cast_success:tezosapi.TezosTxInput)
    UnsafeMergeFrom(*source);
  }
}

void TezosTxInput::MergeFrom(const TezosTxInput& from) {
// @@protoc_insertion_point(class_specific_merge_from_start:tezosapi.TezosTxInput)
  if (GOOGLE_PREDICT_TRUE(&from != this)) {
    UnsafeMergeFrom(from);
  } else {
    MergeFromFail(__LINE__);
  }
}

void TezosTxInput::UnsafeMergeFrom(const TezosTxInput& from) {
  GOOGLE_DCHECK(&from != this);
  if (from.raw_data().size() > 0) {

    raw_data_.AssignWithDefault(&::google::protobuf::internal::GetEmptyStringAlreadyInited(), from.raw_data_);
  }
}

void TezosTxInput::CopyFrom(const ::google::protobuf::Message& from) {
// @@protoc_insertion_point(generalized_copy_from_start:tezosapi.TezosTxInput)
  if (&from == this) return;
  Clear();
  MergeFrom(from);
}

void TezosTxInput::CopyFrom(const TezosTxInput& from) {
// @@protoc_insertion_point(class_specific_copy_from_start:tezosapi.TezosTxInput)
  if (&from == this) return;
  Clear();
  UnsafeMergeFrom(from);
}

bool TezosTxInput::IsInitialized() const {

  return true;
}

void TezosTxInput::Swap(TezosTxInput* other) {
  if (other == this) return;
  InternalSwap(other);
}
void TezosTxInput::InternalSwap(TezosTxInput* other) {
  raw_data_.Swap(&other->raw_data_);
  _internal_metadata_.Swap(&other->_internal_metadata_);
  std::swap(_cached_size_, other->_cached_size_);
}

::google::protobuf::Metadata TezosTxInput::GetMetadata() const {
  protobuf_AssignDescriptorsOnce();
  ::google::protobuf::Metadata metadata;
  metadata.descriptor = TezosTxInput_descriptor_;
  metadata.reflection = TezosTxInput_reflection_;
  return metadata;
}

#if PROTOBUF_INLINE_NOT_IN_HEADERS
// TezosTxInput

// optional string raw_data = 1;
void TezosTxInput::clear_raw_data() {
  raw_data_.ClearToEmptyNoArena(&::google::protobuf::internal::GetEmptyStringAlreadyInited());
}
const ::std::string& TezosTxInput::raw_data() const {
  // @@protoc_insertion_point(field_get:tezosapi.TezosTxInput.raw_data)
  return raw_data_.GetNoArena(&::google::protobuf::internal::GetEmptyStringAlreadyInited());
}
void TezosTxInput::set_raw_data(const ::std::string& value) {
  
  raw_data_.SetNoArena(&::google::protobuf::internal::GetEmptyStringAlreadyInited(), value);
  // @@protoc_insertion_point(field_set:tezosapi.TezosTxInput.raw_data)
}
void TezosTxInput::set_raw_data(const char* value) {
  
  raw_data_.SetNoArena(&::google::protobuf::internal::GetEmptyStringAlreadyInited(), ::std::string(value));
  // @@protoc_insertion_point(field_set_char:tezosapi.TezosTxInput.raw_data)
}
void TezosTxInput::set_raw_data(const char* value, size_t size) {
  
  raw_data_.SetNoArena(&::google::protobuf::internal::GetEmptyStringAlreadyInited(),
      ::std::string(reinterpret_cast<const char*>(value), size));
  // @@protoc_insertion_point(field_set_pointer:tezosapi.TezosTxInput.raw_data)
}
::std::string* TezosTxInput::mutable_raw_data() {
  
  // @@protoc_insertion_point(field_mutable:tezosapi.TezosTxInput.raw_data)
  return raw_data_.MutableNoArena(&::google::protobuf::internal::GetEmptyStringAlreadyInited());
}
::std::string* TezosTxInput::release_raw_data() {
  // @@protoc_insertion_point(field_release:tezosapi.TezosTxInput.raw_data)
  
  return raw_data_.ReleaseNoArena(&::google::protobuf::internal::GetEmptyStringAlreadyInited());
}
void TezosTxInput::set_allocated_raw_data(::std::string* raw_data) {
  if (raw_data != NULL) {
    
  } else {
    
  }
  raw_data_.SetAllocatedNoArena(&::google::protobuf::internal::GetEmptyStringAlreadyInited(), raw_data);
  // @@protoc_insertion_point(field_set_allocated:tezosapi.TezosTxInput.raw_data)
}

inline const TezosTxInput* TezosTxInput::internal_default_instance() {
  return &TezosTxInput_default_instance_.get();
}
#endif  // PROTOBUF_INLINE_NOT_IN_HEADERS

// ===================================================================

#if !defined(_MSC_VER) || _MSC_VER >= 1900
const int TezosTxOutput::kSignatureFieldNumber;
const int TezosTxOutput::kEdsigFieldNumber;
const int TezosTxOutput::kSbytesFieldNumber;
#endif  // !defined(_MSC_VER) || _MSC_VER >= 1900

TezosTxOutput::TezosTxOutput()
  : ::google::protobuf::Message(), _internal_metadata_(NULL) {
  if (this != internal_default_instance()) protobuf_InitDefaults_tezos_2eproto();
  SharedCtor();
  // @@protoc_insertion_point(constructor:tezosapi.TezosTxOutput)
}

void TezosTxOutput::InitAsDefaultInstance() {
}

TezosTxOutput::TezosTxOutput(const TezosTxOutput& from)
  : ::google::protobuf::Message(),
    _internal_metadata_(NULL) {
  SharedCtor();
  UnsafeMergeFrom(from);
  // @@protoc_insertion_point(copy_constructor:tezosapi.TezosTxOutput)
}

void TezosTxOutput::SharedCtor() {
  signature_.UnsafeSetDefault(&::google::protobuf::internal::GetEmptyStringAlreadyInited());
  edsig_.UnsafeSetDefault(&::google::protobuf::internal::GetEmptyStringAlreadyInited());
  sbytes_.UnsafeSetDefault(&::google::protobuf::internal::GetEmptyStringAlreadyInited());
  _cached_size_ = 0;
}

TezosTxOutput::~TezosTxOutput() {
  // @@protoc_insertion_point(destructor:tezosapi.TezosTxOutput)
  SharedDtor();
}

void TezosTxOutput::SharedDtor() {
  signature_.DestroyNoArena(&::google::protobuf::internal::GetEmptyStringAlreadyInited());
  edsig_.DestroyNoArena(&::google::protobuf::internal::GetEmptyStringAlreadyInited());
  sbytes_.DestroyNoArena(&::google::protobuf::internal::GetEmptyStringAlreadyInited());
}

void TezosTxOutput::SetCachedSize(int size) const {
  GOOGLE_SAFE_CONCURRENT_WRITES_BEGIN();
  _cached_size_ = size;
  GOOGLE_SAFE_CONCURRENT_WRITES_END();
}
const ::google::protobuf::Descriptor* TezosTxOutput::descriptor() {
  protobuf_AssignDescriptorsOnce();
  return TezosTxOutput_descriptor_;
}

const TezosTxOutput& TezosTxOutput::default_instance() {
  protobuf_InitDefaults_tezos_2eproto();
  return *internal_default_instance();
}

::google::protobuf::internal::ExplicitlyConstructed<TezosTxOutput> TezosTxOutput_default_instance_;

TezosTxOutput* TezosTxOutput::New(::google::protobuf::Arena* arena) const {
  TezosTxOutput* n = new TezosTxOutput;
  if (arena != NULL) {
    arena->Own(n);
  }
  return n;
}

void TezosTxOutput::Clear() {
// @@protoc_insertion_point(message_clear_start:tezosapi.TezosTxOutput)
  signature_.ClearToEmptyNoArena(&::google::protobuf::internal::GetEmptyStringAlreadyInited());
  edsig_.ClearToEmptyNoArena(&::google::protobuf::internal::GetEmptyStringAlreadyInited());
  sbytes_.ClearToEmptyNoArena(&::google::protobuf::internal::GetEmptyStringAlreadyInited());
}

bool TezosTxOutput::MergePartialFromCodedStream(
    ::google::protobuf::io::CodedInputStream* input) {
#define DO_(EXPRESSION) if (!GOOGLE_PREDICT_TRUE(EXPRESSION)) goto failure
  ::google::protobuf::uint32 tag;
  // @@protoc_insertion_point(parse_start:tezosapi.TezosTxOutput)
  for (;;) {
    ::std::pair< ::google::protobuf::uint32, bool> p = input->ReadTagWithCutoff(127);
    tag = p.first;
    if (!p.second) goto handle_unusual;
    switch (::google::protobuf::internal::WireFormatLite::GetTagFieldNumber(tag)) {
      // optional string signature = 1;
      case 1: {
        if (tag == 10) {
          DO_(::google::protobuf::internal::WireFormatLite::ReadString(
                input, this->mutable_signature()));
          DO_(::google::protobuf::internal::WireFormatLite::VerifyUtf8String(
            this->signature().data(), this->signature().length(),
            ::google::protobuf::internal::WireFormatLite::PARSE,
            "tezosapi.TezosTxOutput.signature"));
        } else {
          goto handle_unusual;
        }
        if (input->ExpectTag(18)) goto parse_edsig;
        break;
      }

      // optional string edsig = 2;
      case 2: {
        if (tag == 18) {
         parse_edsig:
          DO_(::google::protobuf::internal::WireFormatLite::ReadString(
                input, this->mutable_edsig()));
          DO_(::google::protobuf::internal::WireFormatLite::VerifyUtf8String(
            this->edsig().data(), this->edsig().length(),
            ::google::protobuf::internal::WireFormatLite::PARSE,
            "tezosapi.TezosTxOutput.edsig"));
        } else {
          goto handle_unusual;
        }
        if (input->ExpectTag(26)) goto parse_sbytes;
        break;
      }

      // optional string sbytes = 3;
      case 3: {
        if (tag == 26) {
         parse_sbytes:
          DO_(::google::protobuf::internal::WireFormatLite::ReadString(
                input, this->mutable_sbytes()));
          DO_(::google::protobuf::internal::WireFormatLite::VerifyUtf8String(
            this->sbytes().data(), this->sbytes().length(),
            ::google::protobuf::internal::WireFormatLite::PARSE,
            "tezosapi.TezosTxOutput.sbytes"));
        } else {
          goto handle_unusual;
        }
        if (input->ExpectAtEnd()) goto success;
        break;
      }

      default: {
      handle_unusual:
        if (tag == 0 ||
            ::google::protobuf::internal::WireFormatLite::GetTagWireType(tag) ==
            ::google::protobuf::internal::WireFormatLite::WIRETYPE_END_GROUP) {
          goto success;
        }
        DO_(::google::protobuf::internal::WireFormatLite::SkipField(input, tag));
        break;
      }
    }
  }
success:
  // @@protoc_insertion_point(parse_success:tezosapi.TezosTxOutput)
  return true;
failure:
  // @@protoc_insertion_point(parse_failure:tezosapi.TezosTxOutput)
  return false;
#undef DO_
}

void TezosTxOutput::SerializeWithCachedSizes(
    ::google::protobuf::io::CodedOutputStream* output) const {
  // @@protoc_insertion_point(serialize_start:tezosapi.TezosTxOutput)
  // optional string signature = 1;
  if (this->signature().size() > 0) {
    ::google::protobuf::internal::WireFormatLite::VerifyUtf8String(
      this->signature().data(), this->signature().length(),
      ::google::protobuf::internal::WireFormatLite::SERIALIZE,
      "tezosapi.TezosTxOutput.signature");
    ::google::protobuf::internal::WireFormatLite::WriteStringMaybeAliased(
      1, this->signature(), output);
  }

  // optional string edsig = 2;
  if (this->edsig().size() > 0) {
    ::google::protobuf::internal::WireFormatLite::VerifyUtf8String(
      this->edsig().data(), this->edsig().length(),
      ::google::protobuf::internal::WireFormatLite::SERIALIZE,
      "tezosapi.TezosTxOutput.edsig");
    ::google::protobuf::internal::WireFormatLite::WriteStringMaybeAliased(
      2, this->edsig(), output);
  }

  // optional string sbytes = 3;
  if (this->sbytes().size() > 0) {
    ::google::protobuf::internal::WireFormatLite::VerifyUtf8String(
      this->sbytes().data(), this->sbytes().length(),
      ::google::protobuf::internal::WireFormatLite::SERIALIZE,
      "tezosapi.TezosTxOutput.sbytes");
    ::google::protobuf::internal::WireFormatLite::WriteStringMaybeAliased(
      3, this->sbytes(), output);
  }

  // @@protoc_insertion_point(serialize_end:tezosapi.TezosTxOutput)
}

::google::protobuf::uint8* TezosTxOutput::InternalSerializeWithCachedSizesToArray(
    bool deterministic, ::google::protobuf::uint8* target) const {
  (void)deterministic; // Unused
  // @@protoc_insertion_point(serialize_to_array_start:tezosapi.TezosTxOutput)
  // optional string signature = 1;
  if (this->signature().size() > 0) {
    ::google::protobuf::internal::WireFormatLite::VerifyUtf8String(
      this->signature().data(), this->signature().length(),
      ::google::protobuf::internal::WireFormatLite::SERIALIZE,
      "tezosapi.TezosTxOutput.signature");
    target =
      ::google::protobuf::internal::WireFormatLite::WriteStringToArray(
        1, this->signature(), target);
  }

  // optional string edsig = 2;
  if (this->edsig().size() > 0) {
    ::google::protobuf::internal::WireFormatLite::VerifyUtf8String(
      this->edsig().data(), this->edsig().length(),
      ::google::protobuf::internal::WireFormatLite::SERIALIZE,
      "tezosapi.TezosTxOutput.edsig");
    target =
      ::google::protobuf::internal::WireFormatLite::WriteStringToArray(
        2, this->edsig(), target);
  }

  // optional string sbytes = 3;
  if (this->sbytes().size() > 0) {
    ::google::protobuf::internal::WireFormatLite::VerifyUtf8String(
      this->sbytes().data(), this->sbytes().length(),
      ::google::protobuf::internal::WireFormatLite::SERIALIZE,
      "tezosapi.TezosTxOutput.sbytes");
    target =
      ::google::protobuf::internal::WireFormatLite::WriteStringToArray(
        3, this->sbytes(), target);
  }

  // @@protoc_insertion_point(serialize_to_array_end:tezosapi.TezosTxOutput)
  return target;
}

size_t TezosTxOutput::ByteSizeLong() const {
// @@protoc_insertion_point(message_byte_size_start:tezosapi.TezosTxOutput)
  size_t total_size = 0;

  // optional string signature = 1;
  if (this->signature().size() > 0) {
    total_size += 1 +
      ::google::protobuf::internal::WireFormatLite::StringSize(
        this->signature());
  }

  // optional string edsig = 2;
  if (this->edsig().size() > 0) {
    total_size += 1 +
      ::google::protobuf::internal::WireFormatLite::StringSize(
        this->edsig());
  }

  // optional string sbytes = 3;
  if (this->sbytes().size() > 0) {
    total_size += 1 +
      ::google::protobuf::internal::WireFormatLite::StringSize(
        this->sbytes());
  }

  int cached_size = ::google::protobuf::internal::ToCachedSize(total_size);
  GOOGLE_SAFE_CONCURRENT_WRITES_BEGIN();
  _cached_size_ = cached_size;
  GOOGLE_SAFE_CONCURRENT_WRITES_END();
  return total_size;
}

void TezosTxOutput::MergeFrom(const ::google::protobuf::Message& from) {
// @@protoc_insertion_point(generalized_merge_from_start:tezosapi.TezosTxOutput)
  if (GOOGLE_PREDICT_FALSE(&from == this)) MergeFromFail(__LINE__);
  const TezosTxOutput* source =
      ::google::protobuf::internal::DynamicCastToGenerated<const TezosTxOutput>(
          &from);
  if (source == NULL) {
  // @@protoc_insertion_point(generalized_merge_from_cast_fail:tezosapi.TezosTxOutput)
    ::google::protobuf::internal::ReflectionOps::Merge(from, this);
  } else {
  // @@protoc_insertion_point(generalized_merge_from_cast_success:tezosapi.TezosTxOutput)
    UnsafeMergeFrom(*source);
  }
}

void TezosTxOutput::MergeFrom(const TezosTxOutput& from) {
// @@protoc_insertion_point(class_specific_merge_from_start:tezosapi.TezosTxOutput)
  if (GOOGLE_PREDICT_TRUE(&from != this)) {
    UnsafeMergeFrom(from);
  } else {
    MergeFromFail(__LINE__);
  }
}

void TezosTxOutput::UnsafeMergeFrom(const TezosTxOutput& from) {
  GOOGLE_DCHECK(&from != this);
  if (from.signature().size() > 0) {

    signature_.AssignWithDefault(&::google::protobuf::internal::GetEmptyStringAlreadyInited(), from.signature_);
  }
  if (from.edsig().size() > 0) {

    edsig_.AssignWithDefault(&::google::protobuf::internal::GetEmptyStringAlreadyInited(), from.edsig_);
  }
  if (from.sbytes().size() > 0) {

    sbytes_.AssignWithDefault(&::google::protobuf::internal::GetEmptyStringAlreadyInited(), from.sbytes_);
  }
}

void TezosTxOutput::CopyFrom(const ::google::protobuf::Message& from) {
// @@protoc_insertion_point(generalized_copy_from_start:tezosapi.TezosTxOutput)
  if (&from == this) return;
  Clear();
  MergeFrom(from);
}

void TezosTxOutput::CopyFrom(const TezosTxOutput& from) {
// @@protoc_insertion_point(class_specific_copy_from_start:tezosapi.TezosTxOutput)
  if (&from == this) return;
  Clear();
  UnsafeMergeFrom(from);
}

bool TezosTxOutput::IsInitialized() const {

  return true;
}

void TezosTxOutput::Swap(TezosTxOutput* other) {
  if (other == this) return;
  InternalSwap(other);
}
void TezosTxOutput::InternalSwap(TezosTxOutput* other) {
  signature_.Swap(&other->signature_);
  edsig_.Swap(&other->edsig_);
  sbytes_.Swap(&other->sbytes_);
  _internal_metadata_.Swap(&other->_internal_metadata_);
  std::swap(_cached_size_, other->_cached_size_);
}

::google::protobuf::Metadata TezosTxOutput::GetMetadata() const {
  protobuf_AssignDescriptorsOnce();
  ::google::protobuf::Metadata metadata;
  metadata.descriptor = TezosTxOutput_descriptor_;
  metadata.reflection = TezosTxOutput_reflection_;
  return metadata;
}

#if PROTOBUF_INLINE_NOT_IN_HEADERS
// TezosTxOutput

// optional string signature = 1;
void TezosTxOutput::clear_signature() {
  signature_.ClearToEmptyNoArena(&::google::protobuf::internal::GetEmptyStringAlreadyInited());
}
const ::std::string& TezosTxOutput::signature() const {
  // @@protoc_insertion_point(field_get:tezosapi.TezosTxOutput.signature)
  return signature_.GetNoArena(&::google::protobuf::internal::GetEmptyStringAlreadyInited());
}
void TezosTxOutput::set_signature(const ::std::string& value) {
  
  signature_.SetNoArena(&::google::protobuf::internal::GetEmptyStringAlreadyInited(), value);
  // @@protoc_insertion_point(field_set:tezosapi.TezosTxOutput.signature)
}
void TezosTxOutput::set_signature(const char* value) {
  
  signature_.SetNoArena(&::google::protobuf::internal::GetEmptyStringAlreadyInited(), ::std::string(value));
  // @@protoc_insertion_point(field_set_char:tezosapi.TezosTxOutput.signature)
}
void TezosTxOutput::set_signature(const char* value, size_t size) {
  
  signature_.SetNoArena(&::google::protobuf::internal::GetEmptyStringAlreadyInited(),
      ::std::string(reinterpret_cast<const char*>(value), size));
  // @@protoc_insertion_point(field_set_pointer:tezosapi.TezosTxOutput.signature)
}
::std::string* TezosTxOutput::mutable_signature() {
  
  // @@protoc_insertion_point(field_mutable:tezosapi.TezosTxOutput.signature)
  return signature_.MutableNoArena(&::google::protobuf::internal::GetEmptyStringAlreadyInited());
}
::std::string* TezosTxOutput::release_signature() {
  // @@protoc_insertion_point(field_release:tezosapi.TezosTxOutput.signature)
  
  return signature_.ReleaseNoArena(&::google::protobuf::internal::GetEmptyStringAlreadyInited());
}
void TezosTxOutput::set_allocated_signature(::std::string* signature) {
  if (signature != NULL) {
    
  } else {
    
  }
  signature_.SetAllocatedNoArena(&::google::protobuf::internal::GetEmptyStringAlreadyInited(), signature);
  // @@protoc_insertion_point(field_set_allocated:tezosapi.TezosTxOutput.signature)
}

// optional string edsig = 2;
void TezosTxOutput::clear_edsig() {
  edsig_.ClearToEmptyNoArena(&::google::protobuf::internal::GetEmptyStringAlreadyInited());
}
const ::std::string& TezosTxOutput::edsig() const {
  // @@protoc_insertion_point(field_get:tezosapi.TezosTxOutput.edsig)
  return edsig_.GetNoArena(&::google::protobuf::internal::GetEmptyStringAlreadyInited());
}
void TezosTxOutput::set_edsig(const ::std::string& value) {
  
  edsig_.SetNoArena(&::google::protobuf::internal::GetEmptyStringAlreadyInited(), value);
  // @@protoc_insertion_point(field_set:tezosapi.TezosTxOutput.edsig)
}
void TezosTxOutput::set_edsig(const char* value) {
  
  edsig_.SetNoArena(&::google::protobuf::internal::GetEmptyStringAlreadyInited(), ::std::string(value));
  // @@protoc_insertion_point(field_set_char:tezosapi.TezosTxOutput.edsig)
}
void TezosTxOutput::set_edsig(const char* value, size_t size) {
  
  edsig_.SetNoArena(&::google::protobuf::internal::GetEmptyStringAlreadyInited(),
      ::std::string(reinterpret_cast<const char*>(value), size));
  // @@protoc_insertion_point(field_set_pointer:tezosapi.TezosTxOutput.edsig)
}
::std::string* TezosTxOutput::mutable_edsig() {
  
  // @@protoc_insertion_point(field_mutable:tezosapi.TezosTxOutput.edsig)
  return edsig_.MutableNoArena(&::google::protobuf::internal::GetEmptyStringAlreadyInited());
}
::std::string* TezosTxOutput::release_edsig() {
  // @@protoc_insertion_point(field_release:tezosapi.TezosTxOutput.edsig)
  
  return edsig_.ReleaseNoArena(&::google::protobuf::internal::GetEmptyStringAlreadyInited());
}
void TezosTxOutput::set_allocated_edsig(::std::string* edsig) {
  if (edsig != NULL) {
    
  } else {
    
  }
  edsig_.SetAllocatedNoArena(&::google::protobuf::internal::GetEmptyStringAlreadyInited(), edsig);
  // @@protoc_insertion_point(field_set_allocated:tezosapi.TezosTxOutput.edsig)
}

// optional string sbytes = 3;
void TezosTxOutput::clear_sbytes() {
  sbytes_.ClearToEmptyNoArena(&::google::protobuf::internal::GetEmptyStringAlreadyInited());
}
const ::std::string& TezosTxOutput::sbytes() const {
  // @@protoc_insertion_point(field_get:tezosapi.TezosTxOutput.sbytes)
  return sbytes_.GetNoArena(&::google::protobuf::internal::GetEmptyStringAlreadyInited());
}
void TezosTxOutput::set_sbytes(const ::std::string& value) {
  
  sbytes_.SetNoArena(&::google::protobuf::internal::GetEmptyStringAlreadyInited(), value);
  // @@protoc_insertion_point(field_set:tezosapi.TezosTxOutput.sbytes)
}
void TezosTxOutput::set_sbytes(const char* value) {
  
  sbytes_.SetNoArena(&::google::protobuf::internal::GetEmptyStringAlreadyInited(), ::std::string(value));
  // @@protoc_insertion_point(field_set_char:tezosapi.TezosTxOutput.sbytes)
}
void TezosTxOutput::set_sbytes(const char* value, size_t size) {
  
  sbytes_.SetNoArena(&::google::protobuf::internal::GetEmptyStringAlreadyInited(),
      ::std::string(reinterpret_cast<const char*>(value), size));
  // @@protoc_insertion_point(field_set_pointer:tezosapi.TezosTxOutput.sbytes)
}
::std::string* TezosTxOutput::mutable_sbytes() {
  
  // @@protoc_insertion_point(field_mutable:tezosapi.TezosTxOutput.sbytes)
  return sbytes_.MutableNoArena(&::google::protobuf::internal::GetEmptyStringAlreadyInited());
}
::std::string* TezosTxOutput::release_sbytes() {
  // @@protoc_insertion_point(field_release:tezosapi.TezosTxOutput.sbytes)
  
  return sbytes_.ReleaseNoArena(&::google::protobuf::internal::GetEmptyStringAlreadyInited());
}
void TezosTxOutput::set_allocated_sbytes(::std::string* sbytes) {
  if (sbytes != NULL) {
    
  } else {
    
  }
  sbytes_.SetAllocatedNoArena(&::google::protobuf::internal::GetEmptyStringAlreadyInited(), sbytes);
  // @@protoc_insertion_point(field_set_allocated:tezosapi.TezosTxOutput.sbytes)
}

inline const TezosTxOutput* TezosTxOutput::internal_default_instance() {
  return &TezosTxOutput_default_instance_.get();
}
#endif  // PROTOBUF_INLINE_NOT_IN_HEADERS

// @@protoc_insertion_point(namespace_scope)

}  // namespace tezosapi

// @@protoc_insertion_point(global_scope)