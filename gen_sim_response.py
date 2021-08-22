import numpy as np
import matplotlib.pyplot as plot
import json

f = np.linspace(1 / 44100, 1200, 22050)
f_sim = f / (200 + f)
f_sim = f_sim.astype('float32')

plot.plot(f_sim)
plot.show()

corrector = 1 / f_sim
corrector = np.clip(corrector, 0, 2)
plot.plot(corrector)
plot.show()


out = {
    'data': corrector.tolist()
}
with open('corrector.json', 'w') as f:
    json.dump(out, f)
