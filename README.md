repeat_audio
============

Utility for repeating an a piece of audio until a minimum duration is
reached. This is useful for repeating single-cycle `.wav` files so
they can be used on samplers that don't support single-cycle waveforms
but have ample storage, such as the Roland SP-404MKII.

Example use
------------

```bash
repeat_audio 1 waveform.wav
```

will repeat `waveform.wav` until it reaches a duration of at least 1
second. The output file might then be `waveform_0087-copies.wav`,
indicating that the waveform was repeated 87 times to reach 1 second.
