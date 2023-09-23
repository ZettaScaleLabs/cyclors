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
use crate::*;
use derivative::Derivative;
use log::warn;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    ffi::{CStr, CString},
    mem::ManuallyDrop,
    os::raw::c_char,
};

pub const DDS_INFINITE_TIME: i64 = 0x7FFFFFFFFFFFFFFF;
pub const DDS_100MS_DURATION: i64 = 100 * 1_000_000;
pub const DDS_1S_DURATION: i64 = 1_000_000_000;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct Qos {
    pub user_data: Option<Vec<u8>>,
    pub topic_data: Option<Vec<u8>>,
    pub group_data: Option<Vec<u8>>,
    pub durability: Option<Durability>,
    pub durability_service: Option<DurabilityService>,
    pub presentation: Option<Presentation>,
    pub deadline: Option<Deadline>,
    pub latency_budget: Option<LatencyBudget>,
    pub ownership: Option<Ownership>,
    pub ownership_strength: Option<OwnershipStrength>,
    pub liveliness: Option<Liveliness>,
    pub time_based_filter: Option<TimeBasedFilter>,
    pub partition: Option<Vec<String>>,
    pub reliability: Option<Reliability>,
    pub transport_priority: Option<TransportPriority>,
    pub lifespan: Option<Lifespan>,
    pub destination_order: Option<DestinationOrder>,
    pub history: Option<History>,
    pub resource_limits: Option<ResourceLimits>,
    pub writer_data_lifecycle: Option<WriterDataLifecycle>,
    pub reader_data_lifecycle: Option<ReaderDataLifecycle>,
    pub writer_batching: Option<WriterBatching>,
    pub type_consistency: Option<TypeConsistency>,
    pub entity_name: Option<EntityName>,
    pub properties: Option<HashMap<String, String>>,
    pub ignore_local: Option<IgnoreLocal>,
    pub data_representation: Option<Vec<dds_data_representation_id_t>>,
}

#[allow(clippy::missing_safety_doc)]
impl Qos {
    pub unsafe fn from_qos_native(qos: *mut dds_qos_t) -> Self {
        Qos {
            user_data: user_data_from_qos_native(qos),
            topic_data: topic_data_from_qos_native(qos),
            group_data: group_data_from_qos_native(qos),
            durability: durability_from_qos_native(qos),
            durability_service: durability_service_from_qos_native(qos),
            presentation: presentation_from_qos_native(qos),
            deadline: deadline_from_qos_native(qos),
            latency_budget: latency_budget_from_qos_native(qos),
            ownership: ownership_from_qos_native(qos),
            ownership_strength: ownership_strength_from_qos_native(qos),
            liveliness: liveliness_from_qos_native(qos),
            time_based_filter: time_based_filter_from_qos_native(qos),
            partition: partition_from_qos_native(qos),
            reliability: reliability_from_qos_native(qos),
            transport_priority: transport_priority_from_qos_native(qos),
            lifespan: lifespan_from_qos_native(qos),
            destination_order: destination_order_from_qos_native(qos),
            history: history_from_qos_native(qos),
            resource_limits: resource_limits_from_qos_native(qos),
            writer_data_lifecycle: writer_data_lifecycle_from_qos_native(qos),
            reader_data_lifecycle: reader_data_lifecycle_from_qos_native(qos),
            writer_batching: writer_batching_from_qos_native(qos),
            type_consistency: type_consistency_from_qos_native(qos),
            entity_name: entity_name_from_qos_native(qos),
            properties: properties_from_qos_native(qos),
            ignore_local: ignore_local_from_qos_native(qos),
            data_representation: data_representation_from_qos_native(qos),
        }
    }

    pub unsafe fn to_qos_native(&self) -> *mut dds_qos_t {
        unsafe {
            let qos = dds_create_qos();

            user_data_to_qos_native(qos, &self.user_data);
            topic_data_to_qos_native(qos, &self.topic_data);
            group_data_to_qos_native(qos, &self.group_data);
            durability_to_qos_native(qos, &self.durability);
            durability_service_to_qos_native(qos, &self.durability_service);
            presentation_to_qos_native(qos, &self.presentation);
            deadline_to_qos_native(qos, &self.deadline);
            latency_budget_to_qos_native(qos, &self.latency_budget);
            ownership_to_qos_native(qos, &self.ownership);
            ownership_strength_to_qos_native(qos, &self.ownership_strength);
            liveliness_to_qos_native(qos, &self.liveliness);
            time_based_filter_to_qos_native(qos, &self.time_based_filter);
            partition_to_qos_native(qos, &self.partition);
            reliability_to_qos_native(qos, &self.reliability);
            transport_priority_to_qos_native(qos, &self.transport_priority);
            lifespan_to_qos_native(qos, &self.lifespan);
            destination_order_to_qos_native(qos, &self.destination_order);
            history_to_qos_native(qos, &self.history);
            resource_limits_to_qos_native(qos, &self.resource_limits);
            writer_data_lifecycle_to_qos_native(qos, &self.writer_data_lifecycle);
            reader_data_lifecycle_to_qos_native(qos, &self.reader_data_lifecycle);
            writer_batching_to_qos_native(qos, &self.writer_batching);
            type_consistency_to_qos_native(qos, &self.type_consistency);
            entity_name_to_qos_native(qos, &self.entity_name);
            properties_to_qos_native(qos, &self.properties);
            ignore_local_to_qos_native(qos, &self.ignore_local);
            data_representation_to_qos_native(qos, &self.data_representation);

            qos
        }
    }

    pub unsafe fn delete_qos_native(qos: *mut dds_qos_t) {
        unsafe {
            dds_delete_qos(qos);
        }
    }
}

impl Default for Qos {
    fn default() -> Self {
        unsafe {
            let native_qos = dds_create_qos();
            let qos = Qos::from_qos_native(native_qos);
            dds_delete_qos(native_qos);
            qos
        }
    }
}

#[derive(Debug, Default, Clone, Deserialize, Serialize, PartialEq, Eq, Derivative)]
pub struct Durability {
    pub kind: DurabilityKind,
}

#[derive(Debug, Default, Copy, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum DurabilityKind {
    #[default]
    VOLATILE = dds_durability_kind_DDS_DURABILITY_VOLATILE as isize,
    TRANSIENT_LOCAL = dds_durability_kind_DDS_DURABILITY_TRANSIENT_LOCAL as isize,
    TRANSIENT = dds_durability_kind_DDS_DURABILITY_TRANSIENT as isize,
    PERSISTENT = dds_durability_kind_DDS_DURABILITY_PERSISTENT as isize,
}

impl From<&dds_durability_kind_t> for DurabilityKind {
    fn from(from: &dds_durability_kind_t) -> Self {
        #[allow(non_upper_case_globals)]
        match from {
            &dds_durability_kind_DDS_DURABILITY_VOLATILE => DurabilityKind::VOLATILE,
            &dds_durability_kind_DDS_DURABILITY_TRANSIENT_LOCAL => DurabilityKind::TRANSIENT_LOCAL,
            &dds_durability_kind_DDS_DURABILITY_TRANSIENT => DurabilityKind::TRANSIENT,
            &dds_durability_kind_DDS_DURABILITY_PERSISTENT => DurabilityKind::PERSISTENT,
            x => panic!("Invalid numeric value for DurabilityKind: {}", x),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Derivative)]
#[derivative(Default)]
pub struct DurabilityService {
    #[derivative(Default(value = "0"))]
    pub service_cleanup_delay: dds_duration_t,
    #[derivative(Default(value = "HistoryKind::KEEP_LAST"))]
    pub history_kind: HistoryKind,
    #[derivative(Default(value = "1"))]
    pub history_depth: i32,
    #[derivative(Default(value = "DDS_LENGTH_UNLIMITED"))]
    pub max_samples: i32,
    #[derivative(Default(value = "DDS_LENGTH_UNLIMITED"))]
    pub max_instances: i32,
    #[derivative(Default(value = "DDS_LENGTH_UNLIMITED"))]
    pub max_samples_per_instance: i32,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Derivative)]
#[derivative(Default)]
pub struct Reliability {
    #[derivative(Default(value = "ReliabilityKind::BEST_EFFORT"))]
    pub kind: ReliabilityKind,
    #[derivative(Default(value = "DDS_100MS_DURATION"))]
    pub max_blocking_time: dds_duration_t,
}

#[derive(Debug, Copy, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum ReliabilityKind {
    BEST_EFFORT = dds_reliability_kind_DDS_RELIABILITY_BEST_EFFORT as isize,
    RELIABLE = dds_reliability_kind_DDS_RELIABILITY_RELIABLE as isize,
}

impl From<&dds_reliability_kind_t> for ReliabilityKind {
    fn from(from: &dds_reliability_kind_t) -> Self {
        #[allow(non_upper_case_globals)]
        match from {
            &dds_reliability_kind_DDS_RELIABILITY_BEST_EFFORT => ReliabilityKind::BEST_EFFORT,
            &dds_reliability_kind_DDS_RELIABILITY_RELIABLE => ReliabilityKind::RELIABLE,
            x => panic!("Invalid numeric value for ReliabilityKind: {}", x),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Derivative)]
#[derivative(Default)]
pub struct Deadline {
    #[derivative(Default(value = "DDS_INFINITE_TIME"))]
    pub period: dds_duration_t,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Derivative)]
#[derivative(Default)]
pub struct LatencyBudget {
    #[derivative(Default(value = "0"))]
    pub duration: dds_duration_t,
}

#[derive(Debug, Default, Clone, Deserialize, Serialize, PartialEq, Eq, Derivative)]
pub struct DestinationOrder {
    pub kind: DestinationOrderKind,
}

#[derive(Debug, Default, Copy, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum DestinationOrderKind {
    #[default]
    BY_RECEPTION_TIMESTAMP =
        dds_destination_order_kind_DDS_DESTINATIONORDER_BY_RECEPTION_TIMESTAMP as isize,
    BY_SOURCE_TIMESTAMP =
        dds_destination_order_kind_DDS_DESTINATIONORDER_BY_SOURCE_TIMESTAMP as isize,
}

impl From<&dds_destination_order_kind_t> for DestinationOrderKind {
    fn from(from: &dds_destination_order_kind_t) -> Self {
        #[allow(non_upper_case_globals)]
        match from {
            &dds_destination_order_kind_DDS_DESTINATIONORDER_BY_RECEPTION_TIMESTAMP => {
                DestinationOrderKind::BY_RECEPTION_TIMESTAMP
            }
            &dds_destination_order_kind_DDS_DESTINATIONORDER_BY_SOURCE_TIMESTAMP => {
                DestinationOrderKind::BY_SOURCE_TIMESTAMP
            }
            x => panic!("Invalid numeric value for DestinationOrderKind: {}", x),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Derivative)]
#[derivative(Default)]
pub struct Liveliness {
    #[derivative(Default(value = "LivelinessKind::AUTOMATIC"))]
    pub kind: LivelinessKind,
    #[derivative(Default(value = "DDS_INFINITE_TIME"))]
    pub lease_duration: dds_duration_t,
}

#[derive(Debug, Copy, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum LivelinessKind {
    AUTOMATIC = dds_liveliness_kind_DDS_LIVELINESS_AUTOMATIC as isize,
    MANUAL_BY_PARTICIPANT = dds_liveliness_kind_DDS_LIVELINESS_MANUAL_BY_PARTICIPANT as isize,
    MANUAL_BY_TOPIC = dds_liveliness_kind_DDS_LIVELINESS_MANUAL_BY_TOPIC as isize,
}

impl From<&dds_liveliness_kind_t> for LivelinessKind {
    fn from(from: &dds_liveliness_kind_t) -> Self {
        #[allow(non_upper_case_globals)]
        match from {
            &dds_liveliness_kind_DDS_LIVELINESS_AUTOMATIC => LivelinessKind::AUTOMATIC,
            &dds_liveliness_kind_DDS_LIVELINESS_MANUAL_BY_PARTICIPANT => {
                LivelinessKind::MANUAL_BY_PARTICIPANT
            }
            &dds_liveliness_kind_DDS_LIVELINESS_MANUAL_BY_TOPIC => LivelinessKind::MANUAL_BY_TOPIC,
            x => panic!("Invalid numeric value for LivelinessKind: {}", x),
        }
    }
}

#[derive(Debug, Default, Clone, Deserialize, Serialize, PartialEq, Eq, Derivative)]
pub struct Ownership {
    pub kind: OwnershipKind,
}

#[derive(Debug, Default, Copy, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum OwnershipKind {
    #[default]
    SHARED = dds_ownership_kind_DDS_OWNERSHIP_SHARED as isize,
    EXCLUSIVE = dds_ownership_kind_DDS_OWNERSHIP_EXCLUSIVE as isize,
}

