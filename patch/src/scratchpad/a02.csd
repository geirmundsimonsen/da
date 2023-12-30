<CsoundSynthesizer>
<CsOptions>

</CsOptions>
<CsInstruments>
0dbfs = 6

instr 1
alfo vco2 1, 3, 6
klfo = linlin(k(alfo), 0, 0.2, -1, 1)
aenv env1 0, klfo, 0, 1
aosc vco2 0.5, 440
asig = aosc * aenv
asig clfilt asig, 16000, 0, 8
out asig
endin

</CsInstruments>
<CsScore>
i1 0 3600
</CsScore>
</CsoundSynthesizer>
