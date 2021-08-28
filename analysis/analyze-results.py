import wave
import matplotlib.pyplot as plot
import numpy as np
import soundfile as sf
from scipy.ndimage import convolve1d
import json


def decode(contents: str) -> np.ndarray:
    contents = contents.replace('[', '').replace(']', '')
    list_contents = contents.split(',')
    list_contents = list(map(float, list_contents))
    return np.array(list_contents).astype('float32')


def norm(vec: np.ndarray) -> np.ndarray:
    vec = vec.astype('float64')
    vec -= np.mean(vec)
    vec /= np.amax(np.abs(vec))
    vec *= 0.8
    return vec

with open('conved.dat') as f:
    conved_contents = f.read()
conved_vec = decode(conved_contents)
conved_vec = norm(conved_vec)

with open('original.dat') as f:
    original_contents = f.read()
orig_vec = decode(original_contents)
orig_vec = norm(orig_vec)

with open('corrector.json', 'r') as f:
    contents = json.load(f)
corrector = np.fft.fft(contents['data'])
conved_gt = convolve1d(orig_vec, corrector)
conved_gt = norm(conved_gt)

out_orig = np.stack([orig_vec, orig_vec], axis=-1)
out_conved = np.stack([conved_vec, conved_vec], axis=-1)
sf.write('orig.flac', out_orig, 44100)
sf.write('conved.flac', out_conved, 44100)
sf.write('gt.flac', conved_gt, 44100)

plot.plot(conved_vec, label='conved')
plot.plot(orig_vec, label='original')
plot.plot(conved_gt, label='gt ish')
plot.legend()
plot.show()

spec_conved = np.fft.fft(conved_vec)
spec_orig = np.fft.fft(orig_vec)
gt_conv_spec = np.fft.fft(conved_gt)
conv_spec = 10 * np.log10(np.abs(spec_conved ** 2))[:22050 // 2]
og_spec = 10 * np.log10(np.abs(spec_orig ** 2))[:22050 // 2]
gt_conv_spec = 10 * np.log10(np.abs(gt_conv_spec ** 2))[:22050 // 2]
plot.plot(conv_spec, label='conved')
plot.plot(og_spec, label='original')
plot.plot(gt_conv_spec, label='gt')
plot.legend()
plot.show()
