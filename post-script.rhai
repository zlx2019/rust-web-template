// keep the selected license
let license = variable::get("license");
file::rename("licenses/LICENSE-" + license, "LICENSE");
file::delete("licenses");

file::rename("README-TEMPLATE.md", "README.md");
