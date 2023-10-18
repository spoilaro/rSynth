use wasm_bindgen::prelude::*;

use js_sys::Float32Array;

#[wasm_bindgen]
//pub fn process_audio(sample_rate: i32, input_channel: &Float32Array, output_channel: &Float32Array) -> Float32Array {
pub fn process_audio(sample_rate: i32, input_channel: &Float32Array, output_channel: &Float32Array) -> u8 {


    // // Seconds 
    // let reverb_duration: i32 = 3;
    // let reverb_length = sample_rate * reverb_duration;
    // let mut reverb_buffer: Vec<i32> = (0..reverb_length).map(|v| v).collect();
    //
    // let dry: f32 = 0.5;
    // let wet: f32 = 0.2;
    // let room_size: f32 = 0.5;
    //
    // let delay_time = room_size * 2.0;
    //
    // let delay_samples = delay_time as i32 * sample_rate;
    //
    // let mut write_index = 0; 
    // write_index = (write_index + input_channel.length() as i32) % reverb_buffer.len() as i32;
    //
    // let mut read_index = 0;
    // read_index = (write_index + reverb_buffer.len() as i32 - delay_samples) % reverb_buffer.len() as i32;
    //
    // for i in 0..input_channel.length() {
    //     let reverb_sample = input_channel.get_index(read_index as u32);
    //     output_channel.set_index(i, input_channel.get_index(i) * dry + reverb_sample * wet);
    //
    //     reverb_buffer[write_index as usize] = (input_channel.get_index(i) * dry + reverb_sample * wet) as i32;
    //
    //     read_index = (read_index + 1) % reverb_buffer.len() as i32;
    //     write_index = (write_index + 1) % reverb_buffer.len() as i32;
    // }

    // return input_channel.clone();
    return input_channel.length().try_into().unwrap();
}
