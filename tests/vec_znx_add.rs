use spqlios_arith_gpu::gpu::{
    create_bind_group, create_command_encoder, create_compute_pipeline, create_empty_sb,
    create_sb_with_data, execute_pipeline, finish_encoder_and_read_from_gpu, get_device_and_queue,
    create_ub_with_data 
};
use spqlios_arith_gpu::shader::render_simple;
use spqlios_arith_gpu::read_benchmark_data;
use stopwatch::Stopwatch;

#[tokio::test]
pub async fn test_vec_znx_add() {
    let (device, queue) = get_device_and_queue().await;

    // Read benchmark data for 2^17 elements
    let benchmark_data = read_benchmark_data("benchmark_data_131072_1.bin").expect("Failed to read benchmark data");
    assert_eq!(benchmark_data.n, 131072, "Benchmark data size mismatch");

    //let benchmark_data = read_benchmark_data("benchmark_data_128_1.bin").expect("Failed to read benchmark data");
    //assert_eq!(benchmark_data.n, 128, "Benchmark data size mismatch");

    let num_x_workgroups = 256;
    let num_y_workgroups = 1;
    let num_z_workgroups = 1;
    let workgroup_size = 1;

    let additions_per_thread = benchmark_data.n as usize / (num_x_workgroups * num_y_workgroups * num_z_workgroups * workgroup_size);

    let params = &[
        num_x_workgroups as u32,
        num_y_workgroups as u32,
        num_z_workgroups as u32,
        additions_per_thread as u32
    ];

    println!("Additions per thread: {}", additions_per_thread);
    
    let sw = Stopwatch::start_new();

    // Extract first vectors from arrays A and B
    let a = &benchmark_data.array_a[0];
    let b = &benchmark_data.array_b[0];
    
    let a_buf = create_sb_with_data(&device, &a);
    let b_buf = create_sb_with_data(&device, &b);
    let res_buf = create_empty_sb(&device, (std::mem::size_of::<i64>() * benchmark_data.n as usize) as u64);
    let params_buf = create_ub_with_data(&device, params);

    let source = render_simple("test_vec_znx_add.wgsl");
    let compute_pipeline = create_compute_pipeline(&device, &source, "main");
    let mut command_encoder = create_command_encoder(&device);

    let bind_group = create_bind_group(&device, &compute_pipeline, 0, &[&a_buf, &b_buf, &res_buf, &params_buf]);

    execute_pipeline(
        &mut command_encoder,
        &compute_pipeline,
        &bind_group,
        num_x_workgroups,
        num_y_workgroups,
        num_z_workgroups,
    );

    let results =
        finish_encoder_and_read_from_gpu::<i64>(&device, &queue, Box::new(command_encoder), &[
            res_buf,
        ])
        .await;

    let gpu_ms = sw.elapsed_ms();

    // Check results against expected values
    let expected = benchmark_data.compute_expected_results();
    let expected_first_vector = &expected[0];
    
    assert_eq!(results[0].len(), expected_first_vector.len(), "Result length mismatch");
    
    for (i, (&actual, &expected)) in results[0].iter().zip(expected_first_vector.iter()).enumerate() {
        assert_eq!(actual, expected, "Mismatch at index {}: got {}, expected {}", i, actual, expected);
    }
    println!("GPU execution time for {} elements: {}ms", benchmark_data.n, gpu_ms);
    println!("Time taken per element: {:.2}ns", (gpu_ms as f64 * 1_000_000.0) / benchmark_data.n as f64);
}

