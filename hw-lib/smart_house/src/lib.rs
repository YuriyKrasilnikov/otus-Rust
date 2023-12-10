pub mod device;
pub mod house;

use std::sync::RwLock;
use tonic::codegen::Arc;

use core::mem;
use std::ffi::CString;
use std::os::raw::c_char;

use crate::device::outlet::SmartOutlet;
use crate::device::Device;
use crate::device::RwLockDevice;
use crate::house::SmartHouse;

#[repr(C)]
pub struct SmartHouseLib {
    smarthouse: SmartHouse,
}

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

fn save2buf(c_strings: Vec<*mut c_char>, out_buffer: *mut *mut c_char, out_size: *mut usize) {
    let size = c_strings.len();

    // Выделяем память для массива строк
    let buffer = c_strings.into_boxed_slice();
    let raw_buffer = Box::into_raw(buffer);

    // Записываем данные в выходные параметры
    unsafe {
        *out_buffer = raw_buffer as *mut c_char;
        *out_size = size;
    }
}

fn get_smart_house<'a>(lib: *mut SmartHouseLib) -> &'a mut SmartHouse {
    unsafe { &mut (*lib).smarthouse }
}

#[no_mangle]
pub extern "C" fn new(name: *const c_char) -> *mut SmartHouseLib {
    let name_str = c_char2str(name);
    let smarthouse = SmartHouse::new(name_str);
    let smarthouselib = Box::new(SmartHouseLib { smarthouse });
    Box::into_raw(smarthouselib)
}

#[no_mangle]
pub unsafe extern "C" fn destroy(smarthouse: *mut SmartHouseLib) {
    if !smarthouse.is_null() {
        let _ = unsafe { Box::from_raw(smarthouse) };
    }
}

#[no_mangle]
pub extern "C" fn add_room(smarthouselib: *mut SmartHouseLib, name: *const c_char) {
    let smarthouse = get_smart_house(smarthouselib);
    let name_str = c_char2str(name);
    let _ = smarthouse.add_room(name_str);
}

#[no_mangle]
pub extern "C" fn remove_room(smarthouselib: *mut SmartHouseLib, name: *const c_char) {
    let smarthouse = get_smart_house(smarthouselib);
    let name_str = c_char2str(name);
    let _ = smarthouse.remove_room(name_str);
}

#[no_mangle]
pub extern "C" fn get_list_rooms_name(
    smarthouselib: *mut SmartHouseLib,
    out_buffer: *mut *mut c_char,
    out_size: *mut usize,
) {
    let smarthouse = get_smart_house(smarthouselib);
    let list_room = smarthouse.get(None);

    // Преобразуем строки Rust в строки C и выделяем память для них в куче
    let c_strings: Vec<*mut c_char> = list_room
        .iter()
        .map(|s| CString::new(s.name()).unwrap().into_raw())
        .collect();

    save2buf(c_strings, out_buffer, out_size);
}

#[no_mangle]
pub extern "C" fn free_string_list(buffer: *mut c_char, count: usize) {
    unsafe {
        // Приводим указатель к типу *mut *mut c_char
        let cstring_ptrs = buffer as *mut *mut c_char;

        // Преобразуем указатель на массив указателей на строки в слайс
        let cstring_slice = std::slice::from_raw_parts_mut(cstring_ptrs, count);

        // Пройдемся по каждому указателю и преобразуем его обратно в CString
        for &mut cstring in cstring_slice {
            // Преобразуем каждый сырой указатель обратно в CString, автоматически освобождая память
            let _ = CString::from_raw(cstring);
        }

        // Освобождаем память, выделенную под массив указателей
        let _ = Box::from_raw(cstring_ptrs);
    }
}

#[no_mangle]
pub extern "C" fn remove_device(
    smarthouselib: *mut SmartHouseLib,
    room_name: *const c_char,
    device_name: *const c_char,
) {
    let smarthouse = get_smart_house(smarthouselib);
    let room_name_str = c_char2str(room_name);
    let device_name_str = c_char2str(device_name);
    let _ = smarthouse.remove_device(room_name_str, device_name_str);
}

#[no_mangle]
pub extern "C" fn get_list_devices_name(
    smarthouselib: *mut SmartHouseLib,
    room_name: *const c_char,
    out_buffer: *mut *mut c_char,
    out_size: *mut usize,
) {
    let smarthouse = get_smart_house(smarthouselib);
    let room_name_str = c_char2str(room_name);
    let devices = smarthouse.devices(room_name_str).unwrap();
    // Преобразуем строки Rust в строки C и выделяем память для них в куче
    let c_strings: Vec<*mut c_char> = devices
        .iter()
        .map(|s| CString::new(s.as_str()).unwrap().into_raw())
        .collect();

    save2buf(c_strings, out_buffer, out_size);
}

