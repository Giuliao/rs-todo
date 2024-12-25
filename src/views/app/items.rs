use actix_web::HttpResponse;

use super::content_loader::add_component;
use super::content_loader::read_file;

pub async fn items() -> HttpResponse {
    let mut html_data = read_file("src/templates/main.html");
    html_data = add_component("header".to_string(), html_data);
    html_data = add_component("login".to_string(), html_data);
    let javascript_data = read_file("src/javascript/main.js");
    let css_data: String = read_file("src/css/main.css");
    let base_css_data: String = read_file("src/css/base.css");
    html_data = html_data.replace("{{JAVASCRIPT}}", &javascript_data);
    html_data = html_data.replace("{{CSS}", &css_data);
    html_data = html_data.replace("{{BASE_CSS}}", &base_css_data);
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html_data)
}
