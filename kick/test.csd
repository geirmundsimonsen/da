<CsoundSynthesizer>
<CsOptions>
</CsOptions>
<CsInstruments>
0dbfs = 6

gisine ftgen 0, 0, 2^10, 10, 1

instr 1
aenv transeg 1, 0.5, -4, 0
apenv = transeg(chnget:i("param3"), chnget:i("param2"), -4, 0)
asig = oscili:a(0.5, (chnget:k("param0") + apenv), gisine)
asig *= lag(chnget:k("param1"), 0.1) ;^2 * 100 + 1
asig clip asig, 2, 1
asig clfilt asig, 16000, 0, 8
outs asig * aenv
endin

</CsInstruments>
<CsScore>
t 0 120

i1 3600, 0
</CsScore>
</CsoundSynthesizer>
