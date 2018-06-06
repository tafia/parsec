// Copyright 2018 MaidSafe.net limited.
//
// This SAFE Network Software is licensed to you under The General Public License (GPL), version 3.
// Unless required by applicable law or agreed to in writing, the SAFE Network Software distributed
// under the GPL Licence is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied. Please review the Licences for the specific language governing
// permissions and limitations relating to use of the SAFE Network Software.

use super::PackedEvent;
use id::PublicId;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::collections::BTreeMap;
use std::fmt::Debug;

pub struct Request<T: Serialize + DeserializeOwned + Debug + Eq, P: PublicId> {
    events: Vec<PackedEvent<T, P>>,
}

pub struct Response<T: Serialize + DeserializeOwned + Debug + Eq, P: PublicId> {
    events: Vec<PackedEvent<T, P>>,
}
