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

use crate::uprotocol::{UAuthority};
use crate::uprotocol::{REMOTE_ID_MINIMUM_BYTES, REMOTE_ID_MAXIMUM_BYTES};

use std::net::IpAddr;
pub struct RemoteId(Vec<u8>);

impl RemoteId {
    fn new(data: &[u8]) -> Option<Self> {
        if data.len() >= REMOTE_ID_MINIMUM_BYTES && data.len() <= REMOTE_ID_MAXIMUM_BYTES {
            Some(RemoteId(data.to_vec()))
        } else {
            None
        }
    }

    fn to_vec(&self) -> Vec<u8> {
        self.0
    }
}

pub enum Authority {
    Local(UAuthority),
    RemoteName(UAuthority),
    RemoteIpv4(UAuthority),
    RemoteIpv6(UAuthority),
    RemoteId(UAuthority)
}

impl Authority {
    fn local() -> Authority {
        Authority::Local(None)
    }

    fn remote_name(name: &str) {
        // TODO: Are there considerations of what a "valid" name is?
        Authority::RemoteName(name)
    }

    fn remote_ip(ip: IpAddr) -> Authority {
        Some(match ip {
            IpAddr::V4(ipv4) => Authority::RemoteIpv4(ipv4.octets().to_vec()),
            IpAddr::V6(ipv6) => Authority::RemoteIpv6(ipv6.octets().to_vec()),
        })
    }

    fn remote_id(id: RemoteId) -> Authority {
        Authority::RemoteId(Some(id.to_vec()))
    }
}

impl From<Authority> for UAuthority {
    fn from(authority: Authority) -> Self {
        match authority {
            Authority::Local(ua) => ua,
            Authority::RemoteName(ua) => ua,
            Authority::RemoteIpv4(ua) => ua,
            Authority::RemoteIpv6(ua) => ua,
            Authority::RemoteId(ua) => ua,
        }
    }
}