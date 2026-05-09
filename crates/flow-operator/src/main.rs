use flow_types::Pipeline;
use futures::StreamExt;
use kube::{
    api::Api,
    runtime::{controller::Action, watcher::Config, Controller},
    Client, ResourceExt,
};
use std::sync::Arc;
use tokio::time::Duration;

struct ContextData {
    client: Client,
}

async fn reconcile(pipeline: Arc<Pipeline>, _ctx: Arc<ContextData>) -> Result<Action, kube::Error> {
    let name = pipeline.name_any();
    let namespace = pipeline.namespace().unwrap_or_else(|| "default".to_string());

    println!("Detected Pipeline event: {}/{}", namespace, name);

    Ok(Action::requeue(Duration::from_secs(300)))
}

fn error_policy(pipeline: Arc<Pipeline>, error: &kube::Error, _ctx: Arc<ContextData>) -> Action {
    eprintln!(
        "Failed to reconcile Pipeline {}/{}: {:?}",
        pipeline.namespace().unwrap_or_default(),
        pipeline.name_any(),
        error
    );
    Action::requeue(Duration::from_secs(15))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting Oxidized-Flow Operator...");

    let client = Client::try_default().await?;
    println!("Connected to Kubernetes API");

    let pipelines: Api<Pipeline> = Api::all(client.clone());

    let context = Arc::new(ContextData { client });

    // 4. Start the Controller loop
    println!("Listening for new Machine Learning Pipelines...");
    Controller::new(pipelines, Config::default())
        .run(reconcile, error_policy, context)
        .for_each(|res| async move {
            match res {
                Ok(o) => println!("Reconciled {:?}", o),
                Err(e) => eprintln!("Reconcile failed: {:?}", e),
            }
        })
        .await;

    Ok(())
}