impl From<&dds_ownership_kind_t> for OwnershipKind {
    fn from(from: &dds_ownership_kind_t) -> Self {
        #[allow(non_upper_case_globals)]
        match from {
            &dds_ownership_kind_DDS_OWNERSHIP_SHARED => OwnershipKind::SHARED,
            &dds_ownership_kind_DDS_OWNERSHIP_EXCLUSIVE => OwnershipKind::EXCLUSIVE,
            x => panic!("Invalid numeric value for OwnershipKind: {}", x),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Derivative)]
#[derivative(Default)]
pub struct History {
    pub kind: HistoryKind,
    #[derivative(Default(value = "1"))]
    pub depth: i32,
}

#[derive(Debug, Default, Copy, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum HistoryKind {
    #[default]
    KEEP_LAST = dds_history_kind_DDS_HISTORY_KEEP_LAST as isize,
    KEEP_ALL = dds_history_kind_DDS_HISTORY_KEEP_ALL as isize,
}

impl From<&dds_history_kind_t> for HistoryKind {
    fn from(from: &dds_history_kind_t) -> Self {
        #[allow(non_upper_case_globals)]
        match from {
            &dds_history_kind_DDS_HISTORY_KEEP_LAST => HistoryKind::KEEP_LAST,
            &dds_history_kind_DDS_HISTORY_KEEP_ALL => HistoryKind::KEEP_ALL,
            x => panic!("Invalid numeric value for HistoryKind: {}", x),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Derivative)]
#[derivative(Default)]
pub struct ResourceLimits {
    #[derivative(Default(value = "DDS_LENGTH_UNLIMITED"))]
    pub max_samples: i32,
    #[derivative(Default(value = "DDS_LENGTH_UNLIMITED"))]
    pub max_instances: i32,
    #[derivative(Default(value = "DDS_LENGTH_UNLIMITED"))]
    pub max_samples_per_instance: i32,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Derivative)]
#[derivative(Default)]
pub struct Presentation {
    pub access_scope: PresentationAccessScopeKind,
    #[derivative(Default(value = "false"))]
    pub coherent_access: bool,
    #[derivative(Default(value = "false"))]
    pub ordered_access: bool,
}

#[derive(Debug, Default, Copy, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum PresentationAccessScopeKind {
    #[default]
    INSTANCE = dds_presentation_access_scope_kind_DDS_PRESENTATION_INSTANCE as isize,
    TOPIC = dds_presentation_access_scope_kind_DDS_PRESENTATION_TOPIC as isize,
    GROUP = dds_presentation_access_scope_kind_DDS_PRESENTATION_GROUP as isize,
}

impl From<&dds_presentation_access_scope_kind_t> for PresentationAccessScopeKind {
    fn from(from: &dds_presentation_access_scope_kind_t) -> Self {
        #[allow(non_upper_case_globals)]
        match from {
            &dds_presentation_access_scope_kind_DDS_PRESENTATION_INSTANCE => {
                PresentationAccessScopeKind::INSTANCE
            }
            &dds_presentation_access_scope_kind_DDS_PRESENTATION_TOPIC => {
                PresentationAccessScopeKind::TOPIC
            }
            &dds_presentation_access_scope_kind_DDS_PRESENTATION_GROUP => {
                PresentationAccessScopeKind::GROUP
            }
            x => panic!(
                "Invalid numeric value for PresentationAccessScopeKind: {}",
                x
            ),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Derivative)]
#[derivative(Default)]
pub struct Lifespan {
    #[derivative(Default(value = "DDS_INFINITE_TIME"))]
    pub duration: dds_duration_t,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Derivative)]
#[derivative(Default)]
pub struct OwnershipStrength {
    #[derivative(Default(value = "0"))]
    pub value: i32,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Derivative)]
#[derivative(Default)]
pub struct TimeBasedFilter {
    #[derivative(Default(value = "0"))]
    pub minimum_separation: dds_duration_t,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Derivative)]
#[derivative(Default)]
pub struct TransportPriority {
    #[derivative(Default(value = "0"))]
    pub value: i32,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Derivative)]
#[derivative(Default)]
pub struct WriterDataLifecycle {
    #[derivative(Default(value = "true"))]
    pub autodispose_unregistered_instances: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Derivative)]
#[derivative(Default)]
pub struct ReaderDataLifecycle {
    #[derivative(Default(value = "DDS_INFINITE_TIME"))]
    pub autopurge_nowriter_samples_delay: dds_duration_t,
    #[derivative(Default(value = "DDS_INFINITE_TIME"))]
    pub autopurge_disposed_samples_delay: dds_duration_t,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Derivative)]
#[derivative(Default)]
pub struct WriterBatching {
    #[derivative(Default(value = "false"))]
    pub batch_updates: bool,
}

#[derive(Debug, Default, Clone, Deserialize, Serialize, PartialEq, Eq, Derivative)]
pub struct IgnoreLocal {
    pub kind: IgnoreLocalKind,
}

#[derive(Debug, Default, Copy, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum IgnoreLocalKind {
    #[default]
    NONE = dds_ignorelocal_kind_DDS_IGNORELOCAL_NONE as isize,
    PARTICIPANT = dds_ignorelocal_kind_DDS_IGNORELOCAL_PARTICIPANT as isize,
    PROCESS = dds_ignorelocal_kind_DDS_IGNORELOCAL_PROCESS as isize,
}

impl From<&dds_ignorelocal_kind_t> for IgnoreLocalKind {
    fn from(from: &dds_ignorelocal_kind_t) -> Self {
        #[allow(non_upper_case_globals)]
        match from {
            &dds_ignorelocal_kind_DDS_IGNORELOCAL_NONE => IgnoreLocalKind::NONE,
            &dds_ignorelocal_kind_DDS_IGNORELOCAL_PARTICIPANT => IgnoreLocalKind::PARTICIPANT,
            &dds_ignorelocal_kind_DDS_IGNORELOCAL_PROCESS => IgnoreLocalKind::PROCESS,
            x => panic!("Invalid numeric value for IgnoreLocalKind: {}", x),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Derivative)]
#[derivative(Default)]
pub struct TypeConsistency {
    pub kind: TypeConsistencyKind,
    #[derivative(Default(value = "false"))]
    pub ignore_sequence_bounds: bool,
    #[derivative(Default(value = "false"))]
    pub ignore_string_bounds: bool,
    #[derivative(Default(value = "false"))]
    pub ignore_member_names: bool,
    #[derivative(Default(value = "false"))]
    pub prevent_type_widening: bool,
    #[derivative(Default(value = "false"))]
    pub force_type_validation: bool,
}

#[derive(Debug, Default, Copy, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum TypeConsistencyKind {
    #[default]
    DISALLOW_TYPE_COERCION =
        dds_type_consistency_kind_DDS_TYPE_CONSISTENCY_DISALLOW_TYPE_COERCION as isize,
    ALLOW_TYPE_COERCION =
        dds_type_consistency_kind_DDS_TYPE_CONSISTENCY_ALLOW_TYPE_COERCION as isize,
}

