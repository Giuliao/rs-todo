use std::fs;

pub fn read_file(file_path: &str) -> String {
    fs::read_to_string(file_path).expect("Unable to read file")
}

pub fn add_component(component_tag: String, html_data: String) -> String {
    let css_tag: String = component_tag.to_uppercase() + "_CSS";
    let html_tag: String = component_tag.to_uppercase() + "_HTML";
    let css_path =
        String::from("src/templates/components/") + &component_tag.to_lowercase() + ".css";
    let css_loaded = read_file(&css_path);
    let html_path =
        String::from("src/templates/components/") + &component_tag.to_lowercase() + ".html";
    let html_loaded = read_file(&html_path);
    let html_data = html_data.replace(html_tag.as_str(), &html_loaded);
    let html_data = html_data.replace(css_tag.as_str(), &css_loaded);
    html_data
}
