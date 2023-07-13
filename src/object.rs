use std::sync::Arc;

pub trait ObjectAPI {
    fn make_bucket(&self, bucket: &str);
}

unsafe impl Sync for ObjectManager {}
unsafe impl Send for ObjectManager {}

pub struct ObjectManager(Arc<dyn ObjectAPI>);

impl ObjectManager {
    pub fn new<T: ObjectAPI + 'static>(m: T) -> ObjectManager {
        ObjectManager(Arc::new(m))
    }

    pub fn make_bucket(bucket: &str) {}
}

impl Clone for ObjectManager {
    fn clone(&self) -> Self {
        ObjectManager(self.0.clone())
    }
}
