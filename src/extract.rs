//! Common functions

use syn::{DataStruct, DeriveInput, Data, Error, Result};

use crate::faultmsg::{StructIs, Problem};

pub fn named_struct(node: &DeriveInput) -> Result<&DataStruct> {
    match node.data {
        Data::Struct(ref structure) => Ok(structure),
        Data::Enum(_) => Err(
            Error::new_spanned(node, Problem::NotNamedStruct(StructIs::Enum))
        ),
        Data::Union(_) => Err(
            Error::new_spanned(node, Problem::NotNamedStruct(StructIs::Union))
        ),
    }
}
