use std::{ffi::c_void, mem, ptr};

use windows::{
    core::{IUnknown, Interface, Result},
    Win32::{
        Foundation::E_FAIL,
        System::Diagnostics::Debug::{DebugCreate, IDebugClient, IHostDataModelAccess},
    },
};

mod hello_extension;
mod util;

fn init_extension() -> Result<()> {
    let client = unsafe {
        let mut p = ptr::null_mut();
        DebugCreate(&IDebugClient::IID, &mut p)?;
        mem::transmute::<*mut c_void, IUnknown>(p).cast::<IDebugClient>()?
    };

    let mut data_model_manager = None;
    let mut debug_host = None;
    unsafe {
        client
            .cast::<IHostDataModelAccess>()?
            .GetDataModel(&mut data_model_manager, &mut debug_host)?;
    }

    match (data_model_manager, debug_host) {
        (Some(data_model_manager), Some(debug_host)) => {
            let ext = hello_extension::HelloExtension::new(data_model_manager, debug_host);
            Ok(())
        }
        _ => Err(E_FAIL.into()),
    }
}

#[allow(non_snake_case)]
mod exports {
    use windows::{core::HRESULT, Win32::Foundation::S_OK};

    #[no_mangle]
    fn DebugExtensionInitialize(_version: *mut u32, _flags: *mut u32) -> HRESULT {
        super::init_extension().into()
    }

    #[no_mangle]
    fn DebugExtensionCanUnload() -> HRESULT {
        S_OK
    }

    #[no_mangle]
    fn DebugExtensionUninitialize() {}

    #[no_mangle]
    fn DebugExtensionUnload() {}
}
