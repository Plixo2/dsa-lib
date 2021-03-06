<h1 align="center">Dynamic Shader Assembler</h1>
<div align="center">
  <strong>
    A Shader generation tool from TOML to GLSL written in Rust
  </strong>
</div>
<br />
<div align="center">

[//]: # (  <a href="https://crates.io/crates/async-observable">)

[//]: # (    <img src="https://img.shields.io/crates/v/async-observable.svg?style=flat-square")

[//]: # (    alt="crates.io version" />)

[//]: # (  </a>)
</div>

## Example


### Config

```toml
version = 330
profile = "core"

[layout]
a_pos = "vec3"
a_uv = "vec2"

[uniform]
transform = "mat4"
texture_0 = "sampler2D"

[fragment]
output = { result = "vec4" }
source = '''
result = texture(texture_0, uv);
'''

[vertex]
output = { uv = "vec2" }

source = '''
gl_Position = transform * vec4(a_pos, 1.0);
uv = a_uv;
'''
```

## Compiler 
```rust
let toml = fs::read_to_string("example/textured.toml").unwrap();
let (vertex,fragment, config) = compile_toml(toml.as_str()).unwrap();
println!("Vertex {}",vertex);
println!("Fragment {}",fragment);
```

## Output 

### Vertex
```glsl
#version 330 core
layout (location = 0) in vec3 a_pos; 
layout (location = 1) in vec2 a_uv; 

uniform mat4 transform; 
uniform sampler2D texture_0; 

out vec2 uv; 

void main() {
    gl_Position = transform * vec4(a_pos, 1.0);
    uv = a_uv;
}
```
### Fragment
```glsl
#version 330 core
uniform mat4 transform; 
uniform sampler2D texture_0; 

in vec2 uv; 

out vec4 result; 

void main() {
    result = texture(texture_0, uv);
}
```

You can specify `Functions`, `Constants` and `Output` per Shader.
But `Uniforms`, `Version`, `Profile` and `Layout` per Program.

Vertex `Output` are also synced with Fragment input.

For all examples see `examples/textured.toml`

# License

This project is licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or
  http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in toml-rs by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.



