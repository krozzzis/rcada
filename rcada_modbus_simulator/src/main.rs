use clap::Parser;
use std::sync::{Arc, Mutex};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::time::{Duration, interval};

const SLAVE_ID: u8 = 1;
const SENSOR_COUNT: usize = 6;

#[derive(Debug, Clone, clap::Parser)]
struct Args {
    #[arg(default_value = "127.0.0.1:502")]
    addr: String,
}

struct Sensors {
    values: [u16; SENSOR_COUNT],
}

impl Sensors {
    fn new() -> Self {
        Self {
            values: [200, 500, 1013, 120, 1000, 1],
        }
    }

    fn update_time(&mut self, _elapsed: Duration) {
        let t = _elapsed.as_secs_f64();
        self.values[0] = (200.0 + (t * 0.1).sin() * 50.0) as u16;
        self.values[1] = (500.0 + (t * 0.05).cos() * 100.0) as u16;
        self.values[2] = (1013.0 + (t * 0.1).sin() * 10.0) as u16;
        self.values[3] = (120.0 + (t * 0.2).cos() * 20.0) as u16;
        self.values[4] = (1000.0 + (t * 0.3).sin() * 200.0) as u16;
        self.values[5] = 1;
    }

    fn read(&self, addr: u16, count: u16) -> Vec<u16> {
        (0..count)
            .map(|i| {
                let a = addr + i;
                if (a as usize) < SENSOR_COUNT {
                    self.values[a as usize]
                } else {
                    0
                }
            })
            .collect()
    }
}

async fn handle_client(stream: TcpStream, sensors: &Arc<Mutex<Sensors>>) -> std::io::Result<()> {
    let mut buf = [0u8; 260];
    let mut stream = stream;

    loop {
        let n = stream.read(&mut buf).await?;
        if n == 0 {
            break;
        }

        if n < 7 {
            continue;
        }
        if buf[6] != SLAVE_ID && buf[6] != 0xff {
            continue;
        }

        let func = buf[7];
        let start = ((buf[8] as u16) << 8) | (buf[9] as u16);
        let count = ((buf[10] as u16) << 8) | (buf[11] as u16);

        let values = {
            let sensors = sensors.lock().unwrap();
            sensors.read(start, count)
        };

        let byte_count = (count * 2) as u8;
        let mut response = vec![0u8; 9 + byte_count as usize];

        response[0] = buf[0];
        response[1] = buf[1];
        response[2] = 0;
        response[3] = 0;
        response[4] = 0;
        response[5] = 3 + byte_count as u8;
        response[6] = SLAVE_ID;
        response[7] = func;
        response[8] = byte_count;

        for (i, &val) in values.iter().enumerate() {
            response[9 + i * 2] = (val >> 8) as u8;
            response[9 + i * 2 + 1] = val as u8;
        }

        stream.write_all(&response).await?;
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let sensors = Arc::new(Mutex::new(Sensors::new()));
    let sensor_update = sensors.clone();

    tokio::spawn(async move {
        let mut interval = interval(Duration::from_millis(100));
        let start = std::time::Instant::now();
        loop {
            interval.tick().await;
            let elapsed = start.elapsed();
            let mut s = sensor_update.lock().unwrap();
            s.update_time(elapsed);
        }
    });

    let listener = TcpListener::bind(&args.addr).await?;
    println!("Listening on {}", args.addr);

    loop {
        let (stream, _) = listener.accept().await?;
        let sensors = sensors.clone();
        tokio::spawn(async move {
            if let Err(e) = handle_client(stream, &sensors).await {
                println!("Client error: {}", e);
            }
        });
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
