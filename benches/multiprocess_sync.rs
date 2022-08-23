#[macro_use]
extern crate criterion;

use std::{env, fs::OpenOptions, io::{BufWriter, Write}};

use criterion::Criterion;
use log::Record;
use log4rs::{append::{rolling_file::{policy::{compound::{roll::{fixed_window::FixedWindowRoller, Roll}, CompoundPolicy, trigger::size::SizeTrigger}}, LogWriter, RollingFileAppender}, Append}, encode::pattern::PatternEncoder};

fn roll_benchmark(c: &mut Criterion) {
    let active_file = env::temp_dir().join("log4rs-roll.log");
    let roller = FixedWindowRoller::builder().build(active_file.with_extension("{}").to_str().unwrap(), 10).unwrap();
    c.bench_function("::append::rolling_file::policy::compound::roll::fixed_window::roll", move |b| b.iter(|| {
        roller.roll(&active_file).unwrap();
    }));
}

fn write_benchmark(c: &mut Criterion) {
    let file = OpenOptions::new()
        .write(true)
        .append(true)
        .truncate(false)
        .create(true)
        .open(env::temp_dir().join("log4rs-write.log")).unwrap();

    let mut writer = LogWriter {
        file: BufWriter::new(file),
        len: 0,
        check_counter: 0,
    };
    c.bench_function("::append::rolling_file::LogWriter::write", move |b| b.iter(|| {
        writer.write("This is a log message".as_bytes()).unwrap();
    }));
}

fn append_benchmark(c: &mut Criterion) {
    let file_path = env::temp_dir().join("log4rs-append.log");
    let roller = FixedWindowRoller::builder().build(file_path.with_extension("{}").to_str().unwrap(), 10).unwrap();
    let policy = CompoundPolicy::new(Box::new(SizeTrigger::new(1)), Box::new(roller));
    let appender = RollingFileAppender::builder()
        .encoder(Box::new(PatternEncoder::default()))
        .build(file_path, Box::new(policy)).unwrap();

    let record = Record::builder().args(format_args!("This is a log message")).build();
    c.bench_function("::append::rolling_file::RollingFileAppender::append", move |b| b.iter(|| {
        appender.append(&record).unwrap();
    }));
}
 
criterion_group!(benches, roll_benchmark, write_benchmark, append_benchmark);
criterion_main!(benches);