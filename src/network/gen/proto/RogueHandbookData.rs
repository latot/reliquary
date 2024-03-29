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

//! Generated file from `RogueHandbookData.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:RogueHandbookData)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct RogueHandbookData {
    // message fields
    // @@protoc_insertion_point(field:RogueHandbookData.rogue_event)
    pub rogue_event: ::std::vec::Vec<super::RogueHandbookEvent::RogueHandbookEvent>,
    // @@protoc_insertion_point(field:RogueHandbookData.buff_list)
    pub buff_list: ::std::vec::Vec<super::RogueHandbookBuff::RogueHandbookBuff>,
    // @@protoc_insertion_point(field:RogueHandbookData.rogue_aeon_list)
    pub rogue_aeon_list: ::std::vec::Vec<super::RogueHandbookAeon::RogueHandbookAeon>,
    // @@protoc_insertion_point(field:RogueHandbookData.miracle_list)
    pub miracle_list: ::std::vec::Vec<super::RogueHandbookMiracle::RogueHandbookMiracle>,
    // special fields
    // @@protoc_insertion_point(special_field:RogueHandbookData.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a RogueHandbookData {
    fn default() -> &'a RogueHandbookData {
        <RogueHandbookData as ::protobuf::Message>::default_instance()
    }
}

impl RogueHandbookData {
    pub fn new() -> RogueHandbookData {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "rogue_event",
            |m: &RogueHandbookData| { &m.rogue_event },
            |m: &mut RogueHandbookData| { &mut m.rogue_event },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "buff_list",
            |m: &RogueHandbookData| { &m.buff_list },
            |m: &mut RogueHandbookData| { &mut m.buff_list },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "rogue_aeon_list",
            |m: &RogueHandbookData| { &m.rogue_aeon_list },
            |m: &mut RogueHandbookData| { &mut m.rogue_aeon_list },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "miracle_list",
            |m: &RogueHandbookData| { &m.miracle_list },
            |m: &mut RogueHandbookData| { &mut m.miracle_list },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<RogueHandbookData>(
            "RogueHandbookData",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for RogueHandbookData {
    const NAME: &'static str = "RogueHandbookData";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                98 => {
                    self.rogue_event.push(is.read_message()?);
                },
                82 => {
                    self.buff_list.push(is.read_message()?);
                },
                122 => {
                    self.rogue_aeon_list.push(is.read_message()?);
                },
                42 => {
                    self.miracle_list.push(is.read_message()?);
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
        for value in &self.rogue_event {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        for value in &self.buff_list {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        for value in &self.rogue_aeon_list {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        for value in &self.miracle_list {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        for v in &self.rogue_event {
            ::protobuf::rt::write_message_field_with_cached_size(12, v, os)?;
        };
        for v in &self.buff_list {
            ::protobuf::rt::write_message_field_with_cached_size(10, v, os)?;
        };
        for v in &self.rogue_aeon_list {
            ::protobuf::rt::write_message_field_with_cached_size(15, v, os)?;
        };
        for v in &self.miracle_list {
            ::protobuf::rt::write_message_field_with_cached_size(5, v, os)?;
        };
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> RogueHandbookData {
        RogueHandbookData::new()
    }

    fn clear(&mut self) {
        self.rogue_event.clear();
        self.buff_list.clear();
        self.rogue_aeon_list.clear();
        self.miracle_list.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static RogueHandbookData {
        static instance: RogueHandbookData = RogueHandbookData {
            rogue_event: ::std::vec::Vec::new(),
            buff_list: ::std::vec::Vec::new(),
            rogue_aeon_list: ::std::vec::Vec::new(),
            miracle_list: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for RogueHandbookData {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("RogueHandbookData").unwrap()).clone()
    }
}

impl ::std::fmt::Display for RogueHandbookData {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RogueHandbookData {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x17RogueHandbookData.proto\x1a\x18RogueHandbookEvent.proto\x1a\x1aRog\
    ueHandbookMiracle.proto\x1a\x17RogueHandbookBuff.proto\x1a\x17RogueHandb\
    ookAeon.proto\"\xf0\x01\n\x11RogueHandbookData\x124\n\x0brogue_event\x18\
    \x0c\x20\x03(\x0b2\x13.RogueHandbookEventR\nrogueEvent\x12/\n\tbuff_list\
    \x18\n\x20\x03(\x0b2\x12.RogueHandbookBuffR\x08buffList\x12:\n\x0frogue_\
    aeon_list\x18\x0f\x20\x03(\x0b2\x12.RogueHandbookAeonR\rrogueAeonList\
    \x128\n\x0cmiracle_list\x18\x05\x20\x03(\x0b2\x15.RogueHandbookMiracleR\
    \x0bmiracleListB\x15\n\x13emu.lunarcore.protob\x06proto3\
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
            let mut deps = ::std::vec::Vec::with_capacity(4);
            deps.push(super::RogueHandbookEvent::file_descriptor().clone());
            deps.push(super::RogueHandbookMiracle::file_descriptor().clone());
            deps.push(super::RogueHandbookBuff::file_descriptor().clone());
            deps.push(super::RogueHandbookAeon::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(RogueHandbookData::generated_message_descriptor_data());
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
