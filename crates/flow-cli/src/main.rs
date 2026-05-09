use flow_types::Pipeline;
use kube::CustomResourceExt;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.contains(&"--print-crd".to_string()) {
        let crd = Pipeline::crd();

        let yaml = serde_yaml::to_string(&crd).expect("Failed to serialize CRD");

        println!("{}", yaml);
    } else {
        println!("Oxidized-Flow Command Line Interface");
        println!("Available commands:");
        println!("  --print-crd    Generates and prints the Kubernetes CRD YAML");
    }
}
