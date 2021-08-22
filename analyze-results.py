import wave
import matplotlib.pyplot as plot
import numpy as np
import soundfile as sf


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

with open('original.dat') as f:
    original_contents = f.read()
orig_vec = decode(original_contents)

out_orig = np.stack([orig_vec, orig_vec], axis=-1)
out_conved = np.stack([conved_vec, conved_vec], axis=-1)
breakpoint()
sf.write('orig.flac', out_orig, 44100)
sf.write('conved.flac', out_conved, 44100)

reader = wave.open('apology.wav')
_ = reader.readframes(44100 * 5)
gt_data = reader.readframes(44100)
gt_data = np.frombuffer(gt_data, dtype='int32')


conved_vec = norm(conved_vec)
orig_vec = norm(orig_vec)
gt_data = norm(gt_data)
diff = np.abs(conved_vec - orig_vec)
plot.plot(diff)
plot.show()

plot.plot(conved_vec, label='conved')
plot.plot(orig_vec, label='original')
plot.plot(gt_data, label='gt ish')
plot.legend()
plot.show()


spec_conved = np.fft.fft(conved_vec)
spec_orig = np.fft.fft(orig_vec)
plot.plot(np.abs(spec_conved ** 2))
plot.plot(np.abs(spec_orig ** 2))
plot.show()
