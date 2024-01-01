<CsoundSynthesizer>
<CsOptions>

</CsOptions>
<CsInstruments>
0dbfs = 6

instr 1
aenv env4 0, 100, 0, 1, 1, -4, 0, 5, -4, 0.4, 1, 0, 0
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
