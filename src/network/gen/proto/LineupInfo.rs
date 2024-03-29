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

//! Generated file from `LineupInfo.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:LineupInfo)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct LineupInfo {
    // message fields
    // @@protoc_insertion_point(field:LineupInfo.is_virtual)
    pub is_virtual: bool,
    // @@protoc_insertion_point(field:LineupInfo.leader_slot)
    pub leader_slot: u32,
    // @@protoc_insertion_point(field:LineupInfo.avatar_list)
    pub avatar_list: ::std::vec::Vec<super::LineupAvatar::LineupAvatar>,
    // @@protoc_insertion_point(field:LineupInfo.index)
    pub index: u32,
    // @@protoc_insertion_point(field:LineupInfo.extra_lineup_type)
    pub extra_lineup_type: ::protobuf::EnumOrUnknown<super::ExtraLineupType::ExtraLineupType>,
    // @@protoc_insertion_point(field:LineupInfo.max_mp)
    pub max_mp: u32,
    // @@protoc_insertion_point(field:LineupInfo.mp)
    pub mp: u32,
    // @@protoc_insertion_point(field:LineupInfo.name)
    pub name: ::std::string::String,
    // @@protoc_insertion_point(field:LineupInfo.plane_id)
    pub plane_id: u32,
    // special fields
    // @@protoc_insertion_point(special_field:LineupInfo.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a LineupInfo {
    fn default() -> &'a LineupInfo {
        <LineupInfo as ::protobuf::Message>::default_instance()
    }
}

