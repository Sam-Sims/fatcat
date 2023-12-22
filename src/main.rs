use clap::Parser;
use std::fmt::Write;
use std::io::{BufReader, Read};
use std::{
    thread::{spawn, sleep},
    time::Duration
};
use minus::{dynamic_paging, MinusError, Pager};
use noodles::fastq;
use boxy;
use crossterm::style::{Color, ResetColor, SetForegroundColor, Stylize};
use textwrap::{fill, indent, Options, wrap};

mod cli;
fn parse_fastq(path: &str) -> fastq::Reader<BufReader<Box<dyn Read>>>{
    let (reader, format) = match niffler::from_path(path) {
        Ok(result) => result,
        Err(e) => {
            std::process::exit(1);
        }
    };
    let reader  = BufReader::new(reader);
    let mut fastq_reader = fastq::Reader::new(reader);
    return fastq_reader;
}
fn print_wrapped_text<W: Write>(writer: &mut W, text: &str, width: usize) {
    let side = boxy::Char::vertical(boxy::Weight::Normal);
    let initial_indent = 0; // Adjust as needed
    let wrapped_text = wrap(text, width - initial_indent)
        .iter()
        .map(|line| format!("{:indent$}{}", "", line, indent = initial_indent))
        .collect::<Vec<_>>()
        .join("\n");
    // make the wrapped text red
    //let wrapped_text = wrapped_text.stylize().with(Color::Blue).to_string();
    let vertical_line_width = 1;
    let read_num_width = 8;
    for wrapped_line in wrapped_text.lines() {
        write!(writer, "{:<width$}", side, width = vertical_line_width);
        write!(writer, "{:<width$}", "", width = read_num_width);
        write!(writer, "{}", side);
        write!(writer, "{}", wrapped_line.trim_start());
        writeln!(writer);
    }
}

fn print_header(input_file: &str, pager: &mut Pager, h_size: u16) {
    let top = boxy::Char::horizontal(boxy::Weight::Normal);
    let down_tee = boxy::Char::down_tee(boxy::Weight::Normal);
    for _ in 0..h_size {
        pager.push_str(&top.to_string());
    }
    pager.push_str(input_file.to_string());
    pager.push_str("\n");

    for i in 0..h_size {
        if i == 0 || i == 9 {
            pager.push_str(&down_tee.to_string());
        } else {
            pager.push_str(&top.to_string());
        }
    }
}

fn avr_q_score(qual_string: &str) -> f32 {
    let mut total_phred = 0;
    for phred in qual_string.as_bytes() {
        total_phred += (phred - 33) as i32;
    }
    let average_phred = total_phred as f32 / qual_string.len() as f32;
    average_phred
}

fn process_fastq(pager: &mut Pager, fastq_path: &str, h_size: u16){
    let side = boxy::Char::vertical(boxy::Weight::Normal);
    let mut fastq_reader = parse_fastq(fastq_path);
    let mut read_num = 0;

    for result in fastq_reader.records() {
        read_num += 1;
        // First print the indent
        pager.push_str(format!("{:<width$}", side, width = 1));
        pager.push_str(format!("{:<width$}", read_num.to_string(), width = 8));
        pager.push_str(side.to_string());

        // Then process the record
        let record = result.unwrap();
        let read_name_string = std::str::from_utf8(record.name()).unwrap();
        let seq_string = std::str::from_utf8(record.sequence()).unwrap();
        let qual_string = std::str::from_utf8(record.quality_scores()).unwrap();
        let seq_len = seq_string.len();
        let average_phred = avr_q_score(qual_string);

        // Finally print the record
        //if q score is below 20, print in red
        if average_phred < 20.0 {
            pager.push_str(format!("{} {} {} {} {}", read_name_string.stylize().red(), "Length:".stylize().dark_grey(), seq_len.to_string().stylize().dark_grey(), "| Q:".stylize().dark_grey(), average_phred.to_string().stylize().red()));
        } else {
            pager.push_str(format!("{} {} {} {} {}", read_name_string.stylize().dark_green(), "Length:".stylize().dark_grey(), seq_len.to_string().stylize().dark_grey(), "| Q:".stylize().dark_grey(), average_phred.to_string().stylize().dark_grey()));
        }
        //pager.push_str(format!("{} {} {} {} {}", read_name_string.stylize().dark_green(), "Length:".stylize().dark_grey(), seq_len.to_string().stylize().dark_grey(), "| Q:".stylize().dark_grey(), average_phred.to_string().stylize().dark_grey()));
        pager.push_str("\n");
        print_wrapped_text(pager, seq_string, h_size as usize);
        sleep(Duration::from_millis(100));
    }
}
fn main() -> Result<(), MinusError> {
    // Args
    let args = cli::Cli::parse();

    // Spawn pager thread
    let mut pager = Pager::new();
    let pager2 = pager.clone();
    let pager_thread = spawn(move || dynamic_paging(pager2));

    // Get terminal size
    let h_size = crossterm::terminal::size().unwrap().0;

    // First print the header
    print_header(&args.input, &mut pager, h_size);

    // Then spawn the fastq thread
    let fastq_path = args.input;
    let fastq_thread = spawn(move || process_fastq(&mut pager, &fastq_path, h_size));

    pager_thread.join().unwrap()?;
    fastq_thread.join().unwrap();
    Ok(())
}