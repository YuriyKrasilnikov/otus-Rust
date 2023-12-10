use core::ffi::{c_char, c_void};
use core::mem;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::ffi::{CStr, CString};
use std::sync::Arc;
use std::sync::RwLock;

use libloading::{Library, Symbol};

pub static SMARTHOUSESTORE: Lazy<RwLock<HashMap<String, SmartHouseLib>>> =
    Lazy::new(|| RwLock::new(HashMap::new()));

enum LibraryFunction {
    New(Box<dyn Fn(*const c_char) -> *mut c_void>),
    Destroy(Box<dyn Fn(*mut c_void)>),
    AddRoom(Box<dyn Fn(*mut c_void, *const c_char)>),
    RemoveRoom(Box<dyn Fn(*mut c_void, *const c_char)>),
    GetListRoomsName(Box<dyn Fn(*mut c_void, *mut *mut c_char, *mut usize)>),
    FreeStringList(Box<dyn Fn(*mut c_char, usize)>),

    RemoveDevice(Box<dyn Fn(*mut c_void, *const c_char, *const c_char)>),
    GetListDevicesName(Box<dyn Fn(*mut c_void, *const c_char, *mut *mut c_char, *mut usize)>),
    Report(Box<dyn Fn(*mut c_void) -> *const c_char>),
    AddTestDeviceOutlet(Box<dyn Fn(*mut c_void, *const c_char, *const c_char, *const c_char)>),
}

#[allow(dead_code)]
pub struct SmartHouseLib {
    lib: *mut c_void,
    commands: HashMap<String, LibraryFunction>,
    library: Arc<Library>,
}

unsafe impl Send for SmartHouseLib {}
unsafe impl Sync for SmartHouseLib {}

fn str2c_char(text: &str) -> *const c_char {
    let c_str = CString::new(text).expect("CString::new failed");
    let ptr = c_str.as_ptr() as *const c_char;
    mem::forget(c_str);
    ptr
}

fn c_char2str(text: *const c_char) -> String {
    unsafe { CString::from_raw(text as *mut c_char) }
        .to_str()
        .expect("Failed to convert CString to &str")
        .to_string()
}

fn readbuf(buffer: *mut i8, size: usize) -> Vec<String> {
    let string_ptrs = unsafe { std::slice::from_raw_parts(buffer as *mut *mut c_char, size) };
    string_ptrs
        .iter()
        .map(|&s| unsafe {
            CStr::from_ptr(s as *const c_char)
                .to_str()
                .unwrap()
                .to_owned()
        })
        .collect()
}

