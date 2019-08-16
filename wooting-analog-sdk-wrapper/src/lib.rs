#[macro_use]
extern crate lazy_static;
extern crate wooting_analog_sdk_common;
use libloading as libl;
//use std::ffi::c_void;
use std::ops::Deref;
use std::os::raw::{c_float, c_int, c_uint, c_ushort};
//use std::ptr;
pub use wooting_analog_sdk_common::{
    AnalogSDKResult, DeviceEventType, DeviceID, DeviceInfo, KeycodeType,
};

/*pub struct Void(*mut c_void);

impl Default for Void {
    fn default() -> Self {
        Void(ptr::null_mut())
    }
}
*/

macro_rules! dynamic_extern {
    (@as_item $i:item) => {$i};

    (
        #[link=$lib:tt]
        extern $cconv:tt {
            $(
                $(#[$attr:meta])*
                fn $fn_names:ident($($fn_arg_names:ident: $fn_arg_tys:ty),*) $(-> $fn_ret_tys:ty)*;
            )*
        }
    ) => {
        lazy_static! {
            static ref LIB : Option<libl::Library> = {
                #[cfg(all(unix, not(target_os = "macos")))]
                let lib_path = concat!("lib", $lib, ".so");
                #[cfg(all(unix, target_os = "macos"))]
                let lib_path = concat!("lib", $lib, ".dylib");
                #[cfg(windows)]
                let lib_path = $lib;

                //Attempt to load the library, if it fails print the error and discard the error
                libl::Library::new(lib_path).map_err(|e| {
                    println!("Unable to load library: {}\nErr: {}", lib_path, e);
                }).ok()
            };
        }
        $(
            dynamic_extern! {
                @as_item
                $(#[$attr])*
                #[no_mangle]
                pub unsafe extern fn $fn_names($($fn_arg_names: $fn_arg_tys),*) $(-> $fn_ret_tys)* {
                    type FnPtr = unsafe extern $cconv fn($($fn_arg_tys),*) $(-> $fn_ret_tys)*;

                    lazy_static! {
                        static ref FUNC: Option<libl::Symbol<'static, FnPtr>> = {
                            LIB.as_ref().and_then(|lib| unsafe {
                                //Get func, print and discard error as we don't need it again
                                lib.get(stringify!($fn_names).as_bytes()).map_err(|e| {
                                    println!("{}", e);
                                }).ok()
                            })
                        };
                    }
                    match FUNC.deref() {
                        Some(f) => f($($fn_arg_names),*),
                        _ => Default::default()
                    }
                }
            }
        )*
    };
}

dynamic_extern! {
    #[link="wooting_analog_sdk"]
    extern "C" {
        /// Initialises the Analog SDK, this needs to be successfully called before any other functions
        /// of the SDK can be called
        /// 
        /// # Notes
        /// The SDK will use the semi-colon separated list of directories in the environment variable `WOOTING_ANALOG_SDK_PLUGINS_PATH` to search for Plugins.
        /// Plugins must have the extension `.dll` on Windows, `.so` on Linux and `.dylib` on MacOS
        /// 
        /// # Expected Returns
        /// * `Ok`: Meaning the SDK initialised successfully (currently also means that there is at least one plugin initialised with at least one device connected)
        /// * `NoPlugins`: Meaning that either no plugins were found or some were found but none were successfully initialised
        /// 
        fn wasdk_initialise() -> AnalogSDKResult;
        
        /// Returns a bool indicating if the Analog SDK has been initialised
        fn wasdk_is_initialised() -> bool;
        
        /// Uninitialises the SDK, returning it to an empty state, similar to how it would be before first initialisation
        /// # Expected Returns
        /// * `Ok`: Indicates that the SDK was successfully uninitialised
        fn wasdk_uninitialise() -> AnalogSDKResult;
        
        /// Sets the type of Keycodes the Analog SDK will receive (in `read_analog`) and output (in `read_full_buffer`).
        /// 
        /// By default, the mode is set to HID
        /// 
        /// # Notes
        /// * `VirtualKey` and `VirtualKeyTranslate` are only available on Windows
        /// * With all modes except `VirtualKeyTranslate`, the key identifier will point to the physical key on the standard layout. i.e. if you ask for the Q key, it will be the key right to tab regardless of the layout you have selected
        /// * With `VirtualKeyTranslate`, if you request Q, it will be the key that inputs Q on the current layout, not the key that is Q on the standard layout. 
        /// 
        /// # Expected Returns
        /// * `Ok`: The Keycode mode was changed successfully
        /// * `InvalidArgument`: The given `KeycodeType` is not one supported by the SDK
        /// * `NotAvailable`: The given `KeycodeType` is present, but not supported on the current platform
        /// * `UnInitialized`: The SDK is not initialised
        fn wasdk_set_keycode_mode(mode: KeycodeType) -> AnalogSDKResult;
        
        /// Reads the Analog value of the key with identifier `code` from any connected device. The set of key identifiers that is used
        /// depends on the Keycode mode set using `wasdk_set_mode`.
        /// 
        /// # Examples
        /// ```ignore
        /// wasdk_set_mode(KeycodeType::ScanCode1);
        /// wasdk_read_analog(0x10); //This will get you the value for the key which is Q in the standard US layout (The key just right to tab)
        /// 
        /// wasdk_set_mode(KeycodeType::VirtualKey); //This will only work on Windows
        /// wasdk_read_analog(0x51); //This will get you the value for the key that is Q on the standard layout
        /// 
        /// wasdk_set_mode(KeycodeType::VirtualKeyTranslate);
        /// wasdk_read_analog(0x51); //This will get you the value for the key that inputs Q on the current layout
        /// ```
        /// 
        /// # Expected Returns
        /// The float return value can be either a 0->1 analog value, or (if <0) is part of the AnalogSDKResult enum, which is how errors are given back on this call.
        /// So if the value is below 0, you should cast it as AnalogSDKResult to see what the error is.
        /// * `0.0f - 1.0f`: The Analog value of the key with the given id `code`
        /// * `AnalogSDKResult::NoMapping`: No keycode mapping was found from the selected mode (set by wasdk_set_mode) and HID.
        /// * `AnalogSDKResult::UnInitialized`: The SDK is not initialised
        /// * `AnalogSDKResult::NoDevices`: There are no connected devices
        fn wasdk_read_analog(code: c_ushort) -> f32;
        
        /// Reads the Analog value of the key with identifier `code` from the device with id `device_id`. The set of key identifiers that is used
        /// depends on the Keycode mode set using `wasdk_set_mode`.
        /// 
        /// The `device_id` can be found through calling `wasdk_device_info` and getting the DeviceID from one of the DeviceInfo structs
        /// 
        /// # Expected Returns
        /// The float return value can be either a 0->1 analog value, or (if <0) is part of the AnalogSDKResult enum, which is how errors are given back on this call.
        /// So if the value is below 0, you should cast it as AnalogSDKResult to see what the error is.
        /// * `0.0f - 1.0f`: The Analog value of the key with the given id `code` from device with id `device_id`
        /// * `AnalogSDKResult::NoMapping`: No keycode mapping was found from the selected mode (set by wasdk_set_mode) and HID.
        /// * `AnalogSDKResult::UnInitialized`: The SDK is not initialised
        /// * `AnalogSDKResult::NoDevices`: There are no connected devices with id `device_id`
        fn wasdk_read_analog_device(code: c_ushort, device_id: DeviceID) -> f32;
        
        /// Set the callback which is called when there is a DeviceEvent. Currently these events can either be Disconnected or Connected(Currently not properly implemented).
        /// The callback gets given the type of event `DeviceEventType` and a pointer to the DeviceInfo struct that the event applies to
        /// 
        /// # Notes
        /// There's no guarentee to the lifetime of the DeviceInfo pointer given during the callback, if it's a Disconnected event, it's likely the memory
        /// will be freed immediately after the callback, so it's best to copy any data you wish to use.
        /// 
        /// # Expected Returns
        /// * `Ok`: The callback was set successfully
        /// * `UnInitialized`: The SDK is not initialised
        fn wasdk_set_device_event_cb(cb: extern fn(DeviceEventType, *mut DeviceInfo)) -> AnalogSDKResult;

        /// Clears the device event callback that has been set
        /// 
        /// # Expected Returns
        /// * `Ok`: The callback was cleared successfully
        /// * `UnInitialized`: The SDK is not initialised
        fn wasdk_clear_device_event_cb() -> AnalogSDKResult;

        /// Fills up the given `buffer`(that has length `len`) with pointers to the DeviceInfo structs for all connected devices (as many that can fit in the buffer)
        /// 
        /// # Notes
        /// There is no guarenteed lifetime of the DeviceInfo structs given back, so if you wish to use any data from them, please copy it.
        /// 
        /// # Expected Returns
        /// Similar to wasdk_read_analog, the errors and returns are encoded into one type. Values >=0 indicate the number of items filled into the buffer, with `<0` being of type AnalogSDKResult
        /// * `ret>=0`: The number of connected devices that have been filled into the buffer
        /// * `AnalogSDKResult::UnInitialized`: Indicates that the AnalogSDK hasn't been initialised
        fn wasdk_get_connected_devices_info(buffer: *mut *mut DeviceInfo, len: c_uint) -> c_int;
        
        /// Reads all the analog values for pressed keys for all devices and combines their values, filling up `code_buffer` with the
        /// keycode identifying the pressed key and fills up `analog_buffer` with the corresponding float analog values. i.e. The analog
        /// value for they key at index 0 of code_buffer, is at index 0 of analog_buffer.
        /// 
        /// # Notes
        /// `len` is the length of code_buffer & analog_buffer, if the buffers are of unequal length, then pass the lower of the two, as it is the max amount of
        /// key & analog value pairs that can be filled in.
        /// The codes that are filled into the `code_buffer` are of the KeycodeType set with wasdk_set_mode
        /// If two devices have the same key pressed, the greater value will be given
        /// 
        /// # Expected Returns
        /// Similar to other functions like `wasdk_device_info`, the return value encodes both errors and the return value we want.
        /// Where >=0 is the actual return, and <0 should be cast as AnalogSDKResult to find the error.
        /// * `>=0` means the value indicates how many keys & analog values have been read into the buffers
        /// * `AnalogSDKResult::UnInitialized`: Indicates that the AnalogSDK hasn't been initialised
        /// * `AnalogSDKResult::NoDevices`: Indicates no devices are connected
        fn wasdk_read_full_buffer(code_buffer: *mut c_ushort, analog_buffer: *mut c_float, len: c_uint) -> c_int;
        
        /// Reads all the analog values for pressed keys for the device with id `device_id`, filling up `code_buffer` with the
        /// keycode identifying the pressed key and fills up `analog_buffer` with the corresponding float analog values. i.e. The analog
        /// value for they key at index 0 of code_buffer, is at index 0 of analog_buffer.
        /// 
        /// # Notes
        /// `len` is the length of code_buffer & analog_buffer, if the buffers are of unequal length, then pass the lower of the two, as it is the max amount of
        /// key & analog value pairs that can be filled in.
        /// The codes that are filled into the `code_buffer` are of the KeycodeType set with wasdk_set_mode
        /// 
        /// # Expected Returns
        /// Similar to other functions like `wasdk_device_info`, the return value encodes both errors and the return value we want.
        /// Where >=0 is the actual return, and <0 should be cast as AnalogSDKResult to find the error.
        /// * `>=0` means the value indicates how many keys & analog values have been read into the buffers
        /// * `AnalogSDKResult::UnInitialized`: Indicates that the AnalogSDK hasn't been initialised
        /// * `AnalogSDKResult::NoDevices`: Indicates the device with id `device_id` is not connected
        fn wasdk_read_full_buffer_device(code_buffer: *mut c_ushort, analog_buffer: *mut c_float, len: c_uint, device_id: DeviceID) -> c_int;
    }
}

/*fn main() {
    unsafe { println!("We got {}", test_function(16)); };
}*/
