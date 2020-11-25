#[tokio::main]
async fn main() {
    let mut camera = rscam::new("/dev/video0").unwrap();

    camera
        .start(&rscam::Config {
            interval: (1, 10),
            resolution: (1280, 720),
            format: b"YUYV",
            ..Default::default()
        }).await
        .unwrap();

    for i in 1.. {
        let frame = camera.capture().await.unwrap();
        println!("Frame #{} of length {}", i, frame.len());
    }
}
