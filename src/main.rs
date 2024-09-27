use windows::Win32::{
    Media::Audio::{
        Endpoints::IAudioMeterInformation,
        IMMDeviceEnumerator,
        MMDeviceEnumerator,
        eRender,
        eConsole
    },
    System::Com::{
        CoCreateInstance,
        CoInitialize,
        CoUninitialize,
        CLSCTX_ALL
    }
};
use std::{thread, time};

/*
TODO:
auto update device
take 64 averages 
change the defualt devices audio volume based on the average but cap the change to -5 to +5 volume levels
*/

//Many of the methods in WASAPI return error code AUDCLNT_E_DEVICE_INVALIDATED if the audio endpoint device that a client application is using becomes invalid. 
// USE THIS "FEATURE" TO DETERMINE WHEN THE USER HAS SWITCHED AUDIO DEVICES


// Before moving forward, check that the user can control valve audio devices. If the change is reflected
// in the headset then use windows api
// if the change is not reflected in the headset use open vr api to change the voulme in steam vr.

fn main() {
    unsafe{let _ = CoInitialize(None);}; // Null
    let immde: IMMDeviceEnumerator = unsafe{CoCreateInstance(&MMDeviceEnumerator, None, CLSCTX_ALL)}.unwrap();

    let dae = unsafe{immde.GetDefaultAudioEndpoint(eRender, eConsole)}.unwrap();

    // Obtains a reference to the IAudioClient interface of an audio endpoint device by calling the IMMDevice::Activate method with parameter iid set to REFIID IID_IAudioClient
    let meter_info = unsafe{dae.Activate::<IAudioMeterInformation>(CLSCTX_ALL, None)}.unwrap();

    loop {
        let peak_db: f32 = unsafe{meter_info.GetPeakValue()}.unwrap();

        dbg!(peak_db * 100 as f32);

        thread::sleep(time::Duration::from_millis(50));
    }
    

    unsafe{let _ = CoUninitialize();}; // call to free up resources
}

/*
MS REFERENCES
https://learn.microsoft.com/en-us/windows/win32/coreaudio/wasapi
https://learn.microsoft.com/en-us/windows/win32/coreaudio/volume-controls


*/