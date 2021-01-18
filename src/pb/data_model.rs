// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(bare_trait_objects)]
#![allow(clippy::all)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(PartialEq,Clone,Default)]
pub struct Tag {
    // message fields
    key: ::protobuf::SingularField<::std::string::String>,
    value: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Tag {}

impl Tag {
    pub fn new() -> Tag {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Tag {
        static mut instance: ::protobuf::lazy::Lazy<Tag> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Tag,
        };
        unsafe {
            instance.get(Tag::new)
        }
    }

    // required string key = 1;

    pub fn clear_key(&mut self) {
        self.key.clear();
    }

    pub fn has_key(&self) -> bool {
        self.key.is_some()
    }

    // Param is passed by value, moved
    pub fn set_key(&mut self, v: ::std::string::String) {
        self.key = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_key(&mut self) -> &mut ::std::string::String {
        if self.key.is_none() {
            self.key.set_default();
        };
        self.key.as_mut().unwrap()
    }

    // Take field
    pub fn take_key(&mut self) -> ::std::string::String {
        self.key.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_key(&self) -> &str {
        match self.key.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_key_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.key
    }

    fn mut_key_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.key
    }

    // required string value = 2;

    pub fn clear_value(&mut self) {
        self.value.clear();
    }

    pub fn has_value(&self) -> bool {
        self.value.is_some()
    }

    // Param is passed by value, moved
    pub fn set_value(&mut self, v: ::std::string::String) {
        self.value = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_value(&mut self) -> &mut ::std::string::String {
        if self.value.is_none() {
            self.value.set_default();
        };
        self.value.as_mut().unwrap()
    }

    // Take field
    pub fn take_value(&mut self) -> ::std::string::String {
        self.value.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_value(&self) -> &str {
        match self.value.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_value_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.value
    }

    fn mut_value_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.value
    }
}

impl ::protobuf::Message for Tag {
    fn is_initialized(&self) -> bool {
        if self.key.is_none() {
            return false;
        };
        if self.value.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.key)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.value)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.key.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        };
        if let Some(v) = self.value.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.key.as_ref() {
            os.write_string(1, &v)?;
        };
        if let Some(v) = self.value.as_ref() {
            os.write_string(2, &v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Tag {
    fn new() -> Tag {
        Tag::new()
    }

    fn descriptor_static(_: ::std::option::Option<Tag>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "key",
                    Tag::get_key_for_reflect,
                    Tag::mut_key_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "value",
                    Tag::get_value_for_reflect,
                    Tag::mut_value_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Tag>(
                    "Tag",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Tag {
    fn clear(&mut self) {
        self.clear_key();
        self.clear_value();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Tag {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Tag {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Gps {
    // message fields
    lat: ::std::option::Option<f32>,
    lon: ::std::option::Option<f32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Gps {}

impl Gps {
    pub fn new() -> Gps {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Gps {
        static mut instance: ::protobuf::lazy::Lazy<Gps> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Gps,
        };
        unsafe {
            instance.get(Gps::new)
        }
    }

    // required float lat = 1;

    pub fn clear_lat(&mut self) {
        self.lat = ::std::option::Option::None;
    }

    pub fn has_lat(&self) -> bool {
        self.lat.is_some()
    }

    // Param is passed by value, moved
    pub fn set_lat(&mut self, v: f32) {
        self.lat = ::std::option::Option::Some(v);
    }

    pub fn get_lat(&self) -> f32 {
        self.lat.unwrap_or(0.)
    }

    fn get_lat_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.lat
    }

    fn mut_lat_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.lat
    }

    // required float lon = 2;

    pub fn clear_lon(&mut self) {
        self.lon = ::std::option::Option::None;
    }

    pub fn has_lon(&self) -> bool {
        self.lon.is_some()
    }

    // Param is passed by value, moved
    pub fn set_lon(&mut self, v: f32) {
        self.lon = ::std::option::Option::Some(v);
    }

    pub fn get_lon(&self) -> f32 {
        self.lon.unwrap_or(0.)
    }

    fn get_lon_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.lon
    }

    fn mut_lon_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.lon
    }
}

impl ::protobuf::Message for Gps {
    fn is_initialized(&self) -> bool {
        if self.lat.is_none() {
            return false;
        };
        if self.lon.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_float()?;
                    self.lat = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_float()?;
                    self.lon = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.lat {
            my_size += 5;
        };
        if let Some(v) = self.lon {
            my_size += 5;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.lat {
            os.write_float(1, v)?;
        };
        if let Some(v) = self.lon {
            os.write_float(2, v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Gps {
    fn new() -> Gps {
        Gps::new()
    }

    fn descriptor_static(_: ::std::option::Option<Gps>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "lat",
                    Gps::get_lat_for_reflect,
                    Gps::mut_lat_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "lon",
                    Gps::get_lon_for_reflect,
                    Gps::mut_lon_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Gps>(
                    "Gps",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Gps {
    fn clear(&mut self) {
        self.clear_lat();
        self.clear_lon();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Gps {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Gps {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Meta {
    // message fields
    ctime: ::std::option::Option<u64>,
    mtime: ::std::option::Option<u64>,
    finish_time: ::std::option::Option<u64>,
    gps: ::protobuf::SingularPtrField<Gps>,
    tags: ::protobuf::RepeatedField<Tag>,
    due: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Meta {}

impl Meta {
    pub fn new() -> Meta {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Meta {
        static mut instance: ::protobuf::lazy::Lazy<Meta> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Meta,
        };
        unsafe {
            instance.get(Meta::new)
        }
    }

    // required uint64 ctime = 1;

    pub fn clear_ctime(&mut self) {
        self.ctime = ::std::option::Option::None;
    }

    pub fn has_ctime(&self) -> bool {
        self.ctime.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ctime(&mut self, v: u64) {
        self.ctime = ::std::option::Option::Some(v);
    }

    pub fn get_ctime(&self) -> u64 {
        self.ctime.unwrap_or(0)
    }

    fn get_ctime_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.ctime
    }

    fn mut_ctime_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.ctime
    }

    // required uint64 mtime = 2;

    pub fn clear_mtime(&mut self) {
        self.mtime = ::std::option::Option::None;
    }

    pub fn has_mtime(&self) -> bool {
        self.mtime.is_some()
    }

    // Param is passed by value, moved
    pub fn set_mtime(&mut self, v: u64) {
        self.mtime = ::std::option::Option::Some(v);
    }

    pub fn get_mtime(&self) -> u64 {
        self.mtime.unwrap_or(0)
    }

    fn get_mtime_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.mtime
    }

    fn mut_mtime_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.mtime
    }

    // optional uint64 finish_time = 5;

    pub fn clear_finish_time(&mut self) {
        self.finish_time = ::std::option::Option::None;
    }

    pub fn has_finish_time(&self) -> bool {
        self.finish_time.is_some()
    }

    // Param is passed by value, moved
    pub fn set_finish_time(&mut self, v: u64) {
        self.finish_time = ::std::option::Option::Some(v);
    }

    pub fn get_finish_time(&self) -> u64 {
        self.finish_time.unwrap_or(0)
    }

    fn get_finish_time_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.finish_time
    }

    fn mut_finish_time_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.finish_time
    }

    // required .void.Gps gps = 3;

    pub fn clear_gps(&mut self) {
        self.gps.clear();
    }

    pub fn has_gps(&self) -> bool {
        self.gps.is_some()
    }

    // Param is passed by value, moved
    pub fn set_gps(&mut self, v: Gps) {
        self.gps = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_gps(&mut self) -> &mut Gps {
        if self.gps.is_none() {
            self.gps.set_default();
        };
        self.gps.as_mut().unwrap()
    }

    // Take field
    pub fn take_gps(&mut self) -> Gps {
        self.gps.take().unwrap_or_else(|| Gps::new())
    }

    pub fn get_gps(&self) -> &Gps {
        self.gps.as_ref().unwrap_or_else(|| Gps::default_instance())
    }

    fn get_gps_for_reflect(&self) -> &::protobuf::SingularPtrField<Gps> {
        &self.gps
    }

    fn mut_gps_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Gps> {
        &mut self.gps
    }

    // repeated .void.Tag tags = 4;

    pub fn clear_tags(&mut self) {
        self.tags.clear();
    }

    // Param is passed by value, moved
    pub fn set_tags(&mut self, v: ::protobuf::RepeatedField<Tag>) {
        self.tags = v;
    }

    // Mutable pointer to the field.
    pub fn mut_tags(&mut self) -> &mut ::protobuf::RepeatedField<Tag> {
        &mut self.tags
    }

    // Take field
    pub fn take_tags(&mut self) -> ::protobuf::RepeatedField<Tag> {
        ::std::mem::replace(&mut self.tags, ::protobuf::RepeatedField::new())
    }

    pub fn get_tags(&self) -> &[Tag] {
        &self.tags
    }

    fn get_tags_for_reflect(&self) -> &::protobuf::RepeatedField<Tag> {
        &self.tags
    }

    fn mut_tags_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<Tag> {
        &mut self.tags
    }

    // optional uint64 due = 6;

    pub fn clear_due(&mut self) {
        self.due = ::std::option::Option::None;
    }

    pub fn has_due(&self) -> bool {
        self.due.is_some()
    }

    // Param is passed by value, moved
    pub fn set_due(&mut self, v: u64) {
        self.due = ::std::option::Option::Some(v);
    }

    pub fn get_due(&self) -> u64 {
        self.due.unwrap_or(0)
    }

    fn get_due_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.due
    }

    fn mut_due_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.due
    }
}

impl ::protobuf::Message for Meta {
    fn is_initialized(&self) -> bool {
        if self.ctime.is_none() {
            return false;
        };
        if self.mtime.is_none() {
            return false;
        };
        if self.gps.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.ctime = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.mtime = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.finish_time = ::std::option::Option::Some(tmp);
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.gps)?;
                },
                4 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.tags)?;
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.due = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.ctime {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.mtime {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.finish_time {
            my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.gps.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.tags {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.due {
            my_size += ::protobuf::rt::value_size(6, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.ctime {
            os.write_uint64(1, v)?;
        };
        if let Some(v) = self.mtime {
            os.write_uint64(2, v)?;
        };
        if let Some(v) = self.finish_time {
            os.write_uint64(5, v)?;
        };
        if let Some(v) = self.gps.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.tags {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.due {
            os.write_uint64(6, v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Meta {
    fn new() -> Meta {
        Meta::new()
    }

    fn descriptor_static(_: ::std::option::Option<Meta>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "ctime",
                    Meta::get_ctime_for_reflect,
                    Meta::mut_ctime_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "mtime",
                    Meta::get_mtime_for_reflect,
                    Meta::mut_mtime_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "finish_time",
                    Meta::get_finish_time_for_reflect,
                    Meta::mut_finish_time_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Gps>>(
                    "gps",
                    Meta::get_gps_for_reflect,
                    Meta::mut_gps_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Tag>>(
                    "tags",
                    Meta::get_tags_for_reflect,
                    Meta::mut_tags_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "due",
                    Meta::get_due_for_reflect,
                    Meta::mut_due_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Meta>(
                    "Meta",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Meta {
    fn clear(&mut self) {
        self.clear_ctime();
        self.clear_mtime();
        self.clear_finish_time();
        self.clear_gps();
        self.clear_tags();
        self.clear_due();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Meta {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Meta {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Node {
    // message fields
    id: ::std::option::Option<u64>,
    meta: ::protobuf::SingularPtrField<Meta>,
    text: ::protobuf::SingularField<::std::string::String>,
    children: ::std::vec::Vec<u64>,
    collapsed: ::std::option::Option<bool>,
    stricken: ::std::option::Option<bool>,
    hide_stricken: ::std::option::Option<bool>,
    x: ::std::option::Option<u32>,
    y: ::std::option::Option<u32>,
    selected: ::std::option::Option<bool>,
    parent_id: ::std::option::Option<u64>,
    free_text: ::protobuf::SingularField<::std::string::String>,
    auto_arrange: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Node {}

impl Node {
    pub fn new() -> Node {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Node {
        static mut instance: ::protobuf::lazy::Lazy<Node> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Node,
        };
        unsafe {
            instance.get(Node::new)
        }
    }

    // required uint64 id = 1;

    pub fn clear_id(&mut self) {
        self.id = ::std::option::Option::None;
    }

    pub fn has_id(&self) -> bool {
        self.id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: u64) {
        self.id = ::std::option::Option::Some(v);
    }

    pub fn get_id(&self) -> u64 {
        self.id.unwrap_or(0)
    }

    fn get_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.id
    }

    fn mut_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.id
    }

    // required .void.Meta meta = 2;

    pub fn clear_meta(&mut self) {
        self.meta.clear();
    }

    pub fn has_meta(&self) -> bool {
        self.meta.is_some()
    }

    // Param is passed by value, moved
    pub fn set_meta(&mut self, v: Meta) {
        self.meta = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_meta(&mut self) -> &mut Meta {
        if self.meta.is_none() {
            self.meta.set_default();
        };
        self.meta.as_mut().unwrap()
    }

    // Take field
    pub fn take_meta(&mut self) -> Meta {
        self.meta.take().unwrap_or_else(|| Meta::new())
    }

    pub fn get_meta(&self) -> &Meta {
        self.meta.as_ref().unwrap_or_else(|| Meta::default_instance())
    }

    fn get_meta_for_reflect(&self) -> &::protobuf::SingularPtrField<Meta> {
        &self.meta
    }

    fn mut_meta_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Meta> {
        &mut self.meta
    }

    // required string text = 3;

    pub fn clear_text(&mut self) {
        self.text.clear();
    }

    pub fn has_text(&self) -> bool {
        self.text.is_some()
    }

    // Param is passed by value, moved
    pub fn set_text(&mut self, v: ::std::string::String) {
        self.text = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_text(&mut self) -> &mut ::std::string::String {
        if self.text.is_none() {
            self.text.set_default();
        };
        self.text.as_mut().unwrap()
    }

    // Take field
    pub fn take_text(&mut self) -> ::std::string::String {
        self.text.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_text(&self) -> &str {
        match self.text.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_text_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.text
    }

    fn mut_text_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.text
    }

    // repeated uint64 children = 4;

    pub fn clear_children(&mut self) {
        self.children.clear();
    }

    // Param is passed by value, moved
    pub fn set_children(&mut self, v: ::std::vec::Vec<u64>) {
        self.children = v;
    }

    // Mutable pointer to the field.
    pub fn mut_children(&mut self) -> &mut ::std::vec::Vec<u64> {
        &mut self.children
    }

    // Take field
    pub fn take_children(&mut self) -> ::std::vec::Vec<u64> {
        ::std::mem::replace(&mut self.children, ::std::vec::Vec::new())
    }

    pub fn get_children(&self) -> &[u64] {
        &self.children
    }

    fn get_children_for_reflect(&self) -> &::std::vec::Vec<u64> {
        &self.children
    }

    fn mut_children_for_reflect(&mut self) -> &mut ::std::vec::Vec<u64> {
        &mut self.children
    }

    // required bool collapsed = 5;

    pub fn clear_collapsed(&mut self) {
        self.collapsed = ::std::option::Option::None;
    }

    pub fn has_collapsed(&self) -> bool {
        self.collapsed.is_some()
    }

    // Param is passed by value, moved
    pub fn set_collapsed(&mut self, v: bool) {
        self.collapsed = ::std::option::Option::Some(v);
    }

    pub fn get_collapsed(&self) -> bool {
        self.collapsed.unwrap_or(false)
    }

    fn get_collapsed_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.collapsed
    }

    fn mut_collapsed_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.collapsed
    }

    // required bool stricken = 6;

    pub fn clear_stricken(&mut self) {
        self.stricken = ::std::option::Option::None;
    }

    pub fn has_stricken(&self) -> bool {
        self.stricken.is_some()
    }

    // Param is passed by value, moved
    pub fn set_stricken(&mut self, v: bool) {
        self.stricken = ::std::option::Option::Some(v);
    }

    pub fn get_stricken(&self) -> bool {
        self.stricken.unwrap_or(false)
    }

    fn get_stricken_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.stricken
    }

    fn mut_stricken_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.stricken
    }

    // required bool hide_stricken = 7;

    pub fn clear_hide_stricken(&mut self) {
        self.hide_stricken = ::std::option::Option::None;
    }

    pub fn has_hide_stricken(&self) -> bool {
        self.hide_stricken.is_some()
    }

    // Param is passed by value, moved
    pub fn set_hide_stricken(&mut self, v: bool) {
        self.hide_stricken = ::std::option::Option::Some(v);
    }

    pub fn get_hide_stricken(&self) -> bool {
        self.hide_stricken.unwrap_or(false)
    }

    fn get_hide_stricken_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.hide_stricken
    }

    fn mut_hide_stricken_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.hide_stricken
    }

    // required uint32 x = 8;

    pub fn clear_x(&mut self) {
        self.x = ::std::option::Option::None;
    }

    pub fn has_x(&self) -> bool {
        self.x.is_some()
    }

    // Param is passed by value, moved
    pub fn set_x(&mut self, v: u32) {
        self.x = ::std::option::Option::Some(v);
    }

    pub fn get_x(&self) -> u32 {
        self.x.unwrap_or(0)
    }

    fn get_x_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.x
    }

    fn mut_x_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.x
    }

    // required uint32 y = 9;

    pub fn clear_y(&mut self) {
        self.y = ::std::option::Option::None;
    }

    pub fn has_y(&self) -> bool {
        self.y.is_some()
    }

    // Param is passed by value, moved
    pub fn set_y(&mut self, v: u32) {
        self.y = ::std::option::Option::Some(v);
    }

    pub fn get_y(&self) -> u32 {
        self.y.unwrap_or(0)
    }

    fn get_y_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.y
    }

    fn mut_y_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.y
    }

    // required bool selected = 10;

    pub fn clear_selected(&mut self) {
        self.selected = ::std::option::Option::None;
    }

    pub fn has_selected(&self) -> bool {
        self.selected.is_some()
    }

    // Param is passed by value, moved
    pub fn set_selected(&mut self, v: bool) {
        self.selected = ::std::option::Option::Some(v);
    }

    pub fn get_selected(&self) -> bool {
        self.selected.unwrap_or(false)
    }

    fn get_selected_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.selected
    }

    fn mut_selected_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.selected
    }

    // required uint64 parent_id = 11;

    pub fn clear_parent_id(&mut self) {
        self.parent_id = ::std::option::Option::None;
    }

    pub fn has_parent_id(&self) -> bool {
        self.parent_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_parent_id(&mut self, v: u64) {
        self.parent_id = ::std::option::Option::Some(v);
    }

    pub fn get_parent_id(&self) -> u64 {
        self.parent_id.unwrap_or(0)
    }

    fn get_parent_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.parent_id
    }

    fn mut_parent_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.parent_id
    }

    // optional string free_text = 12;

    pub fn clear_free_text(&mut self) {
        self.free_text.clear();
    }

    pub fn has_free_text(&self) -> bool {
        self.free_text.is_some()
    }

    // Param is passed by value, moved
    pub fn set_free_text(&mut self, v: ::std::string::String) {
        self.free_text = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_free_text(&mut self) -> &mut ::std::string::String {
        if self.free_text.is_none() {
            self.free_text.set_default();
        };
        self.free_text.as_mut().unwrap()
    }

    // Take field
    pub fn take_free_text(&mut self) -> ::std::string::String {
        self.free_text.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_free_text(&self) -> &str {
        match self.free_text.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_free_text_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.free_text
    }

    fn mut_free_text_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.free_text
    }

    // required bool auto_arrange = 13;

    pub fn clear_auto_arrange(&mut self) {
        self.auto_arrange = ::std::option::Option::None;
    }

    pub fn has_auto_arrange(&self) -> bool {
        self.auto_arrange.is_some()
    }

    // Param is passed by value, moved
    pub fn set_auto_arrange(&mut self, v: bool) {
        self.auto_arrange = ::std::option::Option::Some(v);
    }

    pub fn get_auto_arrange(&self) -> bool {
        self.auto_arrange.unwrap_or(false)
    }

    fn get_auto_arrange_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.auto_arrange
    }

    fn mut_auto_arrange_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.auto_arrange
    }
}

impl ::protobuf::Message for Node {
    fn is_initialized(&self) -> bool {
        if self.id.is_none() {
            return false;
        };
        if self.meta.is_none() {
            return false;
        };
        if self.text.is_none() {
            return false;
        };
        if self.collapsed.is_none() {
            return false;
        };
        if self.stricken.is_none() {
            return false;
        };
        if self.hide_stricken.is_none() {
            return false;
        };
        if self.x.is_none() {
            return false;
        };
        if self.y.is_none() {
            return false;
        };
        if self.selected.is_none() {
            return false;
        };
        if self.parent_id.is_none() {
            return false;
        };
        if self.auto_arrange.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.meta)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.text)?;
                },
                4 => {
                    ::protobuf::rt::read_repeated_uint64_into(wire_type, is, &mut self.children)?;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.collapsed = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.stricken = ::std::option::Option::Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.hide_stricken = ::std::option::Option::Some(tmp);
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.x = ::std::option::Option::Some(tmp);
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.y = ::std::option::Option::Some(tmp);
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.selected = ::std::option::Option::Some(tmp);
                },
                11 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.parent_id = ::std::option::Option::Some(tmp);
                },
                12 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.free_text)?;
                },
                13 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.auto_arrange = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.meta.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.text.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        };
        for value in &self.children {
            my_size += ::protobuf::rt::value_size(4, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.collapsed {
            my_size += 2;
        };
        if let Some(v) = self.stricken {
            my_size += 2;
        };
        if let Some(v) = self.hide_stricken {
            my_size += 2;
        };
        if let Some(v) = self.x {
            my_size += ::protobuf::rt::value_size(8, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.y {
            my_size += ::protobuf::rt::value_size(9, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.selected {
            my_size += 2;
        };
        if let Some(v) = self.parent_id {
            my_size += ::protobuf::rt::value_size(11, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.free_text.as_ref() {
            my_size += ::protobuf::rt::string_size(12, &v);
        };
        if let Some(v) = self.auto_arrange {
            my_size += 2;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.id {
            os.write_uint64(1, v)?;
        };
        if let Some(v) = self.meta.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.text.as_ref() {
            os.write_string(3, &v)?;
        };
        for v in &self.children {
            os.write_uint64(4, *v)?;
        };
        if let Some(v) = self.collapsed {
            os.write_bool(5, v)?;
        };
        if let Some(v) = self.stricken {
            os.write_bool(6, v)?;
        };
        if let Some(v) = self.hide_stricken {
            os.write_bool(7, v)?;
        };
        if let Some(v) = self.x {
            os.write_uint32(8, v)?;
        };
        if let Some(v) = self.y {
            os.write_uint32(9, v)?;
        };
        if let Some(v) = self.selected {
            os.write_bool(10, v)?;
        };
        if let Some(v) = self.parent_id {
            os.write_uint64(11, v)?;
        };
        if let Some(v) = self.free_text.as_ref() {
            os.write_string(12, &v)?;
        };
        if let Some(v) = self.auto_arrange {
            os.write_bool(13, v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Node {
    fn new() -> Node {
        Node::new()
    }

    fn descriptor_static(_: ::std::option::Option<Node>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "id",
                    Node::get_id_for_reflect,
                    Node::mut_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Meta>>(
                    "meta",
                    Node::get_meta_for_reflect,
                    Node::mut_meta_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "text",
                    Node::get_text_for_reflect,
                    Node::mut_text_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "children",
                    Node::get_children_for_reflect,
                    Node::mut_children_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "collapsed",
                    Node::get_collapsed_for_reflect,
                    Node::mut_collapsed_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "stricken",
                    Node::get_stricken_for_reflect,
                    Node::mut_stricken_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "hide_stricken",
                    Node::get_hide_stricken_for_reflect,
                    Node::mut_hide_stricken_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "x",
                    Node::get_x_for_reflect,
                    Node::mut_x_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "y",
                    Node::get_y_for_reflect,
                    Node::mut_y_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "selected",
                    Node::get_selected_for_reflect,
                    Node::mut_selected_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "parent_id",
                    Node::get_parent_id_for_reflect,
                    Node::mut_parent_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "free_text",
                    Node::get_free_text_for_reflect,
                    Node::mut_free_text_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "auto_arrange",
                    Node::get_auto_arrange_for_reflect,
                    Node::mut_auto_arrange_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Node>(
                    "Node",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Node {
    fn clear(&mut self) {
        self.clear_id();
        self.clear_meta();
        self.clear_text();
        self.clear_children();
        self.clear_collapsed();
        self.clear_stricken();
        self.clear_hide_stricken();
        self.clear_x();
        self.clear_y();
        self.clear_selected();
        self.clear_parent_id();
        self.clear_free_text();
        self.clear_auto_arrange();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Node {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Node {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Arrow {
    // message fields
    from_node: ::std::option::Option<u64>,
    to_node: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Arrow {}

impl Arrow {
    pub fn new() -> Arrow {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Arrow {
        static mut instance: ::protobuf::lazy::Lazy<Arrow> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Arrow,
        };
        unsafe {
            instance.get(Arrow::new)
        }
    }

    // required uint64 from_node = 1;

    pub fn clear_from_node(&mut self) {
        self.from_node = ::std::option::Option::None;
    }

    pub fn has_from_node(&self) -> bool {
        self.from_node.is_some()
    }

    // Param is passed by value, moved
    pub fn set_from_node(&mut self, v: u64) {
        self.from_node = ::std::option::Option::Some(v);
    }

    pub fn get_from_node(&self) -> u64 {
        self.from_node.unwrap_or(0)
    }

    fn get_from_node_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.from_node
    }

    fn mut_from_node_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.from_node
    }

    // required uint64 to_node = 2;

    pub fn clear_to_node(&mut self) {
        self.to_node = ::std::option::Option::None;
    }

    pub fn has_to_node(&self) -> bool {
        self.to_node.is_some()
    }

    // Param is passed by value, moved
    pub fn set_to_node(&mut self, v: u64) {
        self.to_node = ::std::option::Option::Some(v);
    }

    pub fn get_to_node(&self) -> u64 {
        self.to_node.unwrap_or(0)
    }

    fn get_to_node_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.to_node
    }

    fn mut_to_node_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.to_node
    }
}

impl ::protobuf::Message for Arrow {
    fn is_initialized(&self) -> bool {
        if self.from_node.is_none() {
            return false;
        };
        if self.to_node.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.from_node = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.to_node = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.from_node {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.to_node {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.from_node {
            os.write_uint64(1, v)?;
        };
        if let Some(v) = self.to_node {
            os.write_uint64(2, v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Arrow {
    fn new() -> Arrow {
        Arrow::new()
    }

    fn descriptor_static(_: ::std::option::Option<Arrow>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "from_node",
                    Arrow::get_from_node_for_reflect,
                    Arrow::mut_from_node_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "to_node",
                    Arrow::get_to_node_for_reflect,
                    Arrow::mut_to_node_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Arrow>(
                    "Arrow",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Arrow {
    fn clear(&mut self) {
        self.clear_from_node();
        self.clear_to_node();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Arrow {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Arrow {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Screen {
    // message fields
    nodes: ::protobuf::RepeatedField<Node>,
    max_id: ::std::option::Option<u64>,
    arrows: ::protobuf::RepeatedField<Arrow>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Screen {}

impl Screen {
    pub fn new() -> Screen {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Screen {
        static mut instance: ::protobuf::lazy::Lazy<Screen> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Screen,
        };
        unsafe {
            instance.get(Screen::new)
        }
    }

    // repeated .void.Node nodes = 1;

    pub fn clear_nodes(&mut self) {
        self.nodes.clear();
    }

    // Param is passed by value, moved
    pub fn set_nodes(&mut self, v: ::protobuf::RepeatedField<Node>) {
        self.nodes = v;
    }

    // Mutable pointer to the field.
    pub fn mut_nodes(&mut self) -> &mut ::protobuf::RepeatedField<Node> {
        &mut self.nodes
    }

    // Take field
    pub fn take_nodes(&mut self) -> ::protobuf::RepeatedField<Node> {
        ::std::mem::replace(&mut self.nodes, ::protobuf::RepeatedField::new())
    }

    pub fn get_nodes(&self) -> &[Node] {
        &self.nodes
    }

    fn get_nodes_for_reflect(&self) -> &::protobuf::RepeatedField<Node> {
        &self.nodes
    }

    fn mut_nodes_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<Node> {
        &mut self.nodes
    }

    // required uint64 max_id = 2;

    pub fn clear_max_id(&mut self) {
        self.max_id = ::std::option::Option::None;
    }

    pub fn has_max_id(&self) -> bool {
        self.max_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_max_id(&mut self, v: u64) {
        self.max_id = ::std::option::Option::Some(v);
    }

    pub fn get_max_id(&self) -> u64 {
        self.max_id.unwrap_or(0)
    }

    fn get_max_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.max_id
    }

    fn mut_max_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.max_id
    }

    // repeated .void.Arrow arrows = 3;

    pub fn clear_arrows(&mut self) {
        self.arrows.clear();
    }

    // Param is passed by value, moved
    pub fn set_arrows(&mut self, v: ::protobuf::RepeatedField<Arrow>) {
        self.arrows = v;
    }

    // Mutable pointer to the field.
    pub fn mut_arrows(&mut self) -> &mut ::protobuf::RepeatedField<Arrow> {
        &mut self.arrows
    }

    // Take field
    pub fn take_arrows(&mut self) -> ::protobuf::RepeatedField<Arrow> {
        ::std::mem::replace(&mut self.arrows, ::protobuf::RepeatedField::new())
    }

    pub fn get_arrows(&self) -> &[Arrow] {
        &self.arrows
    }

    fn get_arrows_for_reflect(&self) -> &::protobuf::RepeatedField<Arrow> {
        &self.arrows
    }

    fn mut_arrows_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<Arrow> {
        &mut self.arrows
    }
}

impl ::protobuf::Message for Screen {
    fn is_initialized(&self) -> bool {
        if self.max_id.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.nodes)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.max_id = ::std::option::Option::Some(tmp);
                },
                3 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.arrows)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.nodes {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.max_id {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.arrows {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.nodes {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.max_id {
            os.write_uint64(2, v)?;
        };
        for v in &self.arrows {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Screen {
    fn new() -> Screen {
        Screen::new()
    }

    fn descriptor_static(_: ::std::option::Option<Screen>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Node>>(
                    "nodes",
                    Screen::get_nodes_for_reflect,
                    Screen::mut_nodes_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "max_id",
                    Screen::get_max_id_for_reflect,
                    Screen::mut_max_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Arrow>>(
                    "arrows",
                    Screen::get_arrows_for_reflect,
                    Screen::mut_arrows_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Screen>(
                    "Screen",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Screen {
    fn clear(&mut self) {
        self.clear_nodes();
        self.clear_max_id();
        self.clear_arrows();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Screen {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Screen {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x18, 0x69, 0x6e, 0x63, 0x6c, 0x75, 0x64, 0x65, 0x2f, 0x64, 0x61, 0x74, 0x61, 0x5f, 0x6d,
    0x6f, 0x64, 0x65, 0x6c, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x04, 0x76, 0x6f, 0x69, 0x64,
    0x22, 0x2d, 0x0a, 0x03, 0x54, 0x61, 0x67, 0x12, 0x10, 0x0a, 0x03, 0x6b, 0x65, 0x79, 0x18, 0x01,
    0x20, 0x02, 0x28, 0x09, 0x52, 0x03, 0x6b, 0x65, 0x79, 0x12, 0x14, 0x0a, 0x05, 0x76, 0x61, 0x6c,
    0x75, 0x65, 0x18, 0x02, 0x20, 0x02, 0x28, 0x09, 0x52, 0x05, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x22,
    0x29, 0x0a, 0x03, 0x47, 0x70, 0x73, 0x12, 0x10, 0x0a, 0x03, 0x6c, 0x61, 0x74, 0x18, 0x01, 0x20,
    0x02, 0x28, 0x02, 0x52, 0x03, 0x6c, 0x61, 0x74, 0x12, 0x10, 0x0a, 0x03, 0x6c, 0x6f, 0x6e, 0x18,
    0x02, 0x20, 0x02, 0x28, 0x02, 0x52, 0x03, 0x6c, 0x6f, 0x6e, 0x22, 0xa1, 0x01, 0x0a, 0x04, 0x4d,
    0x65, 0x74, 0x61, 0x12, 0x14, 0x0a, 0x05, 0x63, 0x74, 0x69, 0x6d, 0x65, 0x18, 0x01, 0x20, 0x02,
    0x28, 0x04, 0x52, 0x05, 0x63, 0x74, 0x69, 0x6d, 0x65, 0x12, 0x14, 0x0a, 0x05, 0x6d, 0x74, 0x69,
    0x6d, 0x65, 0x18, 0x02, 0x20, 0x02, 0x28, 0x04, 0x52, 0x05, 0x6d, 0x74, 0x69, 0x6d, 0x65, 0x12,
    0x1f, 0x0a, 0x0b, 0x66, 0x69, 0x6e, 0x69, 0x73, 0x68, 0x5f, 0x74, 0x69, 0x6d, 0x65, 0x18, 0x05,
    0x20, 0x01, 0x28, 0x04, 0x52, 0x0a, 0x66, 0x69, 0x6e, 0x69, 0x73, 0x68, 0x54, 0x69, 0x6d, 0x65,
    0x12, 0x1b, 0x0a, 0x03, 0x67, 0x70, 0x73, 0x18, 0x03, 0x20, 0x02, 0x28, 0x0b, 0x32, 0x09, 0x2e,
    0x76, 0x6f, 0x69, 0x64, 0x2e, 0x47, 0x70, 0x73, 0x52, 0x03, 0x67, 0x70, 0x73, 0x12, 0x1d, 0x0a,
    0x04, 0x74, 0x61, 0x67, 0x73, 0x18, 0x04, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x09, 0x2e, 0x76, 0x6f,
    0x69, 0x64, 0x2e, 0x54, 0x61, 0x67, 0x52, 0x04, 0x74, 0x61, 0x67, 0x73, 0x12, 0x10, 0x0a, 0x03,
    0x64, 0x75, 0x65, 0x18, 0x06, 0x20, 0x01, 0x28, 0x04, 0x52, 0x03, 0x64, 0x75, 0x65, 0x22, 0xda,
    0x02, 0x0a, 0x04, 0x4e, 0x6f, 0x64, 0x65, 0x12, 0x0e, 0x0a, 0x02, 0x69, 0x64, 0x18, 0x01, 0x20,
    0x02, 0x28, 0x04, 0x52, 0x02, 0x69, 0x64, 0x12, 0x1e, 0x0a, 0x04, 0x6d, 0x65, 0x74, 0x61, 0x18,
    0x02, 0x20, 0x02, 0x28, 0x0b, 0x32, 0x0a, 0x2e, 0x76, 0x6f, 0x69, 0x64, 0x2e, 0x4d, 0x65, 0x74,
    0x61, 0x52, 0x04, 0x6d, 0x65, 0x74, 0x61, 0x12, 0x12, 0x0a, 0x04, 0x74, 0x65, 0x78, 0x74, 0x18,
    0x03, 0x20, 0x02, 0x28, 0x09, 0x52, 0x04, 0x74, 0x65, 0x78, 0x74, 0x12, 0x1a, 0x0a, 0x08, 0x63,
    0x68, 0x69, 0x6c, 0x64, 0x72, 0x65, 0x6e, 0x18, 0x04, 0x20, 0x03, 0x28, 0x04, 0x52, 0x08, 0x63,
    0x68, 0x69, 0x6c, 0x64, 0x72, 0x65, 0x6e, 0x12, 0x1c, 0x0a, 0x09, 0x63, 0x6f, 0x6c, 0x6c, 0x61,
    0x70, 0x73, 0x65, 0x64, 0x18, 0x05, 0x20, 0x02, 0x28, 0x08, 0x52, 0x09, 0x63, 0x6f, 0x6c, 0x6c,
    0x61, 0x70, 0x73, 0x65, 0x64, 0x12, 0x1a, 0x0a, 0x08, 0x73, 0x74, 0x72, 0x69, 0x63, 0x6b, 0x65,
    0x6e, 0x18, 0x06, 0x20, 0x02, 0x28, 0x08, 0x52, 0x08, 0x73, 0x74, 0x72, 0x69, 0x63, 0x6b, 0x65,
    0x6e, 0x12, 0x23, 0x0a, 0x0d, 0x68, 0x69, 0x64, 0x65, 0x5f, 0x73, 0x74, 0x72, 0x69, 0x63, 0x6b,
    0x65, 0x6e, 0x18, 0x07, 0x20, 0x02, 0x28, 0x08, 0x52, 0x0c, 0x68, 0x69, 0x64, 0x65, 0x53, 0x74,
    0x72, 0x69, 0x63, 0x6b, 0x65, 0x6e, 0x12, 0x0c, 0x0a, 0x01, 0x78, 0x18, 0x08, 0x20, 0x02, 0x28,
    0x0d, 0x52, 0x01, 0x78, 0x12, 0x0c, 0x0a, 0x01, 0x79, 0x18, 0x09, 0x20, 0x02, 0x28, 0x0d, 0x52,
    0x01, 0x79, 0x12, 0x1a, 0x0a, 0x08, 0x73, 0x65, 0x6c, 0x65, 0x63, 0x74, 0x65, 0x64, 0x18, 0x0a,
    0x20, 0x02, 0x28, 0x08, 0x52, 0x08, 0x73, 0x65, 0x6c, 0x65, 0x63, 0x74, 0x65, 0x64, 0x12, 0x1b,
    0x0a, 0x09, 0x70, 0x61, 0x72, 0x65, 0x6e, 0x74, 0x5f, 0x69, 0x64, 0x18, 0x0b, 0x20, 0x02, 0x28,
    0x04, 0x52, 0x08, 0x70, 0x61, 0x72, 0x65, 0x6e, 0x74, 0x49, 0x64, 0x12, 0x1b, 0x0a, 0x09, 0x66,
    0x72, 0x65, 0x65, 0x5f, 0x74, 0x65, 0x78, 0x74, 0x18, 0x0c, 0x20, 0x01, 0x28, 0x09, 0x52, 0x08,
    0x66, 0x72, 0x65, 0x65, 0x54, 0x65, 0x78, 0x74, 0x12, 0x21, 0x0a, 0x0c, 0x61, 0x75, 0x74, 0x6f,
    0x5f, 0x61, 0x72, 0x72, 0x61, 0x6e, 0x67, 0x65, 0x18, 0x0d, 0x20, 0x02, 0x28, 0x08, 0x52, 0x0b,
    0x61, 0x75, 0x74, 0x6f, 0x41, 0x72, 0x72, 0x61, 0x6e, 0x67, 0x65, 0x22, 0x3d, 0x0a, 0x05, 0x41,
    0x72, 0x72, 0x6f, 0x77, 0x12, 0x1b, 0x0a, 0x09, 0x66, 0x72, 0x6f, 0x6d, 0x5f, 0x6e, 0x6f, 0x64,
    0x65, 0x18, 0x01, 0x20, 0x02, 0x28, 0x04, 0x52, 0x08, 0x66, 0x72, 0x6f, 0x6d, 0x4e, 0x6f, 0x64,
    0x65, 0x12, 0x17, 0x0a, 0x07, 0x74, 0x6f, 0x5f, 0x6e, 0x6f, 0x64, 0x65, 0x18, 0x02, 0x20, 0x02,
    0x28, 0x04, 0x52, 0x06, 0x74, 0x6f, 0x4e, 0x6f, 0x64, 0x65, 0x22, 0x66, 0x0a, 0x06, 0x53, 0x63,
    0x72, 0x65, 0x65, 0x6e, 0x12, 0x20, 0x0a, 0x05, 0x6e, 0x6f, 0x64, 0x65, 0x73, 0x18, 0x01, 0x20,
    0x03, 0x28, 0x0b, 0x32, 0x0a, 0x2e, 0x76, 0x6f, 0x69, 0x64, 0x2e, 0x4e, 0x6f, 0x64, 0x65, 0x52,
    0x05, 0x6e, 0x6f, 0x64, 0x65, 0x73, 0x12, 0x15, 0x0a, 0x06, 0x6d, 0x61, 0x78, 0x5f, 0x69, 0x64,
    0x18, 0x02, 0x20, 0x02, 0x28, 0x04, 0x52, 0x05, 0x6d, 0x61, 0x78, 0x49, 0x64, 0x12, 0x23, 0x0a,
    0x06, 0x61, 0x72, 0x72, 0x6f, 0x77, 0x73, 0x18, 0x03, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x0b, 0x2e,
    0x76, 0x6f, 0x69, 0x64, 0x2e, 0x41, 0x72, 0x72, 0x6f, 0x77, 0x52, 0x06, 0x61, 0x72, 0x72, 0x6f,
    0x77, 0x73, 0x4a, 0xb8, 0x10, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x30, 0x01, 0x0a, 0x08, 0x0a,
    0x01, 0x0c, 0x12, 0x03, 0x00, 0x00, 0x12, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x02, 0x08,
    0x0c, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x04, 0x00, 0x07, 0x01, 0x0a, 0x0a, 0x0a,
    0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x04, 0x08, 0x0b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02,
    0x00, 0x12, 0x03, 0x05, 0x02, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04, 0x12,
    0x03, 0x05, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x05,
    0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x05, 0x12, 0x15,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x05, 0x18, 0x19, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x06, 0x02, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x01, 0x04, 0x12, 0x03, 0x06, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x01, 0x05, 0x12, 0x03, 0x06, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01,
    0x12, 0x03, 0x06, 0x12, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03,
    0x06, 0x1a, 0x1b, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x01, 0x12, 0x04, 0x09, 0x00, 0x0c, 0x01, 0x0a,
    0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x03, 0x09, 0x08, 0x0b, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x01, 0x02, 0x00, 0x12, 0x03, 0x0a, 0x02, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00,
    0x04, 0x12, 0x03, 0x0a, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x05, 0x12,
    0x03, 0x0a, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0a,
    0x11, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x03, 0x12, 0x03, 0x0a, 0x17, 0x18,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x01, 0x12, 0x03, 0x0b, 0x02, 0x19, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x01, 0x04, 0x12, 0x03, 0x0b, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x01, 0x05, 0x12, 0x03, 0x0b, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x01, 0x01, 0x12, 0x03, 0x0b, 0x11, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x03,
    0x12, 0x03, 0x0b, 0x17, 0x18, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x02, 0x12, 0x04, 0x0e, 0x00, 0x15,
    0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x02, 0x01, 0x12, 0x03, 0x0e, 0x08, 0x0c, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x02, 0x02, 0x00, 0x12, 0x03, 0x0f, 0x02, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02,
    0x02, 0x00, 0x04, 0x12, 0x03, 0x0f, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00,
    0x05, 0x12, 0x03, 0x0f, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x01, 0x12,
    0x03, 0x0f, 0x12, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x03, 0x12, 0x03, 0x0f,
    0x1a, 0x1b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x01, 0x12, 0x03, 0x10, 0x02, 0x1c, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x04, 0x12, 0x03, 0x10, 0x02, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x02, 0x02, 0x01, 0x05, 0x12, 0x03, 0x10, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x02, 0x02, 0x01, 0x01, 0x12, 0x03, 0x10, 0x12, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02,
    0x01, 0x03, 0x12, 0x03, 0x10, 0x1a, 0x1b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x02, 0x12,
    0x03, 0x11, 0x02, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x04, 0x12, 0x03, 0x11,
    0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x05, 0x12, 0x03, 0x11, 0x0b, 0x11,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x01, 0x12, 0x03, 0x11, 0x12, 0x1d, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x03, 0x12, 0x03, 0x11, 0x20, 0x21, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x02, 0x02, 0x03, 0x12, 0x03, 0x12, 0x02, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02,
    0x03, 0x04, 0x12, 0x03, 0x12, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x03, 0x06,
    0x12, 0x03, 0x12, 0x0b, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x03, 0x01, 0x12, 0x03,
    0x12, 0x0f, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x03, 0x03, 0x12, 0x03, 0x12, 0x15,
    0x16, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x04, 0x12, 0x03, 0x13, 0x02, 0x18, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x02, 0x02, 0x04, 0x04, 0x12, 0x03, 0x13, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x02, 0x02, 0x04, 0x06, 0x12, 0x03, 0x13, 0x0b, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02,
    0x02, 0x04, 0x01, 0x12, 0x03, 0x13, 0x0f, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x04,
    0x03, 0x12, 0x03, 0x13, 0x16, 0x17, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x05, 0x12, 0x03,
    0x14, 0x02, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x05, 0x04, 0x12, 0x03, 0x14, 0x02,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x05, 0x05, 0x12, 0x03, 0x14, 0x0b, 0x11, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x05, 0x01, 0x12, 0x03, 0x14, 0x12, 0x15, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x02, 0x02, 0x05, 0x03, 0x12, 0x03, 0x14, 0x18, 0x19, 0x0a, 0x0a, 0x0a, 0x02, 0x04,
    0x03, 0x12, 0x04, 0x17, 0x00, 0x25, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x03, 0x01, 0x12, 0x03,
    0x17, 0x08, 0x0c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x00, 0x12, 0x03, 0x18, 0x02, 0x19,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x04, 0x12, 0x03, 0x18, 0x02, 0x0a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x05, 0x12, 0x03, 0x18, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x03, 0x02, 0x00, 0x01, 0x12, 0x03, 0x18, 0x12, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03,
    0x02, 0x00, 0x03, 0x12, 0x03, 0x18, 0x17, 0x18, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x01,
    0x12, 0x03, 0x19, 0x02, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x04, 0x12, 0x03,
    0x19, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x06, 0x12, 0x03, 0x19, 0x0b,
    0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x01, 0x12, 0x03, 0x19, 0x10, 0x14, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x03, 0x12, 0x03, 0x19, 0x17, 0x18, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x03, 0x02, 0x02, 0x12, 0x03, 0x1a, 0x02, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03,
    0x02, 0x02, 0x04, 0x12, 0x03, 0x1a, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x02,
    0x05, 0x12, 0x03, 0x1a, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x02, 0x01, 0x12,
    0x03, 0x1a, 0x12, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x02, 0x03, 0x12, 0x03, 0x1a,
    0x19, 0x1a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x03, 0x12, 0x03, 0x1b, 0x02, 0x1f, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x03, 0x04, 0x12, 0x03, 0x1b, 0x02, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x03, 0x02, 0x03, 0x05, 0x12, 0x03, 0x1b, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x03, 0x02, 0x03, 0x01, 0x12, 0x03, 0x1b, 0x12, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02,
    0x03, 0x03, 0x12, 0x03, 0x1b, 0x1d, 0x1e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x04, 0x12,
    0x03, 0x1c, 0x02, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x04, 0x04, 0x12, 0x03, 0x1c,
    0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x04, 0x05, 0x12, 0x03, 0x1c, 0x0b, 0x0f,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x04, 0x01, 0x12, 0x03, 0x1c, 0x10, 0x19, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x03, 0x02, 0x04, 0x03, 0x12, 0x03, 0x1c, 0x1c, 0x1d, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x03, 0x02, 0x05, 0x12, 0x03, 0x1d, 0x02, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02,
    0x05, 0x04, 0x12, 0x03, 0x1d, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x05, 0x05,
    0x12, 0x03, 0x1d, 0x0b, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x05, 0x01, 0x12, 0x03,
    0x1d, 0x10, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x05, 0x03, 0x12, 0x03, 0x1d, 0x1b,
    0x1c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x06, 0x12, 0x03, 0x1e, 0x02, 0x22, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x03, 0x02, 0x06, 0x04, 0x12, 0x03, 0x1e, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x03, 0x02, 0x06, 0x05, 0x12, 0x03, 0x1e, 0x0b, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03,
    0x02, 0x06, 0x01, 0x12, 0x03, 0x1e, 0x10, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x06,
    0x03, 0x12, 0x03, 0x1e, 0x20, 0x21, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x07, 0x12, 0x03,
    0x1f, 0x02, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x07, 0x04, 0x12, 0x03, 0x1f, 0x02,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x07, 0x05, 0x12, 0x03, 0x1f, 0x0b, 0x11, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x07, 0x01, 0x12, 0x03, 0x1f, 0x12, 0x13, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x03, 0x02, 0x07, 0x03, 0x12, 0x03, 0x1f, 0x16, 0x17, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x03, 0x02, 0x08, 0x12, 0x03, 0x20, 0x02, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x08,
    0x04, 0x12, 0x03, 0x20, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x08, 0x05, 0x12,
    0x03, 0x20, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x08, 0x01, 0x12, 0x03, 0x20,
    0x12, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x08, 0x03, 0x12, 0x03, 0x20, 0x16, 0x17,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x09, 0x12, 0x03, 0x21, 0x02, 0x1e, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x03, 0x02, 0x09, 0x04, 0x12, 0x03, 0x21, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x03, 0x02, 0x09, 0x05, 0x12, 0x03, 0x21, 0x0b, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02,
    0x09, 0x01, 0x12, 0x03, 0x21, 0x10, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x09, 0x03,
    0x12, 0x03, 0x21, 0x1b, 0x1d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x0a, 0x12, 0x03, 0x22,
    0x02, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x0a, 0x04, 0x12, 0x03, 0x22, 0x02, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x0a, 0x05, 0x12, 0x03, 0x22, 0x0b, 0x11, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x03, 0x02, 0x0a, 0x01, 0x12, 0x03, 0x22, 0x12, 0x1b, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x03, 0x02, 0x0a, 0x03, 0x12, 0x03, 0x22, 0x1e, 0x20, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03,
    0x02, 0x0b, 0x12, 0x03, 0x23, 0x02, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x0b, 0x04,
    0x12, 0x03, 0x23, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x0b, 0x05, 0x12, 0x03,
    0x23, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x0b, 0x01, 0x12, 0x03, 0x23, 0x12,
    0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x0b, 0x03, 0x12, 0x03, 0x23, 0x1e, 0x20, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x0c, 0x12, 0x03, 0x24, 0x02, 0x22, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x03, 0x02, 0x0c, 0x04, 0x12, 0x03, 0x24, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03,
    0x02, 0x0c, 0x05, 0x12, 0x03, 0x24, 0x0b, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x0c,
    0x01, 0x12, 0x03, 0x24, 0x10, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x0c, 0x03, 0x12,
    0x03, 0x24, 0x1f, 0x21, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x04, 0x12, 0x04, 0x27, 0x00, 0x2a, 0x01,
    0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x04, 0x01, 0x12, 0x03, 0x27, 0x08, 0x0d, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x04, 0x02, 0x00, 0x12, 0x03, 0x28, 0x02, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02,
    0x00, 0x04, 0x12, 0x03, 0x28, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x05,
    0x12, 0x03, 0x28, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x01, 0x12, 0x03,
    0x28, 0x12, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x03, 0x12, 0x03, 0x28, 0x1e,
    0x1f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x01, 0x12, 0x03, 0x29, 0x02, 0x1e, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x04, 0x12, 0x03, 0x29, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x04, 0x02, 0x01, 0x05, 0x12, 0x03, 0x29, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04,
    0x02, 0x01, 0x01, 0x12, 0x03, 0x29, 0x12, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01,
    0x03, 0x12, 0x03, 0x29, 0x1c, 0x1d, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x05, 0x12, 0x04, 0x2c, 0x00,
    0x30, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x05, 0x01, 0x12, 0x03, 0x2c, 0x08, 0x0e, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x05, 0x02, 0x00, 0x12, 0x03, 0x2d, 0x02, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x05, 0x02, 0x00, 0x04, 0x12, 0x03, 0x2d, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02,
    0x00, 0x06, 0x12, 0x03, 0x2d, 0x0b, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x2d, 0x10, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x03, 0x12, 0x03,
    0x2d, 0x18, 0x19, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x01, 0x12, 0x03, 0x2e, 0x02, 0x1d,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x04, 0x12, 0x03, 0x2e, 0x02, 0x0a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x05, 0x12, 0x03, 0x2e, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x05, 0x02, 0x01, 0x01, 0x12, 0x03, 0x2e, 0x12, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05,
    0x02, 0x01, 0x03, 0x12, 0x03, 0x2e, 0x1b, 0x1c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x02,
    0x12, 0x03, 0x2f, 0x02, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x02, 0x04, 0x12, 0x03,
    0x2f, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x02, 0x06, 0x12, 0x03, 0x2f, 0x0b,
    0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x02, 0x01, 0x12, 0x03, 0x2f, 0x11, 0x17, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x02, 0x03, 0x12, 0x03, 0x2f, 0x1a, 0x1b,
];

static mut file_descriptor_proto_lazy: ::protobuf::lazy::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::lazy::Lazy {
    lock: ::protobuf::lazy::ONCE_INIT,
    ptr: 0 as *const ::protobuf::descriptor::FileDescriptorProto,
};

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    unsafe {
        file_descriptor_proto_lazy.get(|| {
            parse_descriptor_proto()
        })
    }
}
