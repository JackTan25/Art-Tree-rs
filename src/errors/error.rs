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

use thiserror::Error;

// 这个文件用来自定义我们自己的error
#[allow(dead_code)]
#[derive(Error, Debug, PartialEq)]
pub enum Errors {
    // 利用thiserror来实现display
    #[error("Fail to insert art_key")]
    FailInsert,
}

#[allow(unused)]
pub type Result<T> = std::result::Result<T, Errors>;
