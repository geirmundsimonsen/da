<CsoundSynthesizer>
<CsInstruments>
0dbfs = 6

instr 1
asig = vco2(1, 80 + rand:k(7))
apulse = (poscil(1, poscil(1, 0.13) * 0.5 + 0.5) * 0.5 + 0.5)^48
apulse_d init 0
apulse_d = apulse * 0.75 + delay(apulse_d * 0.93, 0.2991)
asig *= 0.1 * apulse_d
asig clfilt asig, 16000, 0, 8
outs asig, asig
endin

instr 2
asig = vco2(1, 160 + rand:k(15))
apulse = (poscil(1, poscil(1, 0.197) * 0.5 + 0.5) * 0.5 + 0.5)^48
apulse_d init 0
apulse_d = apulse * 0.75 + delay(apulse_d * 0.93, 0.2791)
asig *= 0.1 * apulse_d
asig clfilt asig, 16000, 0, 8
outs asig, asig
endin

instr 3
asig = vco2(1, 240 + rand:k(30))
apulse = (poscil(1, poscil(1, 0.168) * 0.5 + 0.5) * 0.5 + 0.5)^48
apulse_d init 0
apulse_d = apulse * 0.75 + delay(apulse_d * 0.93, 0.3191)
asig *= 0.1 * apulse_d
asig clfilt asig, 16000, 0, 8
outs asig, asig
endin

</CsInstruments>
<CsScore>
i1 0 3600
i2 0 3600
i3 0 3600
</CsScore>
</CsoundSynthesizer>
