use spqlios_arith_gpu::gpu::{
    create_bind_group, create_command_encoder, create_compute_pipeline, create_empty_sb,
    create_sb_with_data, execute_pipeline, finish_encoder_and_read_from_gpu, get_device_and_queue,
};
use spqlios_arith_gpu::shader::render_simple;

#[tokio::test]
pub async fn test_add_i64() {
    let (device, queue) = get_device_and_queue().await;

    let val = vec![-123i64];

    let val_buf = create_sb_with_data(&device, &val);
    let result_buf = create_empty_sb(&device, (std::mem::size_of::<i64>()) as u64);

    let source = render_simple("add_i64.wgsl");
    let compute_pipeline = create_compute_pipeline(&device, &source, "main");
    let mut command_encoder = create_command_encoder(&device);

    let bind_group = create_bind_group(&device, &compute_pipeline, 0, &[&val_buf, &result_buf]);

    execute_pipeline(
        &mut command_encoder,
        &compute_pipeline,
        &bind_group,
        1,
        1,
        1,
    );

    let results =
        finish_encoder_and_read_from_gpu::<i64>(&device, &queue, Box::new(command_encoder), &[
            result_buf,
        ])
        .await;

    assert_eq!(results[0][0], val[0] + 1);
}