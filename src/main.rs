use windows::Win32::{
    Media::Audio::{
        IMMDeviceEnumerator,
        Endpoints::{
            IAudioMeterInformation,
            IAudioEndpointVolume
        },
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
*/

//Many of the methods in WASAPI return error code AUDCLNT_E_DEVICE_INVALIDATED if the audio endpoint device that a client application is using becomes invalid. 
// USE THIS "FEATURE" TO DETERMINE WHEN THE USER HAS SWITCHED AUDIO DEVICES



fn get_average(samples: &[f32]) -> f32 {
    let sum: f32 = samples.iter().sum();
    let c = samples.len() as f32;

    return sum / c
}

fn main() {
    unsafe{let _ = CoInitialize(None);}; // Null
    let immde: IMMDeviceEnumerator = unsafe{CoCreateInstance(&MMDeviceEnumerator, None, CLSCTX_ALL)}.unwrap();

    let dae = unsafe{immde.GetDefaultAudioEndpoint(eRender, eConsole)}.unwrap();

    // Obtains a reference to the IAudioClient interface of an audio endpoint device by calling the IMMDevice::Activate method with parameter iid set to REFIID IID_IAudioClient
    let peak_meter = unsafe{dae.Activate::<IAudioMeterInformation>(CLSCTX_ALL, None)}.unwrap(); // Peak meter - https://learn.microsoft.com/en-us/windows/win32/api/endpointvolume/nn-endpointvolume-iaudiometerinformation
    let volume_controls = unsafe{dae.Activate::<IAudioEndpointVolume>(CLSCTX_ALL, None)}.unwrap(); // Volume controls - https://learn.microsoft.com/en-us/windows/win32/api/endpointvolume/nn-endpointvolume-iaudioendpointvolume

    // GetVolumeRange to find the range of db that can be set
    let original_flevel: f32 = unsafe{volume_controls.GetMasterVolumeLevelScalar()}.unwrap();

    let max_flevel: f32 = original_flevel + 0.02;
    let min_flevel: f32 = original_flevel - 0.02;

    let target_dbs: f32 = 4.0; // Need to figure out how to figure out the perfect db and also calibrate it auto for new audio devices.
    let max_dbs: f32 = target_dbs + 0.50;
    let min_dbs: f32 = target_dbs - 0.50;

    let mut db_samples: [f32; 25] = [0.0; 25];
    let mut i: usize = 0;

    loop {
        let peak_db: f32 = unsafe{peak_meter.GetPeakValue()}.unwrap() * (100 as f32);
        db_samples[i] = peak_db;

        let avg_db = get_average(&db_samples);
        
        dbg!(avg_db);

        let current_flevel: f32 = unsafe{volume_controls.GetMasterVolumeLevelScalar()}.unwrap();
        let mut new_flevel: f32 = current_flevel;

        //dbg!(current_flevel);

        if avg_db > max_dbs {
            if current_flevel - 0.005 > min_flevel {
                new_flevel = current_flevel - 0.0005;
            }

        } else if avg_db < min_dbs {
            if current_flevel + 0.005 < max_flevel {
                new_flevel = current_flevel + 0.0005;
            }
        }

        if new_flevel != current_flevel {
            unsafe{volume_controls.SetMasterVolumeLevelScalar(new_flevel, std::ptr::null_mut())}.unwrap();
        }

        i += 1;

        if i >= db_samples.len() {
            i = 0;
        }


        thread::sleep(time::Duration::from_millis(5));
    }

    unsafe{let _ = CoUninitialize();}; // call to free up resources
}

/*
MS REFERENCES
https://learn.microsoft.com/en-us/windows/win32/coreaudio/wasapi
https://learn.microsoft.com/en-us/windows/win32/coreaudio/volume-controls
*/