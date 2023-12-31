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

use crate::index::index_pointer::IndexPointer;

#[allow(unused)]
pub enum NodeType {
    PREFIX,
    LEAF,
    NODE4,
    NODE16,
    NODE48,
    NODE256,
    LeafInlined,
}

// Data holds all the information contained in an IndexPointer
// [0 - 7: metadata,
// 8 - 23: offset, 24 - 63: buffer ID]
pub struct Node {
    data: u64,
}

impl IndexPointer for Node {
    fn get(&self) -> u64 {
        self.data
    }

    fn set(&mut self, data_p: u64) {
        self.data = data_p
    }
}
