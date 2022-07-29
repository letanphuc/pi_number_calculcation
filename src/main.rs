use std::{env::args, thread};

struct PiArray {
    index: i128,
}

impl Iterator for PiArray {
    type Item = i128;
    fn next(&mut self) -> Option<Self::Item> {
        let tens = 1_000_000_000_000_000_000_000_000_000_000_000_i128;
        let ret = if self.index > 0 {
            let sign = if self.index % 2 == 0 { -1 } else { 1 };
            let base = (self.index * 2) * (self.index * 2 + 1) * (self.index * 2 + 2);
            let delta = sign * 4_i128 * tens / base;
            Some(delta)
        } else {
            Some(3_i128 * tens)
        };
        self.index += 1;
        ret
    }
}

fn main() {
    let args: Vec<String> = args().collect();
    let num_threads: u128 = args[1].parse().unwrap();
    let chunk_size = 1_000_000 * num_threads;
    let mut pi = 0_i128;
    let mut chunk = 0_u128;
    let real_pi = 3141592653589793238462643383279503_i128;

    loop {
        let chunk_start = chunk * chunk_size;
        let mut threads: Vec<thread::JoinHandle<i128>> = vec![];
        for thread_id in 0..num_threads {
            threads.push(thread::spawn(move || -> i128 {
                let index = chunk_start + chunk_size * thread_id;
                let a = PiArray {
                    index: index as i128,
                };
                let result = a.take(chunk_size as usize).sum();
                result
            }));
        }
        let mut change = 0_i128;
        for t in threads {
            change += t.join().unwrap();
        }
        pi += change;
        if change.abs() < 10 {
            break;
        }
        let diff = (real_pi - pi).abs() as f64;
        let diff = diff.log10().ceil() as i32;
        println!("{chunk}: pi={pi}, change={change}, diff={diff}");
        chunk += 1;
    }

    println!("Done pi = {pi}");
}
