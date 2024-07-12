use askama::Template;

#[derive(Template)]
#[template(path = "index.html")]
pub struct HelloTemplate<'a> {
    pub app_title: &'a str,
    pub content: &'a str,
}

pub struct NavButton<'a> {
    pub name: &'a str,
    pub link: &'a str,
    pub icon: &'a str
}

#[derive(Template)]
#[template(path = "parts/sidebar.html")]
pub struct Navbar<'a>{
    pub buttons: Vec<NavButton<'a>>
}
