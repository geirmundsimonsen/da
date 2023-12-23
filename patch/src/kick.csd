<CsoundSynthesizer>
<CsInstruments>
#include "util.csd"

0dbfs = 6

gisine ftgen 0, 0, 2^10, 10, 1

instr 1
aenv transeg 1, 0.5, -4, 0
apenv = transeg(chnget:i("P.Str."), chnget:i("P.Dec."), -4, 0)
asig = oscili:a(0.5, (chnget:k("Freq") + apenv), gisine)
asig *= lag(chnget:k("Gain"), 0.1)
asig clip asig, 2, 1
asig clfilt asig, 16000, 0, 8
outs asig * aenv
endin

</CsInstruments>
</CsoundSynthesizer>
