use std::{thread::spawn, net::{TcpListener, TcpStream}, io::{BufReader, BufRead, Read, Write}};
use da_interface::{ParamType, Config};
use libloading::{Library, Symbol};

use crate::param::PARAMS;

pub fn create_html(shared_lib: &str, config: &Config) -> String {
  let lib = unsafe { Library::new(shared_lib).unwrap() };

  let initial_host_params_array = PARAMS.lock().unwrap().iter().enumerate().map(|(i, p)|
    match p.param_type {
      ParamType::Linear(min, max, decimals) => format!("{{ name: \"{}\", param: {i}, value: {}, min: {}, max: {}, decimals: {}, type: \"linear\" }}", p.name, p.value, min, max, decimals),
      ParamType::Exponential(min, max, decimals) => format!("{{ name: \"{}\", param: {i}, value: {}, min: {}, max: {}, decimals: {}, type: \"exponential\" }}", p.name, p.value, min, max, decimals),
      ParamType::List(ref list) => format!("{{ name: \"{}\", param: {i}, value: {}, list: {:?}, type: \"list\" }}", p.name, p.value, list),
    }
  ).collect::<Vec<String>>().join(",");
  
  let presets = std::fs::read_to_string(format!("presets/{}.txt", config.name.to_lowercase())).unwrap_or("".to_string()).lines().map(|line| {
    let parts = line.split(",").enumerate().map(|(i, part)| {
      if i == 0 || i % 2 == 1 {
        format!("\"{}\"", part)
      } else {
        format!("{}", part)
      }
    }).collect::<Vec<String>>().join(",");
    let preset = format!("[{parts}]");
    preset
  }).collect::<Vec<String>>().join(",");

  let name = config.name.clone();

  let presets_array = format!("let presets = [{presets}]");
  let initial_host_params = format!("let initial_host_params = [{initial_host_params_array}]");

  let host_js = r#"
async function send(param, value) {
  await fetch(`http://localhost:7878/param/${param}`, {
    method: 'POST',
    body: value
  });
}

async function savePreset(name) {
  await fetch('http://localhost:7878/savepreset', {
    method: 'POST',
    body: name
  });
}
"#;

  let html = unsafe { lib.get::<Symbol<unsafe extern fn() -> String>>(b"html").unwrap()() };
  let js = unsafe { lib.get::<Symbol<unsafe extern fn() -> String>>(b"js").unwrap()() };
  let css = unsafe { lib.get::<Symbol<unsafe extern fn() -> String>>(b"css").unwrap()() };

  format!(r#"
  <!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="utf-8">
    <style>
      {css}
      #name {{
        display: flex;
        padding: 20px;
        justify-content: center;
        align-content: center;
        flex-direction: column;
        background-color: #333;
        color: #fff;
        font-size: 36px;
        margin-bottom: 20px;
      }}

      #name > p {{
      }}

      #app {{
        display: flex;
        flex-direction: row;
      }}

      #presets {{
        width: 200px;
        margin-right: 20px;
      }}

      #params {{
        width: 100%;
      }}
    </style>
    <title>Rust Jack Audio Control App</title>
  </head>
  <body>
    <div id="name">{name}</div>
    <div id="app">
      <div id="presets"></div>
      <div id="params"></div>
    </div>
    {html}
    <script>
      {initial_host_params}
      {presets_array}
      {host_js}
      {js}
    </script>
  </body>
</html>
"#)
}

pub fn start_server(shared_lib: &str, config: &Config) {
  let shared_lib = shared_lib.to_string();
  let config = config.clone();
  spawn(move || {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    println!("HTTP server available at http://localhost:7878/");

    for stream in listener.incoming() {
      let stream = stream.unwrap();

      handle_connection(stream, &shared_lib, &config);
    }
  });
}

fn get_url(request_line: &str) -> &str {
  let byte_index_start = request_line.find("/").unwrap();
  let byte_index_end = request_line.find(" HTTP").unwrap();
  &request_line[byte_index_start..byte_index_end]
}

fn handle_connection(mut stream: TcpStream, shared_lib: &str, config: &Config) {
  let mut buf_reader = BufReader::new(&mut stream);
  let mut request_line = String::new();
  let mut content_length = 0;

  // Read the request headers
  loop {
    let mut line = String::new();
    buf_reader.read_line(&mut line).unwrap();
    if line.starts_with("Content-Length:") {
      content_length = line.split_whitespace().last().unwrap().parse().unwrap();
    }
    request_line.push_str(&line);
    if line == "\r\n" { break; }
  }

  let url = get_url(&request_line);

  // Read the body if Content-Length is present
  let mut body = String::new();
  if content_length > 0 {
    buf_reader.by_ref().take(content_length as u64).read_to_string(&mut body).unwrap();
  }

  if request_line.starts_with("POST /param/") {
    let param = url.split("/").nth(2).unwrap().parse::<usize>().unwrap();
    PARAMS.lock().unwrap().get_mut(param).unwrap().value = body.parse().unwrap();
    let status_line = "HTTP/1.1 200 OK";
    let response = format!("{status_line}\r\nAccess-Control-Allow-Origin: *\r\n\r\n");
    stream.write_all(response.as_bytes()).unwrap();
  } else if request_line.starts_with("GET /") {
    let status_line = "HTTP/1.1 200 OK";
    let html = create_html(&shared_lib, &config);
    let length = html.len();
    let response = format!("{status_line}\r\nContent-Length: {length}\r\nAccess-Control-Allow-Origin: *\r\n\r\n{html}");
    stream.write_all(response.as_bytes()).unwrap();
  } else if request_line.starts_with("POST /savepreset") {
    let preset_name = body;
    let mut presets_file = std::fs::OpenOptions::new().create(true).append(true).open(format!("presets/{}.txt", config.name.to_lowercase())).unwrap();
    let mut preset_line = format!("{},", preset_name);
    for param in PARAMS.lock().unwrap().iter() {
      preset_line.push_str(&format!("{},", param.name));
      preset_line.push_str(&format!("{},", param.value));
    }
    preset_line.push_str("\n");
    presets_file.write_all(preset_line.as_bytes()).unwrap();
    let status_line = "HTTP/1.1 200 OK";
    let response = format!("{status_line}\r\nAccess-Control-Allow-Origin: *\r\n\r\n");
    stream.write_all(response.as_bytes()).unwrap();
  }
}
