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

use crate::fixed_size_allocator::FixedSizeAllocator;
use crate::node::Node;

#[allow(unused)]
struct Art {
    /// ! Root of the tree
    tree: Node,
    /// ! Fixed-size allocators holding the ART nodes
    allocators: Vec<FixedSizeAllocator>,
    ///! True, if the ART owns its data
    owns_data: bool,
}