impl SmartHouseLib {
    pub fn new(library_path: String, name: String) -> Self {
        let mut commands = HashMap::new();

        let library =
            Arc::new(unsafe { Library::new(library_path).expect("Failed to load library") });
        // New
        {
            let library_clone = Arc::clone(&library);
            commands.insert("new".to_string(), LibraryFunction::New(Box::new(move |n| {
          let new_func: Symbol<unsafe extern "C" fn(*const c_char) -> *mut c_void> = unsafe {
              library_clone.get(b"new\0").expect("Failed to load create_struct function")
          };
          unsafe { new_func(n) }
      })));
        }
        // Destroy
        {
            let library_clone = Arc::clone(&library);
            commands.insert(
                "destroy".to_string(),
                LibraryFunction::Destroy(Box::new(move |v| {
                    let destroy: Symbol<unsafe extern "C" fn(*mut c_void)> = unsafe {
                        library_clone
                            .get(b"destroy\0")
                            .expect("Failed to load create_struct function")
                    };
                    unsafe { destroy(v) }
                })),
            );
        }
        // AddRoom
        {
            let library_clone: Arc<Library> = Arc::clone(&library);
            commands.insert("add_room".to_string(), LibraryFunction::AddRoom(Box::new(move |v, n| {
        let add_room: Symbol<unsafe extern "C" fn(*mut c_void, *const c_char)> = unsafe {
          library_clone.get(b"add_room\0").expect("Failed to load create_struct function")
        };
        unsafe { add_room(v, n) }
      })));
        }
        // RemoveRoom
        {
            let library_clone = Arc::clone(&library);
            commands.insert("remove_room".to_string(), LibraryFunction::RemoveRoom(Box::new(move |v, n| {
        let remove_room: Symbol<unsafe extern "C" fn(*mut c_void, *const c_char)> = unsafe {
          library_clone.get(b"remove_room\0").expect("Failed to load create_struct function")
        };
        unsafe { remove_room(v, n) }
      })));
        }
        // GetListRoomsName
        {
            let library_clone = Arc::clone(&library);
            commands.insert(
                "get_list_rooms_name".to_string(),
                LibraryFunction::GetListRoomsName(Box::new(move |v, b, s| {
                    let get_list_rooms_name: Symbol<
                        unsafe extern "C" fn(*mut c_void, *mut *mut c_char, *mut usize),
                    > = unsafe {
                        library_clone
                            .get(b"get_list_rooms_name\0")
                            .expect("Failed to load create_struct function")
                    };
                    unsafe { get_list_rooms_name(v, b, s) }
                })),
            );
        }
        // FreeStringList
        {
            let library_clone = Arc::clone(&library);
            commands.insert("free_string_list".to_string(), LibraryFunction::FreeStringList(Box::new(move |b,s| {
        let free_string_list: Symbol<unsafe extern "C" fn(*mut c_char, usize)> = unsafe {
          library_clone.get(b"free_string_list\0").expect("Failed to load create_struct function")
        };
        unsafe { free_string_list(b, s) }
      })));
        }
        // RemoveDevice(Box<dyn Fn(*mut c_void, *const c_char, *const c_char)>),
        {
            let library_clone = Arc::clone(&library);
            commands.insert(
                "remove_device".to_string(),
                LibraryFunction::RemoveDevice(Box::new(move |v, r, n| {
                    let remove_device: Symbol<
                        unsafe extern "C" fn(*mut c_void, *const c_char, *const c_char),
                    > = unsafe {
                        library_clone
                            .get(b"remove_device\0")
                            .expect("Failed to load create_struct function")
                    };
                    unsafe { remove_device(v, r, n) }
                })),
            );
        }
        //GetListDevicesName(Box<dyn Fn(*mut c_void, *const c_char, *mut *mut c_char, *mut usize)>),
        {
            let library_clone = Arc::clone(&library);
            commands.insert(
                "get_list_devices_name".to_string(),
                LibraryFunction::GetListDevicesName(Box::new(move |v, r, b, s| {
                    let get_list_devices_name: Symbol<
                        unsafe extern "C" fn(
                            *mut c_void,
                            *const c_char,
                            *mut *mut c_char,
                            *mut usize,
                        ),
                    > = unsafe {
                        library_clone
                            .get(b"get_list_devices_name\0")
                            .expect("Failed to load create_struct function")
                    };
                    unsafe { get_list_devices_name(v, r, b, s) }
                })),
            );
        }
        //Report(Box<dyn Fn(*mut c_void) -> *const c_char>),
        {
            let library_clone = Arc::clone(&library);
            commands.insert("report".to_string(), LibraryFunction::Report(Box::new(move |v| {
          let report: Symbol<unsafe extern "C" fn(*mut c_void) -> *const c_char> = unsafe {
              library_clone.get(b"report\0").expect("Failed to load create_struct function")
          };
          unsafe { report(v) }
      })));
        }
        //AddTestDeviceOutlet(Box<dyn Fn(*mut c_void, *const c_char, *const c_char, *const c_char)>)
        {
            let library_clone: Arc<Library> = Arc::clone(&library);
            commands.insert(
                "add_test_device_outlet".to_string(),
                LibraryFunction::AddTestDeviceOutlet(Box::new(move |v, r, n, d| {
                    let add_test_device_outlet: Symbol<
                        unsafe extern "C" fn(
                            *mut c_void,
                            *const c_char,
                            *const c_char,
                            *const c_char,
                        ),
                    > = unsafe {
                        library_clone
                            .get(b"add_test_device_outlet\0")
                            .expect("Failed to load create_struct function")
                    };
                    unsafe { add_test_device_outlet(v, r, n, d) }
                })),
            );
        }

        let lib = {
            let new_func = match commands.get("new").unwrap() {
                LibraryFunction::New(f) => f,
                _ => panic!("Invalid function type"),
            };

            new_func(str2c_char(name.as_str()))
        };

        SmartHouseLib {
            lib,
            commands,
            library,
        }
    }

    pub fn add_room(&self, name: String) {
        let add_room_func = match self.commands.get("add_room").unwrap() {
            LibraryFunction::AddRoom(f) => f,
            _ => panic!("Invalid function type"),
        };

        add_room_func(self.lib, str2c_char(name.as_str()));
    }

    pub fn remove_room(&self, name: String) {
        let remove_room_func = match self.commands.get("remove_room").unwrap() {
            LibraryFunction::RemoveRoom(f) => f,
            _ => panic!("Invalid function type"),
        };

        remove_room_func(self.lib, str2c_char(name.as_str()));
    }

    pub fn get_list_rooms_name(&self) -> Vec<String> {
        let mut buffer: *mut c_char = std::ptr::null_mut();
        let mut size: usize = 0;

        let get_list_rooms_name_func = match self.commands.get("get_list_rooms_name").unwrap() {
            LibraryFunction::GetListRoomsName(f) => f,
            _ => panic!("Invalid function type"),
        };
        let free_string_list_func = match self.commands.get("free_string_list").unwrap() {
            LibraryFunction::FreeStringList(f) => f,
            _ => panic!("Invalid function type"),
        };

        get_list_rooms_name_func(self.lib, &mut buffer, &mut size);

        let vec_str = readbuf(buffer, size);

        free_string_list_func(buffer, size);

        vec_str
    }

