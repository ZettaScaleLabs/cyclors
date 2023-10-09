//
// Copyright (c) 2022 ZettaScale Technology
//
// This program and the accompanying materials are made available under the
// terms of the Eclipse Public License 2.0 which is available at
// http://www.eclipse.org/legal/epl-2.0, or the Apache License, Version 2.0
// which is available at https://www.apache.org/licenses/LICENSE-2.0.
//
// SPDX-License-Identifier: EPL-2.0 OR Apache-2.0
//
// Contributors:
//   ZettaScale Zenoh Team, <zenoh@zettascale.tech>
//
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(improper_ctypes)]

pub const DDS_MIN_PSEUDO_HANDLE: dds_entity_t = 0x7fff0000 as dds_entity_t;

/* @defgroup builtintopic_constants Convenience constants for referring to builtin topics
 *
 * These constants can be used in place of an actual dds_topic_t, when creating
 * readers or writers for builtin-topics.
 *
 * @{
 */
pub const DDS_BUILTIN_TOPIC_DCPSPARTICIPANT: dds_entity_t =
    (DDS_MIN_PSEUDO_HANDLE + 1) as dds_entity_t;
pub const DDS_BUILTIN_TOPIC_DCPSTOPIC: dds_entity_t = (DDS_MIN_PSEUDO_HANDLE + 2) as dds_entity_t;
pub const DDS_BUILTIN_TOPIC_DCPSPUBLICATION: dds_entity_t =
    (DDS_MIN_PSEUDO_HANDLE + 3) as dds_entity_t;
pub const DDS_BUILTIN_TOPIC_DCPSSUBSCRIPTION: dds_entity_t =
    (DDS_MIN_PSEUDO_HANDLE + 4) as dds_entity_t;

/** Special handle representing the entity corresponding to the CycloneDDS library itself */
pub const DDS_CYCLONEDDS_HANDLE: dds_entity_t = (DDS_MIN_PSEUDO_HANDLE + 256) as dds_entity_t;

pub const DDS_DOMAIN_DEFAULT: u32 = 0xffffffff_u32;

pub mod qos;

// deactivate clippy on bindgen generated code
#[allow(clippy::all)]
mod bindings {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}
pub use bindings::*;

/* Additional wrapper functions for select exported inline functions */

extern "C" {
    pub fn ddsi_serdata_size(d: *const ddsi_serdata) -> u32;
}
extern "C" {
    pub fn ddsi_serdata_to_ser_ref(
        d: *const ddsi_serdata,
        off: usize,
        sz: usize,
        ref_: *mut ddsrt_iovec_t,
    ) -> *mut ddsi_serdata;
}
extern "C" {
    pub fn ddsi_serdata_unref(serdata: *mut ddsi_serdata);
}
extern "C" {
    pub fn ddsi_serdata_to_ser_unref(d: *mut ddsi_serdata, ref_: *const ddsrt_iovec_t);
}
extern "C" {
    pub fn ddsi_serdata_from_ser_iov(
        type_: *const ddsi_sertype,
        kind: ddsi_serdata_kind,
        niov: ddsrt_msg_iovlen_t,
        iov: *const ddsrt_iovec_t,
        size: usize,
    ) -> *mut ddsi_serdata;
}
extern "C" {
    pub fn ddsi_serdata_from_sample(
        type_: *const ddsi_sertype,
        kind: ddsi_serdata_kind,
        sample: *const ::std::os::raw::c_void,
    ) -> *mut ddsi_serdata;
}
