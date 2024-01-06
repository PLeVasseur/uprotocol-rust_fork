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

pub enum Entity {
    Incomplete(UEntity),
    MicroUri(UEntity),
    LongUri(UEntity),
    Resolved(UEntity),
}

pub struct EntityBuilder {
    entity: UEntity,
    id_set: bool,
    name_set: bool,
    version_major_set: bool,
}

impl EntityBuilder {

    pub fn new() -> EntityBuilder {
        EntityBuilder {
            entity: UEntity {
                name: String::new(),
                id: None,
                version_major: None,
                version_minor: None,
            },
            id_set: false,
            name_set: false,
            version_major_set: false,
        }
    }

    pub fn add_name(mut self, name: &str) -> Self {
        self.entity.name = name.to_string();
        self.name_set = true;
        self
    }

    pub fn add_version_minor(mut self, minor: u32) -> Self {
        self.entity.version_minor = Some(minor);
        self
    }

    pub fn add_version_major(mut self, major: u32) -> Self {
        self.entity.version_major = Some(major);
        self.version_major_set = true;
        self
    }

    pub fn add_id(mut self, id: u32) -> Self {
        self.entity.id = Some(id);
        self.id_set = true;
        self
    }

    pub fn build(self) -> Entity {
        match (self.id_set, self.name_set, self.version_major_set) {
            (true, true, true) => Entity::Resolved(self.entity),
            (true, false, true) => Entity::MicroUri(self.entity),
            (false, true, true) => Entity::LongUri(self.entity),
            _ => Entity::Incomplete(self.entity),
        }
    }
}

impl From<Entity> for UEntity {
    fn from(entity: Entity) -> Self {
        match entity {
            Entity::Incomplete(ue) => ue,
            Entity::MicroUri(ue) => ue,
            Entity::LongUri(ue) => ue,
            Entity::Resolved(ue) => ue,
        }
    }
}