impl LineupInfo {
    pub fn new() -> LineupInfo {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(9);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "is_virtual",
            |m: &LineupInfo| { &m.is_virtual },
            |m: &mut LineupInfo| { &mut m.is_virtual },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "leader_slot",
            |m: &LineupInfo| { &m.leader_slot },
            |m: &mut LineupInfo| { &mut m.leader_slot },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "avatar_list",
            |m: &LineupInfo| { &m.avatar_list },
            |m: &mut LineupInfo| { &mut m.avatar_list },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "index",
            |m: &LineupInfo| { &m.index },
            |m: &mut LineupInfo| { &mut m.index },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "extra_lineup_type",
            |m: &LineupInfo| { &m.extra_lineup_type },
            |m: &mut LineupInfo| { &mut m.extra_lineup_type },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "max_mp",
            |m: &LineupInfo| { &m.max_mp },
            |m: &mut LineupInfo| { &mut m.max_mp },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "mp",
            |m: &LineupInfo| { &m.mp },
            |m: &mut LineupInfo| { &mut m.mp },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "name",
            |m: &LineupInfo| { &m.name },
            |m: &mut LineupInfo| { &mut m.name },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "plane_id",
            |m: &LineupInfo| { &m.plane_id },
            |m: &mut LineupInfo| { &mut m.plane_id },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<LineupInfo>(
            "LineupInfo",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for LineupInfo {
    const NAME: &'static str = "LineupInfo";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                16 => {
                    self.is_virtual = is.read_bool()?;
                },
                24 => {
                    self.leader_slot = is.read_uint32()?;
                },
                90 => {
                    self.avatar_list.push(is.read_message()?);
                },
                56 => {
                    self.index = is.read_uint32()?;
                },
                48 => {
                    self.extra_lineup_type = is.read_enum_or_unknown()?;
                },
                72 => {
                    self.max_mp = is.read_uint32()?;
                },
                64 => {
                    self.mp = is.read_uint32()?;
                },
                122 => {
                    self.name = is.read_string()?;
                },
                8 => {
                    self.plane_id = is.read_uint32()?;
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
        if self.is_virtual != false {
            my_size += 1 + 1;
        }
        if self.leader_slot != 0 {
            my_size += ::protobuf::rt::uint32_size(3, self.leader_slot);
        }
        for value in &self.avatar_list {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.index != 0 {
            my_size += ::protobuf::rt::uint32_size(7, self.index);
        }
        if self.extra_lineup_type != ::protobuf::EnumOrUnknown::new(super::ExtraLineupType::ExtraLineupType::LINEUP_NONE) {
            my_size += ::protobuf::rt::int32_size(6, self.extra_lineup_type.value());
        }
        if self.max_mp != 0 {
            my_size += ::protobuf::rt::uint32_size(9, self.max_mp);
        }
        if self.mp != 0 {
            my_size += ::protobuf::rt::uint32_size(8, self.mp);
        }
        if !self.name.is_empty() {
            my_size += ::protobuf::rt::string_size(15, &self.name);
        }
        if self.plane_id != 0 {
            my_size += ::protobuf::rt::uint32_size(1, self.plane_id);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.is_virtual != false {
            os.write_bool(2, self.is_virtual)?;
        }
        if self.leader_slot != 0 {
            os.write_uint32(3, self.leader_slot)?;
        }
        for v in &self.avatar_list {
            ::protobuf::rt::write_message_field_with_cached_size(11, v, os)?;
        };
        if self.index != 0 {
            os.write_uint32(7, self.index)?;
        }
        if self.extra_lineup_type != ::protobuf::EnumOrUnknown::new(super::ExtraLineupType::ExtraLineupType::LINEUP_NONE) {
            os.write_enum(6, ::protobuf::EnumOrUnknown::value(&self.extra_lineup_type))?;
        }
        if self.max_mp != 0 {
            os.write_uint32(9, self.max_mp)?;
        }
        if self.mp != 0 {
            os.write_uint32(8, self.mp)?;
        }
        if !self.name.is_empty() {
            os.write_string(15, &self.name)?;
        }
        if self.plane_id != 0 {
            os.write_uint32(1, self.plane_id)?;
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

    fn new() -> LineupInfo {
        LineupInfo::new()
    }

    fn clear(&mut self) {
        self.is_virtual = false;
        self.leader_slot = 0;
        self.avatar_list.clear();
        self.index = 0;
        self.extra_lineup_type = ::protobuf::EnumOrUnknown::new(super::ExtraLineupType::ExtraLineupType::LINEUP_NONE);
        self.max_mp = 0;
        self.mp = 0;
        self.name.clear();
        self.plane_id = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static LineupInfo {
        static instance: LineupInfo = LineupInfo {
            is_virtual: false,
            leader_slot: 0,
            avatar_list: ::std::vec::Vec::new(),
            index: 0,
            extra_lineup_type: ::protobuf::EnumOrUnknown::from_i32(0),
            max_mp: 0,
            mp: 0,
            name: ::std::string::String::new(),
            plane_id: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for LineupInfo {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("LineupInfo").unwrap()).clone()
    }
}

impl ::std::fmt::Display for LineupInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for LineupInfo {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x10LineupInfo.proto\x1a\x15ExtraLineupType.proto\x1a\x12LineupAvatar.\
    proto\"\xa6\x02\n\nLineupInfo\x12\x1d\n\nis_virtual\x18\x02\x20\x01(\x08\
    R\tisVirtual\x12\x1f\n\x0bleader_slot\x18\x03\x20\x01(\rR\nleaderSlot\
    \x12.\n\x0bavatar_list\x18\x0b\x20\x03(\x0b2\r.LineupAvatarR\navatarList\
    \x12\x14\n\x05index\x18\x07\x20\x01(\rR\x05index\x12<\n\x11extra_lineup_\
    type\x18\x06\x20\x01(\x0e2\x10.ExtraLineupTypeR\x0fextraLineupType\x12\
    \x15\n\x06max_mp\x18\t\x20\x01(\rR\x05maxMp\x12\x0e\n\x02mp\x18\x08\x20\
    \x01(\rR\x02mp\x12\x12\n\x04name\x18\x0f\x20\x01(\tR\x04name\x12\x19\n\
    \x08plane_id\x18\x01\x20\x01(\rR\x07planeIdB\x15\n\x13emu.lunarcore.prot\
    ob\x06proto3\
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
            let mut deps = ::std::vec::Vec::with_capacity(2);
            deps.push(super::ExtraLineupType::file_descriptor().clone());
            deps.push(super::LineupAvatar::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(LineupInfo::generated_message_descriptor_data());
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
