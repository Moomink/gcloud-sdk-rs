/// Describes a Compute Engine resource that is being managed by a running
/// \[pipeline][google.genomics.v1alpha2.Pipeline\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ComputeEngine {
    /// The instance on which the operation is running.
    #[prost(string, tag = "1")]
    pub instance_name: ::prost::alloc::string::String,
    /// The availability zone in which the instance resides.
    #[prost(string, tag = "2")]
    pub zone: ::prost::alloc::string::String,
    /// The machine type of the instance.
    #[prost(string, tag = "3")]
    pub machine_type: ::prost::alloc::string::String,
    /// The names of the disks that were created for this pipeline.
    #[prost(string, repeated, tag = "4")]
    pub disk_names: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Runtime metadata that will be populated in the
/// \[runtimeMetadata][google.genomics.v1.OperationMetadata.runtime_metadata\]
/// field of the Operation associated with a RunPipeline execution.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RuntimeMetadata {
    /// Execution information specific to Google Compute Engine.
    #[prost(message, optional, tag = "1")]
    pub compute_engine: ::core::option::Option<ComputeEngine>,
}
/// The pipeline object. Represents a transformation from a set of input
/// parameters to a set of output parameters. The transformation is defined
/// as a docker image and command to run within that image. Each pipeline
/// is run on a Google Compute Engine VM. A pipeline can be created with the
/// `create` method and then later run with the `run` method, or a pipeline can
/// be defined and run all at once with the `run` method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Pipeline {
    /// Required. The project in which to create the pipeline. The caller must have
    /// WRITE access.
    #[prost(string, tag = "1")]
    pub project_id: ::prost::alloc::string::String,
    /// Required. A user specified pipeline name that does not have to be unique.
    /// This name can be used for filtering Pipelines in ListPipelines.
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    /// User-specified description.
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
    /// Input parameters of the pipeline.
    #[prost(message, repeated, tag = "8")]
    pub input_parameters: ::prost::alloc::vec::Vec<PipelineParameter>,
    /// Output parameters of the pipeline.
    #[prost(message, repeated, tag = "9")]
    pub output_parameters: ::prost::alloc::vec::Vec<PipelineParameter>,
    /// Required. Specifies resource requirements for the pipeline run.
    /// Required fields:
    ///
    /// *
    /// \[minimumCpuCores][google.genomics.v1alpha2.PipelineResources.minimum_cpu_cores\]
    ///
    /// *
    /// \[minimumRamGb][google.genomics.v1alpha2.PipelineResources.minimum_ram_gb\]
    #[prost(message, optional, tag = "6")]
    pub resources: ::core::option::Option<PipelineResources>,
    /// Unique pipeline id that is generated by the service when CreatePipeline
    /// is called. Cannot be specified in the Pipeline used in the
    /// CreatePipelineRequest, and will be populated in the response to
    /// CreatePipeline and all subsequent Get and List calls. Indicates that the
    /// service has registered this pipeline.
    #[prost(string, tag = "7")]
    pub pipeline_id: ::prost::alloc::string::String,
    /// Required. The executor indicates in which environment the pipeline runs.
    #[prost(oneof = "pipeline::Executor", tags = "5")]
    pub executor: ::core::option::Option<pipeline::Executor>,
}
/// Nested message and enum types in `Pipeline`.
pub mod pipeline {
    /// Required. The executor indicates in which environment the pipeline runs.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Executor {
        /// Specifies the docker run information.
        #[prost(message, tag = "5")]
        Docker(super::DockerExecutor),
    }
}
/// The request to create a pipeline. The pipeline field here should not have
/// `pipelineId` populated, as that will be populated by the server.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreatePipelineRequest {
    /// The pipeline to create. Should not have `pipelineId` populated.
    #[prost(message, optional, tag = "1")]
    pub pipeline: ::core::option::Option<Pipeline>,
}
/// The pipeline run arguments.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RunPipelineArgs {
    /// Required. The project in which to run the pipeline. The caller must have
    /// WRITER access to all Google Cloud services and resources (e.g. Google
    /// Compute Engine) will be used.
    #[prost(string, tag = "1")]
    pub project_id: ::prost::alloc::string::String,
    /// Pipeline input arguments; keys are defined in the pipeline documentation.
    /// All input parameters that do not have default values  must be specified.
    /// If parameters with defaults are specified here, the defaults will be
    /// overridden.
    #[prost(map = "string, string", tag = "2")]
    pub inputs: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Pipeline output arguments; keys are defined in the pipeline
    /// documentation.  All output parameters of without default values
    /// must be specified.  If parameters with defaults are specified
    /// here, the defaults will be overridden.
    #[prost(map = "string, string", tag = "3")]
    pub outputs: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// The Google Cloud Service Account that will be used to access data and
    /// services. By default, the compute service account associated with
    /// `projectId` is used.
    #[prost(message, optional, tag = "4")]
    pub service_account: ::core::option::Option<ServiceAccount>,
    /// This field is deprecated. Use `labels` instead. Client-specified pipeline
    /// operation identifier.
    #[prost(string, tag = "5")]
    pub client_id: ::prost::alloc::string::String,
    /// Specifies resource requirements/overrides for the pipeline run.
    #[prost(message, optional, tag = "6")]
    pub resources: ::core::option::Option<PipelineResources>,
    /// Required. Logging options. Used by the service to communicate results
    /// to the user.
    #[prost(message, optional, tag = "7")]
    pub logging: ::core::option::Option<LoggingOptions>,
    /// How long to keep the VM up after a failure (for example docker command
    /// failed, copying input or output files failed, etc). While the VM is up, one
    /// can ssh into the VM to debug. Default is 0; maximum allowed value is 1 day.
    #[prost(message, optional, tag = "8")]
    pub keep_vm_alive_on_failure_duration: ::core::option::Option<
        ::prost_types::Duration,
    >,
    /// Labels to apply to this pipeline run. Labels will also be applied to
    /// compute resources (VM, disks) created by this pipeline run. When listing
    /// operations, operations can [filtered by labels]
    /// \[google.longrunning.ListOperationsRequest.filter\].
    /// Label keys may not be empty; label values may be empty. Non-empty labels
    /// must be 1-63 characters long, and comply with \[RFC1035\]
    /// (<https://www.ietf.org/rfc/rfc1035.txt>).
    /// Specifically, the name must be 1-63 characters long and match the regular
    /// expression `\[a-z]([-a-z0-9]*[a-z0-9\])?` which means the first
    /// character must be a lowercase letter, and all following characters must be
    /// a dash, lowercase letter, or digit, except the last character, which cannot
    /// be a dash.
    #[prost(map = "string, string", tag = "9")]
    pub labels: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
}
/// The request to run a pipeline. If `pipelineId` is specified, it
/// refers to a saved pipeline created with CreatePipeline and set as
/// the `pipelineId` of the returned Pipeline object. If
/// `ephemeralPipeline` is specified, that pipeline is run once
/// with the given args and not saved. It is an error to specify both
/// `pipelineId` and `ephemeralPipeline`. `pipelineArgs`
/// must be specified.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RunPipelineRequest {
    /// The arguments to use when running this pipeline.
    #[prost(message, optional, tag = "3")]
    pub pipeline_args: ::core::option::Option<RunPipelineArgs>,
    #[prost(oneof = "run_pipeline_request::Pipeline", tags = "1, 2")]
    pub pipeline: ::core::option::Option<run_pipeline_request::Pipeline>,
}
/// Nested message and enum types in `RunPipelineRequest`.
pub mod run_pipeline_request {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Pipeline {
        /// The already created pipeline to run.
        #[prost(string, tag = "1")]
        PipelineId(::prost::alloc::string::String),
        /// A new pipeline object to run once and then delete.
        #[prost(message, tag = "2")]
        EphemeralPipeline(super::Pipeline),
    }
}
/// A request to get a saved pipeline by id.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPipelineRequest {
    /// Caller must have READ access to the project in which this pipeline
    /// is defined.
    #[prost(string, tag = "1")]
    pub pipeline_id: ::prost::alloc::string::String,
}
/// A request to list pipelines in a given project. Pipelines can be
/// filtered by name using `namePrefix`: all pipelines with names that
/// begin with `namePrefix` will be returned. Uses standard pagination:
/// `pageSize` indicates how many pipelines to return, and
/// `pageToken` comes from a previous ListPipelinesResponse to
/// indicate offset.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPipelinesRequest {
    /// Required. The name of the project to search for pipelines. Caller
    /// must have READ access to this project.
    #[prost(string, tag = "1")]
    pub project_id: ::prost::alloc::string::String,
    /// Pipelines with names that match this prefix should be
    /// returned.  If unspecified, all pipelines in the project, up to
    /// `pageSize`, will be returned.
    #[prost(string, tag = "2")]
    pub name_prefix: ::prost::alloc::string::String,
    /// Number of pipelines to return at once. Defaults to 256, and max
    /// is 2048.
    #[prost(int32, tag = "3")]
    pub page_size: i32,
    /// Token to use to indicate where to start getting results.
    /// If unspecified, returns the first page of results.
    #[prost(string, tag = "4")]
    pub page_token: ::prost::alloc::string::String,
}
/// The response of ListPipelines. Contains at most `pageSize`
/// pipelines. If it contains `pageSize` pipelines, and more pipelines
/// exist, then `nextPageToken` will be populated and should be
/// used as the `pageToken` argument to a subsequent ListPipelines
/// request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPipelinesResponse {
    /// The matched pipelines.
    #[prost(message, repeated, tag = "1")]
    pub pipelines: ::prost::alloc::vec::Vec<Pipeline>,
    /// The token to use to get the next page of results.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// The request to delete a saved pipeline by ID.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeletePipelineRequest {
    /// Caller must have WRITE access to the project in which this pipeline
    /// is defined.
    #[prost(string, tag = "1")]
    pub pipeline_id: ::prost::alloc::string::String,
}
/// Request to get controller configuation.  Should only be used
/// by VMs created by the Pipelines Service and not by end users.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetControllerConfigRequest {
    /// The operation to retrieve controller configuration for.
    #[prost(string, tag = "1")]
    pub operation_id: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub validation_token: u64,
}
/// Stores the information that the controller will fetch from the
/// server in order to run. Should only be used by VMs created by the
/// Pipelines Service and not by end users.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ControllerConfig {
    #[prost(string, tag = "1")]
    pub image: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub cmd: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub gcs_log_path: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub machine_type: ::prost::alloc::string::String,
    #[prost(map = "string, string", tag = "5")]
    pub vars: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    #[prost(map = "string, string", tag = "6")]
    pub disks: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    #[prost(map = "string, message", tag = "7")]
    pub gcs_sources: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        controller_config::RepeatedString,
    >,
    #[prost(map = "string, message", tag = "8")]
    pub gcs_sinks: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        controller_config::RepeatedString,
    >,
}
/// Nested message and enum types in `ControllerConfig`.
pub mod controller_config {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct RepeatedString {
        #[prost(string, repeated, tag = "1")]
        pub values: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    }
}
/// Stores the list of events and times they occured for major events in job
/// execution.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TimestampEvent {
    /// String indicating the type of event
    #[prost(string, tag = "1")]
    pub description: ::prost::alloc::string::String,
    /// The time this event occured.
    #[prost(message, optional, tag = "2")]
    pub timestamp: ::core::option::Option<::prost_types::Timestamp>,
}
/// Request to set operation status. Should only be used by VMs
/// created by the Pipelines Service and not by end users.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetOperationStatusRequest {
    #[prost(string, tag = "1")]
    pub operation_id: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub timestamp_events: ::prost::alloc::vec::Vec<TimestampEvent>,
    #[prost(enumeration = "super::super::rpc::Code", tag = "3")]
    pub error_code: i32,
    #[prost(string, tag = "4")]
    pub error_message: ::prost::alloc::string::String,
    #[prost(uint64, tag = "5")]
    pub validation_token: u64,
}
/// A Google Cloud Service Account.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServiceAccount {
    /// Email address of the service account. Defaults to `default`,
    /// which uses the compute service account associated with the project.
    #[prost(string, tag = "1")]
    pub email: ::prost::alloc::string::String,
    /// List of scopes to be enabled for this service account on the VM.
    /// The following scopes are automatically included:
    ///
    /// * <https://www.googleapis.com/auth/compute>
    /// * <https://www.googleapis.com/auth/devstorage.full_control>
    /// * <https://www.googleapis.com/auth/genomics>
    /// * <https://www.googleapis.com/auth/logging.write>
    /// * <https://www.googleapis.com/auth/monitoring.write>
    #[prost(string, repeated, tag = "2")]
    pub scopes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// The logging options for the pipeline run.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LoggingOptions {
    /// The location in Google Cloud Storage to which the pipeline logs
    /// will be copied. Can be specified as a fully qualified directory
    /// path, in which case logs will be output with a unique identifier
    /// as the filename in that directory, or as a fully specified path,
    /// which must end in `.log`, in which case that path will be
    /// used, and the user must ensure that logs are not
    /// overwritten. Stdout and stderr logs from the run are also
    /// generated and output as `-stdout.log` and `-stderr.log`.
    #[prost(string, tag = "1")]
    pub gcs_path: ::prost::alloc::string::String,
}
/// The system resources for the pipeline run.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PipelineResources {
    /// The minimum number of cores to use. Defaults to 1.
    #[prost(int32, tag = "1")]
    pub minimum_cpu_cores: i32,
    /// Whether to use preemptible VMs. Defaults to `false`. In order to use this,
    /// must be true for both create time and run time. Cannot be true at run time
    /// if false at create time.
    #[prost(bool, tag = "2")]
    pub preemptible: bool,
    /// The minimum amount of RAM to use. Defaults to 3.75 (GB)
    #[prost(double, tag = "3")]
    pub minimum_ram_gb: f64,
    /// Disks to attach.
    #[prost(message, repeated, tag = "4")]
    pub disks: ::prost::alloc::vec::Vec<pipeline_resources::Disk>,
    /// List of Google Compute Engine availability zones to which resource
    /// creation will restricted. If empty, any zone may be chosen.
    #[prost(string, repeated, tag = "5")]
    pub zones: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The size of the boot disk. Defaults to 10 (GB).
    #[prost(int32, tag = "6")]
    pub boot_disk_size_gb: i32,
    /// Whether to assign an external IP to the instance. This is an experimental
    /// feature that may go away. Defaults to false.
    /// Corresponds to `--no_address` flag for [gcloud compute instances create]
    /// (<https://cloud.google.com/sdk/gcloud/reference/compute/instances/create>).
    /// In order to use this, must be true for both create time and run time.
    /// Cannot be true at run time if false at create time. If you need to ssh into
    /// a private IP VM for debugging, you can ssh to a public VM and then ssh into
    /// the private VM's Internal IP.  If noAddress is set, this pipeline run may
    /// only load docker images from Google Container Registry and not Docker Hub.
    /// ** Note: To use this option, your project must be in Google Access for
    /// Private IPs Early Access Program.**
    #[prost(bool, tag = "7")]
    pub no_address: bool,
}
/// Nested message and enum types in `PipelineResources`.
pub mod pipeline_resources {
    /// A Google Compute Engine disk resource specification.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Disk {
        /// Required. The name of the disk that can be used in the pipeline
        /// parameters. Must be 1 - 63 characters.
        /// The name "boot" is reserved for system use.
        #[prost(string, tag = "1")]
        pub name: ::prost::alloc::string::String,
        /// Required. The type of the disk to create.
        #[prost(enumeration = "disk::Type", tag = "2")]
        pub r#type: i32,
        /// The size of the disk. Defaults to 500 (GB).
        /// This field is not applicable for local SSD.
        #[prost(int32, tag = "3")]
        pub size_gb: i32,
        /// The full or partial URL of the persistent disk to attach. See
        /// <https://cloud.google.com/compute/docs/reference/latest/instances#resource>
        /// and
        /// <https://cloud.google.com/compute/docs/disks/persistent-disks#snapshots>
        /// for more details.
        #[prost(string, tag = "4")]
        pub source: ::prost::alloc::string::String,
        /// Deprecated. Disks created by the Pipelines API will be deleted at the end
        /// of the pipeline run, regardless of what this field is set to.
        #[prost(bool, tag = "6")]
        pub auto_delete: bool,
        /// Required at create time and cannot be overridden at run time.
        /// Specifies the path in the docker container where files on
        /// this disk should be located. For example, if `mountPoint`
        /// is `/mnt/disk`, and the parameter has `localPath`
        /// `inputs/file.txt`, the docker container can access the data at
        /// `/mnt/disk/inputs/file.txt`.
        #[prost(string, tag = "8")]
        pub mount_point: ::prost::alloc::string::String,
    }
    /// Nested message and enum types in `Disk`.
    pub mod disk {
        /// The types of disks that may be attached to VMs.
        #[derive(
            Clone,
            Copy,
            Debug,
            PartialEq,
            Eq,
            Hash,
            PartialOrd,
            Ord,
            ::prost::Enumeration
        )]
        #[repr(i32)]
        pub enum Type {
            /// Default disk type. Use one of the other options below.
            Unspecified = 0,
            /// Specifies a Google Compute Engine persistent hard disk. See
            /// <https://cloud.google.com/compute/docs/disks/#pdspecs> for details.
            PersistentHdd = 1,
            /// Specifies a Google Compute Engine persistent solid-state disk. See
            /// <https://cloud.google.com/compute/docs/disks/#pdspecs> for details.
            PersistentSsd = 2,
            /// Specifies a Google Compute Engine local SSD.
            /// See <https://cloud.google.com/compute/docs/disks/local-ssd> for details.
            LocalSsd = 3,
        }
        impl Type {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    Type::Unspecified => "TYPE_UNSPECIFIED",
                    Type::PersistentHdd => "PERSISTENT_HDD",
                    Type::PersistentSsd => "PERSISTENT_SSD",
                    Type::LocalSsd => "LOCAL_SSD",
                }
            }
        }
    }
}
/// Parameters facilitate setting and delivering data into the
/// pipeline's execution environment. They are defined at create time,
/// with optional defaults, and can be overridden at run time.
///
/// If `localCopy` is unset, then the parameter specifies a string that
/// is passed as-is into the pipeline, as the value of the environment
/// variable with the given name.  A default value can be optionally
/// specified at create time. The default can be overridden at run time
/// using the inputs map. If no default is given, a value must be
/// supplied at runtime.
///
/// If `localCopy` is defined, then the parameter specifies a data
/// source or sink, both in Google Cloud Storage and on the Docker container
/// where the pipeline computation is run. The [service account associated with
/// the Pipeline]\[google.genomics.v1alpha2.RunPipelineArgs.service_account\] (by
/// default the project's Compute Engine service account) must have access to the
/// Google Cloud Storage paths.
///
/// At run time, the Google Cloud Storage paths can be overridden if a default
/// was provided at create time, or must be set otherwise. The pipeline runner
/// should add a key/value pair to either the inputs or outputs map. The
/// indicated data copies will be carried out before/after pipeline execution,
/// just as if the corresponding arguments were provided to `gsutil cp`.
///
/// For example: Given the following `PipelineParameter`, specified
/// in the `inputParameters` list:
///
/// ```
/// {name: "input_file", localCopy: {path: "file.txt", disk: "pd1"}}
/// ```
///
/// where `disk` is defined in the `PipelineResources` object as:
///
/// ```
/// {name: "pd1", mountPoint: "/mnt/disk/"}
/// ```
///
/// We create a disk named `pd1`, mount it on the host VM, and map
/// `/mnt/pd1` to `/mnt/disk` in the docker container.  At
/// runtime, an entry for `input_file` would be required in the inputs
/// map, such as:
///
/// ```
///    inputs\["input_file"\] = "gs://my-bucket/bar.txt"
/// ```
///
/// This would generate the following gsutil call:
///
/// ```
///    gsutil cp gs://my-bucket/bar.txt /mnt/pd1/file.txt
/// ```
///
/// The file `/mnt/pd1/file.txt` maps to `/mnt/disk/file.txt` in the
/// Docker container. Acceptable paths are:
///
/// <table>
///    <thead>
///      <tr><th>Google Cloud storage path</th><th>Local path</th></tr>
///    </thead>
///    <tbody>
///      <tr><td>file</td><td>file</td></tr>
///      <tr><td>glob</td><td>directory</td></tr>
///    </tbody>
/// </table>
///
/// For outputs, the direction of the copy is reversed:
///
/// ```
///    gsutil cp /mnt/disk/file.txt gs://my-bucket/bar.txt
/// ```
///
/// Acceptable paths are:
///
/// <table>
///    <thead>
///      <tr><th>Local path</th><th>Google Cloud Storage path</th></tr>
///    </thead>
///    <tbody>
///      <tr><td>file</td><td>file</td></tr>
///      <tr>
///        <td>file</td>
///        <td>directory - directory must already exist</td>
///      </tr>
///      <tr>
///        <td>glob</td>
///        <td>directory - directory will be created if it doesn't exist</td></tr>
///    </tbody>
/// </table>
///
/// One restriction due to docker limitations, is that for outputs that are found
/// on the boot disk, the local path cannot be a glob and must be a file.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PipelineParameter {
    /// Required. Name of the parameter - the pipeline runner uses this string
    /// as the key to the input and output maps in RunPipeline.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Human-readable description.
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    /// The default value for this parameter. Can be overridden at runtime.
    /// If `localCopy` is present, then this must be a Google Cloud Storage path
    /// beginning with `gs://`.
    #[prost(string, tag = "5")]
    pub default_value: ::prost::alloc::string::String,
    /// If present, this parameter is marked for copying to and from the VM.
    /// `LocalCopy` indicates where on the VM the file should be. The value
    /// given to this parameter (either at runtime or using `defaultValue`)
    /// must be the remote path where the file should be.
    #[prost(message, optional, tag = "6")]
    pub local_copy: ::core::option::Option<pipeline_parameter::LocalCopy>,
}
/// Nested message and enum types in `PipelineParameter`.
pub mod pipeline_parameter {
    /// LocalCopy defines how a remote file should be copied to and from the VM.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct LocalCopy {
        /// Required. The path within the user's docker container where
        /// this input should be localized to and from, relative to the specified
        /// disk's mount point. For example: file.txt,
        #[prost(string, tag = "1")]
        pub path: ::prost::alloc::string::String,
        /// Required. The name of the disk where this parameter is
        /// located. Can be the name of one of the disks specified in the
        /// Resources field, or "boot", which represents the Docker
        /// instance's boot disk and has a mount point of `/`.
        #[prost(string, tag = "2")]
        pub disk: ::prost::alloc::string::String,
    }
}
/// The Docker execuctor specification.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DockerExecutor {
    /// Required. Image name from either Docker Hub or Google Container Registry.
    /// Users that run pipelines must have READ access to the image.
    #[prost(string, tag = "1")]
    pub image_name: ::prost::alloc::string::String,
    /// Required. The command or newline delimited script to run. The command
    /// string will be executed within a bash shell.
    ///
    /// If the command exits with a non-zero exit code, output parameter
    /// de-localization will be skipped and the pipeline operation's
    /// \[`error`][google.longrunning.Operation.error\] field will be populated.
    ///
    /// Maximum command string length is 16384.
    #[prost(string, tag = "2")]
    pub cmd: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod pipelines_v1_alpha2_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// A service for running genomics pipelines.
    #[derive(Debug, Clone)]
    pub struct PipelinesV1Alpha2Client<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl PipelinesV1Alpha2Client<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> PipelinesV1Alpha2Client<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> PipelinesV1Alpha2Client<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            PipelinesV1Alpha2Client::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Creates a pipeline that can be run later. Create takes a Pipeline that
        /// has all fields other than `pipelineId` populated, and then returns
        /// the same pipeline with `pipelineId` populated. This id can be used
        /// to run the pipeline.
        ///
        /// Caller must have WRITE permission to the project.
        pub async fn create_pipeline(
            &mut self,
            request: impl tonic::IntoRequest<super::CreatePipelineRequest>,
        ) -> Result<tonic::Response<super::Pipeline>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.genomics.v1alpha2.PipelinesV1Alpha2/CreatePipeline",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Runs a pipeline. If `pipelineId` is specified in the request, then
        /// run a saved pipeline. If `ephemeralPipeline` is specified, then run
        /// that pipeline once without saving a copy.
        ///
        /// The caller must have READ permission to the project where the pipeline
        /// is stored and WRITE permission to the project where the pipeline will be
        /// run, as VMs will be created and storage will be used.
        pub async fn run_pipeline(
            &mut self,
            request: impl tonic::IntoRequest<super::RunPipelineRequest>,
        ) -> Result<
            tonic::Response<super::super::super::longrunning::Operation>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.genomics.v1alpha2.PipelinesV1Alpha2/RunPipeline",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Retrieves a pipeline based on ID.
        ///
        /// Caller must have READ permission to the project.
        pub async fn get_pipeline(
            &mut self,
            request: impl tonic::IntoRequest<super::GetPipelineRequest>,
        ) -> Result<tonic::Response<super::Pipeline>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.genomics.v1alpha2.PipelinesV1Alpha2/GetPipeline",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Lists pipelines.
        ///
        /// Caller must have READ permission to the project.
        pub async fn list_pipelines(
            &mut self,
            request: impl tonic::IntoRequest<super::ListPipelinesRequest>,
        ) -> Result<tonic::Response<super::ListPipelinesResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.genomics.v1alpha2.PipelinesV1Alpha2/ListPipelines",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Deletes a pipeline based on ID.
        ///
        /// Caller must have WRITE permission to the project.
        pub async fn delete_pipeline(
            &mut self,
            request: impl tonic::IntoRequest<super::DeletePipelineRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.genomics.v1alpha2.PipelinesV1Alpha2/DeletePipeline",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Gets controller configuration information. Should only be called
        /// by VMs created by the Pipelines Service and not by end users.
        pub async fn get_controller_config(
            &mut self,
            request: impl tonic::IntoRequest<super::GetControllerConfigRequest>,
        ) -> Result<tonic::Response<super::ControllerConfig>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.genomics.v1alpha2.PipelinesV1Alpha2/GetControllerConfig",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Sets status of a given operation. Any new timestamps (as determined by
        /// description) are appended to TimestampEvents. Should only be called by VMs
        /// created by the Pipelines Service and not by end users.
        pub async fn set_operation_status(
            &mut self,
            request: impl tonic::IntoRequest<super::SetOperationStatusRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.genomics.v1alpha2.PipelinesV1Alpha2/SetOperationStatus",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
