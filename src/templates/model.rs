#[derive(Serialize, Deserialize)]
struct Version {
    id: String,
    template_id: String,
    active: i32,
    name: String,
    plain_content: String,
    subject: String
}

#[derive(Serialize, Deserialize)]
struct Template {
    id: String,
    name: String,
    versions: Vec<Version>
}

struct Templates {
    templates: Vec<Template>
}