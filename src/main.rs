use ffmpeg;
use std::path::Path;
fn main() {
    ffmpeg::init().unwrap();
    let path = Path::new("data/BigBuckBunny_320x180.mp4");
    assert_eq!(path.exists(), true);
    let context = ffmpeg::format::input(&path).unwrap();
    if let Some(stream) = context.streams().best(ffmpeg::media::Type::Video) {
        println!("Best video stream index: {}", stream.index());
    }
    for stream in context.streams() {
        println!("stream index {}:", stream.index());
        println!("\ttime_base: {}", stream.time_base());
        println!("\tstart_time: {}", stream.start_time());
        println!("\tduration (stream timebase): {}", stream.duration());
        println!(
            "\tduration (seconds): {:.2}",
            stream.duration() as f64 * f64::from(stream.time_base())
        );
        println!("\tframes: {}", stream.frames());
        println!("\tdisposition: {:?}", stream.disposition());
        println!("\tdiscard: {:?}", stream.discard());
        println!("\trate: {}", stream.rate());

        let codec = stream.codec();
        println!("\tmedium: {:?}", codec.medium());
        println!("\tid: {:?}", codec.id());
    }
}
