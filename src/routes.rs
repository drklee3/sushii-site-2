use rocket::{Request, State};
use rocket_contrib::templates::Template;

use crate::model::{context::TemplateContext, commands::CommandList};
use crate::assets::ASSETS;

#[get("/")]
pub fn index() -> Template {
    Template::render("index", &TemplateContext {
        title: "sushii 2",
        name: None,
        commands: None,
        assets: &ASSETS,
        parent: "layout",
    })
}

#[get("/about")]
pub fn about() -> Template {
    Template::render("about", &TemplateContext {
        title: "About",
        name: None,
        commands: None,
        assets: &ASSETS,
        parent: "layout",
    })
}

#[get("/commands")]
pub fn commands(cmds: State<CommandList>) -> Template {
    Template::render("commands", &TemplateContext {
        title: "Commands",
        name: None,
        commands: Some(&cmds),
        assets: &ASSETS,
        parent: "layout",
    })
}

#[get("/hello/<name>")]
pub fn hello(name: String) -> Template {
    Template::render("hello", &TemplateContext {
        title: "Hello",
        name: Some(name),
        commands: None,
        assets: &ASSETS,
        parent: "layout",
    })
}

