use windows::Win32::Media::Audio::Endpoints::IAudioMeterInformation;
use windows::Win32::Media::Audio::{IMMDeviceEnumerator, eRender, eConsole};
use windows::Win32::System::Com::{
    CoCreateInstance, CoInitalize, CLSCTX_ALL
};

fn main() {
    CoInitalize();
    let immde: IMMDeviceEnumerator = unsafe{CoCreateInstance(&IMMDeviceEnumerator, None, CLSCTX_ALL)}.unwrap();

    unsafe {
        let dae = immde.GetDefaultAudioEndpoint(eRender, role).unwrap();
        let meter_info = dae.Activate::<IAudioMeterInformation>(CLSCTX_ALL, None).unwrap();

    }
    println!("Hello, world!");
}