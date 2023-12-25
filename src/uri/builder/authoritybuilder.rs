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

enum AuthorityRemote {
    Local(UAuthority),
    RemoteIpv4(UAuthority),
    RemoteIpv6(UAuthority),
    RemoteId(UAuthority)
}

pub struct AuthorityBuilder {}

impl AuthorityBuilder {

    pub fn new() -> AuthorityBuilder {
        AuthorityBuilder {}
    }

    pub fn add_id(self, id: u16) -> AuthorityBuilder {
        AuthorityBuilder {}
    }

    pub fn add_ip(self) -> AuthorityBuilder {
        AuthorityBuilder {}
    }

}