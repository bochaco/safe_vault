// Copyright 2020 MaidSafe.net limited.
//
// This SAFE Network Software is licensed to you under The General Public License (GPL), version 3.
// Unless required by applicable law or agreed to in writing, the SAFE Network Software distributed
// under the GPL Licence is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied. Please review the Licences for the specific language governing
// permissions and limitations relating to use of the SAFE Network Software.

use crate::{utils, Result};
use pickledb::PickleDb;
use std::{path::Path, sync::Arc};
use tokio::sync::Mutex;

const BLOB_META_DB_NAME: &str = "immutable_data.db";
const HOLDER_META_DB_NAME: &str = "holder_data.db";
const FULL_ADULTS_DB_NAME: &str = "full_adults.db";
// The number of separate copies of a blob chunk which should be maintained.

#[derive(Clone)]
pub struct ChunkHolderDbs {
    pub metadata: Arc<Mutex<PickleDb>>,
    pub holders: Arc<Mutex<PickleDb>>,
    pub full_adults: Arc<Mutex<PickleDb>>,
}

impl ChunkHolderDbs {
    ///
    pub fn new(path: &Path, init: utils::Init) -> Result<Self> {
        let metadata = utils::new_auto_dump_db(path, BLOB_META_DB_NAME, init)?;
        let holders = utils::new_auto_dump_db(path, HOLDER_META_DB_NAME, init)?;
        let full_adults = utils::new_auto_dump_db(path, FULL_ADULTS_DB_NAME, init)?;
        let metadata = Arc::new(Mutex::new(metadata));
        let holders = Arc::new(Mutex::new(holders));
        let full_adults = Arc::new(Mutex::new(full_adults));
        Ok(Self {
            metadata,
            holders,
            full_adults,
        })
    }
}
