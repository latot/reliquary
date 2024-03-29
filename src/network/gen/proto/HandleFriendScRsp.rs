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

//! Generated file from `HandleFriendScRsp.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:HandleFriendScRsp)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct HandleFriendScRsp {
    // message fields
    // @@protoc_insertion_point(field:HandleFriendScRsp.handle_result)
    pub handle_result: bool,
    // @@protoc_insertion_point(field:HandleFriendScRsp.retcode)
    pub retcode: u32,
    // @@protoc_insertion_point(field:HandleFriendScRsp.uid)
    pub uid: u32,
    // @@protoc_insertion_point(field:HandleFriendScRsp.handle_friend_info)
    pub handle_friend_info: ::protobuf::MessageField<super::FriendListInfo::FriendListInfo>,
    // special fields
    // @@protoc_insertion_point(special_field:HandleFriendScRsp.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a HandleFriendScRsp {
    fn default() -> &'a HandleFriendScRsp {
        <HandleFriendScRsp as ::protobuf::Message>::default_instance()
    }
}

impl HandleFriendScRsp {
    pub fn new() -> HandleFriendScRsp {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "handle_result",
            |m: &HandleFriendScRsp| { &m.handle_result },
            |m: &mut HandleFriendScRsp| { &mut m.handle_result },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "retcode",
            |m: &HandleFriendScRsp| { &m.retcode },
            |m: &mut HandleFriendScRsp| { &mut m.retcode },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "uid",
            |m: &HandleFriendScRsp| { &m.uid },
            |m: &mut HandleFriendScRsp| { &mut m.uid },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::FriendListInfo::FriendListInfo>(
            "handle_friend_info",
            |m: &HandleFriendScRsp| { &m.handle_friend_info },
            |m: &mut HandleFriendScRsp| { &mut m.handle_friend_info },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<HandleFriendScRsp>(
            "HandleFriendScRsp",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for HandleFriendScRsp {
    const NAME: &'static str = "HandleFriendScRsp";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                40 => {
                    self.handle_result = is.read_bool()?;
                },
                96 => {
                    self.retcode = is.read_uint32()?;
                },
                32 => {
                    self.uid = is.read_uint32()?;
                },
                114 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.handle_friend_info)?;
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
        if self.handle_result != false {
            my_size += 1 + 1;
        }
        if self.retcode != 0 {
            my_size += ::protobuf::rt::uint32_size(12, self.retcode);
        }
        if self.uid != 0 {
            my_size += ::protobuf::rt::uint32_size(4, self.uid);
        }
        if let Some(v) = self.handle_friend_info.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.handle_result != false {
            os.write_bool(5, self.handle_result)?;
        }
        if self.retcode != 0 {
            os.write_uint32(12, self.retcode)?;
        }
        if self.uid != 0 {
            os.write_uint32(4, self.uid)?;
        }
        if let Some(v) = self.handle_friend_info.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(14, v, os)?;
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

    fn new() -> HandleFriendScRsp {
        HandleFriendScRsp::new()
    }

    fn clear(&mut self) {
        self.handle_result = false;
        self.retcode = 0;
        self.uid = 0;
        self.handle_friend_info.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static HandleFriendScRsp {
        static instance: HandleFriendScRsp = HandleFriendScRsp {
            handle_result: false,
            retcode: 0,
            uid: 0,
            handle_friend_info: ::protobuf::MessageField::none(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for HandleFriendScRsp {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("HandleFriendScRsp").unwrap()).clone()
    }
}

impl ::std::fmt::Display for HandleFriendScRsp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for HandleFriendScRsp {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x17HandleFriendScRsp.proto\x1a\x14FriendListInfo.proto\"\xa3\x01\n\
    \x11HandleFriendScRsp\x12#\n\rhandle_result\x18\x05\x20\x01(\x08R\x0chan\
    dleResult\x12\x18\n\x07retcode\x18\x0c\x20\x01(\rR\x07retcode\x12\x10\n\
    \x03uid\x18\x04\x20\x01(\rR\x03uid\x12=\n\x12handle_friend_info\x18\x0e\
    \x20\x01(\x0b2\x0f.FriendListInfoR\x10handleFriendInfoB\x15\n\x13emu.lun\
    arcore.protob\x06proto3\
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
            deps.push(super::FriendListInfo::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(HandleFriendScRsp::generated_message_descriptor_data());
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
