#![feature(associated_type_bounds)]

use simply_wayland;
use crate::types::*;
//use crate::simply_wayland;

#[allow(non_camel_case_types)]
pub struct wl_display {
    pub ptr: *mut simply_wayland::wl_display,
}

//type wl_display = *mut simply_wayland::wl_display;

impl wl_display {
    pub fn connect(name: Option<&str>) -> Option<wl_display> {
        unsafe {
            let name = match name {
                    Some(string) => std::ffi::CString::new(string).expect("Couldn't convert display name").as_ptr(),
                    None => std::ptr::null(),
            };

            let ptr: *mut simply_wayland::wl_display = simply_wayland::wl_display_connect(name as *const std::os::raw::c_char);
            println!("Maybe connected to display");

            //Some(Box::from_raw(ptr))
            match ptr.is_null() {
                true => None,
                false => Some(wl_display { ptr }),
            }
        }
    }

    pub fn dispatch(&mut self) -> bool {
        unsafe {
            println!("before call dispatch");
            let c_int_r: std::os::raw::c_int = simply_wayland::wl_display_dispatch(self.ptr);
            println!("after call dispatch");
            let native_r: i32 = c_int_r.into();

            native_r != -1
        }
    }

    pub fn get_registry(&mut self) -> wl_registry {
        unsafe {
            //wl_registry { ptr: simply_wayland::wl_display_get_registry(self.ptr) }
            println!("inside get registry, get_registry const is {}", simply_wayland::WL_DISPLAY_GET_REGISTRY);
            let registry_ptr: *mut simply_wayland::wl_proxy = simply_wayland::wl_proxy_marshal_constructor(
                std::mem::transmute(self.ptr),
                simply_wayland::WL_DISPLAY_GET_REGISTRY,
                &simply_wayland::wl_registry_interface,
                std::ptr::null::<*const std::ffi::c_void>()
            );

            //println!("called proxy marshal");

            wl_registry {
                ptr: std::mem::transmute(registry_ptr)
            }
        }
    }

    pub fn roundtrip(&mut self) {
        unsafe {
            println!("roundtripping");
            simply_wayland::wl_display_roundtrip(self.ptr);
        }
    }

    /// Restrict passed data regions to impl Copy to require
    /// that destruction is trivial and non-virtual
    #[allow(dead_code)]
    pub unsafe fn add_listener<DisplayListenerData: Copy>(
        &mut self,
        onFatalError: unsafe extern "C" fn(
            data: Option<&mut DisplayListenerData>,
            wl_display: &mut simply_wayland::wl_display,
            object_id: *mut std::os::raw::c_void,
            code: u32,
            *const std::os::raw::c_char),
        onObjectDelete: unsafe extern "C" fn(
            data: &mut DisplayListenerData,
            wl_display: &mut simply_wayland::wl_display,
            id: u32),
        data: Box<DisplayListenerData>
    ) {
        let data_ptr = Box::into_raw(data);

        let listener = simply_wayland::wl_display_listener {
            error: Some(std::mem::transmute(onFatalError)), delete_id: Some(std::mem::transmute(onObjectDelete))
        };

        let listener_ptr = Box::into_raw(Box::new(listener));

        simply_wayland::wl_proxy_add_listener(
            std::mem::transmute::<*mut simply_wayland::wl_display, *mut simply_wayland::wl_proxy>(self.ptr),
            std::mem::transmute::<*mut simply_wayland::wl_display_listener, _>(listener_ptr),
            std::mem::transmute::<*mut DisplayListenerData, *mut std::ffi::c_void>(data_ptr)
        );
    }
}

impl Drop for wl_display {
    fn drop(&mut self) {
        unsafe { 
            println!("dropping display...");
            simply_wayland::wl_display_disconnect(self.ptr);
            self.ptr = std::ptr::null_mut();
        }
    }
}

/*impl std::ops::Deref for wl_display {
    type Target = simply_wayland::wl_display;

    fn deref(&self) -> &Self::Target {
        unsafe {
            &self.ptr.as_ref().unwrap()
        }
    }
}*/

