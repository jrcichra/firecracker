// Copyright 2018 Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

/// The strongly typed that contains general information about the microVM.
#[derive(Debug, Serialize)]
pub struct InstanceInfo {
    /// The ID of the microVM.
    pub id: String,
    /// Whether the microVM has been started.
    pub started: bool,
    /// The version of the VMM that runs the microVM.
    pub vmm_version: String,
}
