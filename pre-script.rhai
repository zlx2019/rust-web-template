// Set the current year to the environment variable
let date = system::date();
variable::set("copyright_year", date.year.to_string());