#[allow(non_camel_case_types)]
pub struct wl_registry {
    pub ptr: *mut simply_wayland::wl_registry,
}

impl Drop for wl_registry {
    fn drop(&mut self) {
        println!("dropping registry...");
        // none yet
    }
}

impl wl_registry {
    pub fn add_listener<RegistryListenerData: Copy>(
        &mut self,
        on_global: unsafe extern "C" fn(
            data: Option<&mut RegistryListenerData>,
            wl_registry: &mut simply_wayland::wl_registry,
            name: u32,
            interface: *const std::os::raw::c_char,
            version: u32
        ),
        on_global_remove: unsafe extern "C" fn (
            data: Option<&mut RegistryListenerData>,
            wl_registry: &mut simply_wayland::wl_registry,
            name: u32
        ),
        data: Option<Box<RegistryListenerData>>
    ) {
        unsafe {
            println!("add listener called...");
            let data_ptr = match data {
                Some(data) => Box::into_raw(data),
                None => std::ptr::null(),
            };
            //println!("extracted data");

            let listener = simply_wayland::wl_registry_listener {
                global: Some(std::mem::transmute(on_global)), global_remove: Some(std::mem::transmute(on_global_remove))
            };

            //println!("created listener");

            let listener_ptr = Box::into_raw(Box::new(listener));

            //println!("changed it to raw");

            let registry_ptr: *mut simply_wayland::wl_registry = self.ptr;

            let proxy_ptr: *mut simply_wayland::wl_proxy = std::mem::transmute(registry_ptr);

            simply_wayland::wl_proxy_add_listener(
                proxy_ptr,
                std::mem::transmute(listener_ptr),
                std::mem::transmute(data_ptr)
            );

            //println!("added listener");
        }
    }
}

/*pub fn registry_handle_global(
    data: *mut std::os::raw::c_void,
    registry: *mut simply_wayland::wl_registry,
    name: u32,
    interface: *const std::os::raw::c_char,
    version: u32,
) {
    unsafe {
        //println!("got global: {}", interface.to_str().unwrap());
        println!("got global: {} with name {}", std::ffi::CStr::from_ptr(interface).to_str().unwrap(), name);
    }
}

pub fn registry_handle_global_remove(
    data: *mut std::ffi::c_void,
    registry: *mut simply_wayland::wl_registry,
    name: u32
) {
    println!("was asked to delete global {}", name);
}*/

/*pub fn bind_registry() {
    //
}*/

//pub fn sw_bind_global_listeners()

/*static simply_wayland::wl_registry_listener {
    registry_handle_global,
    registry_handle_global_remove,
}*/

/*pub fn wl_display_connect(name: Option<&str>) -> Option<Box<simply_wayland::wl_display>> {
    unsafe {
        let name = match name {
                Some(string) => std::ffi::CString::new(string).expect("Couldn't convert display name").as_ptr(),
                None => std::ptr::null(),
        };

        let ptr: *mut simply_wayland::wl_display = simply_wayland::wl_display_connect(name as *const std::os::raw::c_char);

        //Some(Box::from_raw(ptr))
        match ptr.is_null() {
            true => None,
            false => Some(Box::from_raw(ptr)),
        }
    }

}

pub fn wl_display_disconnect(display: Box<simply_wayland::wl_display>) {
    unsafe {
        simply_wayland::wl_display_disconnect(display.as_mut() as *mut simply_wayland::wl_display);
    }
}*/

// False if error occurs
/*pub fn wl_display_dispatch() -> bool {
    unsafe {
        let c_int_r: std::os::raw::c_int = simply_wayland::wl_display_dispatch(display as *mut simply_wayland::wl_display);
        let native_r: i32 = c_int_r.into();

        native_r != -1
    }
}*/

/*fn cast_proxy_generic(proxy: *mut Proxy) -> *mut simply_wayland::wl_proxy {
    unsafe {
        let as_generic: *mut simply_wayland::wl_proxy = std::mem::transmute(proxy);
        as_generic
    }
}*/