    pub fn remove_device(&self, room: String, name: String) {
        let remove_device_func = match self.commands.get("remove_room").unwrap() {
            LibraryFunction::RemoveDevice(f) => f,
            _ => panic!("Invalid function type"),
        };

        remove_device_func(
            self.lib,
            str2c_char(room.as_str()),
            str2c_char(name.as_str()),
        );
    }

    pub fn get_list_devices_name(&self, room: String) -> Vec<String> {
        let mut buffer: *mut c_char = std::ptr::null_mut();
        let mut size: usize = 0;

        let get_list_devices_name_func = match self.commands.get("get_list_devices_name").unwrap() {
            LibraryFunction::GetListDevicesName(f) => f,
            _ => panic!("Invalid function type"),
        };
        let free_string_list_func = match self.commands.get("free_string_list").unwrap() {
            LibraryFunction::FreeStringList(f) => f,
            _ => panic!("Invalid function type"),
        };

        get_list_devices_name_func(self.lib, str2c_char(room.as_str()), &mut buffer, &mut size);

        let vec_str = readbuf(buffer, size);

        free_string_list_func(buffer, size);

        vec_str
    }

    pub fn report(&self) -> String {
        let report_func = match self.commands.get("report").unwrap() {
            LibraryFunction::Report(f) => f,
            _ => panic!("Invalid function type"),
        };

        c_char2str(report_func(self.lib))
    }

    pub fn add_test_device_outlet(&self, room: String, name: String, description: String) {
        let add_test_device_outlet_func = match self.commands.get("add_test_device_outlet").unwrap()
        {
            LibraryFunction::AddTestDeviceOutlet(f) => f,
            _ => panic!("Invalid function type"),
        };

        add_test_device_outlet_func(
            self.lib,
            str2c_char(room.as_str()),
            str2c_char(name.as_str()),
            str2c_char(description.as_str()),
        );
    }
}

impl Drop for SmartHouseLib {
    fn drop(&mut self) {
        let destroy_func = match self.commands.get("destroy").unwrap() {
            LibraryFunction::Destroy(f) => f,
            _ => panic!("Invalid function type"),
        };
        destroy_func(self.lib);
    }
}

#[cfg(test)]
mod tests {
    #[warn(unused_imports)]
    use super::*;

    #[test]
    fn test_add_rooms() {
        let library_path = "libs/libsmart_house.so";

        let my_struct = SmartHouseLib::new(library_path.to_string(), "тестовая".to_string());

        my_struct.add_room("комната 1".to_string());

        my_struct.add_room("комната 2".to_string());

        assert_eq!(
            my_struct.get_list_rooms_name().sort(),
            vec!["комната 1".to_string(), "комната 2".to_string()].sort()
        );

        my_struct.remove_room("комната 1".to_string());

        assert_eq!(
            my_struct.get_list_rooms_name().sort(),
            vec!["комната 2".to_string()].sort()
        );
    }

    #[test]
    fn test_add_devices() {
        let library_path = "libs/libsmart_house.so";

        let my_struct = SmartHouseLib::new(library_path.to_string(), "тестовая".to_string());

        let room_name = "комната 1".to_string();

        my_struct.add_room(room_name.clone());

        my_struct.add_test_device_outlet(
            room_name.clone(),
            "устройство 1".to_string(),
            "тестовое устройство 1".to_string(),
        );

        my_struct.add_test_device_outlet(
            room_name.clone(),
            "устройство 2".to_string(),
            "тестовое устройство 2".to_string(),
        );

        let mut test = my_struct.get_list_devices_name(room_name.clone());

        assert_eq!(
            test.sort(),
            vec!["устройство 1".to_string(), "устройство 2".to_string()].sort()
        );
    }

    #[test]
    fn test_report() {
        let library_path = "libs/libsmart_house.so";

        let my_struct = SmartHouseLib::new(library_path.to_string(), "тестовая".to_string());

        let room_name = "комната 1".to_string();

        my_struct.add_room(room_name.clone());

        my_struct.add_test_device_outlet(
            room_name.clone(),
            "устройство 1".to_string(),
            "тестовое устройство 1".to_string(),
        );

        let test = my_struct.report();

        assert_eq!(test, "Name: тестовая,\nRooms:\n[\n{\nName: комната 1,\nDevices:\n[\n{\nName: устройство 1,\nOn: false,\nDescription: тестовое устройство 1,\nPower: 0\n},\n]\n},\n]".to_string());
    }
}
