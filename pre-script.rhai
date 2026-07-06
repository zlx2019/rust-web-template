// 获取当前年份，供 licenses/ 下 LICENSE 文件中的版权声明渲染使用
let date = system::date();
variable::set("copyright_year", date.year.to_string());
