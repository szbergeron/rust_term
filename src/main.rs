//#![feature(associated_type_bounds)]

//use simply_wayland;

//mod helper;
//use helper::*;
//mod types;

//mod simply_wayland;

use simply_wayland::wl::core as wl;
use simply_wayland::wl::types as wt;

use std::sync::mpsc::channel;

use num_enum::TryFromPrimitive;
//use std::convert::TryFrom;

//use std::cell::RefCell;
//use std::sync::Arc;
//use std::sync::Mutex;
//use once_cell::sync::OnceCell;

mod types;
//use types::*;
//use simply_wayland::wl::constants as wc;

//let registry: Mutex<Option<wl::Registry>> = Mutex::new(None);
//static mut global_registry: Mutex<Option<wl::Registry>> = Mutex::new(None);

/*#[derive(Default)]
struct Names {
    compositor: u32,
}*/
//static mut names: Names = Names { compositor: 0 };
/*struct Globals {
    display: Option<wt::Display>,
    registry: Option<wt::Registry>,
    compositor: Option<wt::Compositor>,
}*/

//static globals: Mutex<Globals> = Mutex::new(Globals { display: None, registry: None, compositor: None });
//static globals: wt::Globals = wt::Globals::new();
//static global_display: Arc<RefCell<Option<wt::Display>>> = Arc::new(RefCell::new(None));
//static global_compositor: Arc<RefCell<Option<wt::Compositor>>> = Arc::new(RefCell::new(None));

unsafe fn from_cstr(input: *const std::os::raw::c_char) -> String {
    String::from(std::ffi::CStr::from_ptr(input).to_str().expect("Couldn't convert c string"))
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
struct GlobalAdvertisement {
    interface: String,
    name: u32,
    version: u32,
}

#[derive(Default)]
struct Globals {
    compositor: Option<wt::Compositor>,
    shm: Option<wt::Shm>,
}

impl Globals {
    pub fn new() -> Globals {
        //Globals { compositor: None, shm: None }
        Default::default()
    }
}

//static global_queue: lockfree::queue::Queue<GlobalAdvertisement> = lockfree::queue::Queue::new();

/*macro_rules! gqueue {
    (of $t:ty, named $n:ident) => (
        fn $n() -> &'static lockfree::queue::Queue<$t> {
            static INSTANCE: OnceCell<lockfree::queue::Queue<$t>> = OnceCell::new();
            INSTANCE.get_or_init(|| {
                lockfree::queue::Queue::new()
            })
        }
    )
}*/

/*fn global_queue() -> &'static lockfree::queue::Queue<GlobalAdvertisement> {
    static INSTANCE: OnceCell<lockfree::queue::Queue<GlobalAdvertisement>> = OnceCell::new();
    INSTANCE.get_or_init(|| {
        let queue = lockfree::queue::Queue::new();

        queue
    })
}*/

//gqueue!(of GlobalAdvertisement, named global_queue);

//type FormatSpecifier = u32;
//gqueue!(of wl::wl_shm_format, named format_queue);


#[allow(dead_code, unused_variables)]
fn main() {
    let (global_send, global_rcv) = channel::<GlobalAdvertisement>();
    let (format_send, format_rcv) = channel::<wl::wl_shm_format>();

    let mut display = wt::Display::connect(None).expect("Couldn't init display!");
    
    let mut registry = display.get_registry().expect("Couldn't acquire registry, wayland server misconfiguration?");

    registry.add_listener(registry_handle_global, registry_handle_global_remove, Some(Box::new(global_send)));

    display.roundtrip();

    let mut globals = Globals::new();

    // can now unload queue entries
    for entry in global_rcv.try_iter() {
        println!("Entry: {:?}", entry);
        unsafe { 
            match entry.interface {
                s if s == from_cstr(wl::wl_compositor_interface.name) => {
                    globals.compositor = Some(registry.bind(entry.name, 4).expect("Couldn't bind for compositor"));
                },
                s if s == from_cstr(wl::wl_shm_interface.name) => {
                    globals.shm = Some(registry.bind(entry.name, 1).expect("Couldn't bind for shm"));
                },
                _ => (),
            }
        }
    }

    globals.shm.unwrap().add_listener(shm_handle_format, Some(Box::new(format_send)));

    display.roundtrip();

    for format in format_rcv.try_iter() {
        println!("entry: {:?}", format);
    }

    let surface = globals.compositor.unwrap().create_surface();
}

#[derive(Copy, Clone)]
struct ShmListenerData {
    //
}


#[derive(Copy, Clone)]
struct DisplayListenerData {
    //
}

unsafe extern "C" fn shm_handle_format(
    channel: Option<&mut std::sync::mpsc::Sender<wl::wl_shm_format>>,
    _: &mut wl::wl_shm,
    format: wl::wl_shm_format
) {
    //println!("Got format of {}", format);
    //format_queue().push(format);
    channel.unwrap().send(format).unwrap();
}

/*unsafe extern "C" fn on_display_fatal_error(
    data: &mut DisplayListenerData,
    wl_display: *mut simply_wayland::wl_display,
    object_id: *mut std::os::raw::c_void*/

#[derive(Copy, Clone)]
struct RegistryListenerData {
    //
}

#[allow(unused_variables)]
unsafe extern "C" fn registry_handle_global(
    channel: Option<&mut std::sync::mpsc::Sender<GlobalAdvertisement>>,
    wl_registry: &mut wl::wl_registry,
    name: u32,
    interface: *const std::os::raw::c_char,
    version: u32
) {
    let interface = std::ffi::CStr::from_ptr(interface).to_str().expect("Got corrupt interface name string");

    let owned_interface_str = String::from(interface);

    //global_queue().push(GlobalAdvertisement { name, version, interface: owned_interface_str });
    channel.unwrap().send(GlobalAdvertisement { name, version, interface: owned_interface_str }).unwrap();
}

#[allow(unused_variables)]
unsafe extern "C" fn registry_handle_global_remove(
    data: Option<&mut std::sync::mpsc::Sender<GlobalAdvertisement>>,
    wl_registry: &mut wl::wl_registry,
    name: u32
) {
    println!("Was told global {} will be removed", name);
}
