use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(CustomResource, Deserialize, Serialize, Clone, Debug, JsonSchema)]
#[kube(
    group = "rustflow.org",
    version = "v1",
    kind = "Pipeline",
    status = "PipelineStatus",
    namespaced
)]
pub struct PipelineSpec {
    pub steps: Vec<Step>,
}

#[derive(Deserialize, Serialize, Clone, Debug, JsonSchema)]
pub struct Step {
    pub name: String,
    pub image: String,
}

#[derive(Deserialize, Serialize, Clone, Debug, Default, JsonSchema)]
pub struct PipelineStatus {
    pub state: String,
    pub current_step: Option<String>,
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_hello_world() {
        println!("Hello, CI!");
        assert_eq!(2 + 2, 4);
    }
}