impl From<&dds_type_consistency_kind_t> for TypeConsistencyKind {
    fn from(from: &dds_type_consistency_kind_t) -> Self {
        #[allow(non_upper_case_globals)]
        match from {
            &dds_type_consistency_kind_DDS_TYPE_CONSISTENCY_DISALLOW_TYPE_COERCION => {
                TypeConsistencyKind::DISALLOW_TYPE_COERCION
            }
            &dds_type_consistency_kind_DDS_TYPE_CONSISTENCY_ALLOW_TYPE_COERCION => {
                TypeConsistencyKind::ALLOW_TYPE_COERCION
            }
            x => panic!("Invalid numeric value for TypeConsistencyKind: {}", x),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Derivative)]
#[derivative(Default)]
pub struct EntityName {
    pub name: String,
}

unsafe fn user_data_from_qos_native(qos: *const dds_qos_t) -> Option<Vec<u8>> {
    let mut sz: usize = 0;
    let mut value: *mut ::std::os::raw::c_void = std::ptr::null_mut();
    if dds_qget_userdata(qos, &mut value, &mut sz) {
        // Cyclone DDS returns a copy of the value so okay to take ownership
        to_option(Vec::from_raw_parts(
            value as *mut ::std::os::raw::c_uchar,
            sz,
            sz,
        ))
    } else {
        None
    }
}

unsafe fn user_data_to_qos_native(qos: *mut dds_qos_t, user_data: &Option<Vec<u8>>) {
    if let Some(user_data) = user_data {
        // Cyclone DDS makes a copy of the data
        let ptr = user_data.as_ptr();
        dds_qset_userdata(qos, ptr as *const ::std::os::raw::c_void, user_data.len());
    }
}

unsafe fn topic_data_from_qos_native(qos: *const dds_qos_t) -> Option<Vec<u8>> {
    let mut sz: usize = 0;
    let mut value: *mut ::std::os::raw::c_void = std::ptr::null_mut();
    if dds_qget_topicdata(qos, &mut value, &mut sz) {
        // Cyclone DDS returns a copy of the value so okay to take ownership
        to_option(Vec::from_raw_parts(
            value as *mut ::std::os::raw::c_uchar,
            sz,
            sz,
        ))
    } else {
        None
    }
}

unsafe fn topic_data_to_qos_native(qos: *mut dds_qos_t, topic_data: &Option<Vec<u8>>) {
    if let Some(topic_data) = topic_data {
        // Cyclone DDS makes a copy of the data
        let ptr = topic_data.as_ptr();
        dds_qset_topicdata(qos, ptr as *const ::std::os::raw::c_void, topic_data.len());
    }
}

unsafe fn group_data_from_qos_native(qos: *const dds_qos_t) -> Option<Vec<u8>> {
    let mut sz: usize = 0;
    let mut value: *mut ::std::os::raw::c_void = std::ptr::null_mut();
    if dds_qget_groupdata(qos, &mut value, &mut sz) {
        // Cyclone DDS returns a copy of the value so okay to take ownership
        to_option(Vec::from_raw_parts(
            value as *mut ::std::os::raw::c_uchar,
            sz,
            sz,
        ))
    } else {
        None
    }
}

unsafe fn group_data_to_qos_native(qos: *mut dds_qos_t, group_data: &Option<Vec<u8>>) {
    if let Some(group_data) = group_data {
        // Cyclone DDS makes a copy of the data
        let ptr = group_data.as_ptr();
        dds_qset_groupdata(qos, ptr as *const ::std::os::raw::c_void, group_data.len());
    }
}

unsafe fn durability_from_qos_native(qos: *const dds_qos_t) -> Option<Durability> {
    let mut dur_kind: dds_durability_kind_t = dds_durability_kind_DDS_DURABILITY_VOLATILE;
    if dds_qget_durability(qos, &mut dur_kind) {
        to_option(Durability {
            kind: DurabilityKind::from(&dur_kind),
        })
    } else {
        None
    }
}

unsafe fn durability_to_qos_native(qos: *mut dds_qos_t, durability: &Option<Durability>) {
    if let Some(durability) = durability {
        dds_qset_durability(qos, durability.kind as dds_durability_kind_t);
    }
}

unsafe fn history_from_qos_native(qos: *const dds_qos_t) -> Option<History> {
    let mut hist_kind: dds_history_kind_t = dds_history_kind_DDS_HISTORY_KEEP_LAST;
    let mut depth: i32 = 1;
    if dds_qget_history(qos, &mut hist_kind, &mut depth) {
        to_option(History {
            kind: HistoryKind::from(&hist_kind),
            depth,
        })
    } else {
        None
    }
}

unsafe fn history_to_qos_native(qos: *mut dds_qos_t, history: &Option<History>) {
    if let Some(history) = history {
        dds_qset_history(qos, history.kind as dds_history_kind_t, history.depth);
    }
}

unsafe fn resource_limits_from_qos_native(qos: *const dds_qos_t) -> Option<ResourceLimits> {
    let mut max_samples = DDS_LENGTH_UNLIMITED;
    let mut max_instances = DDS_LENGTH_UNLIMITED;
    let mut max_samples_per_instance = DDS_LENGTH_UNLIMITED;
    if dds_qget_resource_limits(
        qos,
        &mut max_samples,
        &mut max_instances,
        &mut max_samples_per_instance,
    ) {
        to_option(ResourceLimits {
            max_samples,
            max_instances,
            max_samples_per_instance,
        })
    } else {
        None
    }
}

unsafe fn resource_limits_to_qos_native(
    qos: *mut dds_qos_t,
    resource_limits: &Option<ResourceLimits>,
) {
    if let Some(resource_limits) = resource_limits {
        dds_qset_resource_limits(
            qos,
            resource_limits.max_samples,
            resource_limits.max_instances,
            resource_limits.max_samples_per_instance,
        );
    }
}

unsafe fn presentation_from_qos_native(qos: *const dds_qos_t) -> Option<Presentation> {
    let mut pres_access_scope: dds_presentation_access_scope_kind_t =
        dds_presentation_access_scope_kind_DDS_PRESENTATION_INSTANCE;
    let mut coherent_access: bool = false;
    let mut ordered_access: bool = false;
    if dds_qget_presentation(
        qos,
        &mut pres_access_scope,
        &mut coherent_access,
        &mut ordered_access,
    ) {
        to_option(Presentation {
            access_scope: PresentationAccessScopeKind::from(&pres_access_scope),
            coherent_access,
            ordered_access,
        })
    } else {
        None
    }
}

unsafe fn presentation_to_qos_native(qos: *mut dds_qos_t, presentation: &Option<Presentation>) {
    if let Some(presentation) = presentation {
        dds_qset_presentation(
            qos,
            presentation.access_scope as dds_presentation_access_scope_kind_t,
            presentation.coherent_access,
            presentation.ordered_access,
        );
    }
}

unsafe fn lifespan_from_qos_native(qos: *const dds_qos_t) -> Option<Lifespan> {
    let mut duration: dds_duration_t = DDS_INFINITE_TIME;
    if dds_qget_lifespan(qos, &mut duration) {
        to_option(Lifespan { duration })
    } else {
        None
    }
}

unsafe fn lifespan_to_qos_native(qos: *mut dds_qos_t, lifespan: &Option<Lifespan>) {
    if let Some(lifespan) = lifespan {
        dds_qset_lifespan(qos, lifespan.duration);
    }
}

unsafe fn deadline_from_qos_native(qos: *const dds_qos_t) -> Option<Deadline> {
    let mut period: dds_duration_t = DDS_INFINITE_TIME;
    if dds_qget_deadline(qos, &mut period) {
        to_option(Deadline { period })
    } else {
        None
    }
}

unsafe fn deadline_to_qos_native(qos: *mut dds_qos_t, deadline: &Option<Deadline>) {
    if let Some(deadline) = deadline {
        dds_qset_deadline(qos, deadline.period);
    }
}

unsafe fn latency_budget_from_qos_native(qos: *const dds_qos_t) -> Option<LatencyBudget> {
    let mut duration: dds_duration_t = 0;
    if dds_qget_latency_budget(qos, &mut duration) {
        to_option(LatencyBudget { duration })
    } else {
        None
    }
}

unsafe fn latency_budget_to_qos_native(
    qos: *mut dds_qos_t,
    latency_budget: &Option<LatencyBudget>,
) {
    if let Some(latency_budget) = latency_budget {
        dds_qset_latency_budget(qos, latency_budget.duration);
    }
}

unsafe fn ownership_from_qos_native(qos: *const dds_qos_t) -> Option<Ownership> {
    let mut own_kind: dds_ownership_kind_t = dds_ownership_kind_DDS_OWNERSHIP_SHARED;
    if dds_qget_ownership(qos, &mut own_kind) {
        to_option(Ownership {
            kind: OwnershipKind::from(&own_kind),
        })
    } else {
        None
    }
}

unsafe fn ownership_to_qos_native(qos: *mut dds_qos_t, ownership: &Option<Ownership>) {
    if let Some(ownership) = ownership {
        dds_qset_ownership(qos, ownership.kind as dds_ownership_kind_t);
    }
}

unsafe fn ownership_strength_from_qos_native(qos: *const dds_qos_t) -> Option<OwnershipStrength> {
    let mut value: i32 = 0;
    if dds_qget_ownership_strength(qos, &mut value) {
        to_option(OwnershipStrength { value })
    } else {
        None
    }
}

unsafe fn ownership_strength_to_qos_native(
    qos: *mut dds_qos_t,
    ownership_strength: &Option<OwnershipStrength>,
) {
    if let Some(ownership_strength) = ownership_strength {
        dds_qset_ownership_strength(qos, ownership_strength.value);
    }
}

unsafe fn liveliness_from_qos_native(qos: *const dds_qos_t) -> Option<Liveliness> {
    let mut live_kind: dds_liveliness_kind_t = dds_liveliness_kind_DDS_LIVELINESS_AUTOMATIC;
    let mut lease_duration: dds_duration_t = DDS_INFINITE_TIME;
    if dds_qget_liveliness(qos, &mut live_kind, &mut lease_duration) {
        to_option(Liveliness {
            kind: LivelinessKind::from(&live_kind),
            lease_duration,
        })
    } else {
        None
    }
}

unsafe fn liveliness_to_qos_native(qos: *mut dds_qos_t, liveliness: &Option<Liveliness>) {
    if let Some(liveliness) = liveliness {
        dds_qset_liveliness(
            qos,
            liveliness.kind as dds_liveliness_kind_t,
            liveliness.lease_duration,
        );
    }
}

unsafe fn time_based_filter_from_qos_native(qos: *const dds_qos_t) -> Option<TimeBasedFilter> {
    let mut minimum_separation: dds_duration_t = 0;
    if dds_qget_time_based_filter(qos, &mut minimum_separation) {
        to_option(TimeBasedFilter { minimum_separation })
    } else {
        None
    }
}

unsafe fn time_based_filter_to_qos_native(
    qos: *mut dds_qos_t,
    time_based_filter: &Option<TimeBasedFilter>,
) {
    if let Some(time_based_filter) = time_based_filter {
        dds_qset_time_based_filter(qos, time_based_filter.minimum_separation);
    }
}

unsafe fn partition_from_qos_native(qos: *const dds_qos_t) -> Option<Vec<String>> {
    let mut n: u32 = 0;
    let mut ps: *mut *mut ::std::os::raw::c_char = std::ptr::null_mut();

    if dds_qget_partition(qos, &mut n, &mut ps) {
        let mut partitions: Vec<String> = Vec::with_capacity(n as usize);
        for k in 0..n {
            let p_offset = *(ps.offset(k as isize));

            let p = CStr::from_ptr(p_offset).to_str().unwrap();
            partitions.push(String::from(p));

            // Cyclone DDS returns a copy of the string so need to free the memory
            dds_free(p_offset as *mut ::std::os::raw::c_void);
        }
        // Cyclone DDS returns a copy of the pointer array so need to free the memory
        dds_free(ps as *mut ::std::os::raw::c_void);
        to_option(partitions)
    } else {
        None
    }
}

unsafe fn partition_to_qos_native(qos: *mut dds_qos_t, partitions: &Option<Vec<String>>) {
    if let Some(partitions) = partitions {
        let mut vcs: Vec<CString> = Vec::with_capacity(partitions.len());
        let mut vptr: Vec<*const c_char> = Vec::with_capacity(partitions.len());

        for p in partitions {
            let cs = CString::new(p.as_str()).unwrap();
            vptr.push(cs.as_ptr());
            vcs.push(cs);
        }
        let (ptr, len, cap) = vec_into_raw_parts(vptr);
        // Cyclone DDS makes a copy of the data
        dds_qset_partition(qos, len as u32, ptr);
        drop(Vec::from_raw_parts(ptr, len, cap));
    }
}

unsafe fn reliability_from_qos_native(qos: *const dds_qos_t) -> Option<Reliability> {
    let mut rel_kind: dds_reliability_kind_t = dds_reliability_kind_DDS_RELIABILITY_BEST_EFFORT;
    let mut max_blocking_time: dds_duration_t = DDS_100MS_DURATION;
    if dds_qget_reliability(qos, &mut rel_kind, &mut max_blocking_time) {
        to_option(Reliability {
            kind: ReliabilityKind::from(&rel_kind),
            max_blocking_time,
        })
    } else {
        None
    }
}

unsafe fn reliability_to_qos_native(qos: *mut dds_qos_t, reliability: &Option<Reliability>) {
    if let Some(reliability) = reliability {
        dds_qset_reliability(
            qos,
            reliability.kind as dds_reliability_kind_t,
            reliability.max_blocking_time,
        );
    }
}

unsafe fn transport_priority_from_qos_native(qos: *const dds_qos_t) -> Option<TransportPriority> {
    let mut value: i32 = 0;
    if dds_qget_transport_priority(qos, &mut value) {
        to_option(TransportPriority { value })
    } else {
        None
    }
}

unsafe fn transport_priority_to_qos_native(
    qos: *mut dds_qos_t,
    transport_priority: &Option<TransportPriority>,
) {
    if let Some(transport_priority) = transport_priority {
        dds_qset_transport_priority(qos, transport_priority.value);
    }
}

unsafe fn destination_order_from_qos_native(qos: *const dds_qos_t) -> Option<DestinationOrder> {
    let mut dest_kind: dds_destination_order_kind_t =
        dds_destination_order_kind_DDS_DESTINATIONORDER_BY_RECEPTION_TIMESTAMP;
    if dds_qget_destination_order(qos, &mut dest_kind) {
        to_option(DestinationOrder {
            kind: DestinationOrderKind::from(&dest_kind),
        })
    } else {
        None
    }
}

unsafe fn destination_order_to_qos_native(
    qos: *mut dds_qos_t,
    destination_order: &Option<DestinationOrder>,
) {
    if let Some(destination_order) = destination_order {
        dds_qset_destination_order(qos, destination_order.kind as dds_destination_order_kind_t);
    }
}

unsafe fn writer_data_lifecycle_from_qos_native(
    qos: *const dds_qos_t,
) -> Option<WriterDataLifecycle> {
    let mut autodispose_unregistered_instances: bool = false;
    if dds_qget_writer_data_lifecycle(qos, &mut autodispose_unregistered_instances) {
        to_option(WriterDataLifecycle {
            autodispose_unregistered_instances,
        })
    } else {
        None
    }
}

unsafe fn writer_data_lifecycle_to_qos_native(
    qos: *mut dds_qos_t,
    writer_data_lifecycle: &Option<WriterDataLifecycle>,
) {
    if let Some(writer_data_lifecycle) = writer_data_lifecycle {
        dds_qset_writer_data_lifecycle(
            qos,
            writer_data_lifecycle.autodispose_unregistered_instances,
        );
    }
}

unsafe fn reader_data_lifecycle_from_qos_native(
    qos: *const dds_qos_t,
) -> Option<ReaderDataLifecycle> {
    let mut autopurge_nowriter_samples_delay: dds_duration_t = DDS_INFINITE_TIME;
    let mut autopurge_disposed_samples_delay: dds_duration_t = DDS_INFINITE_TIME;
    if dds_qget_reader_data_lifecycle(
        qos,
        &mut autopurge_nowriter_samples_delay,
        &mut autopurge_disposed_samples_delay,
    ) {
        to_option(ReaderDataLifecycle {
            autopurge_nowriter_samples_delay,
            autopurge_disposed_samples_delay,
        })
    } else {
        None
    }
}

unsafe fn reader_data_lifecycle_to_qos_native(
    qos: *mut dds_qos_t,
    reader_data_lifecycle: &Option<ReaderDataLifecycle>,
) {
    if let Some(reader_data_lifecycle) = reader_data_lifecycle {
        dds_qset_reader_data_lifecycle(
            qos,
            reader_data_lifecycle.autopurge_nowriter_samples_delay,
            reader_data_lifecycle.autopurge_disposed_samples_delay,
        );
    }
}

unsafe fn writer_batching_from_qos_native(qos: *const dds_qos_t) -> Option<WriterBatching> {
    let mut batch_updates: bool = false;
    if dds_qget_writer_batching(qos, &mut batch_updates) {
        to_option(WriterBatching { batch_updates })
    } else {
        None
    }
}

unsafe fn writer_batching_to_qos_native(
    qos: *mut dds_qos_t,
    writer_batching: &Option<WriterBatching>,
) {
    if let Some(writer_batching) = writer_batching {
        dds_qset_writer_batching(qos, writer_batching.batch_updates);
    }
}

unsafe fn durability_service_from_qos_native(qos: *const dds_qos_t) -> Option<DurabilityService> {
    let mut service_cleanup_delay: dds_duration_t = 0;
    let mut durability_history_kind: dds_history_kind_t = dds_history_kind_DDS_HISTORY_KEEP_LAST;
    let mut history_depth: i32 = 1;
    let mut max_samples = DDS_LENGTH_UNLIMITED;
    let mut max_instances = DDS_LENGTH_UNLIMITED;
    let mut max_samples_per_instance = DDS_LENGTH_UNLIMITED;

    if dds_qget_durability_service(
        qos,
        &mut service_cleanup_delay,
        &mut durability_history_kind,
        &mut history_depth,
        &mut max_samples,
        &mut max_instances,
        &mut max_samples_per_instance,
    ) {
        to_option(DurabilityService {
            service_cleanup_delay,
            history_kind: HistoryKind::from(&durability_history_kind),
            history_depth,
            max_samples,
            max_instances,
            max_samples_per_instance,
        })
    } else {
        None
    }
}

unsafe fn durability_service_to_qos_native(
    qos: *mut dds_qos_t,
    durability_service: &Option<DurabilityService>,
) {
    if let Some(durability_service) = durability_service {
        dds_qset_durability_service(
            qos,
            durability_service.service_cleanup_delay,
            durability_service.history_kind as dds_history_kind_t,
            durability_service.history_depth,
            durability_service.max_samples,
            durability_service.max_instances,
            durability_service.max_samples_per_instance,
        );
    }
}

unsafe fn ignore_local_from_qos_native(qos: *const dds_qos_t) -> Option<IgnoreLocal> {
    let mut ignore_kind: dds_ignorelocal_kind_t = dds_ignorelocal_kind_DDS_IGNORELOCAL_NONE;
    if dds_qget_ignorelocal(qos, &mut ignore_kind) {
        to_option(IgnoreLocal {
            kind: IgnoreLocalKind::from(&ignore_kind),
        })
    } else {
        None
    }
}

unsafe fn ignore_local_to_qos_native(qos: *mut dds_qos_t, ignore_local: &Option<IgnoreLocal>) {
    if let Some(ignore_local) = ignore_local {
        dds_qset_ignorelocal(qos, ignore_local.kind as dds_ignorelocal_kind_t);
    }
}

unsafe fn property_from_qos_native(qos: *const dds_qos_t, name: &str) -> Option<String> {
    let mut pvalue: *mut ::std::os::raw::c_char = std::ptr::null_mut();
    let cname = CString::new(name).unwrap();

    if dds_qget_prop(qos, cname.as_ptr(), &mut pvalue) {
        let value = CStr::from_ptr(pvalue).to_str().unwrap();
        let policy = to_option(String::from(value));

        // Cyclone DDS returns a copy of the string so need to free the memory
        dds_free(pvalue as *mut ::std::os::raw::c_void);
        policy
    } else {
        None
    }
}

unsafe fn properties_from_qos_native(qos: *const dds_qos_t) -> Option<HashMap<String, String>> {
    let mut n: u32 = 0;
    let mut ps: *mut *mut ::std::os::raw::c_char = std::ptr::null_mut();

    if dds_qget_propnames(qos, &mut n, &mut ps) {
        let mut map: HashMap<String, String> = HashMap::new();
        for k in 0..n {
            let p_offset = *(ps.offset(k as isize));

            let p = CStr::from_ptr(p_offset).to_str().unwrap();
            let name = String::from(p);

            // Cyclone DDS returns a copy of the string so need to free the memory
            dds_free(p_offset as *mut ::std::os::raw::c_void);

            if let Some(value) = property_from_qos_native(qos, &name) {
                map.insert(name, value);
            } else {
                warn!("Error retrieving QoS property: name={}", name);
                continue;
            }
        }
        // Cyclone DDS returns a copy of the pointer array so need to free the memory
        dds_free(ps as *mut ::std::os::raw::c_void);
        to_option(map)
    } else {
        None
    }
}

unsafe fn properties_to_qos_native(
    qos: *mut dds_qos_t,
    properties: &Option<HashMap<String, String>>,
) {
    if let Some(properties) = properties {
        for (name, value) in properties {
            let cname = CString::new(name.as_str()).unwrap();
            let cvalue = CString::new(value.as_str()).unwrap();
            // Cyclone DDS makes a copy of the data
            dds_qset_prop(qos, cname.as_ptr(), cvalue.as_ptr());
        }
    }
}

unsafe fn type_consistency_from_qos_native(qos: *const dds_qos_t) -> Option<TypeConsistency> {
    let mut type_kind: dds_type_consistency_kind_t =
        dds_type_consistency_kind_DDS_TYPE_CONSISTENCY_DISALLOW_TYPE_COERCION;
    let mut ignore_sequence_bounds: bool = false;
    let mut ignore_string_bounds: bool = false;
    let mut ignore_member_names: bool = false;
    let mut prevent_type_widening: bool = false;
    let mut force_type_validation: bool = false;
    if dds_qget_type_consistency(
        qos,
        &mut type_kind,
        &mut ignore_sequence_bounds,
        &mut ignore_string_bounds,
        &mut ignore_member_names,
        &mut prevent_type_widening,
        &mut force_type_validation,
    ) {
        to_option(TypeConsistency {
            kind: TypeConsistencyKind::from(&type_kind),
            ignore_sequence_bounds,
            ignore_string_bounds,
            ignore_member_names,
            prevent_type_widening,
            force_type_validation,
        })
    } else {
        None
    }
}

unsafe fn type_consistency_to_qos_native(
    qos: *mut dds_qos_t,
    type_consistency: &Option<TypeConsistency>,
) {
    if let Some(type_consistency) = type_consistency {
        dds_qset_type_consistency(
            qos,
            type_consistency.kind as dds_type_consistency_kind_t,
            type_consistency.ignore_sequence_bounds,
            type_consistency.ignore_string_bounds,
            type_consistency.ignore_member_names,
            type_consistency.prevent_type_widening,
            type_consistency.force_type_validation,
        );
    }
}

unsafe fn entity_name_from_qos_native(qos: *const dds_qos_t) -> Option<EntityName> {
    let mut ps: *mut ::std::os::raw::c_char = std::ptr::null_mut();

    if dds_qget_entity_name(qos, &mut ps) {
        let p = CStr::from_ptr(ps).to_str().unwrap();
        let policy = to_option(EntityName {
            name: String::from(p),
        });

        // Cyclone DDS returns a copy of the string so need to free the memory
        dds_free(ps as *mut ::std::os::raw::c_void);
        policy
    } else {
        None
    }
}

unsafe fn entity_name_to_qos_native(qos: *mut dds_qos_t, entity_name: &Option<EntityName>) {
    if let Some(entity_name) = entity_name {
        let cname = CString::new(entity_name.name.as_str()).unwrap();

        // Cyclone DDS makes a copy of the data
        dds_qset_entity_name(qos, cname.as_ptr());
    }
}

unsafe fn data_representation_from_qos_native(
    qos: *const dds_qos_t,
) -> Option<Vec<dds_data_representation_id_t>> {
    let mut n: u32 = 0;
    let mut values_ptr: *mut dds_data_representation_id_t = std::ptr::null_mut();

    if dds_qget_data_representation(qos, &mut n, &mut values_ptr) {
        let mut values: Vec<dds_data_representation_id_t> = Vec::with_capacity(n as usize);
        for k in 0..n {
            let value = *values_ptr.offset(k as isize) as dds_data_representation_id_t;
            values.push(value);
        }
        // Cyclone DDS returns a copy so need to free the memory
        dds_free(values_ptr as *mut ::std::os::raw::c_void);
        to_option(values)
    } else {
        None
    }
}

unsafe fn data_representation_to_qos_native(
    qos: *mut dds_qos_t,
    data_representation: &Option<Vec<dds_data_representation_id_t>>,
) {
    if let Some(values) = data_representation {
        dds_qset_data_representation(qos, values.len() as u32, values.as_ptr());
    }
}

/// Return None if v is the default, Some(v) otherwise
#[inline]
fn to_option<T: Default + Eq + std::fmt::Debug>(v: T) -> Option<T> {
    if is_default(&v) {
        None
    } else {
        Some(v)
    }
}

/// Return true if `v` == `T::default()`
#[inline]
pub fn is_default<T: Default + Eq + std::fmt::Debug>(v: &T) -> bool {
    println!(
        "is_default {v:?} vs {:?} => {:?}",
        &T::default(),
        v == &T::default()
    );
    v == &T::default()
}

/// Return true if `o` is `None` or is `Some(T::default())`
#[inline]
pub fn is_option_default<T: Default + Eq + std::fmt::Debug>(o: &Option<T>) -> bool {
    match o {
        Some(v) => is_default(v),
        None => true,
    }
}

//TODO replace when stable https://github.com/rust-lang/rust/issues/65816
#[inline]
fn vec_into_raw_parts<T>(v: Vec<T>) -> (*mut T, usize, usize) {
    let mut me = ManuallyDrop::new(v);
    (me.as_mut_ptr(), me.len(), me.capacity())
}

#[cfg(test)]
fn assert_no_policies_set(qos: &Qos) {
    assert!(qos.user_data.is_none());
    assert!(qos.topic_data.is_none());
    assert!(qos.group_data.is_none());
    assert!(qos.durability.is_none());
    assert!(qos.durability_service.is_none());
    assert!(qos.presentation.is_none());
    assert!(qos.deadline.is_none());
    assert!(qos.latency_budget.is_none());
    assert!(qos.ownership.is_none());
    assert!(qos.ownership_strength.is_none());
    assert!(qos.liveliness.is_none());
    assert!(qos.time_based_filter.is_none());
    assert!(qos.partition.is_none());
    assert!(qos.reliability.is_none());
    assert!(qos.transport_priority.is_none());
    assert!(qos.lifespan.is_none());
    assert!(qos.destination_order.is_none());
    assert!(qos.history.is_none());
    assert!(qos.resource_limits.is_none());
    assert!(qos.writer_data_lifecycle.is_none());
    assert!(qos.reader_data_lifecycle.is_none());
    assert!(qos.writer_batching.is_none());
    assert!(qos.type_consistency.is_none());
    assert!(qos.entity_name.is_none());
    assert!(qos.properties.is_none());
    assert!(qos.ignore_local.is_none());
    assert!(qos.data_representation.is_none());
}

#[cfg(test)]
fn assert_all_policies_set(qos: &Qos) {
    assert!(qos.user_data.is_some());
    assert!(qos.topic_data.is_some());
    assert!(qos.group_data.is_some());
    assert!(qos.durability.is_some());
    assert!(qos.durability_service.is_some());
    assert!(qos.presentation.is_some());
    assert!(qos.deadline.is_some());
    assert!(qos.latency_budget.is_some());
    assert!(qos.ownership.is_some());
    assert!(qos.ownership_strength.is_some());
    assert!(qos.liveliness.is_some());
    assert!(qos.time_based_filter.is_some());
    assert!(qos.partition.is_some());
    assert!(qos.reliability.is_some());
    assert!(qos.transport_priority.is_some());
    assert!(qos.lifespan.is_some());
    assert!(qos.destination_order.is_some());
    assert!(qos.history.is_some());
    assert!(qos.resource_limits.is_some());
    assert!(qos.writer_data_lifecycle.is_some());
    assert!(qos.reader_data_lifecycle.is_some());
    assert!(qos.writer_batching.is_some());
    assert!(qos.type_consistency.is_some());
    assert!(qos.entity_name.is_some());
    assert!(qos.properties.is_some());
    assert!(qos.ignore_local.is_some());
    assert!(qos.data_representation.is_some());
}

#[test]
fn test_default_qos_from_native() {
    unsafe {
        let qos_native = dds_create_qos();
        let qos = Qos::from_qos_native(qos_native);
        assert_no_policies_set(&qos);
        dds_delete_qos(qos_native);
    }
}

#[cfg(test)]
fn create_u8_vec_for_tests() -> Vec<u8> {
    vec![b'a', b'b', b'c', b'd', b'e']
}

#[test]
fn test_user_data_to_native() {
    unsafe {
        let qos_native = dds_create_qos();

        let policy = Some(create_u8_vec_for_tests());
        user_data_to_qos_native(qos_native, &policy);

        let mut sz: usize = 0;
        let mut value: *mut ::std::os::raw::c_void = std::ptr::null_mut();
        assert!(dds_qget_userdata(qos_native, &mut value, &mut sz));

        let output = Vec::from_raw_parts(value as *mut ::std::os::raw::c_uchar, sz, sz);
        assert!(output.len() == 5);
        assert_eq!(output, policy.unwrap());

        dds_delete_qos(qos_native);
    }
}

#[test]
fn test_user_data_from_native() {
    unsafe {
        let qos_native = dds_create_qos();

        let test_vec = create_u8_vec_for_tests();
        let ptr = test_vec.as_ptr();
        dds_qset_userdata(
            qos_native,
            ptr as *const ::std::os::raw::c_void,
            test_vec.len(),
        );

        let policy = user_data_from_qos_native(qos_native);

        assert!(policy.is_some());
        assert!(policy.unwrap() == test_vec);

        dds_delete_qos(qos_native);
    }
}

#[test]
fn test_topic_data_to_native() {
    unsafe {
        let qos_native = dds_create_qos();

        let policy = Some(create_u8_vec_for_tests());
        topic_data_to_qos_native(qos_native, &policy);

        let mut sz: usize = 0;
        let mut value: *mut ::std::os::raw::c_void = std::ptr::null_mut();
        assert!(dds_qget_topicdata(qos_native, &mut value, &mut sz));

        let output = Vec::from_raw_parts(value as *mut ::std::os::raw::c_uchar, sz, sz);
        assert!(output.len() == 5);
        assert_eq!(output, policy.unwrap());

        dds_delete_qos(qos_native);
    }
}

#[test]
fn test_topic_data_from_native() {
    unsafe {
        let qos_native = dds_create_qos();

        let test_vec = create_u8_vec_for_tests();
        let ptr = test_vec.as_ptr();
        dds_qset_topicdata(
            qos_native,
            ptr as *const ::std::os::raw::c_void,
            test_vec.len(),
        );

        let policy = topic_data_from_qos_native(qos_native);

        assert!(policy.is_some());
        assert!(policy.unwrap() == test_vec);

        dds_delete_qos(qos_native);
    }
}

#[test]
fn test_group_data_to_native() {
    unsafe {
        let qos_native = dds_create_qos();

        let policy = Some(create_u8_vec_for_tests());
        group_data_to_qos_native(qos_native, &policy);

        let mut sz: usize = 0;
        let mut value: *mut ::std::os::raw::c_void = std::ptr::null_mut();
        assert!(dds_qget_groupdata(qos_native, &mut value, &mut sz));

        let output = Vec::from_raw_parts(value as *mut ::std::os::raw::c_uchar, sz, sz);
        assert!(output.len() == 5);
        assert_eq!(output, policy.unwrap());

        dds_delete_qos(qos_native);
    }
}

#[test]
fn test_group_data_from_native() {
    unsafe {
        let qos_native = dds_create_qos();

        let test_vec = create_u8_vec_for_tests();
        let ptr = test_vec.as_ptr();
        dds_qset_groupdata(
            qos_native,
            ptr as *const ::std::os::raw::c_void,
            test_vec.len(),
        );

        let policy = group_data_from_qos_native(qos_native);

        assert!(policy.is_some());
        assert!(policy.unwrap() == test_vec);

        dds_delete_qos(qos_native);
    }
}

#[test]
fn test_durability_to_native() {
    unsafe {
        let kinds = vec![
            (
                DurabilityKind::PERSISTENT,
                dds_durability_kind_DDS_DURABILITY_PERSISTENT,
            ),
            (
                DurabilityKind::TRANSIENT,
                dds_durability_kind_DDS_DURABILITY_TRANSIENT,
            ),
            (
                DurabilityKind::TRANSIENT_LOCAL,
                dds_durability_kind_DDS_DURABILITY_TRANSIENT_LOCAL,
            ),
            (
                DurabilityKind::VOLATILE,
                dds_durability_kind_DDS_DURABILITY_VOLATILE,
            ),
        ];

        for kind in kinds {
            let qos_native = dds_create_qos();

            let policy = Some(Durability { kind: kind.0 });
            durability_to_qos_native(qos_native, &policy);

            let mut dur_kind: dds_durability_kind_t = dds_durability_kind_DDS_DURABILITY_VOLATILE;

            assert!(dds_qget_durability(qos_native, &mut dur_kind));
            assert_eq!(dur_kind, kind.1);

            dds_delete_qos(qos_native);
        }
    }
}

#[test]
fn test_durability_from_native() {
    unsafe {
        let kinds = vec![
            (
                dds_durability_kind_DDS_DURABILITY_PERSISTENT,
                DurabilityKind::PERSISTENT,
            ),
            (
                dds_durability_kind_DDS_DURABILITY_TRANSIENT,
                DurabilityKind::TRANSIENT,
            ),
            (
                dds_durability_kind_DDS_DURABILITY_TRANSIENT_LOCAL,
                DurabilityKind::TRANSIENT_LOCAL,
            ),
            (
                dds_durability_kind_DDS_DURABILITY_VOLATILE,
                DurabilityKind::VOLATILE,
            ),
        ];

        for kind in kinds {
            let qos_native = dds_create_qos();

            dds_qset_durability(qos_native, kind.0);

            let policy = durability_from_qos_native(qos_native);
            if is_default(&kind.1) {
                assert!(policy.is_none())
            } else {
                assert!(policy.is_some());
                assert_eq!(policy.unwrap().kind, kind.1);
            }

            dds_delete_qos(qos_native);
        }
    }
}

#[test]
fn test_durability_service_to_native() {
    unsafe {
        let kinds = vec![
            (
                HistoryKind::KEEP_LAST,
                dds_history_kind_DDS_HISTORY_KEEP_LAST,
            ),
            (HistoryKind::KEEP_ALL, dds_history_kind_DDS_HISTORY_KEEP_ALL),
        ];

        for kind in kinds {
            let qos_native = dds_create_qos();

            let policy = DurabilityService {
                service_cleanup_delay: 100,
                history_kind: kind.0,
                history_depth: 100,
                max_samples: 100,
                max_instances: 100,
                max_samples_per_instance: 100,
            };
            let input = Some(policy.clone());
            durability_service_to_qos_native(qos_native, &input);

            let mut service_cleanup_delay: dds_duration_t = 0;
            let mut durability_history_kind: dds_history_kind_t =
                dds_history_kind_DDS_HISTORY_KEEP_LAST;
            let mut history_depth = 1;
            let mut max_samples = DDS_LENGTH_UNLIMITED;
            let mut max_instances = DDS_LENGTH_UNLIMITED;
            let mut max_samples_per_instance = DDS_LENGTH_UNLIMITED;

            assert!(dds_qget_durability_service(
                qos_native,
                &mut service_cleanup_delay,
                &mut durability_history_kind,
                &mut history_depth,
                &mut max_samples,
                &mut max_instances,
                &mut max_samples_per_instance,
            ));

            assert_eq!(service_cleanup_delay, policy.service_cleanup_delay);
            assert_eq!(durability_history_kind, kind.1);
            assert_eq!(history_depth, policy.history_depth);
            assert_eq!(max_samples, policy.max_samples);
            assert_eq!(max_instances, policy.max_instances);
            assert_eq!(max_samples_per_instance, policy.max_samples_per_instance);

            dds_delete_qos(qos_native);
        }
    }
}

#[test]
fn test_durability_service_from_native() {
    unsafe {
        let kinds = vec![
            (
                dds_history_kind_DDS_HISTORY_KEEP_LAST,
                HistoryKind::KEEP_LAST,
            ),
            (dds_history_kind_DDS_HISTORY_KEEP_ALL, HistoryKind::KEEP_ALL),
        ];

        for kind in kinds {
            let qos_native = dds_create_qos();

            let service_cleanup_delay = 100;
            let history_depth = 100;
            let max_samples = 100;
            let max_instances = 100;
            let max_samples_per_instance = 100;
            dds_qset_durability_service(
                qos_native,
                service_cleanup_delay,
                kind.0,
                history_depth,
                max_samples,
                max_instances,
                max_samples_per_instance,
            );

            let policy = durability_service_from_qos_native(qos_native);

            assert!(policy.is_some());
            let policy = policy.unwrap();
            assert_eq!(policy.service_cleanup_delay, service_cleanup_delay);
            assert_eq!(policy.history_kind, kind.1);
            assert_eq!(policy.max_samples, max_samples);
            assert_eq!(policy.max_instances, max_instances);
            assert_eq!(policy.max_samples_per_instance, max_samples_per_instance);

            dds_delete_qos(qos_native);
        }
    }
}

#[test]
fn test_presentation_to_native() {
    unsafe {
        let kinds = vec![
            (
                PresentationAccessScopeKind::INSTANCE,
                dds_presentation_access_scope_kind_DDS_PRESENTATION_INSTANCE,
            ),
            (
                PresentationAccessScopeKind::TOPIC,
                dds_presentation_access_scope_kind_DDS_PRESENTATION_TOPIC,
            ),
            (
                PresentationAccessScopeKind::GROUP,
                dds_presentation_access_scope_kind_DDS_PRESENTATION_GROUP,
            ),
        ];

        for kind in kinds {
            let qos_native = dds_create_qos();

            let policy = Presentation {
                access_scope: kind.0,
                coherent_access: true,
                ordered_access: true,
            };
            let input = Some(policy.clone());
            presentation_to_qos_native(qos_native, &input.clone());

            let mut pres_access_scope: dds_presentation_access_scope_kind_t =
                dds_presentation_access_scope_kind_DDS_PRESENTATION_INSTANCE;
            let mut coherent_access: bool = false;
            let mut ordered_access: bool = false;
            assert!(dds_qget_presentation(
                qos_native,
                &mut pres_access_scope,
                &mut coherent_access,
                &mut ordered_access,
            ));
            assert_eq!(pres_access_scope, kind.1);
            assert_eq!(coherent_access, policy.coherent_access);
            assert_eq!(ordered_access, policy.ordered_access);

            dds_delete_qos(qos_native);
        }
    }
}

#[test]
fn test_presentation_from_native() {
    unsafe {
        let kinds = vec![
            (
                dds_presentation_access_scope_kind_DDS_PRESENTATION_INSTANCE,
                PresentationAccessScopeKind::INSTANCE,
            ),
            (
                dds_presentation_access_scope_kind_DDS_PRESENTATION_TOPIC,
                PresentationAccessScopeKind::TOPIC,
            ),
            (
                dds_presentation_access_scope_kind_DDS_PRESENTATION_GROUP,
                PresentationAccessScopeKind::GROUP,
            ),
        ];

        for kind in kinds {
            let qos_native = dds_create_qos();

            let coherent_access = true;
            let ordered_access = true;
            dds_qset_presentation(qos_native, kind.0, coherent_access, ordered_access);

            let policy = presentation_from_qos_native(qos_native);
            assert!(policy.is_some());
            let policy = policy.unwrap();
            assert_eq!(policy.access_scope, kind.1);
            assert_eq!(policy.coherent_access, coherent_access);
            assert_eq!(policy.ordered_access, ordered_access);

            dds_delete_qos(qos_native);
        }
    }
}

#[test]
fn test_deadline_to_native() {
    unsafe {
        let qos_native = dds_create_qos();

        let policy = Deadline { period: 1000 };
        let input = Some(policy.clone());
        deadline_to_qos_native(qos_native, &input.clone());

        let mut period: i64 = 0;
        assert!(dds_qget_deadline(qos_native, &mut period,));
        assert_eq!(period, policy.period);

        dds_delete_qos(qos_native);
    }
}

#[test]
fn test_deadline_from_native() {
    unsafe {
        let qos_native = dds_create_qos();

        let period: i64 = 1000;
        dds_qset_deadline(qos_native, period);

        let policy = deadline_from_qos_native(qos_native);
        assert!(policy.is_some());
        let policy = policy.unwrap();
        assert_eq!(policy.period, period);

        dds_delete_qos(qos_native);
    }
}

#[test]
fn test_latency_budget_to_native() {
    unsafe {
        let qos_native = dds_create_qos();

        let policy = LatencyBudget { duration: 1000 };
        let input = Some(policy.clone());
        latency_budget_to_qos_native(qos_native, &input.clone());

        let mut duration: dds_duration_t = 0;
        assert!(dds_qget_latency_budget(qos_native, &mut duration,));
        assert_eq!(duration, policy.duration);

        dds_delete_qos(qos_native);
    }
}

#[test]
fn test_latency_budget_from_native() {
    unsafe {
        let qos_native = dds_create_qos();
        let duration: dds_duration_t = 1000;
        dds_qset_latency_budget(qos_native, duration);

        let policy = latency_budget_from_qos_native(qos_native);
        assert!(policy.is_some());
        let policy = policy.unwrap();
        assert_eq!(policy.duration, duration);

        dds_delete_qos(qos_native);
    }
}

#[test]
fn test_ownership_to_native() {
    unsafe {
        let kinds = vec![
            (
                OwnershipKind::SHARED,
                dds_ownership_kind_DDS_OWNERSHIP_SHARED,
            ),
            (
                OwnershipKind::EXCLUSIVE,
                dds_ownership_kind_DDS_OWNERSHIP_EXCLUSIVE,
            ),
        ];

        for kind in kinds {
            let qos_native = dds_create_qos();

            let input = Some(Ownership { kind: kind.0 });
            ownership_to_qos_native(qos_native, &input);

            let mut own_kind: dds_durability_kind_t = dds_ownership_kind_DDS_OWNERSHIP_SHARED;
            assert!(dds_qget_ownership(qos_native, &mut own_kind));
            assert_eq!(own_kind, kind.1);

            dds_delete_qos(qos_native);
        }
    }
}

#[test]
fn test_ownership_from_native() {
    unsafe {
        let kinds = vec![
            (
                dds_ownership_kind_DDS_OWNERSHIP_SHARED,
                OwnershipKind::SHARED,
            ),
            (
                dds_ownership_kind_DDS_OWNERSHIP_EXCLUSIVE,
                OwnershipKind::EXCLUSIVE,
            ),
        ];

        for kind in kinds {
            let qos_native = dds_create_qos();

            dds_qset_ownership(qos_native, kind.0);

            let policy = ownership_from_qos_native(qos_native);
            if is_default(&kind.1) {
                assert!(policy.is_none())
            } else {
                assert!(policy.is_some());
                assert_eq!(policy.unwrap().kind, kind.1);
            }

            dds_delete_qos(qos_native);
        }
    }
}

#[test]
fn test_ownership_strength_to_native() {
    unsafe {
        let qos_native = dds_create_qos();

        let policy = OwnershipStrength { value: 1000 };
        let input = Some(policy.clone());
        ownership_strength_to_qos_native(qos_native, &input.clone());

        let mut value = 0;
        assert!(dds_qget_ownership_strength(qos_native, &mut value,));
        assert_eq!(value, policy.value);

        dds_delete_qos(qos_native);
    }
}

#[test]
fn test_ownership_strength_from_native() {
    unsafe {
        let qos_native = dds_create_qos();

        let value = 1000;
        dds_qset_ownership_strength(qos_native, value);

        let policy = ownership_strength_from_qos_native(qos_native);
        assert!(policy.is_some());
        let policy = policy.unwrap();
        assert_eq!(policy.value, value);

        dds_delete_qos(qos_native);
    }
}

#[test]
fn test_liveliness_to_native() {
    unsafe {
        let kinds = vec![
            (
                LivelinessKind::AUTOMATIC,
                dds_liveliness_kind_DDS_LIVELINESS_AUTOMATIC,
            ),
            (
                LivelinessKind::MANUAL_BY_PARTICIPANT,
                dds_liveliness_kind_DDS_LIVELINESS_MANUAL_BY_PARTICIPANT,
            ),
            (
                LivelinessKind::MANUAL_BY_TOPIC,
                dds_liveliness_kind_DDS_LIVELINESS_MANUAL_BY_TOPIC,
            ),
        ];

        for kind in kinds {
            let qos_native = dds_create_qos();

            let policy = Liveliness {
                kind: kind.0,
                lease_duration: 1000,
            };
            let input = Some(policy.clone());
            liveliness_to_qos_native(qos_native, &input.clone());

            let mut own_kind: dds_liveliness_kind_t = dds_liveliness_kind_DDS_LIVELINESS_AUTOMATIC;
            let mut lease_duration: i64 = 0;
            assert!(dds_qget_liveliness(
                qos_native,
                &mut own_kind,
                &mut lease_duration,
            ));
            assert_eq!(own_kind, kind.1);
            assert_eq!(lease_duration, policy.lease_duration);

            dds_delete_qos(qos_native);
        }
    }
}

#[test]
fn test_liveliness_from_native() {
    unsafe {
        let kinds = vec![
            (
                dds_liveliness_kind_DDS_LIVELINESS_AUTOMATIC,
                LivelinessKind::AUTOMATIC,
            ),
            (
                dds_liveliness_kind_DDS_LIVELINESS_MANUAL_BY_PARTICIPANT,
                LivelinessKind::MANUAL_BY_PARTICIPANT,
            ),
            (
                dds_liveliness_kind_DDS_LIVELINESS_MANUAL_BY_TOPIC,
                LivelinessKind::MANUAL_BY_TOPIC,
            ),
        ];

        for kind in kinds {
            let qos_native = dds_create_qos();

            let lease_duration: i64 = 1000;
            dds_qset_liveliness(qos_native, kind.0, lease_duration);

            let policy = liveliness_from_qos_native(qos_native);
            assert!(policy.is_some());
            let policy = policy.unwrap();
            assert_eq!(policy.kind, kind.1);
            assert_eq!(policy.lease_duration, lease_duration);

            dds_delete_qos(qos_native);
        }
    }
}

#[test]
fn test_time_based_filter_to_native() {
    unsafe {
        let qos_native = dds_create_qos();

        let policy = TimeBasedFilter {
            minimum_separation: 1000,
        };
        let input = Some(policy.clone());
        time_based_filter_to_qos_native(qos_native, &input.clone());

        let mut minimum_separation = 0;
        assert!(dds_qget_time_based_filter(
            qos_native,
            &mut minimum_separation,
        ));
        assert_eq!(minimum_separation, policy.minimum_separation);

        dds_delete_qos(qos_native);
    }
}

#[test]
fn test_time_based_filter_from_native() {
    unsafe {
        let qos_native = dds_create_qos();
        let minimum_separation: dds_duration_t = 1000;
        dds_qset_time_based_filter(qos_native, minimum_separation);

        let policy = time_based_filter_from_qos_native(qos_native);
        assert!(policy.is_some());
        let policy = policy.unwrap();
        assert_eq!(policy.minimum_separation, minimum_separation);

        dds_delete_qos(qos_native);
    }
}

#[test]
fn test_partition_to_native() {
    unsafe {
        let qos_native = dds_create_qos();

        let mut partitions = Vec::with_capacity(2);
        partitions.push(String::from("P1"));
        partitions.push(String::from("P2"));

        let policy = Some(partitions.clone());
        partition_to_qos_native(qos_native, &policy);

        let mut n: u32 = 0;
        let mut ps: *mut *mut ::std::os::raw::c_char = std::ptr::null_mut();
        assert!(dds_qget_partition(qos_native, &mut n, &mut ps));
        assert_eq!(n, partitions.len() as u32);

        for k in 0..n {
            let p_offset = *(ps.offset(k as isize));
            let p = CStr::from_ptr(p_offset).to_str().unwrap();
            assert_eq!(String::from(p), partitions[k as usize]);

            dds_free(p_offset as *mut ::std::os::raw::c_void);
        }
        dds_free(ps as *mut ::std::os::raw::c_void);

        dds_delete_qos(qos_native);
    }
}

#[test]
fn test_partition_from_native() {
    unsafe {
        let qos_native = dds_create_qos();

        let mut partitions = Vec::with_capacity(2);
        partitions.push(CString::new("P1").unwrap());
        partitions.push(CString::new("P2").unwrap());

        let mut vptr: Vec<*const c_char> = Vec::with_capacity(partitions.len());
        for p in &partitions {
            vptr.push(p.as_ptr());
        }

        let (ptr, len, cap) = vec_into_raw_parts(vptr);
        dds_qset_partition(qos_native, len as u32, ptr);
        drop(Vec::from_raw_parts(ptr, len, cap));

        let policy = partition_from_qos_native(qos_native);
        assert!(policy.is_some());
        let policy = policy.unwrap();
        assert_eq!(policy.len(), partitions.len());

        for k in 0..policy.len() {
            assert_eq!(partitions[k].to_str().unwrap(), policy[k]);
        }

        dds_delete_qos(qos_native);
    }
}

#[test]
fn test_reliability_to_native() {
    unsafe {
        let kinds = vec![
            (
                ReliabilityKind::BEST_EFFORT,
                dds_reliability_kind_DDS_RELIABILITY_BEST_EFFORT,
            ),
            (
                ReliabilityKind::RELIABLE,
                dds_reliability_kind_DDS_RELIABILITY_RELIABLE,
            ),
        ];

        for kind in kinds {
            let qos_native = dds_create_qos();

            let policy = Reliability {
                kind: kind.0,
                max_blocking_time: 1000,
            };
            let input = Some(policy.clone());
            reliability_to_qos_native(qos_native, &input.clone());

            let mut reliability_kind: dds_reliability_kind_t =
                dds_reliability_kind_DDS_RELIABILITY_BEST_EFFORT;
            let mut max_blocking_time: i64 = 0;
            assert!(dds_qget_reliability(
                qos_native,
                &mut reliability_kind,
                &mut max_blocking_time,
            ));
            assert_eq!(reliability_kind, kind.1);
            assert_eq!(max_blocking_time, policy.max_blocking_time);

            dds_delete_qos(qos_native);
        }
    }
}

#[test]
fn test_reliability_from_native() {
    unsafe {
        let kinds = vec![
            (
                dds_reliability_kind_DDS_RELIABILITY_BEST_EFFORT,
                ReliabilityKind::BEST_EFFORT,
            ),
            (
                dds_reliability_kind_DDS_RELIABILITY_RELIABLE,
                ReliabilityKind::RELIABLE,
            ),
        ];

        for kind in kinds {
            let qos_native = dds_create_qos();
            let max_blocking_time: i64 = 1000;
            dds_qset_reliability(qos_native, kind.0, max_blocking_time);

            let policy = reliability_from_qos_native(qos_native);
            assert!(policy.is_some());
            let policy = policy.unwrap();
            assert_eq!(policy.kind, kind.1);
            assert_eq!(policy.max_blocking_time, max_blocking_time);

            dds_delete_qos(qos_native);
        }
    }
}

#[test]
fn test_transport_priority_to_native() {
    unsafe {
        let qos_native = dds_create_qos();

        let policy = TransportPriority { value: 1000 };
        let input = Some(policy.clone());
        transport_priority_to_qos_native(qos_native, &input.clone());

        let mut value = 0;
        assert!(dds_qget_transport_priority(qos_native, &mut value,));
        assert_eq!(value, policy.value);

        dds_delete_qos(qos_native);
    }
}

#[test]
fn test_transport_priority_from_native() {
    unsafe {
        let qos_native = dds_create_qos();

        let value = 1000;
        dds_qset_transport_priority(qos_native, value);

        let policy = transport_priority_from_qos_native(qos_native);
        assert!(policy.is_some());
        let policy = policy.unwrap();
        assert_eq!(policy.value, value);

        dds_delete_qos(qos_native);
    }
}

#[test]
fn test_lifespan_to_native() {
    unsafe {
        let qos_native = dds_create_qos();

        let policy = Lifespan { duration: 1000 };
        let input = Some(policy.clone());
        lifespan_to_qos_native(qos_native, &input.clone());

        let mut duration = 0;
        assert!(dds_qget_lifespan(qos_native, &mut duration,));
        assert_eq!(duration, policy.duration);

        dds_delete_qos(qos_native);
    }
}

#[test]
fn test_lifespan_from_native() {
    unsafe {
        let qos_native = dds_create_qos();

        let duration: dds_duration_t = 1000;
        dds_qset_lifespan(qos_native, duration);

        let policy = lifespan_from_qos_native(qos_native);
        assert!(policy.is_some());
        let policy = policy.unwrap();
        assert_eq!(policy.duration, duration);

        dds_delete_qos(qos_native);
    }
}

#[test]
fn test_destination_order_to_native() {
    unsafe {
        let kinds = vec![
            (
                DestinationOrderKind::BY_RECEPTION_TIMESTAMP,
                dds_destination_order_kind_DDS_DESTINATIONORDER_BY_RECEPTION_TIMESTAMP,
            ),
            (
                DestinationOrderKind::BY_SOURCE_TIMESTAMP,
                dds_destination_order_kind_DDS_DESTINATIONORDER_BY_SOURCE_TIMESTAMP,
            ),
        ];

        for kind in kinds {
            let qos_native = dds_create_qos();

            let policy = DestinationOrder { kind: kind.0 };
            let input = Some(policy.clone());
            destination_order_to_qos_native(qos_native, &input.clone());

            let mut dest_kind: dds_destination_order_kind_t =
                dds_destination_order_kind_DDS_DESTINATIONORDER_BY_RECEPTION_TIMESTAMP;
            assert!(dds_qget_destination_order(qos_native, &mut dest_kind,));
            assert_eq!(dest_kind, kind.1);

            dds_delete_qos(qos_native);
        }
    }
}

#[test]
fn test_destination_order_from_native() {
    unsafe {
        let kinds = vec![
            (
                dds_destination_order_kind_DDS_DESTINATIONORDER_BY_RECEPTION_TIMESTAMP,
                DestinationOrderKind::BY_RECEPTION_TIMESTAMP,
            ),
            (
                dds_destination_order_kind_DDS_DESTINATIONORDER_BY_SOURCE_TIMESTAMP,
                DestinationOrderKind::BY_SOURCE_TIMESTAMP,
            ),
        ];

        for kind in kinds {
            let qos_native = dds_create_qos();

            dds_qset_destination_order(qos_native, kind.0);

            let policy = destination_order_from_qos_native(qos_native);
            if is_default(&kind.1) {
                assert!(policy.is_none())
            } else {
                assert!(policy.is_some());
                assert_eq!(policy.unwrap().kind, kind.1);
            }

            dds_delete_qos(qos_native);
        }
    }
}

#[test]
fn test_history_to_native() {
    unsafe {
        let kinds = vec![
            (
                HistoryKind::KEEP_LAST,
                dds_history_kind_DDS_HISTORY_KEEP_LAST,
            ),
            (HistoryKind::KEEP_ALL, dds_history_kind_DDS_HISTORY_KEEP_ALL),
        ];

        for kind in kinds {
            let qos_native = dds_create_qos();

            let policy = History {
                kind: kind.0,
                depth: 1000,
            };
            let input = Some(policy.clone());
            history_to_qos_native(qos_native, &input.clone());

            let mut history_kind: dds_liveliness_kind_t =
                dds_liveliness_kind_DDS_LIVELINESS_AUTOMATIC;
            let mut depth = 0;
            assert!(dds_qget_history(qos_native, &mut history_kind, &mut depth,));
            assert_eq!(history_kind, kind.1);
            assert_eq!(depth, policy.depth);

            dds_delete_qos(qos_native);
        }
    }
}

#[test]
fn test_history_from_native() {
    unsafe {
        let kinds = vec![
            (
                dds_history_kind_DDS_HISTORY_KEEP_LAST,
                HistoryKind::KEEP_LAST,
            ),
            (dds_history_kind_DDS_HISTORY_KEEP_ALL, HistoryKind::KEEP_ALL),
        ];

        for kind in kinds {
            let qos_native = dds_create_qos();

            let depth = 1000;
            dds_qset_history(qos_native, kind.0, depth);

            let policy = history_from_qos_native(qos_native);
            assert!(policy.is_some());
            let policy = policy.unwrap();
            assert_eq!(policy.kind, kind.1);
            assert_eq!(policy.depth, depth);

            dds_delete_qos(qos_native);
        }
    }
}

#[test]
fn test_resource_limits_to_native() {
    unsafe {
        let qos_native = dds_create_qos();

        let policy = ResourceLimits {
            max_samples: 1000,
            max_instances: 1000,
            max_samples_per_instance: 1000,
        };
        let input = Some(policy.clone());
        resource_limits_to_qos_native(qos_native, &input.clone());

        let mut max_samples = 0;
        let mut max_instances = 0;
        let mut max_samples_per_instance = 0;
        assert!(dds_qget_resource_limits(
            qos_native,
            &mut max_samples,
            &mut max_instances,
            &mut max_samples_per_instance,
        ));
        assert_eq!(max_samples, policy.max_samples);
        assert_eq!(max_instances, policy.max_instances);
        assert_eq!(max_samples_per_instance, policy.max_samples_per_instance);

        dds_delete_qos(qos_native);
    }
}

#[test]
fn test_resource_limits_from_native() {
    unsafe {
        let qos_native = dds_create_qos();

        let max_samples = 1000;
        let max_instances = 1000;
        let max_samples_per_instance = 1000;
        dds_qset_resource_limits(
            qos_native,
            max_samples,
            max_instances,
            max_samples_per_instance,
        );

        let policy = resource_limits_from_qos_native(qos_native);
        assert!(policy.is_some());
        let policy = policy.unwrap();
        assert_eq!(policy.max_samples, max_samples);
        assert_eq!(policy.max_instances, max_instances);
        assert_eq!(policy.max_samples_per_instance, max_samples_per_instance);

        dds_delete_qos(qos_native);
    }
}

#[test]
fn test_writer_data_lifecycle_to_native() {
    unsafe {
        let qos_native = dds_create_qos();

        let policy = WriterDataLifecycle {
            autodispose_unregistered_instances: true,
        };
        let input = Some(policy.clone());
        writer_data_lifecycle_to_qos_native(qos_native, &input.clone());

        let mut autodispose_unregistered_instances: bool = false;
        assert!(dds_qget_writer_data_lifecycle(
            qos_native,
            &mut autodispose_unregistered_instances,
        ));
        assert_eq!(
            autodispose_unregistered_instances,
            policy.autodispose_unregistered_instances
        );

        dds_delete_qos(qos_native);
    }
}

#[test]
fn test_writer_data_lifecycle_from_native() {
    unsafe {
        let qos_native = dds_create_qos();

        let autodispose_unregistered_instances = false;
        dds_qset_writer_data_lifecycle(qos_native, autodispose_unregistered_instances);

        let policy = writer_data_lifecycle_from_qos_native(qos_native);
        assert!(policy.is_some());
        let policy = policy.unwrap();
        assert_eq!(
            policy.autodispose_unregistered_instances,
            autodispose_unregistered_instances
        );

        dds_delete_qos(qos_native);
    }
}

#[test]
fn test_reader_data_lifecycle_to_native() {
    unsafe {
        let qos_native = dds_create_qos();

        let policy = ReaderDataLifecycle {
            autopurge_nowriter_samples_delay: 1000,
            autopurge_disposed_samples_delay: 1000,
        };
        let input = Some(policy.clone());
        reader_data_lifecycle_to_qos_native(qos_native, &input.clone());

        let mut autopurge_nowriter_samples_delay: dds_duration_t = 0;
        let mut autopurge_disposed_samples_delay: dds_duration_t = 0;
        assert!(dds_qget_reader_data_lifecycle(
            qos_native,
            &mut autopurge_nowriter_samples_delay,
            &mut autopurge_disposed_samples_delay,
        ));
        assert_eq!(
            autopurge_nowriter_samples_delay,
            policy.autopurge_nowriter_samples_delay
        );
        assert_eq!(
            autopurge_disposed_samples_delay,
            policy.autopurge_disposed_samples_delay
        );

        dds_delete_qos(qos_native);
    }
}

#[test]
fn test_reader_data_lifecycle_from_native() {
    unsafe {
        let qos_native = dds_create_qos();

        let autopurge_nowriter_samples_delay: dds_duration_t = 1000;
        let autopurge_disposed_samples_delay: dds_duration_t = 1000;
        dds_qset_reader_data_lifecycle(
            qos_native,
            autopurge_nowriter_samples_delay,
            autopurge_disposed_samples_delay,
        );

        let policy = reader_data_lifecycle_from_qos_native(qos_native);
        assert!(policy.is_some());
        let policy = policy.unwrap();
        assert_eq!(
            policy.autopurge_nowriter_samples_delay,
            autopurge_nowriter_samples_delay
        );
        assert_eq!(
            policy.autopurge_disposed_samples_delay,
            autopurge_disposed_samples_delay
        );

        dds_delete_qos(qos_native);
    }
}

#[test]
fn test_writer_batching_to_native() {
    unsafe {
        let qos_native = dds_create_qos();

        let policy = WriterBatching {
            batch_updates: true,
        };
        let input = Some(policy.clone());
        writer_batching_to_qos_native(qos_native, &input.clone());

        let mut batch_updates: bool = false;
        assert!(dds_qget_writer_batching(qos_native, &mut batch_updates,));
        assert_eq!(batch_updates, policy.batch_updates);

        dds_delete_qos(qos_native);
    }
}

#[test]
fn test_writer_batching_from_native() {
    unsafe {
        let qos_native = dds_create_qos();

        let batch_updates = true;
        dds_qset_writer_batching(qos_native, batch_updates);

        let policy = writer_batching_from_qos_native(qos_native);
        assert!(policy.is_some());
        let policy = policy.unwrap();
        assert_eq!(policy.batch_updates, batch_updates);

        dds_delete_qos(qos_native);
    }
}

#[test]
fn test_type_consistency_to_native() {
    unsafe {
        let kinds = vec![
            (
                TypeConsistencyKind::DISALLOW_TYPE_COERCION,
                dds_type_consistency_kind_DDS_TYPE_CONSISTENCY_DISALLOW_TYPE_COERCION,
            ),
            (
                TypeConsistencyKind::ALLOW_TYPE_COERCION,
                dds_type_consistency_kind_DDS_TYPE_CONSISTENCY_ALLOW_TYPE_COERCION,
            ),
        ];

        for kind in kinds {
            let qos_native = dds_create_qos();

            let policy = TypeConsistency {
                kind: kind.0,
                ignore_sequence_bounds: true,
                ignore_string_bounds: true,
                ignore_member_names: true,
                prevent_type_widening: true,
                force_type_validation: true,
            };
            let input = Some(policy.clone());
            type_consistency_to_qos_native(qos_native, &input.clone());

            let mut type_kind: dds_type_consistency_kind_t =
                dds_type_consistency_kind_DDS_TYPE_CONSISTENCY_DISALLOW_TYPE_COERCION;
            let mut ignore_sequence_bounds = false;
            let mut ignore_string_bounds = false;
            let mut ignore_member_names = false;
            let mut prevent_type_widening = false;
            let mut force_type_validation = false;
            assert!(dds_qget_type_consistency(
                qos_native,
                &mut type_kind,
                &mut ignore_sequence_bounds,
                &mut ignore_string_bounds,
                &mut ignore_member_names,
                &mut prevent_type_widening,
                &mut force_type_validation,
            ));
            assert_eq!(type_kind, kind.1);
            assert_eq!(ignore_sequence_bounds, policy.ignore_sequence_bounds);
            assert_eq!(ignore_string_bounds, policy.ignore_string_bounds);
            assert_eq!(ignore_member_names, policy.ignore_member_names);
            assert_eq!(prevent_type_widening, policy.prevent_type_widening);
            assert_eq!(force_type_validation, policy.force_type_validation);

            dds_delete_qos(qos_native);
        }
    }
}

#[test]
fn test_type_consistency_from_native() {
    unsafe {
        let kinds = vec![
            (
                dds_type_consistency_kind_DDS_TYPE_CONSISTENCY_DISALLOW_TYPE_COERCION,
                TypeConsistencyKind::DISALLOW_TYPE_COERCION,
            ),
            (
                dds_type_consistency_kind_DDS_TYPE_CONSISTENCY_ALLOW_TYPE_COERCION,
                TypeConsistencyKind::ALLOW_TYPE_COERCION,
            ),
        ];

        for kind in kinds {
            let qos_native = dds_create_qos();

            let ignore_sequence_bounds = true;
            let ignore_string_bounds = true;
            let ignore_member_names = true;
            let prevent_type_widening = true;
            let force_type_validation = true;
            dds_qset_type_consistency(
                qos_native,
                kind.0,
                ignore_sequence_bounds,
                ignore_string_bounds,
                ignore_member_names,
                prevent_type_widening,
                force_type_validation,
            );

            let policy = type_consistency_from_qos_native(qos_native);
            assert!(policy.is_some());
            let policy = policy.unwrap();
            assert_eq!(policy.kind, kind.1);
            assert_eq!(policy.ignore_sequence_bounds, ignore_sequence_bounds);
            assert_eq!(policy.ignore_string_bounds, ignore_string_bounds);
            assert_eq!(policy.ignore_member_names, ignore_member_names);
            assert_eq!(policy.prevent_type_widening, prevent_type_widening);
            assert_eq!(policy.force_type_validation, force_type_validation);

            dds_delete_qos(qos_native);
        }
    }
}

#[test]
fn test_entity_name_to_native() {
    unsafe {
        let qos_native = dds_create_qos();

        let name = Some(EntityName {
            name: String::from("TEST_ENTITY_NAME"),
        });
        entity_name_to_qos_native(qos_native, &name);

        let mut ps: *mut ::std::os::raw::c_char = std::ptr::null_mut();
        assert!(dds_qget_entity_name(qos_native, &mut ps));
        let p = CStr::from_ptr(ps).to_str().unwrap();
        assert_eq!(p, name.unwrap().name);
        dds_free(ps as *mut ::std::os::raw::c_void);

        dds_delete_qos(qos_native);
    }
}

#[test]
fn test_entity_name_from_native() {
    unsafe {
        let qos_native = dds_create_qos();

        let name = CString::new("TEST_ENTITY_NAME").unwrap();
        dds_qset_entity_name(qos_native, name.as_ptr());

        let policy = entity_name_from_qos_native(qos_native);
        assert!(policy.is_some());
        let policy = policy.unwrap();
        assert_eq!(name.to_str().unwrap(), policy.name);

        dds_delete_qos(qos_native);
    }
}

#[cfg(test)]
unsafe fn assert_property_is_set(qos: *const dds_qos, prop_name: &String, expected_value: &String) {
    let mut pvalue: *mut ::std::os::raw::c_char = std::ptr::null_mut();
    let cname = CString::new(prop_name.as_str()).unwrap();

    assert!(dds_qget_prop(qos, cname.as_ptr(), &mut pvalue));
    let value = CStr::from_ptr(pvalue).to_str().unwrap();
    assert_eq!(value, expected_value);

    dds_free(pvalue as *mut ::std::os::raw::c_void);
}

#[test]
fn test_properties_to_native() {
    unsafe {
        let qos_native = dds_create_qos();

        let mut properties = HashMap::new();
        properties.insert(String::from("PROP_1"), String::from("VALUE_1"));
        properties.insert(String::from("PROP_2"), String::from("VALUE_2"));
        properties.insert(String::from("PROP_3"), String::from("VALUE_3"));

        let policy = Some(properties.clone());
        properties_to_qos_native(qos_native, &policy);

        let mut n: u32 = 0;
        let mut ps: *mut *mut ::std::os::raw::c_char = std::ptr::null_mut();
        assert!(dds_qget_propnames(qos_native, &mut n, &mut ps));
        assert_eq!(n, properties.len() as u32);

        for k in 0..n {
            let p_offset = *(ps.offset(k as isize));

            let p = CStr::from_ptr(p_offset).to_str().unwrap();
            let name = String::from(p);
            dds_free(p_offset as *mut ::std::os::raw::c_void);

            let value = properties.get(&name);
            assert!(value.is_some());
            assert_property_is_set(qos_native, &name, value.unwrap());
        }

        dds_delete_qos(qos_native);
    }
}

#[test]
fn test_properties_from_native() {
    unsafe {
        let qos_native = dds_create_qos();

        let mut properties = HashMap::new();
        properties.insert(String::from("PROP_1"), String::from("VALUE_1"));
        properties.insert(String::from("PROP_2"), String::from("VALUE_2"));
        properties.insert(String::from("PROP_3"), String::from("VALUE_3"));

        for (name, value) in &properties {
            let cname = CString::new(name.as_str()).unwrap();
            let cvalue = CString::new(value.as_str()).unwrap();
            dds_qset_prop(qos_native, cname.as_ptr(), cvalue.as_ptr());
        }

        let policy = properties_from_qos_native(qos_native);
        assert!(policy.is_some());
        let policy = policy.unwrap();
        assert_eq!(policy.len(), properties.len());

        for (name, value) in properties {
            let prop_value = policy.get(&name);
            assert!(prop_value.is_some());
            assert_eq!(prop_value.unwrap(), &value);
        }

        dds_delete_qos(qos_native);
    }
}

#[test]
fn test_ignore_local_to_native() {
    unsafe {
        let kinds = vec![
            (
                IgnoreLocalKind::NONE,
                dds_ignorelocal_kind_DDS_IGNORELOCAL_NONE,
            ),
            (
                IgnoreLocalKind::PARTICIPANT,
                dds_ignorelocal_kind_DDS_IGNORELOCAL_PARTICIPANT,
            ),
            (
                IgnoreLocalKind::PROCESS,
                dds_ignorelocal_kind_DDS_IGNORELOCAL_PROCESS,
            ),
        ];

        for kind in kinds {
            let qos_native = dds_create_qos();

            let policy = IgnoreLocal { kind: kind.0 };
            let input = Some(policy.clone());
            ignore_local_to_qos_native(qos_native, &input.clone());

            let mut ignore_kind: dds_ignorelocal_kind_t = dds_ignorelocal_kind_DDS_IGNORELOCAL_NONE;
            assert!(dds_qget_ignorelocal(qos_native, &mut ignore_kind,));
            assert_eq!(ignore_kind, kind.1);

            dds_delete_qos(qos_native);
        }
    }
}

#[test]
fn test_ignore_local_from_native() {
    unsafe {
        let kinds = vec![
            (
                dds_ignorelocal_kind_DDS_IGNORELOCAL_NONE,
                IgnoreLocalKind::NONE,
            ),
            (
                dds_ignorelocal_kind_DDS_IGNORELOCAL_PARTICIPANT,
                IgnoreLocalKind::PARTICIPANT,
            ),
            (
                dds_ignorelocal_kind_DDS_IGNORELOCAL_PROCESS,
                IgnoreLocalKind::PROCESS,
            ),
        ];

        for kind in kinds {
            let qos_native = dds_create_qos();
            dds_qset_ignorelocal(qos_native, kind.0);

            let policy = ignore_local_from_qos_native(qos_native);
            if is_default(&kind.1) {
                assert!(policy.is_none())
            } else {
                assert!(policy.is_some());
                assert_eq!(policy.unwrap().kind, kind.1);
            }

            dds_delete_qos(qos_native);
        }
    }
}

#[test]
fn test_data_representation_to_native() {
    unsafe {
        let qos_native = dds_create_qos();

        let values: Vec<dds_data_representation_id_t> = vec![
            DDS_DATA_REPRESENTATION_XCDR1 as dds_data_representation_id_t,
            DDS_DATA_REPRESENTATION_XCDR2 as dds_data_representation_id_t,
        ];
        let data_representation: Option<Vec<dds_data_representation_id_t>> = Some(values.clone());
        data_representation_to_qos_native(qos_native, &data_representation);

        let mut n: u32 = 0;
        let mut values_ptr: *mut dds_data_representation_id_t = std::ptr::null_mut();
        assert!(dds_qget_data_representation(
            qos_native,
            &mut n,
            &mut values_ptr
        ));
        assert_eq!(n, values.len() as u32);

        for k in 0..n {
            let value = *values_ptr.offset(k as isize) as dds_data_representation_id_t;
            assert_eq!(values[k as usize], value);
        }

        dds_free(values_ptr as *mut ::std::os::raw::c_void);
        dds_delete_qos(qos_native);
    }
}

#[test]
fn test_data_representation_from_native() {
    unsafe {
        let qos_native = dds_create_qos();

        let values: Vec<dds_data_representation_id_t> = vec![
            DDS_DATA_REPRESENTATION_XCDR1 as dds_data_representation_id_t,
            DDS_DATA_REPRESENTATION_XCDR2 as dds_data_representation_id_t,
        ];
        dds_qset_data_representation(qos_native, values.len() as u32, values.as_ptr());

        let policy = data_representation_from_qos_native(qos_native);
        assert!(policy.is_some());
        let policy = policy.unwrap();
        assert_eq!(policy.len(), values.len());

        for k in 0..policy.len() {
            assert_eq!(values[k], policy[k]);
        }

        dds_delete_qos(qos_native);
    }
}

#[test]
fn test_default_qos_serialization() {
    unsafe {
        let qos_native = dds_create_qos();
        let qos = Qos::from_qos_native(qos_native);
        dds_delete_qos(qos_native);

        let json = serde_json::to_string(&qos).unwrap();
        println!("{json}");

        let qos2 = serde_json::from_str::<Qos>(&json).unwrap();
        assert!(qos == qos2);
        assert_no_policies_set(&qos2);

        let bincode = bincode::serialize(&qos).unwrap();
        println!("len={} : {:x?}", bincode.len(), &bincode);
        let qos3 = bincode::deserialize::<Qos>(&bincode).unwrap();
        assert!(qos == qos3);
        assert_no_policies_set(&qos3);
    }
}

#[test]
fn test_fully_populated_qos_serialization() {
    unsafe {
        let qos_native = dds_create_qos();

        // Set various properties on Qos struct
        let user_data = Some(create_u8_vec_for_tests());
        user_data_to_qos_native(qos_native, &user_data);

        let topic_data = Some(create_u8_vec_for_tests());
        topic_data_to_qos_native(qos_native, &topic_data);

        let group_data = Some(create_u8_vec_for_tests());
        group_data_to_qos_native(qos_native, &group_data);

        let durability = Some(Durability {
            kind: DurabilityKind::TRANSIENT_LOCAL,
        });
        durability_to_qos_native(qos_native, &durability);

        let durability_service = Some(DurabilityService {
            history_kind: HistoryKind::KEEP_ALL,
            ..Default::default()
        });
        durability_service_to_qos_native(qos_native, &durability_service);

        let presentation = Some(Presentation {
            access_scope: PresentationAccessScopeKind::TOPIC,
            ..Default::default()
        });
        presentation_to_qos_native(qos_native, &presentation);

        let deadline = Some(Deadline { period: 15 });
        deadline_to_qos_native(qos_native, &deadline);

        let latency_budget = Some(LatencyBudget { duration: 42 });
        latency_budget_to_qos_native(qos_native, &latency_budget);

        let ownership = Some(Ownership {
            kind: OwnershipKind::EXCLUSIVE,
        });
        ownership_to_qos_native(qos_native, &ownership);

        let ownership_strength = Some(OwnershipStrength { value: 12 });
        ownership_strength_to_qos_native(qos_native, &ownership_strength);

        let liveliness = Some(Liveliness {
            kind: LivelinessKind::MANUAL_BY_PARTICIPANT,
            lease_duration: 3,
        });
        liveliness_to_qos_native(qos_native, &liveliness);

        let time_based_filter = Some(TimeBasedFilter {
            minimum_separation: 56,
        });
        time_based_filter_to_qos_native(qos_native, &time_based_filter);

        let mut partitions = Vec::with_capacity(2);
        partitions.push(String::from("P1"));
        partitions.push(String::from("P2"));
        let parititons = Some(partitions);
        partition_to_qos_native(qos_native, &parititons);

        let reliability = Some(Reliability {
            kind: ReliabilityKind::RELIABLE,
            max_blocking_time: 500,
        });
        reliability_to_qos_native(qos_native, &reliability);

        let transport_priority = Some(TransportPriority { value: 3 });
        transport_priority_to_qos_native(qos_native, &transport_priority);

        let lifespan = Some(Lifespan { duration: 10 });
        lifespan_to_qos_native(qos_native, &lifespan);

        let destination_order = Some(DestinationOrder {
            kind: DestinationOrderKind::BY_SOURCE_TIMESTAMP,
        });
        destination_order_to_qos_native(qos_native, &destination_order);

        let history = Some(History {
            kind: HistoryKind::KEEP_LAST,
            depth: 10,
        });
        history_to_qos_native(qos_native, &history);

        let resource_limits = Some(ResourceLimits {
            max_samples: 100,
            ..Default::default()
        });
        resource_limits_to_qos_native(qos_native, &resource_limits);

        let writer_data_lifecycle = Some(WriterDataLifecycle {
            autodispose_unregistered_instances: false,
        });
        writer_data_lifecycle_to_qos_native(qos_native, &writer_data_lifecycle);

        let reader_data_lifecycle = Some(ReaderDataLifecycle {
            autopurge_disposed_samples_delay: 30,
            ..Default::default()
        });
        reader_data_lifecycle_to_qos_native(qos_native, &reader_data_lifecycle);

        let writer_batching = Some(WriterBatching {
            batch_updates: true,
        });
        writer_batching_to_qos_native(qos_native, &writer_batching);

        let type_consistency = Some(TypeConsistency {
            kind: TypeConsistencyKind::ALLOW_TYPE_COERCION,
            ..Default::default()
        });
        type_consistency_to_qos_native(qos_native, &type_consistency);

        let entity_name = Some(EntityName {
            name: String::from("TEST_ENTITY_NAME"),
        });
        entity_name_to_qos_native(qos_native, &entity_name);

        let mut properties = HashMap::new();
        properties.insert(String::from("PROP_1"), String::from("VALUE_1"));
        properties.insert(String::from("PROP_2"), String::from("VALUE_2"));
        properties.insert(String::from("PROP_3"), String::from("VALUE_3"));
        let properties = Some(properties);
        properties_to_qos_native(qos_native, &properties);

        let ignore_local = Some(IgnoreLocal {
            kind: IgnoreLocalKind::PARTICIPANT,
        });
        ignore_local_to_qos_native(qos_native, &ignore_local);

        let data_representation: Option<Vec<dds_data_representation_id_t>> = Some(vec![
            DDS_DATA_REPRESENTATION_XCDR1 as dds_data_representation_id_t,
            DDS_DATA_REPRESENTATION_XCDR2 as dds_data_representation_id_t,
        ]);
        data_representation_to_qos_native(qos_native, &data_representation);

        // Now test serialization with a populated Qos struct
        let qos = Qos::from_qos_native(qos_native);
        dds_delete_qos(qos_native);

        let json = serde_json::to_string(&qos).unwrap();
        println!("{json}");

        let qos2 = serde_json::from_str::<Qos>(&json).unwrap();
        assert!(qos == qos2);
        assert_all_policies_set(&qos2);

        let bincode = bincode::serialize(&qos).unwrap();
        println!("len={} : {:x?}", bincode.len(), &bincode);
        let qos3 = bincode::deserialize::<Qos>(&bincode).unwrap();
        assert!(qos == qos3);
        assert_all_policies_set(&qos3);
    }
}
