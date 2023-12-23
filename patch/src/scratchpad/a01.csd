<CsoundSynthesizer>
<CsOptions>
-+rtaudio=null -odac
</CsOptions>
<CsInstruments>
0dbfs = 6

ga0 init 0
gisine ftgen 0, 0, 2^10, 10, 1

instr 1
asig vco2 0.5, 110
asig *= 0.1
outs asig, asig
endin

instr 10
endin

</CsInstruments>
<CsScore>
t 0 120
i1 0 60

i10 3600, 0
</CsScore>
</CsoundSynthesizer>
