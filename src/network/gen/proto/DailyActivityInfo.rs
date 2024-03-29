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

//! Generated file from `DailyActivityInfo.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:DailyActivityInfo)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct DailyActivityInfo {
    // message fields
    // @@protoc_insertion_point(field:DailyActivityInfo.level)
    pub level: u32,
    // @@protoc_insertion_point(field:DailyActivityInfo.world_level)
    pub world_level: u32,
    // @@protoc_insertion_point(field:DailyActivityInfo.daily_active_point)
    pub daily_active_point: u32,
    // @@protoc_insertion_point(field:DailyActivityInfo.is_has_taken)
    pub is_has_taken: bool,
    // special fields
    // @@protoc_insertion_point(special_field:DailyActivityInfo.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a DailyActivityInfo {
    fn default() -> &'a DailyActivityInfo {
        <DailyActivityInfo as ::protobuf::Message>::default_instance()
    }
}

impl DailyActivityInfo {
    pub fn new() -> DailyActivityInfo {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "level",
            |m: &DailyActivityInfo| { &m.level },
            |m: &mut DailyActivityInfo| { &mut m.level },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "world_level",
            |m: &DailyActivityInfo| { &m.world_level },
            |m: &mut DailyActivityInfo| { &mut m.world_level },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "daily_active_point",
            |m: &DailyActivityInfo| { &m.daily_active_point },
            |m: &mut DailyActivityInfo| { &mut m.daily_active_point },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "is_has_taken",
            |m: &DailyActivityInfo| { &m.is_has_taken },
            |m: &mut DailyActivityInfo| { &mut m.is_has_taken },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<DailyActivityInfo>(
            "DailyActivityInfo",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for DailyActivityInfo {
    const NAME: &'static str = "DailyActivityInfo";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                80 => {
                    self.level = is.read_uint32()?;
                },
                32 => {
                    self.world_level = is.read_uint32()?;
                },
                40 => {
                    self.daily_active_point = is.read_uint32()?;
                },
                16 => {
                    self.is_has_taken = is.read_bool()?;
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
        if self.level != 0 {
            my_size += ::protobuf::rt::uint32_size(10, self.level);
        }
        if self.world_level != 0 {
            my_size += ::protobuf::rt::uint32_size(4, self.world_level);
        }
        if self.daily_active_point != 0 {
            my_size += ::protobuf::rt::uint32_size(5, self.daily_active_point);
        }
        if self.is_has_taken != false {
            my_size += 1 + 1;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.level != 0 {
            os.write_uint32(10, self.level)?;
        }
        if self.world_level != 0 {
            os.write_uint32(4, self.world_level)?;
        }
        if self.daily_active_point != 0 {
            os.write_uint32(5, self.daily_active_point)?;
        }
        if self.is_has_taken != false {
            os.write_bool(2, self.is_has_taken)?;
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

    fn new() -> DailyActivityInfo {
        DailyActivityInfo::new()
    }

    fn clear(&mut self) {
        self.level = 0;
        self.world_level = 0;
        self.daily_active_point = 0;
        self.is_has_taken = false;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static DailyActivityInfo {
        static instance: DailyActivityInfo = DailyActivityInfo {
            level: 0,
            world_level: 0,
            daily_active_point: 0,
            is_has_taken: false,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for DailyActivityInfo {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("DailyActivityInfo").unwrap()).clone()
    }
}

impl ::std::fmt::Display for DailyActivityInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DailyActivityInfo {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x17DailyActivityInfo.proto\"\x9a\x01\n\x11DailyActivityInfo\x12\x14\n\
    \x05level\x18\n\x20\x01(\rR\x05level\x12\x1f\n\x0bworld_level\x18\x04\
    \x20\x01(\rR\nworldLevel\x12,\n\x12daily_active_point\x18\x05\x20\x01(\r\
    R\x10dailyActivePoint\x12\x20\n\x0cis_has_taken\x18\x02\x20\x01(\x08R\ni\
    sHasTakenB\x15\n\x13emu.lunarcore.protob\x06proto3\
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
            let mut deps = ::std::vec::Vec::with_capacity(0);
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(DailyActivityInfo::generated_message_descriptor_data());
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
