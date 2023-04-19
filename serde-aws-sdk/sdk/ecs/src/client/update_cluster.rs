// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`UpdateCluster`](crate::operation::update_cluster::builders::UpdateClusterFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`cluster(impl Into<String>)`](crate::operation::update_cluster::builders::UpdateClusterFluentBuilder::cluster) / [`set_cluster(Option<String>)`](crate::operation::update_cluster::builders::UpdateClusterFluentBuilder::set_cluster): <p>The name of the cluster to modify the settings for.</p>
    ///   - [`settings(Vec<ClusterSetting>)`](crate::operation::update_cluster::builders::UpdateClusterFluentBuilder::settings) / [`set_settings(Option<Vec<ClusterSetting>>)`](crate::operation::update_cluster::builders::UpdateClusterFluentBuilder::set_settings): <p>The cluster settings for your cluster.</p>
    ///   - [`configuration(ClusterConfiguration)`](crate::operation::update_cluster::builders::UpdateClusterFluentBuilder::configuration) / [`set_configuration(Option<ClusterConfiguration>)`](crate::operation::update_cluster::builders::UpdateClusterFluentBuilder::set_configuration): <p>The execute command configuration for the cluster.</p>
    ///   - [`service_connect_defaults(ClusterServiceConnectDefaultsRequest)`](crate::operation::update_cluster::builders::UpdateClusterFluentBuilder::service_connect_defaults) / [`set_service_connect_defaults(Option<ClusterServiceConnectDefaultsRequest>)`](crate::operation::update_cluster::builders::UpdateClusterFluentBuilder::set_service_connect_defaults): <p>Use this parameter to set a default Service Connect namespace. After you set a default Service Connect namespace, any new services with Service Connect turned on that are created in the cluster are added as client services in the namespace. This setting only applies to new services that set the <code>enabled</code> parameter to <code>true</code> in the <code>ServiceConnectConfiguration</code>. You can set the namespace of each service individually in the <code>ServiceConnectConfiguration</code> to override this default parameter.</p>  <p>Tasks that run in a namespace can use short names to connect to services in the namespace. Tasks can connect to services across all of the clusters in the namespace. Tasks connect through a managed proxy container that collects logs and metrics for increased visibility. Only the tasks that Amazon ECS services create are supported with Service Connect. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/service-connect.html">Service Connect</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p>
    /// - On success, responds with [`UpdateClusterOutput`](crate::operation::update_cluster::UpdateClusterOutput) with field(s):
    ///   - [`cluster(Option<Cluster>)`](crate::operation::update_cluster::UpdateClusterOutput::cluster): <p>Details about the cluster.</p>
    /// - On failure, responds with [`SdkError<UpdateClusterError>`](crate::operation::update_cluster::UpdateClusterError)
    pub fn update_cluster(
        &self,
    ) -> crate::operation::update_cluster::builders::UpdateClusterFluentBuilder {
        crate::operation::update_cluster::builders::UpdateClusterFluentBuilder::new(
            self.handle.clone(),
        )
    }
}