use indexmap::IndexMap;
use serde_derive::Deserialize;
use toml::de::Error;
use toml::Value;

#[cfg(test)]
mod tests {
    use std::fs;
    use std::fs::File;
    use std::io::Write;
    use std::path::Path;

    use crate::compile_toml;

    #[test]
    fn it_works() {
        let toml = fs::read_to_string("example/textured.toml").unwrap();
        let (vertex, fragment, _config) = compile_toml(toml.as_str()).unwrap();
        println!("Vertex {}", vertex);
        println!("Fragment {}", fragment);

        let out_file = File::create(Path::new("example/textured.vert"));
        out_file.unwrap().write_all(vertex.as_bytes()).expect("write to file");

        let out_file = File::create(Path::new("example/textured.frag"));
        out_file.unwrap().write_all(fragment.as_bytes()).expect("write to file");
    }
}

pub fn compile_toml(config: &str) -> Result<(String, String, Config), Error> {
    let result = toml::from_str(config);
    match result {
        Ok(config) => {
            Ok((compile_vertex(&config), compile_fragment(&config), config))
        }
        Err(result) => Err(result)
    }
}

pub fn compile_vertex(config: &Config) -> String {
    let mut builder: String = String::new();
    builder.push_str(format!("#version {} {}\n", &config.version, &config.profile).as_str());
    for (index, (name, type_)) in config.layout.iter().enumerate() {
        builder.push_str(
            format!("layout (location = {}) in {} {}; \n", index, type_, name).as_str());
    }
    builder.push('\n');
    for (name, type_) in &config.uniform {
        builder.push_str(
            format!("uniform {} {}; \n", type_, name).as_str());
    }
    builder.push('\n');
    for (name, _type) in &config.vertex.output {
        builder.push_str(format!("out {} {}; \n", _type, name).as_str());
    }
    builder.push('\n');
    if let Some(constants) = &config.vertex.constants.as_ref() {
        for (name, value) in constants.iter() {
            if let Value::String(str) = value {
                builder.push_str(format!("#define {}{} \n", name, str).as_str());
            } else {
                builder.push_str(format!("#define {} {} \n", name, value).as_str());
            }
        }
    }

    builder.push('\n');
    builder.push_str("void main() {\n \n");
    builder.push_str(config.vertex.source.as_str());
    builder.push_str("\n}\n");

    builder.push('\n');
    if let Some(constants) = &config.vertex.functions.as_ref() {
        for (_, value) in constants.iter() {
            builder.push_str(format!("{} \n\n", value).as_str());
        }
    }

    builder
}

pub fn compile_fragment(config: &Config) -> String {
    let mut builder: String = String::new();
    builder.push_str(format!("#version {} {}\n", &config.version, &config.profile).as_str());

    for (name, type_) in &config.uniform {
        builder.push_str(
            format!("uniform {} {}; \n", type_, name).as_str());
    }
    builder.push('\n');
    for (name, _type) in &config.vertex.output {
        builder.push_str(format!("in {} {}; \n", _type, name).as_str());
    }
    builder.push('\n');
    for (name, _type) in &config.fragment.output {
        builder.push_str(format!("out {} {}; \n", _type, name).as_str());
    }
    builder.push('\n');
    if let Some(constants) = &config.fragment.constants.as_ref() {
        for (name, value) in constants.iter() {
            if let Value::String(str) = value {
                builder.push_str(format!("#define {}{} \n", name, str).as_str());
            } else {
                builder.push_str(format!("#define {} {} \n", name, value).as_str());
            }
        }
    }

    builder.push('\n');
    builder.push_str("void main() {\n \n");
    builder.push_str(config.fragment.source.as_str());
    builder.push_str("\n}\n");

    builder.push('\n');
    if let Some(constants) = &config.fragment.functions.as_ref() {
        for (_, value) in constants.iter() {
            builder.push_str(format!("{} \n\n", value).as_str());
        }
    }

    builder
}

#[derive(Deserialize, Debug)]
pub struct Config {
    version: u32,
    profile: String,
    layout: IndexMap<String, String>,
    uniform: IndexMap<String, String>,
    vertex: ShaderConfig,
    fragment: ShaderConfig,
}

#[derive(Deserialize, Debug)]
pub struct ShaderConfig {
    constants: Option<IndexMap<String, Value>>,
    functions: Option<IndexMap<String, String>>,
    output: IndexMap<String, String>,
    source: String,
}
