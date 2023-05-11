use regex_ops::Query;

fn main() {
    let args: Vec<String> = std::env::args().collect();    
    let s = Query::new(&args);
    s.execute();
}
