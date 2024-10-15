use serde::{Deserialize, Serialize};

macro_rules! create_struct_id {
    ($name:ident, $id_conter:ident) => {
        #[derive(
            Debug,
            Clone,
            Copy,
            Eq,
            PartialEq,
            Hash,
            Ord,
            PartialOrd,
            Default,
            Serialize,
            Deserialize,
        )]
        pub struct $name {
            id: usize,
        }

        static $id_conter: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(1);

        impl $name {
            pub fn null() -> Self {
                $name { id: 0 }
            }

            #[allow(dead_code)]
            pub(crate) fn new() -> Self {
                $name {
                    id: $id_conter.fetch_add(1, std::sync::atomic::Ordering::SeqCst),
                }
            }

            pub(crate) fn update(&mut self) {
                self.id = $id_conter.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            }

            pub fn value(&self) -> usize {
                self.id
            }
        }

        impl From<usize> for $name {
            fn from(id: usize) -> Self {
                $name { id }
            }
        }
    };
}

create_struct_id!(CoordId, COORD_ID_COUNTER);
create_struct_id!(LayerId, LAYER_ID_COUNTER);
