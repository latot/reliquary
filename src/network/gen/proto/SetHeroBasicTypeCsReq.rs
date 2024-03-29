// This file is generated by rust-protobuf 3.4.0. Do not edit
// .proto file is parsed by pure
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![cfg_attr(rustfmt, rustfmt::skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_results)]
#![allow(unused_mut)]

//! Generated file from `SetHeroBasicTypeCsReq.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:SetHeroBasicTypeCsReq)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct SetHeroBasicTypeCsReq {
    // message fields
    // @@protoc_insertion_point(field:SetHeroBasicTypeCsReq.basic_type)
    pub basic_type: ::protobuf::EnumOrUnknown<super::HeroBasicType::HeroBasicType>,
    // special fields
    // @@protoc_insertion_point(special_field:SetHeroBasicTypeCsReq.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a SetHeroBasicTypeCsReq {
    fn default() -> &'a SetHeroBasicTypeCsReq {
        <SetHeroBasicTypeCsReq as ::protobuf::Message>::default_instance()
    }
}

impl SetHeroBasicTypeCsReq {
    pub fn new() -> SetHeroBasicTypeCsReq {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(1);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "basic_type",
            |m: &SetHeroBasicTypeCsReq| { &m.basic_type },
            |m: &mut SetHeroBasicTypeCsReq| { &mut m.basic_type },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<SetHeroBasicTypeCsReq>(
            "SetHeroBasicTypeCsReq",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for SetHeroBasicTypeCsReq {
    const NAME: &'static str = "SetHeroBasicTypeCsReq";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                88 => {
                    self.basic_type = is.read_enum_or_unknown()?;
                },
                tag => {
                    ::protobuf::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        if self.basic_type != ::protobuf::EnumOrUnknown::new(super::HeroBasicType::HeroBasicType::None) {
            my_size += ::protobuf::rt::int32_size(11, self.basic_type.value());
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.basic_type != ::protobuf::EnumOrUnknown::new(super::HeroBasicType::HeroBasicType::None) {
            os.write_enum(11, ::protobuf::EnumOrUnknown::value(&self.basic_type))?;
        }
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> SetHeroBasicTypeCsReq {
        SetHeroBasicTypeCsReq::new()
    }

    fn clear(&mut self) {
        self.basic_type = ::protobuf::EnumOrUnknown::new(super::HeroBasicType::HeroBasicType::None);
        self.special_fields.clear();
    }

    fn default_instance() -> &'static SetHeroBasicTypeCsReq {
        static instance: SetHeroBasicTypeCsReq = SetHeroBasicTypeCsReq {
            basic_type: ::protobuf::EnumOrUnknown::from_i32(0),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for SetHeroBasicTypeCsReq {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("SetHeroBasicTypeCsReq").unwrap()).clone()
    }
}

impl ::std::fmt::Display for SetHeroBasicTypeCsReq {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SetHeroBasicTypeCsReq {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1bSetHeroBasicTypeCsReq.proto\x1a\x13HeroBasicType.proto\"F\n\x15Set\
    HeroBasicTypeCsReq\x12-\n\nbasic_type\x18\x0b\x20\x01(\x0e2\x0e.HeroBasi\
    cTypeR\tbasicTypeB\x15\n\x13emu.lunarcore.protob\x06proto3\
";

/// `FileDescriptorProto` object which was a source for this generated file
fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    static file_descriptor_proto_lazy: ::protobuf::rt::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::Lazy::new();
    file_descriptor_proto_lazy.get(|| {
        ::protobuf::Message::parse_from_bytes(file_descriptor_proto_data).unwrap()
    })
}

/// `FileDescriptor` object which allows dynamic access to files
pub fn file_descriptor() -> &'static ::protobuf::reflect::FileDescriptor {
    static generated_file_descriptor_lazy: ::protobuf::rt::Lazy<::protobuf::reflect::GeneratedFileDescriptor> = ::protobuf::rt::Lazy::new();
    static file_descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::FileDescriptor> = ::protobuf::rt::Lazy::new();
    file_descriptor.get(|| {
        let generated_file_descriptor = generated_file_descriptor_lazy.get(|| {
            let mut deps = ::std::vec::Vec::with_capacity(1);
            deps.push(super::HeroBasicType::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(SetHeroBasicTypeCsReq::generated_message_descriptor_data());
            let mut enums = ::std::vec::Vec::with_capacity(0);
            ::protobuf::reflect::GeneratedFileDescriptor::new_generated(
                file_descriptor_proto(),
                deps,
                messages,
                enums,
            )
        });
        ::protobuf::reflect::FileDescriptor::new_generated_2(generated_file_descriptor)
    })
}
