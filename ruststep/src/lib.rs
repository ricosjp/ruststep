//! Reference of ruststep, a STEP toolkit written in pure Rust
//!
//! See [README.md][README] and [ARCHITECTURE.md][ARCHITECTURE] on [GitHub][GitHub]
//! for the overview and high-level architecture of this project.
//! This reference focus on the detail usage of this crate.
//!
//! [GitHub]: https://github.com/ricosjp/ruststep
//! [README]: https://github.com/ricosjp/ruststep/blob/master/README.md
//! [ARCHITECTURE]: https://github.com/ricosjp/ruststep/blob/master/ARCHITECTURE.md
//!

#![deny(broken_intra_doc_links)]

pub mod ast;
pub mod error;
pub mod header;
pub mod parser;
pub mod place_holder;
pub mod primitive;
pub mod tables;

pub mod ap000;
// pub mod ap201;

#[macro_export]
macro_rules! custom_any {
    ($trait_name:ident) => {
        pub trait $trait_name: ::std::any::Any + ::std::fmt::Debug + ::dyn_clone::DynClone {}

        ::dyn_clone::clone_trait_object!($trait_name);

        impl dyn $trait_name + 'static {
            pub fn is<Sub: $trait_name + 'static>(&self) -> bool {
                self.type_id() == ::std::any::TypeId::of::<Sub>()
            }
            pub fn downcast_ref<Sub: $trait_name + 'static>(&self) -> Option<&Sub> {
                if self.is::<Sub>() {
                    // See also the document of core::any::Any
                    // https://doc.rust-lang.org/src/core/any.rs.html#220
                    unsafe { Some(&*(self as *const dyn $trait_name as *const Sub)) }
                } else {
                    None
                }
            }
            pub fn downcast_mut<Sub: $trait_name + 'static>(&mut self) -> Option<&mut Sub> {
                if self.is::<Sub>() {
                    // See also the document of core::any::Any
                    // https://doc.rust-lang.org/src/core/any.rs.html#256
                    unsafe { Some(&mut *(self as *mut dyn $trait_name as *mut Sub)) }
                } else {
                    None
                }
            }
        }
    };
}
