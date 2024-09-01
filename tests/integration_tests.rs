#[cfg(test)]
mod integration_tests {
    #[test]
    fn test_repeat_audio() {
        repeat_audio::repeat_audio(4.0, "ei_sine render 001.wav");
        let (reference_samples, reference_sample_rate) =
            wavers::read::<i16, _>("ei_sine render 001_0345-copies_reference.wav").unwrap();
        let (samples, sample_rate) =
            wavers::read::<i16, _>("ei_sine render 001_0345-copies.wav").unwrap();
        // let (samples, sample_rate) =
        //     wavers::read::<i16, _>("ei_sine render 001.wav").unwrap();

        assert_eq!(reference_samples.len(), samples.len());
        assert_eq!(reference_sample_rate, sample_rate);

        let ops =
            similar::capture_diff_slices(similar::Algorithm::Myers, &reference_samples, &samples);

        let n_diffs =  ops.iter().filter(|x| x.tag() != similar::DiffTag::Equal).count();
        assert_eq!(n_diffs, 0);

        // let changes: Vec<_> = ops
        //     .iter()
        //     .flat_map(|x| x.iter_changes(reference_samples.as_ref(), samples.as_ref()))
        //     .map(|x| x.tag())
        //     .take_while(|x| *x == similar::ChangeTag::Equal)
        //     .collect();

        // assert_eq!(changes.len(), reference_samples.len());
        // assert_eq!(changes.len(), samples.len());
    }
}
