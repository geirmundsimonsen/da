<CsoundSynthesizer>
<CsInstruments>
0dbfs = 6

ga0 init 0

instr 1
gaenv transeg 1, 1, -4, 0
gkfreq1 = mtof(p4 - chnget:k("Detune"))
gkfreq2 = mtof(p4 + chnget:k("Detune"))
endin

instr 2
asig vco2 0.5, gkfreq1
asig2 vco2 0.5, gkfreq2
asig = (asig + asig2) * gaenv
kfreq = lag(chnget:k("F.Freq"), 0.01)
kres = lag(chnget:k("F.Q"), 0.01)
if (chnget:k("F.Type") == 0.0) then
asig lowpass2 asig, kfreq, kres * 300 + 1
asig *= 2
elseif (chnget:k("F.Type") == 1.0) then
asig lpf18 asig, kfreq, kres, 10
asig *= 0.2
elseif (chnget:k("F.Type") == 2.0) then
asig moogladder asig, kfreq, kres
asig *= 2.5
elseif (chnget:k("F.Type") == 3.0) then
asig mvclpf3 asig, kfreq, kres
asig *= 2.5
elseif (chnget:k("F.Type") == 4.0) then
asig spf asig, ga0, ga0, kfreq, 2 - (kres * 2)
asig *= 1.5
elseif (chnget:k("F.Type") == 5.0) then
asig skf asig, kfreq, kres * 2 + 1
asig *= 1.5
elseif (chnget:k("F.Type") == 6.0) then
ahp, asig, abp, abr svn asig, kfreq, kres * 200 + 0.5, 0.3
asig *= 1.5
elseif (chnget:k("F.Type") == 7.0) then
asig zdf_ladder asig, kfreq, kres * 16.35 + 0.5
asig *= 1.5
elseif (chnget:k("F.Type") == 8.0) then
asig K35_lpf asig, kfreq, kres * 6.01 + 1
endif
asig clfilt asig, 16000, 0, 8
out asig
endin

</CsInstruments>
<CsScore>
i2 0 3600
</CsScore>
</CsoundSynthesizer>
