// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

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

#[derive(Clone,Default)]
pub struct Tag {
    // message fields
    key: ::protobuf::SingularField<::std::string::String>,
    value: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
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
            instance.get(|| {
                Tag {
                    key: ::protobuf::SingularField::none(),
                    value: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
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
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.key));
                },
                2 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.value));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.key {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        for value in &self.value {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.key.as_ref() {
            try!(os.write_string(1, &v));
        };
        if let Some(v) = self.value.as_ref() {
            try!(os.write_string(2, &v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<Tag>()
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
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "key",
                    Tag::has_key,
                    Tag::get_key,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "value",
                    Tag::has_value,
                    Tag::get_value,
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

impl ::std::cmp::PartialEq for Tag {
    fn eq(&self, other: &Tag) -> bool {
        self.key == other.key &&
        self.value == other.value &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Tag {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct Gps {
    // message fields
    lat: ::std::option::Option<f32>,
    lon: ::std::option::Option<f32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
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
            instance.get(|| {
                Gps {
                    lat: ::std::option::Option::None,
                    lon: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
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
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_float());
                    self.lat = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_float());
                    self.lon = ::std::option::Option::Some(tmp);
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if self.lat.is_some() {
            my_size += 5;
        };
        if self.lon.is_some() {
            my_size += 5;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.lat {
            try!(os.write_float(1, v));
        };
        if let Some(v) = self.lon {
            try!(os.write_float(2, v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<Gps>()
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
                fields.push(::protobuf::reflect::accessor::make_singular_f32_accessor(
                    "lat",
                    Gps::has_lat,
                    Gps::get_lat,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f32_accessor(
                    "lon",
                    Gps::has_lon,
                    Gps::get_lon,
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

impl ::std::cmp::PartialEq for Gps {
    fn eq(&self, other: &Gps) -> bool {
        self.lat == other.lat &&
        self.lon == other.lon &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Gps {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct Meta {
    // message fields
    ctime: ::std::option::Option<u64>,
    mtime: ::std::option::Option<u64>,
    finish_time: ::std::option::Option<u64>,
    gps: ::protobuf::SingularPtrField<Gps>,
    tags: ::protobuf::RepeatedField<Tag>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
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
            instance.get(|| {
                Meta {
                    ctime: ::std::option::Option::None,
                    mtime: ::std::option::Option::None,
                    finish_time: ::std::option::Option::None,
                    gps: ::protobuf::SingularPtrField::none(),
                    tags: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
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

    // required .climate.Gps gps = 3;

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

    // repeated .climate.Tag tags = 4;

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
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint64());
                    self.ctime = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint64());
                    self.mtime = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint64());
                    self.finish_time = ::std::option::Option::Some(tmp);
                },
                3 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.gps));
                },
                4 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.tags));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.ctime {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.mtime {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.finish_time {
            my_size += ::protobuf::rt::value_size(5, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.gps {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.tags {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.ctime {
            try!(os.write_uint64(1, v));
        };
        if let Some(v) = self.mtime {
            try!(os.write_uint64(2, v));
        };
        if let Some(v) = self.finish_time {
            try!(os.write_uint64(5, v));
        };
        if let Some(v) = self.gps.as_ref() {
            try!(os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        for v in &self.tags {
            try!(os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<Meta>()
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
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "ctime",
                    Meta::has_ctime,
                    Meta::get_ctime,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "mtime",
                    Meta::has_mtime,
                    Meta::get_mtime,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "finish_time",
                    Meta::has_finish_time,
                    Meta::get_finish_time,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "gps",
                    Meta::has_gps,
                    Meta::get_gps,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "tags",
                    Meta::get_tags,
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
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Meta {
    fn eq(&self, other: &Meta) -> bool {
        self.ctime == other.ctime &&
        self.mtime == other.mtime &&
        self.finish_time == other.finish_time &&
        self.gps == other.gps &&
        self.tags == other.tags &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Meta {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
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
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
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
            instance.get(|| {
                Node {
                    id: ::std::option::Option::None,
                    meta: ::protobuf::SingularPtrField::none(),
                    text: ::protobuf::SingularField::none(),
                    children: ::std::vec::Vec::new(),
                    collapsed: ::std::option::Option::None,
                    stricken: ::std::option::Option::None,
                    hide_stricken: ::std::option::Option::None,
                    x: ::std::option::Option::None,
                    y: ::std::option::Option::None,
                    selected: ::std::option::Option::None,
                    parent_id: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
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

    // required .climate.Meta meta = 2;

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
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint64());
                    self.id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.meta));
                },
                3 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.text));
                },
                4 => {
                    try!(::protobuf::rt::read_repeated_uint64_into(wire_type, is, &mut self.children));
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.collapsed = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.stricken = ::std::option::Option::Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.hide_stricken = ::std::option::Option::Some(tmp);
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint32());
                    self.x = ::std::option::Option::Some(tmp);
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint32());
                    self.y = ::std::option::Option::Some(tmp);
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.selected = ::std::option::Option::Some(tmp);
                },
                11 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint64());
                    self.parent_id = ::std::option::Option::Some(tmp);
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.id {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.meta {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.text {
            my_size += ::protobuf::rt::string_size(3, &value);
        };
        for value in &self.children {
            my_size += ::protobuf::rt::value_size(4, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        if self.collapsed.is_some() {
            my_size += 2;
        };
        if self.stricken.is_some() {
            my_size += 2;
        };
        if self.hide_stricken.is_some() {
            my_size += 2;
        };
        for value in &self.x {
            my_size += ::protobuf::rt::value_size(8, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.y {
            my_size += ::protobuf::rt::value_size(9, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        if self.selected.is_some() {
            my_size += 2;
        };
        for value in &self.parent_id {
            my_size += ::protobuf::rt::value_size(11, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.id {
            try!(os.write_uint64(1, v));
        };
        if let Some(v) = self.meta.as_ref() {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.text.as_ref() {
            try!(os.write_string(3, &v));
        };
        for v in &self.children {
            try!(os.write_uint64(4, *v));
        };
        if let Some(v) = self.collapsed {
            try!(os.write_bool(5, v));
        };
        if let Some(v) = self.stricken {
            try!(os.write_bool(6, v));
        };
        if let Some(v) = self.hide_stricken {
            try!(os.write_bool(7, v));
        };
        if let Some(v) = self.x {
            try!(os.write_uint32(8, v));
        };
        if let Some(v) = self.y {
            try!(os.write_uint32(9, v));
        };
        if let Some(v) = self.selected {
            try!(os.write_bool(10, v));
        };
        if let Some(v) = self.parent_id {
            try!(os.write_uint64(11, v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<Node>()
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
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "id",
                    Node::has_id,
                    Node::get_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "meta",
                    Node::has_meta,
                    Node::get_meta,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "text",
                    Node::has_text,
                    Node::get_text,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_u64_accessor(
                    "children",
                    Node::get_children,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "collapsed",
                    Node::has_collapsed,
                    Node::get_collapsed,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "stricken",
                    Node::has_stricken,
                    Node::get_stricken,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "hide_stricken",
                    Node::has_hide_stricken,
                    Node::get_hide_stricken,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "x",
                    Node::has_x,
                    Node::get_x,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "y",
                    Node::has_y,
                    Node::get_y,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "selected",
                    Node::has_selected,
                    Node::get_selected,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "parent_id",
                    Node::has_parent_id,
                    Node::get_parent_id,
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
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Node {
    fn eq(&self, other: &Node) -> bool {
        self.id == other.id &&
        self.meta == other.meta &&
        self.text == other.text &&
        self.children == other.children &&
        self.collapsed == other.collapsed &&
        self.stricken == other.stricken &&
        self.hide_stricken == other.hide_stricken &&
        self.x == other.x &&
        self.y == other.y &&
        self.selected == other.selected &&
        self.parent_id == other.parent_id &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Node {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct Arrow {
    // message fields
    from_node: ::std::option::Option<u64>,
    to_node: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
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
            instance.get(|| {
                Arrow {
                    from_node: ::std::option::Option::None,
                    to_node: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
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
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint64());
                    self.from_node = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint64());
                    self.to_node = ::std::option::Option::Some(tmp);
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.from_node {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.to_node {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.from_node {
            try!(os.write_uint64(1, v));
        };
        if let Some(v) = self.to_node {
            try!(os.write_uint64(2, v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<Arrow>()
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
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "from_node",
                    Arrow::has_from_node,
                    Arrow::get_from_node,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "to_node",
                    Arrow::has_to_node,
                    Arrow::get_to_node,
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

impl ::std::cmp::PartialEq for Arrow {
    fn eq(&self, other: &Arrow) -> bool {
        self.from_node == other.from_node &&
        self.to_node == other.to_node &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Arrow {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct Screen {
    // message fields
    nodes: ::protobuf::RepeatedField<Node>,
    max_id: ::std::option::Option<u64>,
    arrows: ::protobuf::RepeatedField<Arrow>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
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
            instance.get(|| {
                Screen {
                    nodes: ::protobuf::RepeatedField::new(),
                    max_id: ::std::option::Option::None,
                    arrows: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // repeated .climate.Node nodes = 1;

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

    // repeated .climate.Arrow arrows = 3;

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
}

impl ::protobuf::Message for Screen {
    fn is_initialized(&self) -> bool {
        if self.max_id.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.nodes));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint64());
                    self.max_id = ::std::option::Option::Some(tmp);
                },
                3 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.arrows));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
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
        for value in &self.max_id {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
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
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.max_id {
            try!(os.write_uint64(2, v));
        };
        for v in &self.arrows {
            try!(os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<Screen>()
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
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "nodes",
                    Screen::get_nodes,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "max_id",
                    Screen::has_max_id,
                    Screen::get_max_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "arrows",
                    Screen::get_arrows,
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

impl ::std::cmp::PartialEq for Screen {
    fn eq(&self, other: &Screen) -> bool {
        self.nodes == other.nodes &&
        self.max_id == other.max_id &&
        self.arrows == other.arrows &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Screen {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x18, 0x69, 0x6e, 0x63, 0x6c, 0x75, 0x64, 0x65, 0x2f, 0x64, 0x61, 0x74, 0x61, 0x5f, 0x6d,
    0x6f, 0x64, 0x65, 0x6c, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x07, 0x63, 0x6c, 0x69, 0x6d,
    0x61, 0x74, 0x65, 0x22, 0x21, 0x0a, 0x03, 0x54, 0x61, 0x67, 0x12, 0x0b, 0x0a, 0x03, 0x6b, 0x65,
    0x79, 0x18, 0x01, 0x20, 0x02, 0x28, 0x09, 0x12, 0x0d, 0x0a, 0x05, 0x76, 0x61, 0x6c, 0x75, 0x65,
    0x18, 0x02, 0x20, 0x02, 0x28, 0x09, 0x22, 0x1f, 0x0a, 0x03, 0x47, 0x70, 0x73, 0x12, 0x0b, 0x0a,
    0x03, 0x6c, 0x61, 0x74, 0x18, 0x01, 0x20, 0x02, 0x28, 0x02, 0x12, 0x0b, 0x0a, 0x03, 0x6c, 0x6f,
    0x6e, 0x18, 0x02, 0x20, 0x02, 0x28, 0x02, 0x22, 0x70, 0x0a, 0x04, 0x4d, 0x65, 0x74, 0x61, 0x12,
    0x0d, 0x0a, 0x05, 0x63, 0x74, 0x69, 0x6d, 0x65, 0x18, 0x01, 0x20, 0x02, 0x28, 0x04, 0x12, 0x0d,
    0x0a, 0x05, 0x6d, 0x74, 0x69, 0x6d, 0x65, 0x18, 0x02, 0x20, 0x02, 0x28, 0x04, 0x12, 0x13, 0x0a,
    0x0b, 0x66, 0x69, 0x6e, 0x69, 0x73, 0x68, 0x5f, 0x74, 0x69, 0x6d, 0x65, 0x18, 0x05, 0x20, 0x01,
    0x28, 0x04, 0x12, 0x19, 0x0a, 0x03, 0x67, 0x70, 0x73, 0x18, 0x03, 0x20, 0x02, 0x28, 0x0b, 0x32,
    0x0c, 0x2e, 0x63, 0x6c, 0x69, 0x6d, 0x61, 0x74, 0x65, 0x2e, 0x47, 0x70, 0x73, 0x12, 0x1a, 0x0a,
    0x04, 0x74, 0x61, 0x67, 0x73, 0x18, 0x04, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x0c, 0x2e, 0x63, 0x6c,
    0x69, 0x6d, 0x61, 0x74, 0x65, 0x2e, 0x54, 0x61, 0x67, 0x22, 0xc6, 0x01, 0x0a, 0x04, 0x4e, 0x6f,
    0x64, 0x65, 0x12, 0x0a, 0x0a, 0x02, 0x69, 0x64, 0x18, 0x01, 0x20, 0x02, 0x28, 0x04, 0x12, 0x1b,
    0x0a, 0x04, 0x6d, 0x65, 0x74, 0x61, 0x18, 0x02, 0x20, 0x02, 0x28, 0x0b, 0x32, 0x0d, 0x2e, 0x63,
    0x6c, 0x69, 0x6d, 0x61, 0x74, 0x65, 0x2e, 0x4d, 0x65, 0x74, 0x61, 0x12, 0x0c, 0x0a, 0x04, 0x74,
    0x65, 0x78, 0x74, 0x18, 0x03, 0x20, 0x02, 0x28, 0x09, 0x12, 0x10, 0x0a, 0x08, 0x63, 0x68, 0x69,
    0x6c, 0x64, 0x72, 0x65, 0x6e, 0x18, 0x04, 0x20, 0x03, 0x28, 0x04, 0x12, 0x11, 0x0a, 0x09, 0x63,
    0x6f, 0x6c, 0x6c, 0x61, 0x70, 0x73, 0x65, 0x64, 0x18, 0x05, 0x20, 0x02, 0x28, 0x08, 0x12, 0x10,
    0x0a, 0x08, 0x73, 0x74, 0x72, 0x69, 0x63, 0x6b, 0x65, 0x6e, 0x18, 0x06, 0x20, 0x02, 0x28, 0x08,
    0x12, 0x15, 0x0a, 0x0d, 0x68, 0x69, 0x64, 0x65, 0x5f, 0x73, 0x74, 0x72, 0x69, 0x63, 0x6b, 0x65,
    0x6e, 0x18, 0x07, 0x20, 0x02, 0x28, 0x08, 0x12, 0x09, 0x0a, 0x01, 0x78, 0x18, 0x08, 0x20, 0x02,
    0x28, 0x0d, 0x12, 0x09, 0x0a, 0x01, 0x79, 0x18, 0x09, 0x20, 0x02, 0x28, 0x0d, 0x12, 0x10, 0x0a,
    0x08, 0x73, 0x65, 0x6c, 0x65, 0x63, 0x74, 0x65, 0x64, 0x18, 0x0a, 0x20, 0x02, 0x28, 0x08, 0x12,
    0x11, 0x0a, 0x09, 0x70, 0x61, 0x72, 0x65, 0x6e, 0x74, 0x5f, 0x69, 0x64, 0x18, 0x0b, 0x20, 0x02,
    0x28, 0x04, 0x22, 0x2b, 0x0a, 0x05, 0x41, 0x72, 0x72, 0x6f, 0x77, 0x12, 0x11, 0x0a, 0x09, 0x66,
    0x72, 0x6f, 0x6d, 0x5f, 0x6e, 0x6f, 0x64, 0x65, 0x18, 0x01, 0x20, 0x02, 0x28, 0x04, 0x12, 0x0f,
    0x0a, 0x07, 0x74, 0x6f, 0x5f, 0x6e, 0x6f, 0x64, 0x65, 0x18, 0x02, 0x20, 0x02, 0x28, 0x04, 0x22,
    0x56, 0x0a, 0x06, 0x53, 0x63, 0x72, 0x65, 0x65, 0x6e, 0x12, 0x1c, 0x0a, 0x05, 0x6e, 0x6f, 0x64,
    0x65, 0x73, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x0d, 0x2e, 0x63, 0x6c, 0x69, 0x6d, 0x61,
    0x74, 0x65, 0x2e, 0x4e, 0x6f, 0x64, 0x65, 0x12, 0x0e, 0x0a, 0x06, 0x6d, 0x61, 0x78, 0x5f, 0x69,
    0x64, 0x18, 0x02, 0x20, 0x02, 0x28, 0x04, 0x12, 0x1e, 0x0a, 0x06, 0x61, 0x72, 0x72, 0x6f, 0x77,
    0x73, 0x18, 0x03, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x0e, 0x2e, 0x63, 0x6c, 0x69, 0x6d, 0x61, 0x74,
    0x65, 0x2e, 0x41, 0x72, 0x72, 0x6f, 0x77, 0x4a, 0xa0, 0x0f, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00,
    0x2e, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x02, 0x08, 0x0f, 0x0a, 0x0a, 0x0a, 0x02,
    0x04, 0x00, 0x12, 0x04, 0x04, 0x00, 0x07, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12,
    0x03, 0x04, 0x08, 0x0b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x05, 0x02,
    0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04, 0x12, 0x03, 0x05, 0x02, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x05, 0x0b, 0x11, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x05, 0x12, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x05, 0x18, 0x19, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02,
    0x01, 0x12, 0x03, 0x06, 0x02, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x04, 0x12,
    0x03, 0x06, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x05, 0x12, 0x03, 0x06,
    0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x06, 0x12, 0x17,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x06, 0x1a, 0x1b, 0x0a, 0x0a,
    0x0a, 0x02, 0x04, 0x01, 0x12, 0x04, 0x09, 0x00, 0x0c, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01,
    0x01, 0x12, 0x03, 0x09, 0x08, 0x0b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x00, 0x12, 0x03,
    0x0a, 0x02, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x04, 0x12, 0x03, 0x0a, 0x02,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x05, 0x12, 0x03, 0x0a, 0x0b, 0x10, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0a, 0x11, 0x14, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x00, 0x03, 0x12, 0x03, 0x0a, 0x17, 0x18, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x01, 0x02, 0x01, 0x12, 0x03, 0x0b, 0x02, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01,
    0x04, 0x12, 0x03, 0x0b, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x05, 0x12,
    0x03, 0x0b, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x01, 0x12, 0x03, 0x0b,
    0x11, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x03, 0x12, 0x03, 0x0b, 0x17, 0x18,
    0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x02, 0x12, 0x04, 0x0e, 0x00, 0x14, 0x01, 0x0a, 0x0a, 0x0a, 0x03,
    0x04, 0x02, 0x01, 0x12, 0x03, 0x0e, 0x08, 0x0c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x00,
    0x12, 0x03, 0x0f, 0x02, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x04, 0x12, 0x03,
    0x0f, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x05, 0x12, 0x03, 0x0f, 0x0b,
    0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0f, 0x12, 0x17, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x03, 0x12, 0x03, 0x0f, 0x1a, 0x1b, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x02, 0x02, 0x01, 0x12, 0x03, 0x10, 0x02, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02,
    0x02, 0x01, 0x04, 0x12, 0x03, 0x10, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01,
    0x05, 0x12, 0x03, 0x10, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x01, 0x12,
    0x03, 0x10, 0x12, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x03, 0x12, 0x03, 0x10,
    0x1a, 0x1b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x02, 0x12, 0x03, 0x11, 0x02, 0x22, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x04, 0x12, 0x03, 0x11, 0x02, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x02, 0x02, 0x02, 0x05, 0x12, 0x03, 0x11, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x02, 0x02, 0x02, 0x01, 0x12, 0x03, 0x11, 0x12, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02,
    0x02, 0x03, 0x12, 0x03, 0x11, 0x20, 0x21, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x03, 0x12,
    0x03, 0x12, 0x02, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x03, 0x04, 0x12, 0x03, 0x12,
    0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x03, 0x06, 0x12, 0x03, 0x12, 0x0b, 0x0e,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x03, 0x01, 0x12, 0x03, 0x12, 0x0f, 0x12, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x02, 0x02, 0x03, 0x03, 0x12, 0x03, 0x12, 0x15, 0x16, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x02, 0x02, 0x04, 0x12, 0x03, 0x13, 0x02, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02,
    0x04, 0x04, 0x12, 0x03, 0x13, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x04, 0x06,
    0x12, 0x03, 0x13, 0x0b, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x04, 0x01, 0x12, 0x03,
    0x13, 0x0f, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x04, 0x03, 0x12, 0x03, 0x13, 0x16,
    0x17, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x03, 0x12, 0x04, 0x16, 0x00, 0x22, 0x01, 0x0a, 0x0a, 0x0a,
    0x03, 0x04, 0x03, 0x01, 0x12, 0x03, 0x16, 0x08, 0x0c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02,
    0x00, 0x12, 0x03, 0x17, 0x02, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x04, 0x12,
    0x03, 0x17, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x05, 0x12, 0x03, 0x17,
    0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x01, 0x12, 0x03, 0x17, 0x12, 0x14,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x03, 0x12, 0x03, 0x17, 0x17, 0x18, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x03, 0x02, 0x01, 0x12, 0x03, 0x18, 0x02, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x03, 0x02, 0x01, 0x04, 0x12, 0x03, 0x18, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02,
    0x01, 0x06, 0x12, 0x03, 0x18, 0x0b, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x01,
    0x12, 0x03, 0x18, 0x10, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x03, 0x12, 0x03,
    0x18, 0x17, 0x18, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x02, 0x12, 0x03, 0x19, 0x02, 0x1b,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x02, 0x04, 0x12, 0x03, 0x19, 0x02, 0x0a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x03, 0x02, 0x02, 0x05, 0x12, 0x03, 0x19, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x03, 0x02, 0x02, 0x01, 0x12, 0x03, 0x19, 0x12, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03,
    0x02, 0x02, 0x03, 0x12, 0x03, 0x19, 0x19, 0x1a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x03,
    0x12, 0x03, 0x1a, 0x02, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x03, 0x04, 0x12, 0x03,
    0x1a, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x03, 0x05, 0x12, 0x03, 0x1a, 0x0b,
    0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x03, 0x01, 0x12, 0x03, 0x1a, 0x12, 0x1a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x03, 0x03, 0x12, 0x03, 0x1a, 0x1d, 0x1e, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x03, 0x02, 0x04, 0x12, 0x03, 0x1b, 0x02, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03,
    0x02, 0x04, 0x04, 0x12, 0x03, 0x1b, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x04,
    0x05, 0x12, 0x03, 0x1b, 0x0b, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x04, 0x01, 0x12,
    0x03, 0x1b, 0x10, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x04, 0x03, 0x12, 0x03, 0x1b,
    0x1c, 0x1d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x05, 0x12, 0x03, 0x1c, 0x02, 0x1d, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x05, 0x04, 0x12, 0x03, 0x1c, 0x02, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x03, 0x02, 0x05, 0x05, 0x12, 0x03, 0x1c, 0x0b, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x03, 0x02, 0x05, 0x01, 0x12, 0x03, 0x1c, 0x10, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02,
    0x05, 0x03, 0x12, 0x03, 0x1c, 0x1b, 0x1c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x06, 0x12,
    0x03, 0x1d, 0x02, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x06, 0x04, 0x12, 0x03, 0x1d,
    0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x06, 0x05, 0x12, 0x03, 0x1d, 0x0b, 0x0f,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x06, 0x01, 0x12, 0x03, 0x1d, 0x10, 0x1d, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x03, 0x02, 0x06, 0x03, 0x12, 0x03, 0x1d, 0x20, 0x21, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x03, 0x02, 0x07, 0x12, 0x03, 0x1e, 0x02, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02,
    0x07, 0x04, 0x12, 0x03, 0x1e, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x07, 0x05,
    0x12, 0x03, 0x1e, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x07, 0x01, 0x12, 0x03,
    0x1e, 0x12, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x07, 0x03, 0x12, 0x03, 0x1e, 0x16,
    0x17, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x08, 0x12, 0x03, 0x1f, 0x02, 0x18, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x03, 0x02, 0x08, 0x04, 0x12, 0x03, 0x1f, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x03, 0x02, 0x08, 0x05, 0x12, 0x03, 0x1f, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03,
    0x02, 0x08, 0x01, 0x12, 0x03, 0x1f, 0x12, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x08,
    0x03, 0x12, 0x03, 0x1f, 0x16, 0x17, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x09, 0x12, 0x03,
    0x20, 0x02, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x09, 0x04, 0x12, 0x03, 0x20, 0x02,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x09, 0x05, 0x12, 0x03, 0x20, 0x0b, 0x0f, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x09, 0x01, 0x12, 0x03, 0x20, 0x10, 0x18, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x03, 0x02, 0x09, 0x03, 0x12, 0x03, 0x20, 0x1b, 0x1d, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x03, 0x02, 0x0a, 0x12, 0x03, 0x21, 0x02, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x0a,
    0x04, 0x12, 0x03, 0x21, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x0a, 0x05, 0x12,
    0x03, 0x21, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x0a, 0x01, 0x12, 0x03, 0x21,
    0x12, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x0a, 0x03, 0x12, 0x03, 0x21, 0x1e, 0x20,
    0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x04, 0x12, 0x04, 0x24, 0x00, 0x27, 0x01, 0x0a, 0x0a, 0x0a, 0x03,
    0x04, 0x04, 0x01, 0x12, 0x03, 0x24, 0x08, 0x0d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x00,
    0x12, 0x03, 0x25, 0x02, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x04, 0x12, 0x03,
    0x25, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x05, 0x12, 0x03, 0x25, 0x0b,
    0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x01, 0x12, 0x03, 0x25, 0x12, 0x1b, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x03, 0x12, 0x03, 0x25, 0x1e, 0x1f, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x04, 0x02, 0x01, 0x12, 0x03, 0x26, 0x02, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04,
    0x02, 0x01, 0x04, 0x12, 0x03, 0x26, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01,
    0x05, 0x12, 0x03, 0x26, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x01, 0x12,
    0x03, 0x26, 0x12, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x03, 0x12, 0x03, 0x26,
    0x1c, 0x1d, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x05, 0x12, 0x04, 0x29, 0x00, 0x2e, 0x01, 0x0a, 0x0a,
    0x0a, 0x03, 0x04, 0x05, 0x01, 0x12, 0x03, 0x29, 0x08, 0x0e, 0x0a, 0x4c, 0x0a, 0x04, 0x04, 0x05,
    0x02, 0x00, 0x12, 0x03, 0x2b, 0x02, 0x1a, 0x1a, 0x3f, 0x20, 0x54, 0x4f, 0x44, 0x4f, 0x20, 0x6d,
    0x61, 0x6b, 0x65, 0x20, 0x73, 0x63, 0x72, 0x65, 0x65, 0x6e, 0x20, 0x68, 0x61, 0x76, 0x65, 0x20,
    0x61, 0x20, 0x73, 0x65, 0x74, 0x20, 0x6f, 0x66, 0x20, 0x53, 0x63, 0x65, 0x6e, 0x65, 0x73, 0x20,
    0x72, 0x61, 0x74, 0x68, 0x65, 0x72, 0x20, 0x74, 0x68, 0x61, 0x6e, 0x20, 0x72, 0x61, 0x77, 0x20,
    0x61, 0x6e, 0x63, 0x68, 0x6f, 0x72, 0x73, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00,
    0x04, 0x12, 0x03, 0x2b, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x06, 0x12,
    0x03, 0x2b, 0x0b, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x01, 0x12, 0x03, 0x2b,
    0x10, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x03, 0x12, 0x03, 0x2b, 0x18, 0x19,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x01, 0x12, 0x03, 0x2c, 0x02, 0x1d, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x05, 0x02, 0x01, 0x04, 0x12, 0x03, 0x2c, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x05, 0x02, 0x01, 0x05, 0x12, 0x03, 0x2c, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02,
    0x01, 0x01, 0x12, 0x03, 0x2c, 0x12, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x03,
    0x12, 0x03, 0x2c, 0x1b, 0x1c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x02, 0x12, 0x03, 0x2d,
    0x02, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x02, 0x04, 0x12, 0x03, 0x2d, 0x02, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x02, 0x06, 0x12, 0x03, 0x2d, 0x0b, 0x10, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x05, 0x02, 0x02, 0x01, 0x12, 0x03, 0x2d, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x05, 0x02, 0x02, 0x03, 0x12, 0x03, 0x2d, 0x1a, 0x1b,
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
