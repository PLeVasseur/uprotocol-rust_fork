/********************************************************************************
 * Copyright (c) 2024 Contributors to the Eclipse Foundation
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

use crate::{UMessage, UStatus};
use std::any::{Any, TypeId};
use std::hash::{Hash, Hasher};


/// `UListener` is the uP-L1 interface that provides a means to create listeners which are registered to `UTransport`
///
/// Implementations of `UListener` contain the details for what should occur when a message is received
/// For more information, please refer to
/// [uProtocol Specification](https://github.com/eclipse-uprotocol/uprotocol-spec/blob/main/up-l1/README.adoc).
pub trait UListener: Any + Send + Sync {
    /// Performs some action on receipt
    ///
    /// # Arguments
    ///
    /// * `received` - Either the message or error `UStatus` received
    fn on_receive(&self, received: Result<UMessage, UStatus>);
}

/// A wrapper type around UListener that can be used by `up-client-foo-rust` UPClient libraries
/// to ease some common development scenarios
///
/// # Rationale
///
/// The wrapper type is implemented such that it can be used in any location you may wish to
/// hold a generic UListener
///
///
pub struct ListenerWrapper {
    listener: Box<dyn UListener + 'static>,
    type_id: TypeId
}

impl ListenerWrapper {
    pub fn new<T>(listener: T) -> Self
        where
            T: UListener + 'static,
    {
        let boxed_listener: Box<dyn UListener + 'static> = Box::new(listener);
        Self {
            listener: boxed_listener,
            type_id: TypeId::of::<T>(),
        }
    }
}

impl UListener for ListenerWrapper {
    fn on_receive(&self, received: Result<UMessage, UStatus>) {
        self.listener.on_receive(received)
    }
}

pub trait UListenerTypeTag {
    fn as_any(&self) -> &dyn Any;
}

impl UListenerTypeTag for ListenerWrapper {
    fn as_any(&self) -> &dyn Any {
        &self.type_id
    }
}

impl PartialEq for ListenerWrapper {
    fn eq(&self, other: &Self) -> bool {
        self.type_id == other.type_id
    }
}

impl Eq for ListenerWrapper {}

impl Hash for ListenerWrapper {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.type_id.hash(state);
    }
}
