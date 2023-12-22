function createSlider(name, param, min, max, decimals=3, type="linear") {
    let initial = initial_host_params[param].value;
    let initialMapped;

    if (type == "linear") {
        initialMapped = (initial - min) / (max - min);
    } else if (type == "exp") {
        initialMapped = Math.sqrt((initial - min) / (max - min));
    } else if (type == "step") {
        initialMapped = Math.floor(initial);
    }

    const slider = document.createElement("div");

    const sliderParams = type == 'step' 
        ? `min="${min}" max="${max}" step="1"` 
        : 'min="0" max="1" step="any"';
    
    slider.innerHTML = `
        <p>${name}</p>
        <input type="range" ${sliderParams} orient="vertical" />
        <p></p>
    `;
    slider.style.display = "inline-flex";
    slider.style.flexDirection = "column";
    slider.style.width = "50px";
    slider.style.alignItems = "center";

    const label = slider.querySelector("p:first-child");
    label.style.margin = "0";
    label.style.padding = "0";

    const input = slider.querySelector("input");
    input.style.appearance = "slider-vertical";
    input.style.width = "20px";
    input.style.height = "200px";

    const output = slider.querySelector("p:last-child");

    const slider2param = () => {
        let mapped;
        if (type == "linear") {
            mapped = input.value * (max - min) + min;
        } else if (type == "exp") {
            mapped = input.value ** 2 * (max - min) + min;
        } else if (type == "step") {
            mapped = Math.floor(input.value);
        }
        send(param, mapped);
        output.innerText = parseFloat(mapped).toFixed(decimals);
    }

    input.oninput = slider2param;
    input.value = initialMapped;
    slider2param();

    document.body.appendChild(slider);
}

function createList(name, param, options) {
    let initial = initial_host_params[param].value;

    const list = document.createElement("div");

    list.innerHTML = `
        <p>${name}</p>
        <select></select>
    `;

    list.style.display = "inline-flex";
    list.style.flexDirection = "column";
    list.style.width = "100px";
    list.style.alignItems = "center";

    const label = list.querySelector("p");
    label.style.margin = "0";
    label.style.padding = "0";

    const select = list.querySelector("select");
    select.style.width = "100%";

    for (let i = 0; i < options.length; i++) {
        const option = document.createElement("option");
        option.innerText = options[i];
        option.value = i;
        select.appendChild(option);
    }

    const list2param = () => {
        send(param, select.value);
    }

    select.onchange = list2param;
    select.value = initial;

    document.body.appendChild(list);
}

function createDefaultUI() {
    for (let i = 0; i < initial_host_params.length; i++) {
        const param = initial_host_params[i];
        if (param.type == "linear") {
            createSlider(param.name, param.param, param.min, param.max, param.decimals, "linear");
        } else if (param.type == "exponential") {
            createSlider(param.name, param.param, param.min, param.max, param.decimals, "exp");
        } else if (param.type == "list") {
            createList(param.name, param.param, param.list);
        }
    }
}