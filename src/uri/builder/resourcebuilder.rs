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

use crate::uprotocol::UResource;

const MAX_RPC_ID: u16 = 1000;

pub enum Resource {
    RpcResponse(UResource),
    IncompleteRpcRequest(UResource),
    CompleteRpcRequest(UResource),
    IncompleteTopic(UResource),
    CompleteTopic(UResource),
    IncompleteNonRpc(UResource),
    CompleteNonRpc(UResource),
}

// TODO: Can RPC request / response have #message attached? I don't see anything disallowing it in the spec
pub struct RpcResponseResourceBuilder {}

impl RpcResponseResourceBuilder {
    pub fn build() -> Resource {
        Resource::RpcResponse(UResource {
            name: String::from("rpc"),
            instance: Some(String::from("response")),
            id: Some(0),
            message: None, // TODO: Can the message ever be set for an RpcResponse?
        })
    }
}

pub struct RpcRequestResourceBuilder {
    resource: UResource,
}

impl RpcRequestResourceBuilder {
    pub fn new(method: Option<String>, id: Option<u16>) -> Self {
        let id = id.map(|val| val as u32);

        RpcRequestResourceBuilder {
            resource: UResource {
                name: String::new(),
                instance: method,
                id: id,
                message: None, // TODO: Can the message ever be set for an RpcRequest?
            },
        }
    }

    pub fn add_id(mut self, id: u16) -> Self {
        self.resource.id = Some(id as u32);
        self
    }

    pub fn add_method(mut self, method: &str) -> Self {
        self.resource.instance = Some(method.to_string());
        self
    }

    pub fn build(self) -> Resource {
        if !self.resource.id.is_some()
            || self.resource.name.trim().is_empty()
            || !self.resource.instance.is_some()
        {
            Resource::CompleteRpcRequest(self.resource)
        } else {
            Resource::IncompleteRpcRequest(self.resource)
        }
    }
}

pub struct TopicResourceBuilder {
    resource: UResource,
}

impl TopicResourceBuilder {
    pub fn new(id: u16) -> Option<Self> {
        if id <= MAX_RPC_ID {
            return None;
        }

        Some(TopicResourceBuilder {
            resource: UResource {
                name: String::new(),
                instance: None,
                id: Some(id as u32),
                message: None,
            },
        })
    }

    pub fn add_name(mut self, name: String) -> Self {
        self.resource.name = name;
        self
    }

    pub fn add_message(mut self, message: String) -> Self {
        self.resource.message = Some(message);
        self
    }

    pub fn build(self) -> Resource {
        if !self.resource.name.trim().is_empty() && self.resource.message.is_some() {
            Resource::CompleteTopic(self.resource)
        } else {
            Resource::IncompleteTopic(self.resource)
        }
    }
}

// TODO: Discuss what other kinds of "things" should be available / representable by UURIs
pub struct NonRpcResourceBuilder {
    resource: UResource,
}

impl NonRpcResourceBuilder {
    pub fn new(id: u16) -> Option<Self> {
        if id <= MAX_RPC_ID {
            return None;
        }

        Some(NonRpcResourceBuilder {
            resource: UResource {
                name: String::new(),
                instance: None,
                id: Some(id as u32),
                message: None,
            },
        })
    }

    pub fn add_name(mut self, name: String) -> Self {
        self.resource.name = name;
        self
    }

    pub fn build(self) -> Resource {
        if !self.resource.name.trim().is_empty() {
            Resource::CompleteNonRpc(self.resource)
        } else {
            Resource::IncompleteNonRpc(self.resource)
        }
    }
}

impl From<Resource> for UResource {
    fn from(res: Resource) -> Self {
        match res {
            Resource::RpcResponse(ur) => ur,
            Resource::IncompleteRpcRequest(ur) => ur,
            Resource::CompleteRpcRequest(ur) => ur,
            Resource::IncompleteNonRpc(ur) => ur,
            Resource::CompleteNonRpc(ur) => ur,
            Resource::IncompleteTopic(ur) => ur,
            Resource::CompleteTopic(ur) => ur,
        }
    }
}

pub struct UResourceBuilder {}

impl UResourceBuilder {
    /// Builds a `UResource` for an RPC response.
    ///
    /// # Returns
    /// Returns a `UResource` for an RPC response.
    pub fn for_rpc_response() -> UResource {
        UResource {
            name: String::from("rpc"),
            instance: Some(String::from("response")),
            id: Some(0),
            message: None,
        }
    }

    /// Builds a `UResource` for an RPC request with an ID and method name.
    ///
    /// # Arguments
    /// * `method` - The method to be invoked. Pass `None` if no method is specified.
    /// * `id` - The ID of the request. Pass `None` if no ID is specified.
    ///
    /// # Returns
    /// Returns a `UResource` for an RPC request.
    pub fn for_rpc_request(method: Option<String>, id: Option<u32>) -> UResource {
        UResource {
            name: String::from("rpc"),
            instance: method,
            id,
            message: None,
        }
    }

