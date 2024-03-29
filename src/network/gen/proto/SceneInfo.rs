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

//! Generated file from `SceneInfo.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:SceneInfo)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct SceneInfo {
    // message fields
    // @@protoc_insertion_point(field:SceneInfo.extra_data)
    pub extra_data: ::std::collections::HashMap<::std::string::String, i32>,
    // @@protoc_insertion_point(field:SceneInfo.entity_group_list)
    pub entity_group_list: ::std::vec::Vec<super::SceneEntityGroupInfo::SceneEntityGroupInfo>,
    // @@protoc_insertion_point(field:SceneInfo.client_pos_version)
    pub client_pos_version: u32,
    // @@protoc_insertion_point(field:SceneInfo.group_id_list)
    pub group_id_list: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:SceneInfo.plane_id)
    pub plane_id: u32,
    // @@protoc_insertion_point(field:SceneInfo.world_id)
    pub world_id: u32,
    // @@protoc_insertion_point(field:SceneInfo.lighten_section_list)
    pub lighten_section_list: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:SceneInfo.leader_entity_id)
    pub leader_entity_id: u32,
    // @@protoc_insertion_point(field:SceneInfo.floor_id)
    pub floor_id: u32,
    // @@protoc_insertion_point(field:SceneInfo.entry_id)
    pub entry_id: u32,
    // @@protoc_insertion_point(field:SceneInfo.game_mode_type)
    pub game_mode_type: u32,
    // @@protoc_insertion_point(field:SceneInfo.entity_list)
    pub entity_list: ::std::vec::Vec<super::SceneEntityInfo::SceneEntityInfo>,
    // @@protoc_insertion_point(field:SceneInfo.group_state_list)
    pub group_state_list: ::std::vec::Vec<super::SceneGroupState::SceneGroupState>,
    // special fields
    // @@protoc_insertion_point(special_field:SceneInfo.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a SceneInfo {
    fn default() -> &'a SceneInfo {
        <SceneInfo as ::protobuf::Message>::default_instance()
    }
}

