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

use crate::uprotocol::{UEntity};

pub struct EntityBuilder {
    entity: UEntity
}

impl EntityBuilder {
    pub fn add_name(self, name: &str) -> UriBuilder {
        EntityBuilder { entity: () }
    }

    pub fn add_version(self) -> UriBuilder {
        EntityBuilder { entity: () }
    }

    pub fn add_id(self, id: u16) -> UriBuilder {
        EntityBuilder { entity: () }
    }

    pub fn build(self) -> UEntity {
        self.entity
    }
}
