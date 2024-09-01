import wave

from argparse import ArgumentParser
from math import ceil
from pathlib import Path


if __name__ == '__main__':
    parser = ArgumentParser()
    parser.add_argument('min-duration')
    parser.add_argument('file')
    args = parser.parse_args()
    min_duration_seconds = float(getattr(args, 'min-duration'))
    file = Path(getattr(args, 'file'))

    with wave.open(file.as_posix(), 'rb') as input_audio_file:
        nchannels, sampwidth, framerate, nframes, comptype, compname = (
            input_audio_file.getparams()
        )
        frames = input_audio_file.readframes(nframes)

    duration_seconds = nframes / framerate
    n_copies = int(ceil(min_duration_seconds / duration_seconds))
    if n_copies > 1:
        print(
            f'To achieve a duration of at least {min_duration_seconds} seconds, '
            f'"{file}" will be repeated {n_copies} times for a final duration of '
            f'{n_copies * nframes / framerate:0.4g} seconds.'
        )
        with wave.open(
                file.with_suffix('').name + f'_{n_copies:04d}-copies' + file.suffix,
                'wb',
        ) as output_audio_file:
            output_audio_file.setparams(
                (nchannels, sampwidth, framerate, nframes * n_copies, comptype, compname)
            )
            for _ in range(n_copies):
                output_audio_file.writeframesraw(frames)
    else:
        print(
            f'{min_duration_seconds} seconds is too short; '
            f'"{file}" would occur only once.\n'
            'No output file will be created.'
        )
