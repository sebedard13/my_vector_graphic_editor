static ID_COUNTER: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(1);

macro_rules! create_struct_id {
    ($name:ident) => {
        #[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Ord, PartialOrd, Default)]
        pub struct $name {
            id: usize,
        }

        impl $name {
            pub fn null() -> Self {
                $name { id: 0 }
            }

            #[allow(dead_code)]
            pub(crate) fn new() -> Self {
                $name {
                    id: ID_COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst),
                }
            }

            pub(crate) fn update(&mut self) {
                self.id = ID_COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            }
        }
    };
}

create_struct_id!(CoordId);
create_struct_id!(LayerId);
