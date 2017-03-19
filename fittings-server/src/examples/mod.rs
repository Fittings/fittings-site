use rocket;

mod servers
{
    pub mod paste_bin;
    mod paste_id;
}


pub fn get_routes(base_address: &str) -> Vec<rocket::Route> {
    let base_address = [base_address, "examples/"].concat();
    let mut routes = Vec::new();
    routes.extend(servers::paste_bin::get_routes());

    //Update all routes with this modules modules prefix;
    for route in &mut routes {
        let path = format!("{}{}", base_address, route.path.path());
        route.set_path(path);
    }

    routes
}
