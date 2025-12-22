#![cfg_attr(docsrs, feature(doc_cfg))]
#![no_std]

extern crate alloc;

pub mod cfg {
    pub use vc_cfg::std;

    vc_cfg::define_alias! {
        #[cfg(feature = "rayon")] => rayon,
    }
}

pub mod hash;
pub mod vec;

mod default;
mod range_invoke;
mod typeid_map;

pub use default::default;
pub use typeid_map::TypeIdMap;

pub mod prelude {
    pub use alloc::{
        borrow::{Cow, ToOwned},
        boxed::Box,
        format,
        string::String,
        string::ToString,
        vec,
        vec::Vec,
    };

    pub use crate::default;
}
