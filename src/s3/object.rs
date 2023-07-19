use super::storage::StorageAPI;
use crate::util::error::S3Result;
use chrono::prelude::*;
use std::ffi::OsString;
use std::sync::Arc;

pub struct BucketInfo {
    pub name: OsString,
    pub created_time: DateTime<Local>,
}

pub trait ObjectAPI {
    fn make_bucket(&self, bucket: &str) -> S3Result<()>;
    fn list_bucket(&self) -> S3Result<Vec<BucketInfo>>;
}

unsafe impl Sync for ObjectManager {}
unsafe impl Send for ObjectManager {}

pub struct ObjectManager(Arc<dyn ObjectAPI>);

impl ObjectManager {
    pub fn new<T: ObjectAPI + 'static>(m: T) -> ObjectManager {
        ObjectManager(Arc::new(m))
    }

    pub fn make_bucket(&self, bucket: &str) -> S3Result<()> {
        self.0.make_bucket(bucket)
    }

    pub fn list_bucket(&self) -> S3Result<Vec<BucketInfo>> {
        self.0.list_bucket()
    }
}

impl Clone for ObjectManager {
    fn clone(&self) -> Self {
        ObjectManager(self.0.clone())
    }
}

pub struct MultiObject {
    storages: Vec<Arc<dyn StorageAPI>>,
}
