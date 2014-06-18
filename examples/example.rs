extern crate http;
extern crate floor;

use floor::{ Floor, Request, Response };

fn main() {

    let mut server = Floor::new();
    
    // we would love to use a closure for the handler but it seems to be hard
    // to achieve with the current version of rust.

    fn logger (req: &Request, res: &mut Response) -> bool{
        println!("logging!");
        true
    }

    server.utilize(logger);

    fn user_handler (request: &Request, response: &mut Response) {
        let text = format!("This is user: {}", request.params.get(&"userid".to_string()));
        response.write(text.as_slice());
    };

    // go to http://localhost:6767/user/4711 to see this route in action
    server.get("/user/:userid", user_handler);

    fn bar_handler (request: &Request, response: &mut Response) { 
        response.write("This is the /bar handler"); 
    };

    // go to http://localhost:6767/bar to see this route in action
    server.get("/bar", bar_handler);

    fn simple_wildcard (request: &Request, response: &mut Response) { 
        response.write("This matches /some/crazy/route but not /some/super/crazy/route"); 
    };

    // go to http://localhost:6767/some/crazy/route to see this route in action
    server.get("/some/*/route", simple_wildcard);

    fn double_wildcard (request: &Request, response: &mut Response) { 
        response.write("This matches /a/crazy/route and also /a/super/crazy/route"); 
    };

    // go to http://localhost:6767/a/nice/route or http://localhost:6767/a/super/nice/route to see this route in action
    server.get("/a/**/route", double_wildcard);

    fn post_handler (request: &Request, response: &mut Response) { 
        response.write("This matches a POST request to /a/post/request"); 
    };

    // go to http://localhost:6767/a/post/request to see this route in action
    server.post("/a/post/request", post_handler);

    server.listen(6767);
}
