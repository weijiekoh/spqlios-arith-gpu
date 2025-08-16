@group(0) @binding(0) var<storage, read> a: array<i64>;
@group(0) @binding(1) var<storage, read> b: array<i64>;
@group(0) @binding(2) var<storage, read_write> res: array<i64>;
@group(0) @binding(3) var<uniform> params: vec4<u32>;

@compute @workgroup_size(1)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let gidx = global_id.x; 
    let gidy = global_id.y; 
    let gidz = global_id.z; 
    let num_x_workgroups = params[0];
    let num_y_workgroups = params[1];
    let num_z_workgroups = params[2];
    let id = (gidx * num_y_workgroups + gidy) * num_z_workgroups + gidz;

    let n = params[3];
    let start = id * n;
    let end = start + n;
    for (var i = start; i < end; i++) {
        res[i] = a[i] + b[i];
    }
}
