pub fn format_time(time_in_secs: u32) -> String {
    // time_in_secs % 60 will give secs
    // time_in_secs / 60 will give mins
    // pad them to 00:00
    let minutes = format!("{:0>2}", time_in_secs / 60);
    let seconds = format!("{:0>2}", time_in_secs % 60);

    format!("{} : {}", minutes, seconds)
}
