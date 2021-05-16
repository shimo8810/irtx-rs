use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Duration;

use rppal::pwm::{Channel, Polarity, Pwm};
use timerfd::{SetTimeFlags, TimerFd, TimerState};

const PERIOD: u64 = (1000.0 / 38.0) as u64;
const PULSE_WITDH: u64 = (1000.0 / 38.0 / 3.0) as u64;
fn main() {
    let filename = env::args()
        .nth(1)
        .unwrap_or("/home/pi/tick.txt".to_string());

    // ToDo: ファイルがない場合の例外処理を書く
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let data: Vec<u64> = reader
        .lines()
        .map(|s| s.unwrap().parse().unwrap())
        .collect();
    let data: Vec<u64> = data.iter().map(|x| x * 562).collect();

    println!("{} {}", PERIOD, PULSE_WITDH);
    let pwm = Pwm::with_period(
        Channel::Pwm0,
        Duration::from_micros(PERIOD),
        Duration::from_micros(PULSE_WITDH),
        Polarity::Normal,
        false,
    )
    .unwrap();

    let mut timerfd = TimerFd::new().unwrap();

    for x in data.chunks(2) {
        // if x.len() == 2 {
        let on = x[0];
        let off = x[1];
        // }
        // println!("{:?}", a);
        timerfd.set_state(
            TimerState::Oneshot(Duration::from_micros(on)),
            SetTimeFlags::Default,
        );
        pwm.enable().unwrap();
        timerfd.read();
        // thread::sleep(Duration::from_micros(on));

        timerfd.set_state(
            TimerState::Oneshot(Duration::from_micros(off)),
            SetTimeFlags::Default,
        );
        pwm.disable().unwrap();
        timerfd.read();
        // thread::sleep(Duration::from_micros(off));
    }

    // pwm.disable().unwrap();
}
