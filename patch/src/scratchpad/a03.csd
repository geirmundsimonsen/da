<Config>
;name = Kick
upsample = 16
audio_channels = 0, 1
midi_ports = 1, 0
m_connect = keyboard, self:1
a_connect = self:1, system:9
param = lin, Freq, 50, 30, 120, 1
param = exp, Gain, 2, 1, 100, 1
param = lin, P.Dec., 0.3, 0, 1, 3
param = exp, P.Str., 100, 0, 10000, 0
</Config>
<CsoundSynthesizer>
<CsInstruments>
0dbfs = 6
ksmps = 64

instr 1
asig = vco2(1, 80 + rand:k(7))
outs asig, asig
endin

</CsInstruments>
<CsScore>
i1 0 3600
</CsScore>
</CsoundSynthesizer>
