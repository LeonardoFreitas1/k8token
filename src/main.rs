use yaml_rust::YamlLoader;
use std::{fs, env};

fn main() {
    const PATH_QA: &str = "/Users/leonardo/.kube/kubeconf-qa";
    const PATH_PRD: &str = "/Users/leonardo/.kube/kubeconf-prod";
    
    let mut path = PATH_QA;
    let args: Vec<String> = env::args().collect();
    let query = &args[1]; 

    if query == "prd" {
       path = PATH_PRD; 
    }
    let data = fs::read_to_string(path)
        .expect("Não foi possivel ler o arquivo"); 
    let docs = YamlLoader::load_from_str(&data).unwrap();
    let doc = &docs[0];

    let data = doc["users"][0]["user"]["auth-provider"]["config"]["id-token"]
    .as_str()
    .map(|s| s.to_string())
    .unwrap();

    cli_clipboard::set_contents(data.to_owned()).unwrap();
    println!("Token {:?} copied", data);
}
