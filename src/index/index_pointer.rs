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

// Bit-shifting
static SHIFT_OFFSET: u64 = 32;
static SHIFT_METADATA: u64 = 56;
//  AND operations
static AND_OFFSET: u64 = 0x0000000000FFFFFF;
static AND_BUFFER_ID: u64 = 0x00000000FFFFFFFF;
static AND_METADATA: u64 = 0xFF00000000000000;

// Data holds all the information contained in an IndexPointer
// [0 - 7: metadata,
// 8 - 23: offset, 24 - 63: buffer ID]
pub trait IndexPointer {
    // Get data (all 64 bits)
    fn get(&self) -> u64;

    // Set data (all 64 bits)
    fn set(&mut self, data_p: u64);

    // Returns false, if the metadata is empty
    fn has_metadata(&self) -> bool {
        self.get() & AND_METADATA == 0
    }

    // Get metadata (zero to 7th bit)
    fn get_metadata(&self) -> u8 {
        (self.get() >> SHIFT_METADATA) as u8
    }

    // Set metadata (zero to 7th bit)
    fn set_metadata(&mut self, metadata: u8) {
        self.set(self.get() + (metadata as u64) << SHIFT_METADATA);
    }

    // Get the offset (8th to 23rd bit)
    fn get_offset(&self) -> u64 {
        (self.get() >> SHIFT_OFFSET) & AND_OFFSET
    }

    // Get the buffer ID (24th to 63rd bit)
    fn get_buffer_id(&self) -> u64 {
        return self.get() & AND_BUFFER_ID;
    }

    // Resets the IndexPointer
    fn clear(&mut self) {
        self.set(0)
    }

    // Adds an u64 to a buffer ID, the rightmost 32 bits of data contain the buffer ID
    fn increase_buffer_id(&mut self, summand: u64) {
        self.set(self.get() + summand);
    }

    // Comparison operator
    fn equal(&self, ptr: &dyn IndexPointer) -> bool {
        self.get() == ptr.get()
    }
}
