use std::{any::{Any, TypeId}, collections::HashMap, sync::Arc};

/// A pocket is a storage where middleware can store their data that then can be processed inside
/// the request handler
#[derive(Default, Debug, Clone)]
pub struct Pocket {
    data: HashMap<TypeId, Arc<dyn Any + Send + Sync>>,
}

impl Pocket {
    /// Creates a new empty pocket
    pub fn new() -> Pocket {
        Pocket {
            data: HashMap::new()
        }
    } 

    /// Inserts data into a pocket
    pub fn insert <T>(&mut self, value: T)
    where 
        T: Any + Send + Sync + 'static
    {
        self.data.insert(TypeId::of::<T>(), Arc::new(value));
    }

    /// Gets data from a pocket
    pub fn get<T>(&self) -> Option<Arc<T>>
    where
        T: Any + Send + Sync + 'static
    {
        self.data.get(&TypeId::of::<T>()).and_then(|arc| {
            arc.clone().downcast::<T>().ok()
        })
    }
}
