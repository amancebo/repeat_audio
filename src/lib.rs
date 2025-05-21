use wavers::{read, write, Samples};

pub fn repeat_audio(min_duration_seconds: f64, input_file_name: &str) {
    let file_extension = input_file_name.split('.').last().unwrap();
    let file_extension = format!(".{}", file_extension);

    let (samples, sample_rate): (Samples<i16>, i32) = read::<i16, _>(input_file_name).unwrap();

    let duration_seconds = samples.len() as f64 / sample_rate as f64;
    let n_repeats = (min_duration_seconds / duration_seconds).ceil() as usize;

    if n_repeats > 1 {
        println!("To achieve a duration of at least {min_duration_seconds} seconds, '{input_file_name}' will be repeated {n_repeats} times for a final duration of {:0.4} seconds.", (n_repeats * samples.len()) as f64 / sample_rate as f64);

        let _duration = n_repeats as f64 * duration_seconds;
        let out_samples: Vec<_> = samples
            .iter()
            .cycle()
            .take(samples.len() * n_repeats)
            .copied()
            .collect();
        let out_samples: Samples<i16> = Samples::from(out_samples.into_boxed_slice()).convert();

        let output_file_name = format!(
            "{}_{:04}-copies{}",
            input_file_name
                .strip_suffix(file_extension.as_str())
                .unwrap(),
            n_repeats,
            file_extension
        );
        write(output_file_name.clone(), &out_samples, sample_rate, 1).unwrap();
        println!("Saved to '{output_file_name}'");
    } else {
        eprintln!("{min_duration_seconds} seconds is too short; '{input_file_name}' would occur only once.\nNo output file will be created.")
    }
}
