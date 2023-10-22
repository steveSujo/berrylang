use std::{env::args_os, ffi::OsString, fs::File, io::Read, path::Path};

#[allow(dead_code)]
#[derive(Debug)]
struct ArgsCli {
    bin_path: OsString,
    arg_vec: Vec<OsString>,
    input_file: Option<OsString>,
    output_file: Option<OsString>,
}

//To Parse the Command line Arguments//////////////////////////////////////////

fn parse_args() -> ArgsCli {
    let mut raw_args: Vec<OsString> = args_os().collect();

    let bin_path: OsString = raw_args[0].to_owned();
    raw_args.remove(0);

    let mut input_file: Option<OsString> = None;
    let mut output_file: Option<OsString> = None;
    let arg_vec: Vec<OsString>;

    // match raw_args.iter().position(|x| x == "-i" || x == "--input") {
    //     Some(index) => {
    //         arg_input = Some(raw_args[index + 1].to_owned());
    //         raw_args.remove(index);
    //         raw_args.remove(index + 1);
    //         // filtering the arg vector
    //     }
    //     None => arg_input = None,
    // }

    // match raw_args.iter().position(|x| x == "-o" || x == "--output") {
    //     Some(index) => {
    //         arg_output = Some(raw_args[index + 1].to_owned());
    //         raw_args.remove(index);
    //         raw_args.remove(index + 1);
    //         // filtering the arg vector
    //     }
    //     None => arg_output = None,
    // }
    let mut clone_raw = raw_args.clone();
    raw_args.iter().enumerate().for_each(|(i, arg)| {
        match arg.to_str().unwrap() {
            "-i" | "--input" => {
                input_file = Some(raw_args[i + 1].to_owned());
                clone_raw.remove(i + 1);
                clone_raw.remove(i);
            }
            "-o" | "--output" => {
                output_file = Some(raw_args[i + 1].to_owned());
                clone_raw.remove(i + 1);
                clone_raw.remove(i);
            }
            &_ => (),
        };
    });

    arg_vec = clone_raw;

    ArgsCli {
        bin_path,
        arg_vec,
        input_file,
        output_file,
    }
}

//Handle commands and thier panic//////////////////////////////////////////////

pub fn handle_commands() -> Result<(), Box<dyn std::error::Error>> {
    let args = parse_args();
    //handling non existant input file path
    if args.input_file.is_some()
        && !Path::new(args.input_file.as_ref().unwrap().as_os_str()).exists()
    {
        panic!("File {:?} is not found!", args.input_file)
    }

    //handling options
    parse_args().arg_vec.iter().for_each(|cmd| {
        let _ = match cmd.to_str().unwrap() {
            "-h" | "--help" => help_cmd(&mut std::io::stdout()),
            "run" => run_cmd(&args, &mut std::io::stdout()), //FIXME run cmd call
            // "build" => build_cmd(&mut std::io::stdout(), &mut std::io::stderr()), //TODO call build
            &_ => panic!("Command {cmd:?} is unknown! use --help or -h for the list of commands"),
        };
    });

    Ok(())
}

fn help_cmd(mut writer: impl std::io::Write) -> Result<(), Box<dyn std::error::Error>> {
    write!(
        writer,
        "\n Berry code Compiler

Usage: berry [COMMAD] [OPTIONS]

Options:
    -h, --help      : To print this help
    -i, --input     : Input file path
    -o, --output    : Output file path or name

Options:
run                 :!!todo();
build               :!!todo();
\n"
    )
    .expect("unexpected stdio error");

    Ok(())
}

/// .
///
/// # Panics
///
/// Panics if .
fn run_cmd(
    args: &ArgsCli,
    mut stdout: impl std::io::Write,
) -> Result<(), Box<dyn std::error::Error>> {
    // write!(stdout, "TO IMPLIMENT!!").expect("unexpected stdio error");
    // write!(stderr, "TO IMPLIMENT!!").expect("unexpected stdio error");

    let mut input = File::open(args.input_file.as_ref().unwrap()).expect("Couldn't open file");
    let mut contents = String::new();

    input.read_to_string(&mut contents)?;

    contents = contents.replace("now", "ever");

    write!(stdout, "the new contents of the file: {}", contents)
        .expect("somethings wrong with the buffer");

    Ok(())
}

// fn build_cmd() {
//     todo!()
// }
//TODO impl build cmd