#[no_mangle]
pub extern "C" fn report(smarthouselib: *mut SmartHouseLib) -> *const c_char {
    let smarthouse = get_smart_house(smarthouselib);
    str2c_char(smarthouse.report(None).as_str())
}

//todo: В будущем подумать как сделать универсальную функцию добавления
#[no_mangle]
pub extern "C" fn add_test_device_outlet(
    smarthouselib: *mut SmartHouseLib,
    room_name: *const c_char,
    device_name: *const c_char,
    device_description: *const c_char,
) {
    let smarthouse = get_smart_house(smarthouselib);
    let room_name_str = c_char2str(room_name);
    let device_name_str = c_char2str(device_name);
    let device_description_str = c_char2str(device_description);
    let device = RwLockDevice::new(Arc::new(RwLock::new(Device::new(
        device_name_str,
        Arc::new(RwLock::new(SmartOutlet::new(device_description_str, None))),
        None,
    ))));
    let _ = smarthouse.add_device(room_name_str, device);
}

#[cfg(test)]
mod tests {
    #[warn(unused_imports)]
    use super::*;
    // use core::ffi::c_void;

    use std::ffi::CStr;

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

    fn get_list_rooms_name_vec(lib: *mut SmartHouseLib) -> Vec<String> {
        let mut buffer: *mut c_char = std::ptr::null_mut();
        let mut size: usize = 0;

        get_list_rooms_name(lib, &mut buffer, &mut size);

        let vec_str = readbuf(buffer, size);

        free_string_list(buffer, size);

        vec_str
    }

    fn get_list_devices_name_vec(lib: *mut SmartHouseLib, room_name: &str) -> Vec<String> {
        let mut buffer: *mut c_char = std::ptr::null_mut();
        let mut size: usize = 0;

        get_list_devices_name(lib, str2c_char(room_name), &mut buffer, &mut size);

        let vec_str = readbuf(buffer, size);

        free_string_list(buffer, size);

        vec_str
    }

    #[test]
    fn test_add_rooms() {
        let my_struct = new(str2c_char("тестовая"));

        add_room(my_struct, str2c_char("комната 1"));

        add_room(my_struct, str2c_char("комната 2"));

        assert_eq!(
            get_list_rooms_name_vec(my_struct).sort(),
            vec!["комната 1".to_string(), "комната 2".to_string()].sort()
        );

        remove_room(my_struct, str2c_char("комната 1"));

        assert_eq!(
            get_list_rooms_name_vec(my_struct).sort(),
            vec!["комната 2".to_string()].sort()
        );

        unsafe { destroy(my_struct) };
    }

    #[test]
    fn test_add_devices() {
        let my_struct = new(str2c_char("тестовая"));
        let room_name = "комната 1";

        add_room(my_struct, str2c_char(room_name));

        add_test_device_outlet(
            my_struct,
            str2c_char(room_name),
            str2c_char("устройство 1"),
            str2c_char("тестовое устройство 1"),
        );

        add_test_device_outlet(
            my_struct,
            str2c_char(room_name),
            str2c_char("устройство 2"),
            str2c_char("тестовое устройство 2"),
        );

        let mut test = get_list_devices_name_vec(my_struct, room_name);

        assert_eq!(
            test.sort(),
            vec!["устройство 1".to_string(), "устройство 2".to_string()].sort()
        );

        remove_room(my_struct, str2c_char(room_name));

        unsafe { destroy(my_struct) };
    }

    #[test]
    fn test_report() {
        let my_struct = new(str2c_char("тестовая"));
        let room_name = "комната 1";

        add_room(my_struct, str2c_char(room_name));

        add_test_device_outlet(
            my_struct,
            str2c_char(room_name),
            str2c_char("устройство 1"),
            str2c_char("тестовое устройство 1"),
        );

        let test = c_char2str(report(my_struct));

        assert_eq!(test, "Name: тестовая,\nRooms:\n[\n{\nName: комната 1,\nDevices:\n[\n{\nName: устройство 1,\nOn: false,\nDescription: тестовое устройство 1,\nPower: 0\n},\n]\n},\n]".to_string());

        unsafe { destroy(my_struct) };
    }
}
