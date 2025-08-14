@group(0) @binding(0)
var<storage, read> input: array<i64>;

@group(0) @binding(1)
var<storage, read_write> output: array<i64>;

@compute @workgroup_size(1)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let index = global_id.x;
    output[index] = input[index] + 1;
}