# Basic DSP project for Raspberry PI #

## Intention ##

This is a simple project that is just intended to allow play a known signal,
compute the frequency (and maybe in the future impulse) response, and run DSP
correction on a raspberry-pi 4B. This is all basically me messing around, so
some things are probably going to be theoretically incorrect, and some more are
probably going to be implemented incorrectly. If anyone wants to contribute to
either area, please let me know.

## Components ##

### 1. Measuring frequency/impulse response ###

TODO

### 2. Generating correction based on measured response ###

Correction is simply given by `1 / frequecy_response`. If impulse response can
be measured, it will have to go to something like

```
c1 = 1 / frequecy_response
c2 = 1 / fft( impulse_response)
c = c1 + λc2
```

Where `λ` is some predetermined constant. In an ideal world with perfect infinite
measurements, `c = c1 = c2` should be true? Feel free to correct me if not.

### 3. Accepting input from system sound ###

TODO mostly, I have some stuff laying around using https://github.com/RustAudio/cpal
that should help with this

### 4. Processing singal in real time ###

Processing will be done by convolution. To make this more efficient, this will be
done by element-wise multiplication of the input signal and the correction
signal in the Fourier domain.

```
Given
    s <- input signal
    c <- correction signal, computed above
Start:
    s_f = fft(s)
    s'_f = s .* c
    s' = ifft(s'_f)
```

Note that all corrections will likely have to be clipped to a reasonable range
so that the DSP doesn't just make things crazy quiet and bassy.

### 5. Writing processed signal to system sound ###

TODO mostly, I have some stuff laying around using https://github.com/RustAudio/cpal
that should help with this

### 6. Sharing memory between 3, 4, and 5 ###

This code is mostly written, and just has to be added and made to work with CPAL

## Division of labor ##

### Analysis ###

The analysis portion of this project will be done in python for a few reasons.

- It doesn't have to run _very_ quickly. Running in less than a minute would be
nice, but wouldn't be a deal killer if it doesn't happen
- There are a lot of really good tools for the analysis that I want to do.
    * numpy
    * scipy
    * matplotlib

### DSP ###

The actual signal correction will be done by a rust module so that the algorithm
can run in real time on a raspberry pi 4B. The DSP should take on the order of
~4MFLOPS by my calculations, and the CPU usage should be `O(log(n))`  with sample
size `n`. So I think this should be definitely doable, and on my dev machine
I'm only using ~600ms of CPU time on one core per 0.5 second sample, so this
seems like not an impossible task
