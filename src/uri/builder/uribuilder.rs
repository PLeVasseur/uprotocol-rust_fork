/********************************************************************************
 * Copyright (c) 2023 Contributors to the Eclipse Foundation
 *
 * See the NOTICE file(s) distributed with this work for additional
 * information regarding copyright ownership.
 *
 * This program and the accompanying materials are made available under the
 * terms of the Apache License Version 2.0 which is available at
 * https://www.apache.org/licenses/LICENSE-2.0
 *
 * SPDX-License-Identifier: Apache-2.0
 ********************************************************************************/

use crate::uprotocol::{UEntity, UResource, Uuri};
use crate::uri::builder::authoritybuilder::Authority;
use crate::uri::builder::entitybuilder::Entity;
use crate::uri::builder::resourcebuilder::Resource;
use crate::uri::validator::urivalidator::UriValidator;

pub enum Uri {
    Incomplete(Uuri),
    MicroForm(Uuri),
    LongForm(Uuri),
    Resolved(Uuri),
}
pub struct UriBuilder {
    authority: Option<Authority>,
    entity: Option<Entity>,
    resource: Option<Resource>
}

impl UriBuilder {
    pub fn new() -> Self {
        UriBuilder {
            authority: None,
            entity: None,
            resource: None
        }
    }

    pub fn add_authority(mut self, authority: Authority) -> Self {
        self.authority = Some(authority);
        self
    }

    pub fn add_entity(mut self, entity: Entity) -> Self {
        self.entity = Some(entity);
        self
    }

    pub fn add_resource(mut self, resource: Resource) -> Self {
        self.resource = Some(resource);
        self
    }

    pub fn is_micro_form(&self) -> bool {
        UriValidator::is_micro_form(self)
    }

    pub fn is_long_form(&self) -> bool {
        UriValidator::is_long_form(self)
    }

    pub fn is_resolved(&self) -> bool {
        UriValidator::is_resolved()
    }

    pub fn is_topic(&self) -> bool {
        todo!()
    }

    pub fn build(mut self) -> Uri {
        todo!()
    }
}

pub struct Uuri {
    // Fields and methods of Uuri
}

pub trait UriType {}

pub struct RpcRequest;
impl UriType for RpcRequest {}

pub struct RpcResponse;
impl UriType for RpcResponse {}

pub struct Topic;
impl UriType for Topic {}

pub struct Incomplete<T: UriType> {
    value: T,
    uuri: Uuri,
}

pub struct MicroForm<T: UriType> {
    value: T,
    uuri: Uuri,
}

pub struct LongForm<T: UriType> {
    value: T,
    uuri: Uuri,
}

pub struct Resolved<T: UriType> {
    value: T,
    uuri: Uuri,
}

// Trait for marking a URI as resolved
pub trait IsResolved {}

// Only the Resolved type implements IsResolved
impl<T: UriType> IsResolved for Resolved<T> {}

// The send function only accepts URIs that are resolved
pub fn send<T: UriType + IsResolved>(uri: T) {
    let uuri = uri.get_uuri();
    // Use uuri for sending...
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_resolved_uri_send() {
        let resolved_uri = Resolved(RpcRequest);
        send(resolved_uri);
    }

}

// impl From<Uri> for Uuri {
//     pub fn from(uri: Uri) -> Self {
//
//
//         Uuri {
//             authority:
//         }
//     }
// }

// impl UriBuilder {
//     pub fn add_entity(self) -> UriBuilder {
//         UriBuilder {}
//     }
// }