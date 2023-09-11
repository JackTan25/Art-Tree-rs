// Copyright 2023 JackTan25
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use bytes::Bytes;

use crate::{
    art_key::ArtKey,
    index::nodes::{fixed_size_allocator::FixedSizeAllocator, node::Node},
};

// todo:(JackTan25) concurrent art-tree
#[allow(unused)]
pub struct Art {
    /// ! Root of the tree
    tree: Node,
    /// ! Fixed-size allocators holding the ART nodes
    allocators: Vec<FixedSizeAllocator>,
    ///! True, if the ART owns its data
    owns_data: bool,
}

#[allow(dead_code, unused)]
impl Art {
    ///! insert a k-v pair into art-tree
    pub fn insert(&mut self, art_key: ArtKey, values: Bytes) {}

    ///! delete key from art-tree, if not-existed, do nothing.
    pub fn delete(&mut self, art_key: ArtKey) {}

    ///! Find the node with a matching key, or return nullptr if not found
    pub fn look_up(node: Node, key: &mut ArtKey, depth: usize) -> Node {
        todo!()
    }
}

impl Art {
    fn insert_internal(&mut self, art_key: ArtKey, values: Bytes) {}
}
