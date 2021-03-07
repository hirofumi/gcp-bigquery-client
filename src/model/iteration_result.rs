//! Information about a single iteration of the training run.
use crate::model::arima_result::ArimaResult;
use crate::model::cluster_info::ClusterInfo;
use crate::model::principal_component_info::PrincipalComponentInfo;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IterationResult {
    pub arima_result: Option<ArimaResult>,
    /// Time taken to run the iteration in milliseconds.
    pub duration_ms: Option<i64>,
    /// Index of the iteration, 0 based.
    pub index: Option<i32>,
    /// Loss computed on the training data at the end of iteration.
    pub training_loss: Option<f64>,
    /// Learn rate used for this iteration.
    pub learn_rate: Option<f64>,
    /// Loss computed on the eval data at the end of iteration.
    pub eval_loss: Option<f64>,
    /// The information of the principal components.
    pub principal_component_infos: Option<Vec<PrincipalComponentInfo>>,
    /// Information about top clusters for clustering models.
    pub cluster_infos: Option<Vec<ClusterInfo>>,
}
