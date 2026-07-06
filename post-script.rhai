// 根据用户所选许可证，保留对应文件为 LICENSE，并清理其余候选
let license = variable::get("license");
file::rename("licenses/LICENSE-" + license, "LICENSE");
file::delete("licenses");

// 模板仓库自述文件与生成项目自述文件相互独立，此处将项目自述文件就位
file::rename("README-TEMPLATE.md", "README.md");
