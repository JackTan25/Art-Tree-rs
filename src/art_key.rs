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
use std::cmp::min;
use std::cmp::Ordering;
#[derive(PartialEq, Eq, PartialOrd)]
#[allow(unused)]
pub struct ArtKey {
    data: Bytes,
}

#[allow(unused)]
impl ArtKey {
    fn new_art_key(len: usize) -> Self {
        let inited_data: Vec<u8> = vec![0; len];
        ArtKey {
            data: Bytes::copy_from_slice(&inited_data),
        }
    }

    fn new_art_key_from(bytes: Bytes) -> Self {
        ArtKey { data: bytes }
    }
}

impl Ord for ArtKey {
    fn cmp(&self, other: &Self) -> Ordering {
        let len1 = self.data.len();
        let len2 = other.data.len();
        for i in 0..(min(len1, len2)) {
            if self.data.get(i) > other.data.get(i) {
                return Ordering::Greater;
            } else if self.data.get(i) < other.data.get(i) {
                return Ordering::Less;
            }
        }
        if len1 > len2 {
            Ordering::Greater
        } else if len1 == len2 {
            Ordering::Equal
        } else {
            Ordering::Less
        }
    }
}
