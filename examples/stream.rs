use fibra::*;
use futures::Stream;
use std::task::Poll;
use std::io::{BufReader, Read};

struct FileStream(BufReader<std::fs::File>);

impl FileStream {
    pub fn new() -> FibraResult<Self> {
        Ok(Self(BufReader::new(std::fs::File::open(std::env::temp_dir().join("sample.txt"))?)))
    }
}

impl Stream for FileStream {
    type Item = FibraResult<Vec<u8>>;

    fn poll_next(mut self: std::pin::Pin<&mut Self>, _cx: &mut std::task::Context<'_>) -> Poll<Option<Self::Item>> {
        let mut buffer = vec![0; 10];

        match self.0.read(&mut buffer) {
            Ok(0) => Poll::Ready(None),
            Ok(n) => {
                buffer.truncate(n);
                Poll::Ready(Some(Ok(buffer)))
            },
            Err(e) => Poll::Ready(Some(Err(e.into()))),
        }
    }
}

#[tokio::main]
async fn main() -> FibraResult<()> {
    // generate a temp file
    std::fs::write(std::env::temp_dir().join("sample.txt"), "The quick brown fox jumps over the lazy dog.")?;

    let mut app = Fibra::new();
    app.mount(addon::Logger::new());

    // read the sample by stream
    // $ http -v localip.cc:3000
    app.get("/", |_| async { Ok(Response::new().stream(FileStream::new()?)) })?;

    app.bind(3000)?;
    app.run().await
}