<Config>
name: kickk
upsample: 16
audio_in: 0
audio_out: 1
midi_in: 1
midi_out: 0
midi_connect:
    - [keyboard, self:1]
audio_connect:
    - [self:1, system:9]
param:
    - [lin, Freq, 50, 30, 120, 1]
    - [exp, Gain, 2, 1, 100, 1]
    - [lin, P.Dec., 0.3, 0, 1, 3]
    - [exp, P.Str., 100, 0, 10000, 0]
midi_routing:
    - { mode: PolyTrig, channel: 0, instr: 1 }
</Config>
<CsoundSynthesizer>
<CsInstruments>
0dbfs = 6
ksmps = 64

instr 1
asig = vco2(1, mtof(p4))
out asig
endin

</CsInstruments>
<CsScore>

</CsScore>
</CsoundSynthesizer>
