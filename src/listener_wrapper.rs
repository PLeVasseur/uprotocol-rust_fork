use crate::ulistener::UListener;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::ops::Deref;
use std::sync::Arc;

/// A wrapper type around UListener that can be used by `up-client-foo-rust` UPClient libraries
/// to ease some common development scenarios
///
/// # Note
///
/// Not necessary for end-user uEs to use. Primarily intended for `up-client-foo-rust` UPClient libraries
///
/// # Rationale
///
/// The wrapper type is implemented such that it can be used in any location you may wish to
/// hold a generic UListener
///
/// Also implements necessary traits to allow hashing, so that you may hold the wrapper type in
/// collections which require that, such as a `HashMap` or `HashSet`
///
/// Note that the implementation is such that if the same instance of Arc<T> is used multiple times
/// to construct multiple different wrappers, then these wrappers are all considered equivalent
/// due to using the Arc's pointer to as unique identifier
///
/// In the reverse, if two different instances of Arc<T> are used to construct two ArcListener,
/// they will not be considered equivalent due to the Arc pointer being different for each instance
pub struct ListenerWrapper {
    listener: Arc<dyn UListener>,
    pointer_hash: u64,
}

impl ListenerWrapper {
    pub fn new<T: UListener>(listener: &Arc<T>) -> Self {
        let mut hasher = DefaultHasher::new();
        let pointer = Arc::as_ptr(listener) as *const ();
        pointer.hash(&mut hasher);
        let pointer_hash = hasher.finish();
        let listener = listener.clone();

        ListenerWrapper {
            listener,
            pointer_hash,
        }
    }
}

impl Deref for ListenerWrapper {
    type Target = dyn UListener;

    fn deref(&self) -> &Self::Target {
        &*self.listener
    }
}

impl Hash for ListenerWrapper {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.pointer_hash.hash(state);
    }
}

impl PartialEq for ListenerWrapper {
    fn eq(&self, other: &Self) -> bool {
        self.pointer_hash == other.pointer_hash
    }
}

impl Eq for ListenerWrapper {}