impl SceneInfo {
    pub fn new() -> SceneInfo {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(13);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_map_simpler_accessor::<_, _, _>(
            "extra_data",
            |m: &SceneInfo| { &m.extra_data },
            |m: &mut SceneInfo| { &mut m.extra_data },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "entity_group_list",
            |m: &SceneInfo| { &m.entity_group_list },
            |m: &mut SceneInfo| { &mut m.entity_group_list },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "client_pos_version",
            |m: &SceneInfo| { &m.client_pos_version },
            |m: &mut SceneInfo| { &mut m.client_pos_version },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "group_id_list",
            |m: &SceneInfo| { &m.group_id_list },
            |m: &mut SceneInfo| { &mut m.group_id_list },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "plane_id",
            |m: &SceneInfo| { &m.plane_id },
            |m: &mut SceneInfo| { &mut m.plane_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "world_id",
            |m: &SceneInfo| { &m.world_id },
            |m: &mut SceneInfo| { &mut m.world_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "lighten_section_list",
            |m: &SceneInfo| { &m.lighten_section_list },
            |m: &mut SceneInfo| { &mut m.lighten_section_list },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "leader_entity_id",
            |m: &SceneInfo| { &m.leader_entity_id },
            |m: &mut SceneInfo| { &mut m.leader_entity_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "floor_id",
            |m: &SceneInfo| { &m.floor_id },
            |m: &mut SceneInfo| { &mut m.floor_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "entry_id",
            |m: &SceneInfo| { &m.entry_id },
            |m: &mut SceneInfo| { &mut m.entry_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "game_mode_type",
            |m: &SceneInfo| { &m.game_mode_type },
            |m: &mut SceneInfo| { &mut m.game_mode_type },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "entity_list",
            |m: &SceneInfo| { &m.entity_list },
            |m: &mut SceneInfo| { &mut m.entity_list },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "group_state_list",
            |m: &SceneInfo| { &m.group_state_list },
            |m: &mut SceneInfo| { &mut m.group_state_list },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<SceneInfo>(
            "SceneInfo",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for SceneInfo {
    const NAME: &'static str = "SceneInfo";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                650 => {
                    let len = is.read_raw_varint32()?;
                    let old_limit = is.push_limit(len as u64)?;
                    let mut key = ::std::default::Default::default();
                    let mut value = ::std::default::Default::default();
                    while let Some(tag) = is.read_raw_tag_or_eof()? {
                        match tag {
                            10 => key = is.read_string()?,
                            16 => value = is.read_int32()?,
                            _ => ::protobuf::rt::skip_field_for_tag(tag, is)?,
                        };
                    }
                    is.pop_limit(old_limit);
                    self.extra_data.insert(key, value);
                },
                13194 => {
                    self.entity_group_list.push(is.read_message()?);
                },
                56 => {
                    self.client_pos_version = is.read_uint32()?;
                },
                106 => {
                    is.read_repeated_packed_uint32_into(&mut self.group_id_list)?;
                },
                104 => {
                    self.group_id_list.push(is.read_uint32()?);
                },
                64 => {
                    self.plane_id = is.read_uint32()?;
                },
                88 => {
                    self.world_id = is.read_uint32()?;
                },
                26 => {
                    is.read_repeated_packed_uint32_into(&mut self.lighten_section_list)?;
                },
                24 => {
                    self.lighten_section_list.push(is.read_uint32()?);
                },
                112 => {
                    self.leader_entity_id = is.read_uint32()?;
                },
                80 => {
                    self.floor_id = is.read_uint32()?;
                },
                40 => {
                    self.entry_id = is.read_uint32()?;
                },
                16 => {
                    self.game_mode_type = is.read_uint32()?;
                },
                50 => {
                    self.entity_list.push(is.read_message()?);
                },
                9954 => {
                    self.group_state_list.push(is.read_message()?);
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
        for (k, v) in &self.extra_data {
            let mut entry_size = 0;
            entry_size += ::protobuf::rt::string_size(1, &k);
            entry_size += ::protobuf::rt::int32_size(2, *v);
            my_size += 2 + ::protobuf::rt::compute_raw_varint64_size(entry_size) + entry_size
        };
        for value in &self.entity_group_list {
            let len = value.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.client_pos_version != 0 {
            my_size += ::protobuf::rt::uint32_size(7, self.client_pos_version);
        }
        for value in &self.group_id_list {
            my_size += ::protobuf::rt::uint32_size(13, *value);
        };
        if self.plane_id != 0 {
            my_size += ::protobuf::rt::uint32_size(8, self.plane_id);
        }
        if self.world_id != 0 {
            my_size += ::protobuf::rt::uint32_size(11, self.world_id);
        }
        for value in &self.lighten_section_list {
            my_size += ::protobuf::rt::uint32_size(3, *value);
        };
        if self.leader_entity_id != 0 {
            my_size += ::protobuf::rt::uint32_size(14, self.leader_entity_id);
        }
        if self.floor_id != 0 {
            my_size += ::protobuf::rt::uint32_size(10, self.floor_id);
        }
        if self.entry_id != 0 {
            my_size += ::protobuf::rt::uint32_size(5, self.entry_id);
        }
        if self.game_mode_type != 0 {
            my_size += ::protobuf::rt::uint32_size(2, self.game_mode_type);
        }
        for value in &self.entity_list {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        for value in &self.group_state_list {
            let len = value.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        for (k, v) in &self.extra_data {
            let mut entry_size = 0;
            entry_size += ::protobuf::rt::string_size(1, &k);
            entry_size += ::protobuf::rt::int32_size(2, *v);
            os.write_raw_varint32(650)?; // Tag.
            os.write_raw_varint32(entry_size as u32)?;
            os.write_string(1, &k)?;
            os.write_int32(2, *v)?;
        };
        for v in &self.entity_group_list {
            ::protobuf::rt::write_message_field_with_cached_size(1649, v, os)?;
        };
        if self.client_pos_version != 0 {
            os.write_uint32(7, self.client_pos_version)?;
        }
        for v in &self.group_id_list {
            os.write_uint32(13, *v)?;
        };
        if self.plane_id != 0 {
            os.write_uint32(8, self.plane_id)?;
        }
        if self.world_id != 0 {
            os.write_uint32(11, self.world_id)?;
        }
        for v in &self.lighten_section_list {
            os.write_uint32(3, *v)?;
        };
        if self.leader_entity_id != 0 {
            os.write_uint32(14, self.leader_entity_id)?;
        }
        if self.floor_id != 0 {
            os.write_uint32(10, self.floor_id)?;
        }
        if self.entry_id != 0 {
            os.write_uint32(5, self.entry_id)?;
        }
        if self.game_mode_type != 0 {
            os.write_uint32(2, self.game_mode_type)?;
        }
        for v in &self.entity_list {
            ::protobuf::rt::write_message_field_with_cached_size(6, v, os)?;
        };
        for v in &self.group_state_list {
            ::protobuf::rt::write_message_field_with_cached_size(1244, v, os)?;
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

    fn new() -> SceneInfo {
        SceneInfo::new()
    }

    fn clear(&mut self) {
        self.extra_data.clear();
        self.entity_group_list.clear();
        self.client_pos_version = 0;
        self.group_id_list.clear();
        self.plane_id = 0;
        self.world_id = 0;
        self.lighten_section_list.clear();
        self.leader_entity_id = 0;
        self.floor_id = 0;
        self.entry_id = 0;
        self.game_mode_type = 0;
        self.entity_list.clear();
        self.group_state_list.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static SceneInfo {
        static instance: ::protobuf::rt::Lazy<SceneInfo> = ::protobuf::rt::Lazy::new();
        instance.get(SceneInfo::new)
    }
}

impl ::protobuf::MessageFull for SceneInfo {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("SceneInfo").unwrap()).clone()
    }
}

impl ::std::fmt::Display for SceneInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SceneInfo {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x0fSceneInfo.proto\x1a\x15SceneEntityInfo.proto\x1a\x15SceneGroupStat\
    e.proto\x1a\x1aSceneEntityGroupInfo.proto\"\xf7\x04\n\tSceneInfo\x128\n\
    \nextra_data\x18Q\x20\x03(\x0b2\x19.SceneInfo.ExtraDataEntryR\textraData\
    \x12B\n\x11entity_group_list\x18\xf1\x0c\x20\x03(\x0b2\x15.SceneEntityGr\
    oupInfoR\x0fentityGroupList\x12,\n\x12client_pos_version\x18\x07\x20\x01\
    (\rR\x10clientPosVersion\x12\"\n\rgroup_id_list\x18\r\x20\x03(\rR\x0bgro\
    upIdList\x12\x19\n\x08plane_id\x18\x08\x20\x01(\rR\x07planeId\x12\x19\n\
    \x08world_id\x18\x0b\x20\x01(\rR\x07worldId\x120\n\x14lighten_section_li\
    st\x18\x03\x20\x03(\rR\x12lightenSectionList\x12(\n\x10leader_entity_id\
    \x18\x0e\x20\x01(\rR\x0eleaderEntityId\x12\x19\n\x08floor_id\x18\n\x20\
    \x01(\rR\x07floorId\x12\x19\n\x08entry_id\x18\x05\x20\x01(\rR\x07entryId\
    \x12$\n\x0egame_mode_type\x18\x02\x20\x01(\rR\x0cgameModeType\x121\n\x0b\
    entity_list\x18\x06\x20\x03(\x0b2\x10.SceneEntityInfoR\nentityList\x12;\
    \n\x10group_state_list\x18\xdc\t\x20\x03(\x0b2\x10.SceneGroupStateR\x0eg\
    roupStateList\x1a<\n\x0eExtraDataEntry\x12\x10\n\x03key\x18\x01\x20\x01(\
    \tR\x03key\x12\x14\n\x05value\x18\x02\x20\x01(\x05R\x05value:\x028\x01B\
    \x15\n\x13emu.lunarcore.protob\x06proto3\
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
            let mut deps = ::std::vec::Vec::with_capacity(3);
            deps.push(super::SceneEntityInfo::file_descriptor().clone());
            deps.push(super::SceneGroupState::file_descriptor().clone());
            deps.push(super::SceneEntityGroupInfo::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(SceneInfo::generated_message_descriptor_data());
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
