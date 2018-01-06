use std::process::{Command, Stdio};
pub fn get_temperature() -> f64{
    let read_temperature_command = "/opt/vc/bin/vcgencmd";
    let read_temperature_argument = "measure_temp";
    let temperature_command_result = Command::new(read_temperature_command)
        .arg(read_temperature_argument)
        .stdout(Stdio::piped())
        .spawn()
        .unwrap_or_else(|e|{
            panic!("Could not run\n{} {}:\n{}",
                   read_temperature_command,
                   read_temperature_argument,
                   e);
        });

    let grep_command = "egrep";
    let grep_filter = "[0-9.]{4,}";
    let grep_print_only_matching = "-o";
    let grep_result = Command::new(grep_command)
        .arg(grep_filter)
        .arg(grep_print_only_matching)
        .stdin(temperature_command_result.stdout.unwrap())
        .output()
        .unwrap_or_else(|e| {
            panic!("Could no filter the temperature number from the string using:\n{} {} {}:\n{}",
                   grep_command,
                   grep_filter,
                   grep_print_only_matching,
                   e);
        });

    let temp_as_str = String::from_utf8_lossy(&grep_result.stdout).trim().to_string();
    let temperature = temp_as_str.parse::<f64>().unwrap_or_else(|e|{
        panic!("Could not parse the string:\n{}\ninto a f64 value:\n{}",
               temp_as_str,
               e);
    });
    temperature
}