use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use askama::Template;
use std::path::Path;
use std::fs;
use actix_files as afs;

#[derive(Template)]
#[template(path = "home.html", escape = "none")]
struct HomeTemplate {
    data: NavElem,
}

impl HomeTemplate {
    fn get_folder_content(&self, folder: &NavElem, out: &mut String, id: i32) {
        for elem in &folder.contains {
            println!("{}", elem.name);
            if let FileTypes::Page = elem.file_type {
                out.push_str(&("<li><a href=\"".to_string() + &elem.path + &"\" title=\"".to_string() + &elem.name + &"\">" + &elem.name + &"</a></li>".to_string()));
            }
            else {
                out.push_str(&("<li class=\"dirName\" id=\"folder_".to_string() + &id.to_string() + &"\"><a title=\"".to_string() + &elem.name + "\" href=\"javascript:void(0)\" onClick=\"showFolderContent(\'" + &elem.name + "\');\">" + &elem.name + "</a></li>"));
                out.push_str(&("<ul class=\"dir clair openedDir\" id=\"".to_string() + &id.to_string() + "\">"));
                self.get_folder_content(&elem, out, id + 1);
                out.push_str("</ul>");
            }
        }
    }

    pub fn get_nav(&self) -> String {
        let mut out = String::new();

        self.get_folder_content(&self.data, &mut out, 0);
        out
    }
}

enum FileTypes {
    Page,
    Category
}

struct NavElem {
    name: String,
    path: String,
    file_type: FileTypes,
    contains: Vec<NavElem>
}

fn build_nav_vec(path: &Path, nav: &mut Vec<NavElem>) {

    for file_wrapped in fs::read_dir(path).unwrap() {
        let file = file_wrapped.unwrap();
        if file.file_name().into_string().unwrap().starts_with(".") {
            continue;
        }
        if file.file_type().unwrap().is_file() {
            nav.push(NavElem {
                name: file.file_name().into_string().unwrap(),
                path: file.path().into_os_string().into_string().unwrap(),
                file_type: FileTypes::Page,
                contains: vec![]
            });
        }
        else {
            let mut vec = Vec::new();
            build_nav_vec(file.path().as_path(), &mut vec);
            nav.push(NavElem {
                name: file.file_name().into_string().unwrap(),
                path: file.path().into_os_string().into_string().unwrap(),
                file_type: FileTypes::Category,
                contains: vec
            });
        }
    }
}

#[get("/")]
async fn home() -> impl Responder {
    let mut nav = Vec::new();

    build_nav_vec(Path::new("pages"), &mut nav);
    let tpl = HomeTemplate {data: NavElem {
        name: "root".to_string(),
        path: "/".to_string(),
        file_type: FileTypes::Category,
        contains: nav,
    }};
    HttpResponse::Ok().body(tpl.render().unwrap())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(home)
            .service(afs::Files::new("/imgs", "./imgs"))
            .service(afs::Files::new("/js", "./js"))
            .service(afs::Files::new("/css", "./css"))
            .service(afs::Files::new("/page", "./pages"))
    })
    .bind(("localhost", 8080))?
    .run()
    .await
}