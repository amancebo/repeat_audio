use audio_samples::AudioSamples;
use audio_samples_io::{AudioIOError, read, write};

pub fn repeat_audio(min_duration_seconds: f64, input_file_name: &str) -> Result<(), AudioIOError> {
    let file_extension = input_file_name.split('.').last().unwrap();
    let file_extension = format!(".{}", file_extension);

    let audio: AudioSamples<'static, i16> = read(input_file_name)?;
    let samples = audio.slice_samples(0..audio.total_samples())?;
    let n_samples = samples.len();
    let sample_rate = audio.sample_rate();

    let duration_seconds = n_samples as f64 / sample_rate.get() as f64;
    let n_repeats = (min_duration_seconds / duration_seconds).ceil() as usize;

    if n_repeats > 1 {
        println!("To achieve a duration of at least {min_duration_seconds} seconds, '{input_file_name}' will be repeated {n_repeats} times for a final duration of {:0.4} seconds.", (n_repeats * n_samples) as f64 / sample_rate.get() as f64);

        let out_samples: Vec<i16> = samples
            .clone()
            .into_iter()
            .cycle()
            .take(n_samples * n_repeats)
            .collect();
        let out_samples = AudioSamples::new_mono_from_slice(&out_samples, sample_rate);

        let output_file_name = format!(
            "{}_{:04}-copies{}",
            input_file_name
                .strip_suffix(file_extension.as_str())
                .unwrap(),
            n_repeats,
            file_extension
        );
        write(output_file_name.clone(), &out_samples)?;
        println!("Saved to '{output_file_name}'");
    } else {
        eprintln!("{min_duration_seconds} seconds is too short; '{input_file_name}' would occur only once.\nNo output file will be created.");
    }
    Ok(())
}
