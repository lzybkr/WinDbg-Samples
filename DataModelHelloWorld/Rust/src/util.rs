use windows::{
    core::Result,
    Win32::{
        Foundation::{BSTR, E_NOTIMPL, PWSTR},
        System::{
            Com::VARIANT,
            Diagnostics::Debug::{IDataModelManager, IModelObject, IModelPropertyAccessor},
        },
    },
};

pub fn create_property(property: IModelPropertyAccessor) -> Result<IModelObject> {
    let variant: VARIANT;
    Err(E_NOTIMPL.into())
}
