#[cfg(test)]
mod tests {
    use youtube_dl::YoutubeDl;
    #[test]
    fn it_works() {
        let out = YoutubeDl::new("https://www.youtube.com/watch?v=VFbhKZFzbzk")
            .socket_timeout("15")
            .run()
            .unwrap();
        dbg!(out);
    }
}
