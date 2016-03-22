extern crate gtk;
extern crate git2;

use gtk::prelude::*;

mod static_resource;

use git2::{Repository, Error};

fn get_repository() -> Result<git2::Repository, Error> {
    let repo = try!(Repository::open("."));
    if repo.is_bare() {
        return Err(Error::from_str("repo is bare"));
    }
    return Ok(repo);
}

fn get_branch_name(repo: &Repository) -> Result<String, Error> {
    let head = try!(repo.head());
    Ok(head.shorthand().unwrap_or("Empty").to_owned())
}

fn main() {
    let repo = get_repository().expect("Failed to get repository");
    
    gtk::init().unwrap();
     
    static_resource::init(); 
  
    let builder = gtk::Builder::new_from_resource("/org/example/ExampleApp/main.ui");
     
    let window : gtk::Window = builder.get_object("window1").unwrap();
    
    let entry1 : gtk::Entry = builder.get_object("entry1").unwrap();
    let button1 : gtk::Button = builder.get_object("button1").unwrap();
    button1.connect_clicked(move |_| {
        let text = match get_branch_name(&repo) {
            Ok(name) => name,
            Err(e) => format!("Failed to get: {}", e.message())
        };
        entry1.set_text(&text);
    });
    
    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });
    
    window.show_all();
    gtk::main();
}
