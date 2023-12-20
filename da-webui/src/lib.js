function createSlider(name, param, min, max, precision=3, type="linear") {
    let initial = initial_host_params[param];
    let initialMapped;

    if (type == "linear") {
        initialMapped = (initial - min) / (max - min);
    } else if (type == "exp") {
        initialMapped = Math.sqrt((initial - min) / (max - min));
    }

    const slider = document.createElement("div");
    slider.className = "slider";
    
    slider.innerHTML = `
        <p>${name}</p>
        <input type="range" min="0" max="1" step="any" orient="vertical" />
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
        }
        send(param, mapped);
        output.innerText = parseFloat(mapped).toFixed(precision);
    }

    input.oninput = slider2param;
    input.value = initialMapped;
    slider2param();

    document.body.appendChild(slider);
}
