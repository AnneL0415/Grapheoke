import scipy.io.wavfile as wavfile
import scipy
import scipy.fftpack as fftpk
from scipy.fft import fft
import numpy as np
from matplotlib import pyplot as plt

s_rate, signal = wavfile.read("C:/Users/Ali/Documents/GitHub/Grapheoke/fourier_transform/never_gonna_give_you_up_compressed.wav") 

FFTpure = fft(signal)
FFT = abs(FFTpure)

freqs = fftpk.fftfreq(len(FFT), (1.0/s_rate))
plt.plot(np.arange(0,len(freqs)),freqs)
plt.show()
freqs = [freqs for _, freqs in sorted(zip(FFT,freqs))]
FFTpure = [FFTpure for _, FFTpure in sorted(zip(FFT,FFTpure))]

FFT.sort()



truncateLen = 0.2
freqs = freqs[:int(len(freqs)*truncateLen)]
FFT = FFT[:int(len(FFT)*truncateLen)]
FFTpure = FFTpure[:int(len(FFT)*truncateLen)]

FFT = [FFT for _, FFT in sorted(zip(freqs,FFT))]
FFTpure = [FFTpure for _, FFTpure in sorted(zip(freqs,FFTpure))]

freqs.sort() 

NEWfreqs = [0]*len(freqs)
for x in range(int(len(freqs)//2)-1):
    NEWfreqs[x]=freqs[int(len(freqs)//2)-1+x]
for x in range(int(len(freqs))//2 -1 , int(len(freqs))-1):
    NEWfreqs[x]= 

# freqs.sort()
# print(freqs[0])
# print(freqs[len(freqs)-1])


# plt.plot(freqs[range(len(FFT)//2)], FFT[range(len(FFT)//2)])                                                          
# plt.xlabel('Frequency (Hz)')
# plt.ylabel('Amplitude')
# plt.show()

