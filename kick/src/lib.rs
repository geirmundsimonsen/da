use once_cell::sync::Lazy;
use da_interface::{Config, Midi, NoteOn, NoteOff, CC, PB};

static CONFIG: Lazy<Config> = Lazy::new(|| {
    Config {
        upsampling_factor: 16,
        num_in_channels: 1,
        num_out_channels: 1,
        num_midi_in_ports: 1,
        num_midi_out_ports: 1,
        name: "Test".to_string(),
        connections: vec![
            ("Test:out1".to_string(), "system:playback_9".to_string()),
            //("Test:out1".to_string(), "REAPER:in1".to_string()),
            ("a2j:A-Series Keyboard [28] (capture): A-Series Keyboard Keyboard".to_string(), "Test:midi_in1".to_string()),
        ]
    }
});

#[no_mangle]
pub fn init() -> Config {
    da_csound::init("test.csd", 64, &CONFIG);
    CONFIG.clone()
}

#[no_mangle]
pub fn html() -> String {
    r#""#.to_string()
}

#[no_mangle]
pub fn js() -> String {
    r#"
    function createSlider(name, param, min, max, precision=3, type="linear") {
        let initial = initial_host_params[param];
        let initialMapped;

        if (type == "linear") {
            initialMapped = (initial - min) / (max - min);
        } else if (type == "exp") {
            initialMapped = Math.sqrt((initial - min) / (max - min));
        }

        console.log(initial);
        console.log(initialMapped);

        const slider = document.createElement("div");
        slider.className = "slider";

        slider.innerHTML = `
            <p>${name}</p>
            <input type="range" min="0" max="1" step="any" orient="vertical" />
            <p></p>
        `;

        const input = slider.querySelector("input");
        const output = slider.querySelector("p:last-child");

        const slider2param = () => {
            let mapped;
            if (type == "linear") {
                mapped = input.value * (max - min) + min;
            } else if (type == "exp") {
                mapped = input.value ** 2 * (max - min) + min;
            }
            send(param, mapped);
            output.innerText = parseFloat(mapped).toFixed(precision);
        }

        input.oninput = slider2param;
        input.value = initialMapped;
        slider2param();

        document.body.appendChild(slider);
    }

    createSlider("Freq", 0, 30, 120, 1);
    createSlider("Gain", 1, 1, 100, 1, "exp");
    createSlider("P.Dec.", 2, 0, 1, 3);
    createSlider("P.Str.", 3, 0, 10000, 0, "exp");

    "#.to_string()
}

#[no_mangle]
pub fn css() -> String {
    r#"
    .slider {
        display: inline-flex;
        flex-direction: column;
        width: 50px;
        align-items: center;   
    }

    .slider p {
        margin: 0;
        padding: 0;
    }

    .slider input {
        appearance: slider-vertical;
        width: 20px;
        height: 200px;
    }
    "#.to_string()
}

#[no_mangle]
pub fn next(samples: &mut [f64; 32], time_in_samples: u64, midi_in: &Vec<Midi>, midi_out: &mut Vec<Midi>, params: &mut Vec<f64>) {
    if time_in_samples == 0 {
        params[0] = 50.0;
        params[1] = 2.0;
        params[2] = 0.3;
        params[3] = 100.0;
    }

    midi_in.iter().for_each(|m| {
        match m {
            Midi::On(on) => {
                da_csound::send_instr_event(&vec![1.0, 0.0, 0.5, on.note as f64]);
            },
            _ => (),
        }
    });

    da_csound::process(time_in_samples, params, samples);
}