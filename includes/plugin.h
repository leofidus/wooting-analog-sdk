#include <stdint.h>
#include <stdio.h>
#include <stdbool.h>
#include "wooting-analog-sdk-common-plugin.h"

#if defined(_WIN32) || defined(WIN32)
#ifdef ANALOGSDK_EXPORTS
#define ANALOGSDK_API __declspec(dllexport)
#else
#define ANALOGSDK_API __declspec(dllimport)
#endif
#else
#define ANALOGSDK_API
#endif

typedef void(*device_event)(WASDK_DeviceEventType, WASDK_DeviceInfo*);

/// Get a name describing the `Plugin`.
ANALOGSDK_API const char* _name();

/// A callback fired immediately after the plugin is loaded. Usually used
/// for initialization.
ANALOGSDK_API AnalogSDKResult initialise();

/// A function fired to check if the plugin is currently initialised
ANALOGSDK_API bool is_initialised();

/// A callback fired immediately before the plugin is unloaded. Use this if
/// you need to do any cleanup.
ANALOGSDK_API void unload();

/// Function called to get the full analog read buffer for a particular device with ID `device`. `len` is the maximum amount
/// of keys that can be accepted, any more beyond this will be ignored by the SDK.
/// If `device` is 0 then no specific device is specified and the data should be read from all devices and combined
ANALOGSDK_API int _read_full_buffer(uint16_t code_buffer[], float analog_buffer[], int len, WASDK_DeviceID device);

/// This function is fired by the SDK to collect up all Device Info structs. The memory for the struct should be retained and only dropped
/// when the device is disconnected or the plugin is unloaded. This ensures that the Device Info is not garbled when it's being accessed by the client.
///
/// # Notes
///
/// Although, the client should be copying any data they want to use for a prolonged time as there is no lifetime guarantee on the data.
ANALOGSDK_API int _device_info(WASDK_DeviceInfo* buffer[], int len);

/// Function called to get the analog value for a particular HID key `code` from the device with ID `device`.
/// If `device` is 0 then no specific device is specified and the value should be read from all devices and combined
ANALOGSDK_API float read_analog(uint16_t code, WASDK_DeviceID device);

/// Set a callback which should be fired when a device handled by the `Plugin` is connected or disconnected
ANALOGSDK_API AnalogSDKResult set_device_event_cb(device_event cb);

/// Clear the device event callback
ANALOGSDK_API AnalogSDKResult clear_device_event_cb();