<CsoundSynthesizer>
<CsInstruments>
0dbfs = 6

instr 1
asig = vco2(1, transeg(400, 0.1, -7, 80))
asig *= 0.1
outs asig, asig
endin

</CsInstruments>
<CsScore>
i1 0 60
</CsScore>
</CsoundSynthesizer>
