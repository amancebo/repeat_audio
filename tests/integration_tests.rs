#[cfg(test)]
mod integration_tests {
    #[test]
    fn test_repeat_audio() {
        _ = repeat_audio::repeat_audio(4.0, "ei_sine render 001.wav");
        let reference_audio: audio_samples::AudioSamples<'static, i16> =
            audio_samples_io::read("ei_sine render 001_0345-copies_reference.wav").unwrap();
        let audio: audio_samples::AudioSamples<'static, i16> =
            audio_samples_io::read("ei_sine render 001_0345-copies.wav").unwrap();

        assert_eq!(reference_audio.len(), audio.len());
        assert_eq!(reference_audio.sample_rate.get(), audio.sample_rate.get());

        let ops =
            similar::capture_diff_slices(similar::Algorithm::Myers, reference_audio.data.as_slice().unwrap(), audio.data.as_slice().unwrap());

        let n_diffs =  ops.iter().filter(|x| x.tag() != similar::DiffTag::Equal).count();
        assert_eq!(n_diffs, 0);
    }
}
