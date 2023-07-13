use crate::object::ObjectAPI;

unsafe impl Send for Fs {}
unsafe impl Sync for Fs {}

pub struct Fs {}

impl ObjectAPI for Fs {
    fn make_bucket(&self, bucket: &str) {}
}
