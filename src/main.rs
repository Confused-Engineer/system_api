#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] 

use axum::{
    routing::get,
    Router,
};

const HELP: &str = include_str!("../assets/help.txt");
#[cfg(target_os="linux")]
const SETUP: &str = include_str!("../assets/setup_linux.txt");
#[cfg(target_os="windows")]
const SETUP: &str = include_str!("../assets/setup_windows.txt");

fn main() {
    
    let args: Vec<String> = std::env::args().collect();

    if let Some(arg) = args.get(1)
    {

        match arg.as_str() {
            "--help" => 
            {
                println!("{}", HELP)
            },
            "-h" => 
            {
                println!("{}", HELP)
            },
            "/help" => 
            {
                println!("{}", HELP)
            },
            "/?" => 
            {
                println!("{}", HELP)
            },

            "--setup" => 
            {
                println!("{}", SETUP)
            },

            "/setup" => 
            {
                println!("{}", SETUP)
            },

            _ => 
            {
                #[cfg(debug_assertions)]
                println!("Starting with key: \n{}", arg);
                api(Key::Some(arg.to_string()));
            }
        }
        

    } else {
        #[cfg(debug_assertions)]
        println!("Starting without key");
        api(Key::None);
    }


}

#[tokio::main]
async fn api(key: Key)
{

    let app: Router = Router::new()
    .route("/test", get(test));


    match key {
        Key::None => {
            let api_routes: Router = Router::new()
            .merge(app)
            .merge(app_router());
            let listener = tokio::net::TcpListener::bind("0.0.0.0:5002").await.unwrap();
            axum::serve(listener, api_routes).await.unwrap();
        },
        Key::Some(key) => {
            let api_routes: Router = Router::new()
            .merge(app)
            .merge(app_router_keyed(key));
            let listener = tokio::net::TcpListener::bind("0.0.0.0:5002").await.unwrap();
            axum::serve(listener, api_routes).await.unwrap();
        }
    }




}

fn app_router() -> Router
{

    return Router::new()
    .route("/api/v1/shutdown", get(shutdown))
    .route("/api/v1/restart", get(restart))
    .route("/api/v1/quit", get(quit))
    .route("/api/v1/systemctl/restart/gdm3", get(systemctl_restart_gdm3))
    .route("/api/v1/systemctl/restart/display-manager", get(systemctl_restart_displaymanager))
    

}

fn app_router_keyed(key: String) -> Router
{

    return Router::new()
    .route(&format!("/api/v1/{}/shutdown", key), get(shutdown))
    .route(&format!("/api/v1/{}/restart", key), get(restart))
    .route(&format!("/api/v1/{}/quit", key), get(quit))
    .route(&format!("/api/v1/{}/systemctl/restart/gdm3", key), get(systemctl_restart_gdm3))
    .route(&format!("/api/v1/{}/systemctl/restart/display-manager", key), get(systemctl_restart_displaymanager))


}




async fn test() -> String
{
    "Connection Successful".to_string()
}

async fn quit()
{
    std::process::exit(0);
}



async fn shutdown()
{
    #[cfg(target_os="windows")]
    let _ = std::process::Command::new("shutdown")
        .arg("/p")
        .spawn();

    #[cfg(target_os="linux")]
    let _ = std::process::Command::new("shutdown")
        .arg("now")
        .spawn();
}

async fn systemctl_restart_gdm3()
{

    #[cfg(target_os="linux")]
    let _ = std::process::Command::new("systemctl")
        .arg("restart")
        .arg("gdm3")
        .spawn();
}

async fn systemctl_restart_displaymanager()
{

    #[cfg(target_os="linux")]
    let _ = std::process::Command::new("systemctl")
        .arg("restart")
        .arg("display-manager")
        .spawn();
}

async fn restart()
{
    #[cfg(target_os="windows")]
    let _ = std::process::Command::new("shutdown")
        .arg("/t")
        .arg("1")
        .arg("/r")
        .spawn();

    #[cfg(target_os="linux")]
    let _ = std::process::Command::new("reboot")
        .arg("now")
        .spawn();
}

enum Key
{
    Some(String),
    None
}