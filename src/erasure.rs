use crate::object::ObjectAPI;

unsafe impl Send for Erasure {}
unsafe impl Sync for Erasure {}

struct Erasure {}

impl ObjectAPI for Erasure {
    fn make_bucket(&self, bucket: &str) {}
}
