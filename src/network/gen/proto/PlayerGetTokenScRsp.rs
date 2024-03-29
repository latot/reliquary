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

//! Generated file from `PlayerGetTokenScRsp.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:PlayerGetTokenScRsp)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct PlayerGetTokenScRsp {
    // message fields
    // @@protoc_insertion_point(field:PlayerGetTokenScRsp.secret_key_seed)
    pub secret_key_seed: u64,
    // @@protoc_insertion_point(field:PlayerGetTokenScRsp.black_info)
    pub black_info: ::protobuf::MessageField<super::BlackInfo::BlackInfo>,
    // @@protoc_insertion_point(field:PlayerGetTokenScRsp.uid)
    pub uid: u32,
    // @@protoc_insertion_point(field:PlayerGetTokenScRsp.msg)
    pub msg: ::std::string::String,
    // @@protoc_insertion_point(field:PlayerGetTokenScRsp.retcode)
    pub retcode: u32,
    // special fields
    // @@protoc_insertion_point(special_field:PlayerGetTokenScRsp.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a PlayerGetTokenScRsp {
    fn default() -> &'a PlayerGetTokenScRsp {
        <PlayerGetTokenScRsp as ::protobuf::Message>::default_instance()
    }
}

impl PlayerGetTokenScRsp {
    pub fn new() -> PlayerGetTokenScRsp {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(5);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "secret_key_seed",
            |m: &PlayerGetTokenScRsp| { &m.secret_key_seed },
            |m: &mut PlayerGetTokenScRsp| { &mut m.secret_key_seed },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::BlackInfo::BlackInfo>(
            "black_info",
            |m: &PlayerGetTokenScRsp| { &m.black_info },
            |m: &mut PlayerGetTokenScRsp| { &mut m.black_info },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "uid",
            |m: &PlayerGetTokenScRsp| { &m.uid },
            |m: &mut PlayerGetTokenScRsp| { &mut m.uid },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "msg",
            |m: &PlayerGetTokenScRsp| { &m.msg },
            |m: &mut PlayerGetTokenScRsp| { &mut m.msg },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "retcode",
            |m: &PlayerGetTokenScRsp| { &m.retcode },
            |m: &mut PlayerGetTokenScRsp| { &mut m.retcode },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<PlayerGetTokenScRsp>(
            "PlayerGetTokenScRsp",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for PlayerGetTokenScRsp {
    const NAME: &'static str = "PlayerGetTokenScRsp";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                88 => {
                    self.secret_key_seed = is.read_uint64()?;
                },
                122 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.black_info)?;
                },
                8 => {
                    self.uid = is.read_uint32()?;
                },
                42 => {
                    self.msg = is.read_string()?;
                },
                56 => {
                    self.retcode = is.read_uint32()?;
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
        if self.secret_key_seed != 0 {
            my_size += ::protobuf::rt::uint64_size(11, self.secret_key_seed);
        }
        if let Some(v) = self.black_info.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.uid != 0 {
            my_size += ::protobuf::rt::uint32_size(1, self.uid);
        }
        if !self.msg.is_empty() {
            my_size += ::protobuf::rt::string_size(5, &self.msg);
        }
        if self.retcode != 0 {
            my_size += ::protobuf::rt::uint32_size(7, self.retcode);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.secret_key_seed != 0 {
            os.write_uint64(11, self.secret_key_seed)?;
        }
        if let Some(v) = self.black_info.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(15, v, os)?;
        }
        if self.uid != 0 {
            os.write_uint32(1, self.uid)?;
        }
        if !self.msg.is_empty() {
            os.write_string(5, &self.msg)?;
        }
        if self.retcode != 0 {
            os.write_uint32(7, self.retcode)?;
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

    fn new() -> PlayerGetTokenScRsp {
        PlayerGetTokenScRsp::new()
    }

    fn clear(&mut self) {
        self.secret_key_seed = 0;
        self.black_info.clear();
        self.uid = 0;
        self.msg.clear();
        self.retcode = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static PlayerGetTokenScRsp {
        static instance: PlayerGetTokenScRsp = PlayerGetTokenScRsp {
            secret_key_seed: 0,
            black_info: ::protobuf::MessageField::none(),
            uid: 0,
            msg: ::std::string::String::new(),
            retcode: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for PlayerGetTokenScRsp {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("PlayerGetTokenScRsp").unwrap()).clone()
    }
}

impl ::std::fmt::Display for PlayerGetTokenScRsp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PlayerGetTokenScRsp {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x19PlayerGetTokenScRsp.proto\x1a\x0fBlackInfo.proto\"\xa6\x01\n\x13Pl\
    ayerGetTokenScRsp\x12&\n\x0fsecret_key_seed\x18\x0b\x20\x01(\x04R\rsecre\
    tKeySeed\x12)\n\nblack_info\x18\x0f\x20\x01(\x0b2\n.BlackInfoR\tblackInf\
    o\x12\x10\n\x03uid\x18\x01\x20\x01(\rR\x03uid\x12\x10\n\x03msg\x18\x05\
    \x20\x01(\tR\x03msg\x12\x18\n\x07retcode\x18\x07\x20\x01(\rR\x07retcodeB\
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
            let mut deps = ::std::vec::Vec::with_capacity(1);
            deps.push(super::BlackInfo::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(PlayerGetTokenScRsp::generated_message_descriptor_data());
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