    /// Builds a `UResource` from an ID.
    ///
    /// This method determines the type of `UResource` to create based on the ID value.
    /// If the ID is less than `MAX_RPC_ID`, it is considered an ID for an RPC request
    /// and a corresponding `UResource` for an RPC request is created. Otherwise, a
    /// generic `UResource` is created with the provided ID and default values for other fields.
    ///
    /// # Arguments
    /// * `id` - The ID used to determine the type of `UResource` to create.
    ///
    /// # Returns
    /// Returns a `UResource` instance corresponding to the given ID.
    pub fn from_id(id: u16) -> UResource {
        if id < MAX_RPC_ID {
            return Self::for_rpc_request(None, Some(id as u32));
        }

        UResource {
            name: String::new(),
            instance: None,
            id: Some(id as u32),
            message: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::uri::builder::resourcebuilder::{
        NonRpcResourceBuilder, Resource, RpcRequestResourceBuilder, RpcResponseResourceBuilder,
        TopicResourceBuilder, MAX_RPC_ID,
    };

    #[test]
    fn test_rpc_response_creation() {
        let resource = RpcResponseResourceBuilder::build();
        match resource {
            Resource::RpcResponse(u) => {
                assert_eq!(u.name, "rpc");
                assert_eq!(u.instance, Some(String::from("response")));
                assert_eq!(u.id, Some(0));
                assert!(u.message.is_none());
            }
            _ => panic!("Expected RpcResponse"),
        }
    }

    #[test]
    fn test_rpc_request_creation_without_id_and_method() {
        let builder = RpcRequestResourceBuilder::new(None, None);
        let resource = builder.build();
        matches!(resource, Resource::IncompleteRpcRequest(_));
    }

    #[test]
    fn test_rpc_request_creation_with_id() {
        let id = 500u16;
        let builder = RpcRequestResourceBuilder::new(None, Some(id));
        let resource = builder.build();
        matches!(resource, Resource::IncompleteRpcRequest(_));
    }

    #[test]
    fn test_rpc_request_creation_with_id_and_method() {
        let id = 500u16;
        let method = "some_method";
        let builder = RpcRequestResourceBuilder::new(Some(method.to_string()), Some(id));
        let resource = builder.build();
        matches!(resource, Resource::CompleteRpcRequest(_));
    }

    #[test]
    fn test_rpc_request_add_method() {
        let method = "new_method";
        let builder = RpcRequestResourceBuilder::new(None, None).add_method(method);
        assert_eq!(builder.resource.instance, Some(method.to_string()));
    }

    #[test]
    fn test_rpc_request_add_id() {
        let id = 500u16;
        let builder = RpcRequestResourceBuilder::new(None, None).add_id(id);
        assert_eq!(builder.resource.id, Some(id as u32));
    }

    #[test]
    fn test_topic_new_with_low_id() {
        let builder = TopicResourceBuilder::new(MAX_RPC_ID);
        assert!(builder.is_none());
    }

    #[test]
    fn test_topic_new_with_high_id() {
        let builder = TopicResourceBuilder::new(MAX_RPC_ID + 1);
        assert!(builder.is_some());
    }

    #[test]
    fn test_topic_add_name() {
        let name = "TestName".to_string();
        let builder = TopicResourceBuilder::new(MAX_RPC_ID + 1)
            .unwrap()
            .add_name(name.clone());
        assert_eq!(builder.resource.name, name);
    }

    #[test]
    fn test_topic_add_message() {
        let message = "TestMessage".to_string();
        let builder = TopicResourceBuilder::new(MAX_RPC_ID + 1)
            .unwrap()
            .add_message(message.clone());
        assert_eq!(builder.resource.message, Some(message));
    }

    #[test]
    fn test_topic_build_complete_topic() {
        let builder = TopicResourceBuilder::new(MAX_RPC_ID + 1)
            .unwrap()
            .add_name("TestName".to_string())
            .add_message("TestMessage".to_string());
        match builder.build() {
            Resource::CompleteTopic(_) => {}
            _ => panic!("Expected CompleteTopic"),
        };
    }

    #[test]
    fn test_topic_build_incomplete_topic() {
        let builder = TopicResourceBuilder::new(MAX_RPC_ID + 1).unwrap();
        match builder.build() {
            Resource::IncompleteTopic(_) => {}
            _ => panic!("Expected IncompleteTopic"),
        };
    }

    #[test]
    fn test_non_rpc_resource_creation_with_valid_id() {
        let id = MAX_RPC_ID + 1;
        let builder = NonRpcResourceBuilder::new(id);
        assert!(builder.is_some());
    }

    #[test]
    fn test_non_rpc_resource_creation_with_invalid_id() {
        let id = MAX_RPC_ID;
        let builder = NonRpcResourceBuilder::new(id);
        assert!(builder.is_none());
    }

    #[test]
    fn test_non_rpc_resource_add_name() {
        let id = MAX_RPC_ID + 1;
        let name = "non_rpc";
        let builder = NonRpcResourceBuilder::new(id)
            .unwrap()
            .add_name(name.to_string());
        assert_eq!(builder.resource.name, name);
    }

    #[test]
    fn test_non_rpc_resource_build_complete() {
        let id = MAX_RPC_ID + 1;
        let name = "non_rpc";
        let resource = NonRpcResourceBuilder::new(id)
            .unwrap()
            .add_name(name.to_string())
            .build();
        matches!(resource, Resource::CompleteNonRpc(_));
    }

    #[test]
    fn test_non_rpc_resource_build_id_only() {
        let id = MAX_RPC_ID + 1;
        let resource = NonRpcResourceBuilder::new(id).unwrap().build();
        matches!(resource, Resource::IncompleteNonRpc(_));
    }
}
