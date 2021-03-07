//! Evaluation metrics for multi-class classification/classifier models.
use crate::model::aggregate_classification_metrics::AggregateClassificationMetrics;
use crate::model::confusion_matrix::ConfusionMatrix;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MultiClassClassificationMetrics {
    /// Confusion matrix at different thresholds.
    pub confusion_matrix_list: Option<Vec<ConfusionMatrix>>,
    /// Aggregate classification metrics.
    pub aggregate_classification_metrics: Option<AggregateClassificationMetrics>,
}
