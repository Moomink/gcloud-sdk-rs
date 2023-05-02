/// Pull target.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PullTarget {}
/// The pull message contains data that can be used by the caller of
/// \[LeaseTasks][google.cloud.tasks.v2beta2.CloudTasks.LeaseTasks\] to process the
/// task.
///
/// This proto can only be used for tasks in a queue which has
/// \[pull_target][google.cloud.tasks.v2beta2.Queue.pull_target\] set.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PullMessage {
    /// A data payload consumed by the worker to execute the task.
    #[prost(bytes = "vec", tag = "1")]
    pub payload: ::prost::alloc::vec::Vec<u8>,
    /// The task's tag.
    ///
    /// Tags allow similar tasks to be processed in a batch. If you label
    /// tasks with a tag, your worker can
    /// [lease tasks]\[google.cloud.tasks.v2beta2.CloudTasks.LeaseTasks\] with the
    /// same tag using
    /// \[filter][google.cloud.tasks.v2beta2.LeaseTasksRequest.filter\]. For example,
    /// if you want to aggregate the events associated with a specific user once a
    /// day, you could tag tasks with the user ID.
    ///
    /// The task's tag can only be set when the
    /// [task is created]\[google.cloud.tasks.v2beta2.CloudTasks.CreateTask\].
    ///
    /// The tag must be less than 500 characters.
    ///
    /// SDK compatibility: Although the SDK allows tags to be either
    /// string or
    /// \[bytes\](<https://cloud.google.com/appengine/docs/standard/java/javadoc/com/google/appengine/api/taskqueue/TaskOptions.html#tag-byte:A->),
    /// only UTF-8 encoded tags can be used in Cloud Tasks. If a tag isn't UTF-8
    /// encoded, the tag will be empty when the task is returned by Cloud Tasks.
    #[prost(string, tag = "2")]
    pub tag: ::prost::alloc::string::String,
}
/// App Engine HTTP target.
///
/// The task will be delivered to the App Engine application hostname
/// specified by its
/// \[AppEngineHttpTarget][google.cloud.tasks.v2beta2.AppEngineHttpTarget\] and
/// \[AppEngineHttpRequest][google.cloud.tasks.v2beta2.AppEngineHttpRequest\]. The
/// documentation for
/// \[AppEngineHttpRequest][google.cloud.tasks.v2beta2.AppEngineHttpRequest\]
/// explains how the task's host URL is constructed.
///
/// Using \[AppEngineHttpTarget][google.cloud.tasks.v2beta2.AppEngineHttpTarget\]
/// requires
/// \[`appengine.applications.get`\](<https://cloud.google.com/appengine/docs/admin-api/access-control>)
/// Google IAM permission for the project
/// and the following scope:
///
/// `<https://www.googleapis.com/auth/cloud-platform`>
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AppEngineHttpTarget {
    /// Overrides for the
    /// [task-level
    /// app_engine_routing]\[google.cloud.tasks.v2beta2.AppEngineHttpRequest.app_engine_routing\].
    ///
    /// If set, `app_engine_routing_override` is used for all tasks in
    /// the queue, no matter what the setting is for the
    /// [task-level
    /// app_engine_routing]\[google.cloud.tasks.v2beta2.AppEngineHttpRequest.app_engine_routing\].
    #[prost(message, optional, tag = "1")]
    pub app_engine_routing_override: ::core::option::Option<AppEngineRouting>,
}
/// App Engine HTTP request.
///
/// The message defines the HTTP request that is sent to an App Engine app when
/// the task is dispatched.
///
/// This proto can only be used for tasks in a queue which has
/// \[app_engine_http_target][google.cloud.tasks.v2beta2.Queue.app_engine_http_target\]
/// set.
///
/// Using \[AppEngineHttpRequest][google.cloud.tasks.v2beta2.AppEngineHttpRequest\]
/// requires
/// \[`appengine.applications.get`\](<https://cloud.google.com/appengine/docs/admin-api/access-control>)
/// Google IAM permission for the project
/// and the following scope:
///
/// `<https://www.googleapis.com/auth/cloud-platform`>
///
/// The task will be delivered to the App Engine app which belongs to the same
/// project as the queue. For more information, see
/// [How Requests are
/// Routed](<https://cloud.google.com/appengine/docs/standard/python/how-requests-are-routed>)
/// and how routing is affected by
/// [dispatch
/// files](<https://cloud.google.com/appengine/docs/python/config/dispatchref>).
/// Traffic is encrypted during transport and never leaves Google datacenters.
/// Because this traffic is carried over a communication mechanism internal to
/// Google, you cannot explicitly set the protocol (for example, HTTP or HTTPS).
/// The request to the handler, however, will appear to have used the HTTP
/// protocol.
///
/// The \[AppEngineRouting][google.cloud.tasks.v2beta2.AppEngineRouting\] used to
/// construct the URL that the task is delivered to can be set at the queue-level
/// or task-level:
///
/// * If set,
///    \[app_engine_routing_override][google.cloud.tasks.v2beta2.AppEngineHttpTarget.app_engine_routing_override\]
///    is used for all tasks in the queue, no matter what the setting
///    is for the
///    [task-level
///    app_engine_routing]\[google.cloud.tasks.v2beta2.AppEngineHttpRequest.app_engine_routing\].
///
///
/// The `url` that the task will be sent to is:
///
/// * `url =` \[host][google.cloud.tasks.v2beta2.AppEngineRouting.host\] `+`
///    \[relative_url][google.cloud.tasks.v2beta2.AppEngineHttpRequest.relative_url\]
///
/// Tasks can be dispatched to secure app handlers, unsecure app handlers, and
/// URIs restricted with
/// [`login:
/// admin`](<https://cloud.google.com/appengine/docs/standard/python/config/appref>).
/// Because tasks are not run as any user, they cannot be dispatched to URIs
/// restricted with
/// [`login:
/// required`](<https://cloud.google.com/appengine/docs/standard/python/config/appref>)
/// Task dispatches also do not follow redirects.
///
/// The task attempt has succeeded if the app's request handler returns an HTTP
/// response code in the range [`200` - `299`]. The task attempt has failed if
/// the app's handler returns a non-2xx response code or Cloud Tasks does
/// not receive response before the \[deadline][Task.dispatch_deadline\]. Failed
/// tasks will be retried according to the
/// [retry configuration]\[google.cloud.tasks.v2beta2.Queue.retry_config\]. `503`
/// (Service Unavailable) is considered an App Engine system error instead of an
/// application error and will cause Cloud Tasks' traffic congestion control to
/// temporarily throttle the queue's dispatches. Unlike other types of task
/// targets, a `429` (Too Many Requests) response from an app handler does not
/// cause traffic congestion control to throttle the queue.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AppEngineHttpRequest {
    /// The HTTP method to use for the request. The default is POST.
    ///
    /// The app's request handler for the task's target URL must be able to handle
    /// HTTP requests with this http_method, otherwise the task attempt fails with
    /// error code 405 (Method Not Allowed). See [Writing a push task request
    /// handler](<https://cloud.google.com/appengine/docs/java/taskqueue/push/creating-handlers#writing_a_push_task_request_handler>)
    /// and the App Engine documentation for your runtime on [How Requests are
    /// Handled](<https://cloud.google.com/appengine/docs/standard/python3/how-requests-are-handled>).
    #[prost(enumeration = "HttpMethod", tag = "1")]
    pub http_method: i32,
    /// Task-level setting for App Engine routing.
    ///
    /// If set,
    /// \[app_engine_routing_override][google.cloud.tasks.v2beta2.AppEngineHttpTarget.app_engine_routing_override\]
    /// is used for all tasks in the queue, no matter what the setting is for the
    /// [task-level
    /// app_engine_routing]\[google.cloud.tasks.v2beta2.AppEngineHttpRequest.app_engine_routing\].
    #[prost(message, optional, tag = "2")]
    pub app_engine_routing: ::core::option::Option<AppEngineRouting>,
    /// The relative URL.
    ///
    /// The relative URL must begin with "/" and must be a valid HTTP relative URL.
    /// It can contain a path and query string arguments.
    /// If the relative URL is empty, then the root path "/" will be used.
    /// No spaces are allowed, and the maximum length allowed is 2083 characters.
    #[prost(string, tag = "3")]
    pub relative_url: ::prost::alloc::string::String,
    /// HTTP request headers.
    ///
    /// This map contains the header field names and values.
    /// Headers can be set when the
    /// [task is created]\[google.cloud.tasks.v2beta2.CloudTasks.CreateTask\].
    /// Repeated headers are not supported but a header value can contain commas.
    ///
    /// Cloud Tasks sets some headers to default values:
    ///
    /// * `User-Agent`: By default, this header is
    ///    `"AppEngine-Google; (+<http://code.google.com/appengine>)"`.
    ///    This header can be modified, but Cloud Tasks will append
    ///    `"AppEngine-Google; (+<http://code.google.com/appengine>)"` to the
    ///    modified `User-Agent`.
    ///
    /// If the task has a
    /// \[payload][google.cloud.tasks.v2beta2.AppEngineHttpRequest.payload\], Cloud
    /// Tasks sets the following headers:
    ///
    /// * `Content-Type`: By default, the `Content-Type` header is set to
    ///    `"application/octet-stream"`. The default can be overridden by explicitly
    ///    setting `Content-Type` to a particular media type when the
    ///    [task is created]\[google.cloud.tasks.v2beta2.CloudTasks.CreateTask\].
    ///    For example, `Content-Type` can be set to `"application/json"`.
    /// * `Content-Length`: This is computed by Cloud Tasks. This value is
    ///    output only.   It cannot be changed.
    ///
    /// The headers below cannot be set or overridden:
    ///
    /// * `Host`
    /// * `X-Google-*`
    /// * `X-AppEngine-*`
    ///
    /// In addition, Cloud Tasks sets some headers when the task is dispatched,
    /// such as headers containing information about the task; see
    /// [request
    /// headers](<https://cloud.google.com/appengine/docs/python/taskqueue/push/creating-handlers#reading_request_headers>).
    /// These headers are set only when the task is dispatched, so they are not
    /// visible when the task is returned in a Cloud Tasks response.
    ///
    /// Although there is no specific limit for the maximum number of headers or
    /// the size, there is a limit on the maximum size of the
    /// \[Task][google.cloud.tasks.v2beta2.Task\]. For more information, see the
    /// \[CreateTask][google.cloud.tasks.v2beta2.CloudTasks.CreateTask\]
    /// documentation.
    #[prost(map = "string, string", tag = "4")]
    pub headers: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Payload.
    ///
    /// The payload will be sent as the HTTP message body. A message
    /// body, and thus a payload, is allowed only if the HTTP method is
    /// POST or PUT. It is an error to set a data payload on a task with
    /// an incompatible \[HttpMethod][google.cloud.tasks.v2beta2.HttpMethod\].
    #[prost(bytes = "vec", tag = "5")]
    pub payload: ::prost::alloc::vec::Vec<u8>,
}
/// App Engine Routing.
///
/// Defines routing characteristics specific to App Engine - service, version,
/// and instance.
///
/// For more information about services, versions, and instances see
/// [An Overview of App
/// Engine](<https://cloud.google.com/appengine/docs/python/an-overview-of-app-engine>),
/// [Microservices Architecture on Google App
/// Engine](<https://cloud.google.com/appengine/docs/python/microservices-on-app-engine>),
/// [App Engine Standard request
/// routing](<https://cloud.google.com/appengine/docs/standard/python/how-requests-are-routed>),
/// and [App Engine Flex request
/// routing](<https://cloud.google.com/appengine/docs/flexible/python/how-requests-are-routed>).
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AppEngineRouting {
    /// App service.
    ///
    /// By default, the task is sent to the service which is the default
    /// service when the task is attempted.
    ///
    /// For some queues or tasks which were created using the App Engine
    /// Task Queue API, \[host][google.cloud.tasks.v2beta2.AppEngineRouting.host\] is
    /// not parsable into
    /// \[service][google.cloud.tasks.v2beta2.AppEngineRouting.service\],
    /// \[version][google.cloud.tasks.v2beta2.AppEngineRouting.version\], and
    /// \[instance][google.cloud.tasks.v2beta2.AppEngineRouting.instance\]. For
    /// example, some tasks which were created using the App Engine SDK use a
    /// custom domain name; custom domains are not parsed by Cloud Tasks. If
    /// \[host][google.cloud.tasks.v2beta2.AppEngineRouting.host\] is not parsable,
    /// then \[service][google.cloud.tasks.v2beta2.AppEngineRouting.service\],
    /// \[version][google.cloud.tasks.v2beta2.AppEngineRouting.version\], and
    /// \[instance][google.cloud.tasks.v2beta2.AppEngineRouting.instance\] are the
    /// empty string.
    #[prost(string, tag = "1")]
    pub service: ::prost::alloc::string::String,
    /// App version.
    ///
    /// By default, the task is sent to the version which is the default
    /// version when the task is attempted.
    ///
    /// For some queues or tasks which were created using the App Engine
    /// Task Queue API, \[host][google.cloud.tasks.v2beta2.AppEngineRouting.host\] is
    /// not parsable into
    /// \[service][google.cloud.tasks.v2beta2.AppEngineRouting.service\],
    /// \[version][google.cloud.tasks.v2beta2.AppEngineRouting.version\], and
    /// \[instance][google.cloud.tasks.v2beta2.AppEngineRouting.instance\]. For
    /// example, some tasks which were created using the App Engine SDK use a
    /// custom domain name; custom domains are not parsed by Cloud Tasks. If
    /// \[host][google.cloud.tasks.v2beta2.AppEngineRouting.host\] is not parsable,
    /// then \[service][google.cloud.tasks.v2beta2.AppEngineRouting.service\],
    /// \[version][google.cloud.tasks.v2beta2.AppEngineRouting.version\], and
    /// \[instance][google.cloud.tasks.v2beta2.AppEngineRouting.instance\] are the
    /// empty string.
    #[prost(string, tag = "2")]
    pub version: ::prost::alloc::string::String,
    /// App instance.
    ///
    /// By default, the task is sent to an instance which is available when
    /// the task is attempted.
    ///
    /// Requests can only be sent to a specific instance if
    /// [manual scaling is used in App Engine
    /// Standard](<https://cloud.google.com/appengine/docs/python/an-overview-of-app-engine?hl=en_US#scaling_types_and_instance_classes>).
    /// App Engine Flex does not support instances. For more information, see
    /// [App Engine Standard request
    /// routing](<https://cloud.google.com/appengine/docs/standard/python/how-requests-are-routed>)
    /// and [App Engine Flex request
    /// routing](<https://cloud.google.com/appengine/docs/flexible/python/how-requests-are-routed>).
    #[prost(string, tag = "3")]
    pub instance: ::prost::alloc::string::String,
    /// Output only. The host that the task is sent to.
    ///
    /// For more information, see
    /// [How Requests are
    /// Routed](<https://cloud.google.com/appengine/docs/standard/python/how-requests-are-routed>).
    ///
    /// The host is constructed as:
    ///
    ///
    /// * `host = \[application_domain_name\]`</br>
    ///    `| \[service\] + '.' + \[application_domain_name\]`</br>
    ///    `| \[version\] + '.' + \[application_domain_name\]`</br>
    ///    `| \[version_dot_service\]+ '.' + \[application_domain_name\]`</br>
    ///    `| \[instance\] + '.' + \[application_domain_name\]`</br>
    ///    `| \[instance_dot_service\] + '.' + \[application_domain_name\]`</br>
    ///    `| \[instance_dot_version\] + '.' + \[application_domain_name\]`</br>
    ///    `| \[instance_dot_version_dot_service\] + '.' + \[application_domain_name\]`
    ///
    /// * `application_domain_name` = The domain name of the app, for
    ///    example <app-id>.appspot.com, which is associated with the
    ///    queue's project ID. Some tasks which were created using the App Engine
    ///    SDK use a custom domain name.
    ///
    /// * `service =`
    /// \[service][google.cloud.tasks.v2beta2.AppEngineRouting.service\]
    ///
    /// * `version =`
    /// \[version][google.cloud.tasks.v2beta2.AppEngineRouting.version\]
    ///
    /// * `version_dot_service =`
    ///    \[version][google.cloud.tasks.v2beta2.AppEngineRouting.version\] `+ '.' +`
    ///    \[service][google.cloud.tasks.v2beta2.AppEngineRouting.service\]
    ///
    /// * `instance =`
    /// \[instance][google.cloud.tasks.v2beta2.AppEngineRouting.instance\]
    ///
    /// * `instance_dot_service =`
    ///    \[instance][google.cloud.tasks.v2beta2.AppEngineRouting.instance\] `+ '.'
    ///    +` \[service][google.cloud.tasks.v2beta2.AppEngineRouting.service\]
    ///
    /// * `instance_dot_version =`
    ///    \[instance][google.cloud.tasks.v2beta2.AppEngineRouting.instance\] `+ '.'
    ///    +` \[version][google.cloud.tasks.v2beta2.AppEngineRouting.version\]
    ///
    /// * `instance_dot_version_dot_service =`
    ///    \[instance][google.cloud.tasks.v2beta2.AppEngineRouting.instance\] `+ '.'
    ///    +` \[version][google.cloud.tasks.v2beta2.AppEngineRouting.version\] `+ '.'
    ///    +` \[service][google.cloud.tasks.v2beta2.AppEngineRouting.service\]
    ///
    /// If \[service][google.cloud.tasks.v2beta2.AppEngineRouting.service\] is empty,
    /// then the task will be sent to the service which is the default service when
    /// the task is attempted.
    ///
    /// If \[version][google.cloud.tasks.v2beta2.AppEngineRouting.version\] is empty,
    /// then the task will be sent to the version which is the default version when
    /// the task is attempted.
    ///
    /// If \[instance][google.cloud.tasks.v2beta2.AppEngineRouting.instance\] is
    /// empty, then the task will be sent to an instance which is available when
    /// the task is attempted.
    ///
    /// If \[service][google.cloud.tasks.v2beta2.AppEngineRouting.service\],
    /// \[version][google.cloud.tasks.v2beta2.AppEngineRouting.version\], or
    /// \[instance][google.cloud.tasks.v2beta2.AppEngineRouting.instance\] is
    /// invalid, then the task will be sent to the default version of the default
    /// service when the task is attempted.
    #[prost(string, tag = "4")]
    pub host: ::prost::alloc::string::String,
}
/// The HTTP method used to execute the task.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum HttpMethod {
    /// HTTP method unspecified
    Unspecified = 0,
    /// HTTP POST
    Post = 1,
    /// HTTP GET
    Get = 2,
    /// HTTP HEAD
    Head = 3,
    /// HTTP PUT
    Put = 4,
    /// HTTP DELETE
    Delete = 5,
}
impl HttpMethod {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            HttpMethod::Unspecified => "HTTP_METHOD_UNSPECIFIED",
            HttpMethod::Post => "POST",
            HttpMethod::Get => "GET",
            HttpMethod::Head => "HEAD",
            HttpMethod::Put => "PUT",
            HttpMethod::Delete => "DELETE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "HTTP_METHOD_UNSPECIFIED" => Some(Self::Unspecified),
            "POST" => Some(Self::Post),
            "GET" => Some(Self::Get),
            "HEAD" => Some(Self::Head),
            "PUT" => Some(Self::Put),
            "DELETE" => Some(Self::Delete),
            _ => None,
        }
    }
}
/// A queue is a container of related tasks. Queues are configured to manage
/// how those tasks are dispatched. Configurable properties include rate limits,
/// retry options, target types, and others.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Queue {
    /// Caller-specified and required in
    /// \[CreateQueue][google.cloud.tasks.v2beta2.CloudTasks.CreateQueue\], after
    /// which it becomes output only.
    ///
    /// The queue name.
    ///
    /// The queue name must have the following format:
    /// `projects/PROJECT_ID/locations/LOCATION_ID/queues/QUEUE_ID`
    ///
    /// * `PROJECT_ID` can contain letters (\[A-Za-z\]), numbers (\[0-9\]),
    ///     hyphens (-), colons (:), or periods (.).
    ///     For more information, see
    ///     [Identifying
    ///     projects](<https://cloud.google.com/resource-manager/docs/creating-managing-projects#identifying_projects>)
    /// * `LOCATION_ID` is the canonical ID for the queue's location.
    ///     The list of available locations can be obtained by calling
    ///     \[ListLocations][google.cloud.location.Locations.ListLocations\].
    ///     For more information, see <https://cloud.google.com/about/locations/.>
    /// * `QUEUE_ID` can contain letters (\[A-Za-z\]), numbers (\[0-9\]), or
    ///    hyphens (-). The maximum length is 100 characters.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Rate limits for task dispatches.
    ///
    /// \[rate_limits][google.cloud.tasks.v2beta2.Queue.rate_limits\] and
    /// \[retry_config][google.cloud.tasks.v2beta2.Queue.retry_config\] are related
    /// because they both control task attempts however they control how tasks are
    /// attempted in different ways:
    ///
    /// * \[rate_limits][google.cloud.tasks.v2beta2.Queue.rate_limits\] controls the
    /// total rate of
    ///    dispatches from a queue (i.e. all traffic dispatched from the
    ///    queue, regardless of whether the dispatch is from a first
    ///    attempt or a retry).
    /// * \[retry_config][google.cloud.tasks.v2beta2.Queue.retry_config\] controls
    /// what happens to
    ///    particular a task after its first attempt fails. That is,
    ///    \[retry_config][google.cloud.tasks.v2beta2.Queue.retry_config\] controls
    ///    task retries (the second attempt, third attempt, etc).
    #[prost(message, optional, tag = "5")]
    pub rate_limits: ::core::option::Option<RateLimits>,
    /// Settings that determine the retry behavior.
    ///
    /// * For tasks created using Cloud Tasks: the queue-level retry settings
    ///    apply to all tasks in the queue that were created using Cloud Tasks.
    ///    Retry settings cannot be set on individual tasks.
    /// * For tasks created using the App Engine SDK: the queue-level retry
    ///    settings apply to all tasks in the queue which do not have retry settings
    ///    explicitly set on the task and were created by the App Engine SDK. See
    ///    [App Engine
    ///    documentation](<https://cloud.google.com/appengine/docs/standard/python/taskqueue/push/retrying-tasks>).
    #[prost(message, optional, tag = "6")]
    pub retry_config: ::core::option::Option<RetryConfig>,
    /// Output only. The state of the queue.
    ///
    /// `state` can only be changed by calling
    /// \[PauseQueue][google.cloud.tasks.v2beta2.CloudTasks.PauseQueue\],
    /// \[ResumeQueue][google.cloud.tasks.v2beta2.CloudTasks.ResumeQueue\], or
    /// uploading
    /// \[queue.yaml/xml\](<https://cloud.google.com/appengine/docs/python/config/queueref>).
    /// \[UpdateQueue][google.cloud.tasks.v2beta2.CloudTasks.UpdateQueue\] cannot be
    /// used to change `state`.
    #[prost(enumeration = "queue::State", tag = "7")]
    pub state: i32,
    /// Output only. The last time this queue was purged.
    ///
    /// All tasks that were \[created][google.cloud.tasks.v2beta2.Task.create_time\]
    /// before this time were purged.
    ///
    /// A queue can be purged using
    /// \[PurgeQueue][google.cloud.tasks.v2beta2.CloudTasks.PurgeQueue\], the [App
    /// Engine Task Queue SDK, or the Cloud
    /// Console](<https://cloud.google.com/appengine/docs/standard/python/taskqueue/push/deleting-tasks-and-queues#purging_all_tasks_from_a_queue>).
    ///
    /// Purge time will be truncated to the nearest microsecond. Purge
    /// time will be unset if the queue has never been purged.
    #[prost(message, optional, tag = "8")]
    pub purge_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The maximum amount of time that a task will be retained in
    /// this queue.
    ///
    /// Queues created by Cloud Tasks have a default `task_ttl` of 31 days.
    /// After a task has lived for `task_ttl`, the task will be deleted
    /// regardless of whether it was dispatched or not.
    ///
    /// The `task_ttl` for queues created via queue.yaml/xml is equal to the
    /// maximum duration because there is a
    /// [storage quota](<https://cloud.google.com/appengine/quotas#Task_Queue>) for
    /// these queues. To view the maximum valid duration, see the documentation for
    /// \[Duration][google.protobuf.Duration\].
    #[prost(message, optional, tag = "9")]
    pub task_ttl: ::core::option::Option<::prost_types::Duration>,
    /// The task tombstone time to live (TTL).
    ///
    /// After a task is deleted or completed, the task's tombstone is
    /// retained for the length of time specified by `tombstone_ttl`.
    /// The tombstone is used by task de-duplication; another task with the same
    /// name can't be created until the tombstone has expired. For more information
    /// about task de-duplication, see the documentation for
    /// \[CreateTaskRequest][google.cloud.tasks.v2beta2.CreateTaskRequest.task\].
    ///
    /// Queues created by Cloud Tasks have a default `tombstone_ttl` of 1 hour.
    #[prost(message, optional, tag = "10")]
    pub tombstone_ttl: ::core::option::Option<::prost_types::Duration>,
    /// Output only. The realtime, informational statistics for a queue. In order
    /// to receive the statistics the caller should include this field in the
    /// FieldMask.
    #[prost(message, optional, tag = "16")]
    pub stats: ::core::option::Option<QueueStats>,
    /// Caller-specified and required in
    /// \[CreateQueue][google.cloud.tasks.v2beta2.CloudTasks.CreateQueue][\], after
    /// which the queue config type becomes output only, though fields within the
    /// config are mutable.
    ///
    /// The queue's target.
    ///
    /// The target applies to all tasks in the queue.
    #[prost(oneof = "queue::TargetType", tags = "3, 4")]
    pub target_type: ::core::option::Option<queue::TargetType>,
}
/// Nested message and enum types in `Queue`.
pub mod queue {
    /// State of the queue.
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
    pub enum State {
        /// Unspecified state.
        Unspecified = 0,
        /// The queue is running. Tasks can be dispatched.
        ///
        /// If the queue was created using Cloud Tasks and the queue has
        /// had no activity (method calls or task dispatches) for 30 days,
        /// the queue may take a few minutes to re-activate. Some method
        /// calls may return \[NOT_FOUND][google.rpc.Code.NOT_FOUND\] and
        /// tasks may not be dispatched for a few minutes until the queue
        /// has been re-activated.
        Running = 1,
        /// Tasks are paused by the user. If the queue is paused then Cloud
        /// Tasks will stop delivering tasks from it, but more tasks can
        /// still be added to it by the user. When a pull queue is paused,
        /// all \[LeaseTasks][google.cloud.tasks.v2beta2.CloudTasks.LeaseTasks\] calls
        /// will return a \[FAILED_PRECONDITION][google.rpc.Code.FAILED_PRECONDITION\].
        Paused = 2,
        /// The queue is disabled.
        ///
        /// A queue becomes `DISABLED` when
        /// \[queue.yaml\](<https://cloud.google.com/appengine/docs/python/config/queueref>)
        /// or
        /// \[queue.xml\](<https://cloud.google.com/appengine/docs/standard/java/config/queueref>)
        /// is uploaded which does not contain the queue. You cannot directly disable
        /// a queue.
        ///
        /// When a queue is disabled, tasks can still be added to a queue
        /// but the tasks are not dispatched and
        /// \[LeaseTasks][google.cloud.tasks.v2beta2.CloudTasks.LeaseTasks\] calls
        /// return a `FAILED_PRECONDITION` error.
        ///
        /// To permanently delete this queue and all of its tasks, call
        /// \[DeleteQueue][google.cloud.tasks.v2beta2.CloudTasks.DeleteQueue\].
        Disabled = 3,
    }
    impl State {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                State::Unspecified => "STATE_UNSPECIFIED",
                State::Running => "RUNNING",
                State::Paused => "PAUSED",
                State::Disabled => "DISABLED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "RUNNING" => Some(Self::Running),
                "PAUSED" => Some(Self::Paused),
                "DISABLED" => Some(Self::Disabled),
                _ => None,
            }
        }
    }
    /// Caller-specified and required in
    /// \[CreateQueue][google.cloud.tasks.v2beta2.CloudTasks.CreateQueue][\], after
    /// which the queue config type becomes output only, though fields within the
    /// config are mutable.
    ///
    /// The queue's target.
    ///
    /// The target applies to all tasks in the queue.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum TargetType {
        /// App Engine HTTP target.
        ///
        /// An App Engine queue is a queue that has an
        /// \[AppEngineHttpTarget][google.cloud.tasks.v2beta2.AppEngineHttpTarget\].
        #[prost(message, tag = "3")]
        AppEngineHttpTarget(super::AppEngineHttpTarget),
        /// Pull target.
        ///
        /// A pull queue is a queue that has a
        /// \[PullTarget][google.cloud.tasks.v2beta2.PullTarget\].
        #[prost(message, tag = "4")]
        PullTarget(super::PullTarget),
    }
}
/// Rate limits.
///
/// This message determines the maximum rate that tasks can be dispatched by a
/// queue, regardless of whether the dispatch is a first task attempt or a retry.
///
/// Note: The debugging command,
/// \[RunTask][google.cloud.tasks.v2beta2.CloudTasks.RunTask\], will run a task
/// even if the queue has reached its
/// \[RateLimits][google.cloud.tasks.v2beta2.RateLimits\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RateLimits {
    /// The maximum rate at which tasks are dispatched from this queue.
    ///
    /// If unspecified when the queue is created, Cloud Tasks will pick the
    /// default.
    ///
    /// * For [App Engine queues]\[google.cloud.tasks.v2beta2.AppEngineHttpTarget\],
    /// the maximum allowed value
    ///    is 500.
    /// * This field is output only   for [pull
    /// queues]\[google.cloud.tasks.v2beta2.PullTarget\]. In addition to the
    ///    `max_tasks_dispatched_per_second` limit, a maximum of 10 QPS of
    ///    \[LeaseTasks][google.cloud.tasks.v2beta2.CloudTasks.LeaseTasks\] requests
    ///    are allowed per pull queue.
    ///
    ///
    /// This field has the same meaning as
    /// [rate in
    /// queue.yaml/xml](<https://cloud.google.com/appengine/docs/standard/python/config/queueref#rate>).
    #[prost(double, tag = "1")]
    pub max_tasks_dispatched_per_second: f64,
    /// The max burst size.
    ///
    /// Max burst size limits how fast tasks in queue are processed when
    /// many tasks are in the queue and the rate is high. This field
    /// allows the queue to have a high rate so processing starts shortly
    /// after a task is enqueued, but still limits resource usage when
    /// many tasks are enqueued in a short period of time.
    ///
    /// The [token bucket](<https://wikipedia.org/wiki/Token_Bucket>)
    /// algorithm is used to control the rate of task dispatches. Each
    /// queue has a token bucket that holds tokens, up to the maximum
    /// specified by `max_burst_size`. Each time a task is dispatched, a
    /// token is removed from the bucket. Tasks will be dispatched until
    /// the queue's bucket runs out of tokens. The bucket will be
    /// continuously refilled with new tokens based on
    /// \[max_dispatches_per_second][RateLimits.max_dispatches_per_second\].
    ///
    /// The default value of `max_burst_size` is picked by Cloud Tasks
    /// based on the value of
    /// \[max_dispatches_per_second][RateLimits.max_dispatches_per_second\].
    ///
    /// The maximum value of `max_burst_size` is 500.
    ///
    /// For App Engine queues that were created or updated using
    /// `queue.yaml/xml`, `max_burst_size` is equal to
    /// \[bucket_size\](<https://cloud.google.com/appengine/docs/standard/python/config/queueref#bucket_size>).
    /// If
    /// \[UpdateQueue][google.cloud.tasks.v2beta2.CloudTasks.UpdateQueue\] is called
    /// on a queue without explicitly setting a value for `max_burst_size`,
    /// `max_burst_size` value will get updated if
    /// \[UpdateQueue][google.cloud.tasks.v2beta2.CloudTasks.UpdateQueue\] is
    /// updating \[max_dispatches_per_second][RateLimits.max_dispatches_per_second\].
    ///
    #[prost(int32, tag = "2")]
    pub max_burst_size: i32,
    /// The maximum number of concurrent tasks that Cloud Tasks allows
    /// to be dispatched for this queue. After this threshold has been
    /// reached, Cloud Tasks stops dispatching tasks until the number of
    /// concurrent requests decreases.
    ///
    /// If unspecified when the queue is created, Cloud Tasks will pick the
    /// default.
    ///
    ///
    /// The maximum allowed value is 5,000.
    ///
    /// This field is output only for
    /// [pull queues]\[google.cloud.tasks.v2beta2.PullTarget\] and always -1, which
    /// indicates no limit. No other queue types can have `max_concurrent_tasks`
    /// set to -1.
    ///
    ///
    /// This field has the same meaning as
    /// [max_concurrent_requests in
    /// queue.yaml/xml](<https://cloud.google.com/appengine/docs/standard/python/config/queueref#max_concurrent_requests>).
    #[prost(int32, tag = "3")]
    pub max_concurrent_tasks: i32,
}
/// Retry config.
///
/// These settings determine how a failed task attempt is retried.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RetryConfig {
    /// If positive, `max_retry_duration` specifies the time limit for
    /// retrying a failed task, measured from when the task was first
    /// attempted. Once `max_retry_duration` time has passed *and* the
    /// task has been attempted
    /// \[max_attempts][google.cloud.tasks.v2beta2.RetryConfig.max_attempts\] times,
    /// no further attempts will be made and the task will be deleted.
    ///
    /// If zero, then the task age is unlimited.
    ///
    /// If unspecified when the queue is created, Cloud Tasks will pick the
    /// default.
    ///
    /// This field is output only for [pull
    /// queues]\[google.cloud.tasks.v2beta2.PullTarget\].
    ///
    ///
    /// `max_retry_duration` will be truncated to the nearest second.
    ///
    /// This field has the same meaning as
    /// [task_age_limit in
    /// queue.yaml/xml](<https://cloud.google.com/appengine/docs/standard/python/config/queueref#retry_parameters>).
    #[prost(message, optional, tag = "3")]
    pub max_retry_duration: ::core::option::Option<::prost_types::Duration>,
    /// A task will be \[scheduled][google.cloud.tasks.v2beta2.Task.schedule_time\]
    /// for retry between
    /// \[min_backoff][google.cloud.tasks.v2beta2.RetryConfig.min_backoff\] and
    /// \[max_backoff][google.cloud.tasks.v2beta2.RetryConfig.max_backoff\] duration
    /// after it fails, if the queue's
    /// \[RetryConfig][google.cloud.tasks.v2beta2.RetryConfig\] specifies that the
    /// task should be retried.
    ///
    /// If unspecified when the queue is created, Cloud Tasks will pick the
    /// default.
    ///
    /// This field is output only for [pull
    /// queues]\[google.cloud.tasks.v2beta2.PullTarget\].
    ///
    ///
    /// `min_backoff` will be truncated to the nearest second.
    ///
    /// This field has the same meaning as
    /// [min_backoff_seconds in
    /// queue.yaml/xml](<https://cloud.google.com/appengine/docs/standard/python/config/queueref#retry_parameters>).
    #[prost(message, optional, tag = "4")]
    pub min_backoff: ::core::option::Option<::prost_types::Duration>,
    /// A task will be \[scheduled][google.cloud.tasks.v2beta2.Task.schedule_time\]
    /// for retry between
    /// \[min_backoff][google.cloud.tasks.v2beta2.RetryConfig.min_backoff\] and
    /// \[max_backoff][google.cloud.tasks.v2beta2.RetryConfig.max_backoff\] duration
    /// after it fails, if the queue's
    /// \[RetryConfig][google.cloud.tasks.v2beta2.RetryConfig\] specifies that the
    /// task should be retried.
    ///
    /// If unspecified when the queue is created, Cloud Tasks will pick the
    /// default.
    ///
    /// This field is output only for [pull
    /// queues]\[google.cloud.tasks.v2beta2.PullTarget\].
    ///
    ///
    /// `max_backoff` will be truncated to the nearest second.
    ///
    /// This field has the same meaning as
    /// [max_backoff_seconds in
    /// queue.yaml/xml](<https://cloud.google.com/appengine/docs/standard/python/config/queueref#retry_parameters>).
    #[prost(message, optional, tag = "5")]
    pub max_backoff: ::core::option::Option<::prost_types::Duration>,
    /// The time between retries will double `max_doublings` times.
    ///
    /// A task's retry interval starts at
    /// \[min_backoff][google.cloud.tasks.v2beta2.RetryConfig.min_backoff\], then
    /// doubles `max_doublings` times, then increases linearly, and finally retries
    /// at intervals of
    /// \[max_backoff][google.cloud.tasks.v2beta2.RetryConfig.max_backoff\] up to
    /// \[max_attempts][google.cloud.tasks.v2beta2.RetryConfig.max_attempts\] times.
    ///
    /// For example, if
    /// \[min_backoff][google.cloud.tasks.v2beta2.RetryConfig.min_backoff\] is 10s,
    /// \[max_backoff][google.cloud.tasks.v2beta2.RetryConfig.max_backoff\] is 300s,
    /// and `max_doublings` is 3, then the a task will first be retried in 10s. The
    /// retry interval will double three times, and then increase linearly by 2^3 *
    /// 10s.  Finally, the task will retry at intervals of
    /// \[max_backoff][google.cloud.tasks.v2beta2.RetryConfig.max_backoff\] until the
    /// task has been attempted
    /// \[max_attempts][google.cloud.tasks.v2beta2.RetryConfig.max_attempts\] times.
    /// Thus, the requests will retry at 10s, 20s, 40s, 80s, 160s, 240s, 300s,
    /// 300s, ....
    ///
    /// If unspecified when the queue is created, Cloud Tasks will pick the
    /// default.
    ///
    /// This field is output only for [pull
    /// queues]\[google.cloud.tasks.v2beta2.PullTarget\].
    ///
    ///
    /// This field has the same meaning as
    /// [max_doublings in
    /// queue.yaml/xml](<https://cloud.google.com/appengine/docs/standard/python/config/queueref#retry_parameters>).
    #[prost(int32, tag = "6")]
    pub max_doublings: i32,
    /// Number of attempts per task.
    ///
    /// If unspecified when the queue is created, Cloud Tasks will pick the
    /// default.
    ///
    ///
    ///
    /// This field has the same meaning as
    /// [task_retry_limit in
    /// queue.yaml/xml](<https://cloud.google.com/appengine/docs/standard/python/config/queueref#retry_parameters>).
    #[prost(oneof = "retry_config::NumAttempts", tags = "1, 2")]
    pub num_attempts: ::core::option::Option<retry_config::NumAttempts>,
}
/// Nested message and enum types in `RetryConfig`.
pub mod retry_config {
    /// Number of attempts per task.
    ///
    /// If unspecified when the queue is created, Cloud Tasks will pick the
    /// default.
    ///
    ///
    ///
    /// This field has the same meaning as
    /// [task_retry_limit in
    /// queue.yaml/xml](<https://cloud.google.com/appengine/docs/standard/python/config/queueref#retry_parameters>).
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum NumAttempts {
        /// The maximum number of attempts for a task.
        ///
        /// Cloud Tasks will attempt the task `max_attempts` times (that
        /// is, if the first attempt fails, then there will be
        /// `max_attempts - 1` retries).  Must be > 0.
        #[prost(int32, tag = "1")]
        MaxAttempts(i32),
        /// If true, then the number of attempts is unlimited.
        #[prost(bool, tag = "2")]
        UnlimitedAttempts(bool),
    }
}
/// Statistics for a queue.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueueStats {
    /// Output only. An estimation of the number of tasks in the queue, that is,
    /// the tasks in the queue that haven't been executed, the tasks in the queue
    /// which the queue has dispatched but has not yet received a reply for, and
    /// the failed tasks that the queue is retrying.
    #[prost(int64, tag = "1")]
    pub tasks_count: i64,
    /// Output only. An estimation of the nearest time in the future where a task
    /// in the queue is scheduled to be executed.
    #[prost(message, optional, tag = "2")]
    pub oldest_estimated_arrival_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The number of tasks that the queue has dispatched and received
    /// a reply for during the last minute. This variable counts both successful
    /// and non-successful executions.
    #[prost(int64, tag = "3")]
    pub executed_last_minute_count: i64,
    /// Output only. The number of requests that the queue has dispatched but has
    /// not received a reply for yet.
    #[prost(int64, tag = "4")]
    pub concurrent_dispatches_count: i64,
    /// Output only. The current maximum number of tasks per second executed by the
    /// queue. The maximum value of this variable is controlled by the RateLimits
    /// of the Queue. However, this value could be less to avoid overloading the
    /// endpoints tasks in the queue are targeting.
    #[prost(double, tag = "5")]
    pub effective_execution_rate: f64,
}
/// A unit of scheduled work.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Task {
    /// Optionally caller-specified in
    /// \[CreateTask][google.cloud.tasks.v2beta2.CloudTasks.CreateTask\].
    ///
    /// The task name.
    ///
    /// The task name must have the following format:
    /// `projects/PROJECT_ID/locations/LOCATION_ID/queues/QUEUE_ID/tasks/TASK_ID`
    ///
    /// * `PROJECT_ID` can contain letters (\[A-Za-z\]), numbers (\[0-9\]),
    ///     hyphens (-), colons (:), or periods (.).
    ///     For more information, see
    ///     [Identifying
    ///     projects](<https://cloud.google.com/resource-manager/docs/creating-managing-projects#identifying_projects>)
    /// * `LOCATION_ID` is the canonical ID for the task's location.
    ///     The list of available locations can be obtained by calling
    ///     \[ListLocations][google.cloud.location.Locations.ListLocations\].
    ///     For more information, see <https://cloud.google.com/about/locations/.>
    /// * `QUEUE_ID` can contain letters (\[A-Za-z\]), numbers (\[0-9\]), or
    ///    hyphens (-). The maximum length is 100 characters.
    /// * `TASK_ID` can contain only letters (\[A-Za-z\]), numbers (\[0-9\]),
    ///    hyphens (-), or underscores (_). The maximum length is 500 characters.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The time when the task is scheduled to be attempted.
    ///
    /// For App Engine queues, this is when the task will be attempted or retried.
    ///
    /// For pull queues, this is the time when the task is available to
    /// be leased; if a task is currently leased, this is the time when
    /// the current lease expires, that is, the time that the task was
    /// leased plus the
    /// \[lease_duration][google.cloud.tasks.v2beta2.LeaseTasksRequest.lease_duration\].
    ///
    /// `schedule_time` will be truncated to the nearest microsecond.
    #[prost(message, optional, tag = "5")]
    pub schedule_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The time that the task was created.
    ///
    /// `create_time` will be truncated to the nearest second.
    #[prost(message, optional, tag = "6")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The task status.
    #[prost(message, optional, tag = "7")]
    pub status: ::core::option::Option<TaskStatus>,
    /// Output only. The view specifies which subset of the
    /// \[Task][google.cloud.tasks.v2beta2.Task\] has been returned.
    #[prost(enumeration = "task::View", tag = "8")]
    pub view: i32,
    /// Required.
    ///
    /// The task's payload is used by the task's target to process the task.
    /// A payload is valid only if it is compatible with the queue's target.
    #[prost(oneof = "task::PayloadType", tags = "3, 4")]
    pub payload_type: ::core::option::Option<task::PayloadType>,
}
/// Nested message and enum types in `Task`.
pub mod task {
    /// The view specifies a subset of \[Task][google.cloud.tasks.v2beta2.Task\]
    /// data.
    ///
    /// When a task is returned in a response, not all
    /// information is retrieved by default because some data, such as
    /// payloads, might be desirable to return only when needed because
    /// of its large size or because of the sensitivity of data that it
    /// contains.
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
    pub enum View {
        /// Unspecified. Defaults to BASIC.
        Unspecified = 0,
        /// The basic view omits fields which can be large or can contain
        /// sensitive data.
        ///
        /// This view does not include the
        /// ([payload in
        /// AppEngineHttpRequest]\[google.cloud.tasks.v2beta2.AppEngineHttpRequest\]
        /// and [payload in
        /// PullMessage]\[google.cloud.tasks.v2beta2.PullMessage.payload\]). These
        /// payloads are desirable to return only when needed, because they can be
        /// large and because of the sensitivity of the data that you choose to store
        /// in it.
        Basic = 1,
        /// All information is returned.
        ///
        /// Authorization for \[FULL][google.cloud.tasks.v2beta2.Task.View.FULL\]
        /// requires `cloudtasks.tasks.fullView` [Google
        /// IAM](<https://cloud.google.com/iam/>) permission on the
        /// \[Queue][google.cloud.tasks.v2beta2.Queue\] resource.
        Full = 2,
    }
    impl View {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                View::Unspecified => "VIEW_UNSPECIFIED",
                View::Basic => "BASIC",
                View::Full => "FULL",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "VIEW_UNSPECIFIED" => Some(Self::Unspecified),
                "BASIC" => Some(Self::Basic),
                "FULL" => Some(Self::Full),
                _ => None,
            }
        }
    }
    /// Required.
    ///
    /// The task's payload is used by the task's target to process the task.
    /// A payload is valid only if it is compatible with the queue's target.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum PayloadType {
        /// App Engine HTTP request that is sent to the task's target. Can
        /// be set only if
        /// \[app_engine_http_target][google.cloud.tasks.v2beta2.Queue.app_engine_http_target\]
        /// is set on the queue.
        ///
        /// An App Engine task is a task that has
        /// \[AppEngineHttpRequest][google.cloud.tasks.v2beta2.AppEngineHttpRequest\]
        /// set.
        #[prost(message, tag = "3")]
        AppEngineHttpRequest(super::AppEngineHttpRequest),
        /// \[LeaseTasks][google.cloud.tasks.v2beta2.CloudTasks.LeaseTasks\] to process
        /// the task. Can be set only if
        /// \[pull_target][google.cloud.tasks.v2beta2.Queue.pull_target\] is set on the
        /// queue.
        ///
        /// A pull task is a task that has
        /// \[PullMessage][google.cloud.tasks.v2beta2.PullMessage\] set.
        #[prost(message, tag = "4")]
        PullMessage(super::PullMessage),
    }
}
/// Status of the task.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TaskStatus {
    /// Output only. The number of attempts dispatched.
    ///
    /// This count includes attempts which have been dispatched but haven't
    /// received a response.
    #[prost(int32, tag = "1")]
    pub attempt_dispatch_count: i32,
    /// Output only. The number of attempts which have received a response.
    ///
    /// This field is not calculated for [pull
    /// tasks]\[google.cloud.tasks.v2beta2.PullMessage\].
    #[prost(int32, tag = "2")]
    pub attempt_response_count: i32,
    /// Output only. The status of the task's first attempt.
    ///
    /// Only
    /// \[dispatch_time][google.cloud.tasks.v2beta2.AttemptStatus.dispatch_time\]
    /// will be set. The other
    /// \[AttemptStatus][google.cloud.tasks.v2beta2.AttemptStatus\] information is
    /// not retained by Cloud Tasks.
    ///
    /// This field is not calculated for [pull
    /// tasks]\[google.cloud.tasks.v2beta2.PullMessage\].
    #[prost(message, optional, tag = "3")]
    pub first_attempt_status: ::core::option::Option<AttemptStatus>,
    /// Output only. The status of the task's last attempt.
    ///
    /// This field is not calculated for [pull
    /// tasks]\[google.cloud.tasks.v2beta2.PullMessage\].
    #[prost(message, optional, tag = "4")]
    pub last_attempt_status: ::core::option::Option<AttemptStatus>,
}
/// The status of a task attempt.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AttemptStatus {
    /// Output only. The time that this attempt was scheduled.
    ///
    /// `schedule_time` will be truncated to the nearest microsecond.
    #[prost(message, optional, tag = "1")]
    pub schedule_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The time that this attempt was dispatched.
    ///
    /// `dispatch_time` will be truncated to the nearest microsecond.
    #[prost(message, optional, tag = "2")]
    pub dispatch_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The time that this attempt response was received.
    ///
    /// `response_time` will be truncated to the nearest microsecond.
    #[prost(message, optional, tag = "3")]
    pub response_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The response from the target for this attempt.
    ///
    /// If the task has not been attempted or the task is currently running
    /// then the response status is unset.
    #[prost(message, optional, tag = "4")]
    pub response_status: ::core::option::Option<super::super::super::rpc::Status>,
}
/// Request message for
/// \[ListQueues][google.cloud.tasks.v2beta2.CloudTasks.ListQueues\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListQueuesRequest {
    /// Required. The location name.
    /// For example: `projects/PROJECT_ID/locations/LOCATION_ID`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// `filter` can be used to specify a subset of queues. Any
    /// \[Queue][google.cloud.tasks.v2beta2.Queue\] field can be used as a filter and
    /// several operators as supported. For example: `<=, <, >=, >, !=, =, :`. The
    /// filter syntax is the same as described in [Stackdriver's Advanced Logs
    /// Filters](<https://cloud.google.com/logging/docs/view/advanced_filters>).
    ///
    /// Sample filter "app_engine_http_target: *".
    ///
    /// Note that using filters might cause fewer queues than the
    /// requested_page size to be returned.
    #[prost(string, tag = "2")]
    pub filter: ::prost::alloc::string::String,
    /// Requested page size.
    ///
    /// The maximum page size is 9800. If unspecified, the page size will
    /// be the maximum. Fewer queues than requested might be returned,
    /// even if more queues exist; use the
    /// \[next_page_token][google.cloud.tasks.v2beta2.ListQueuesResponse.next_page_token\]
    /// in the response to determine if more queues exist.
    #[prost(int32, tag = "3")]
    pub page_size: i32,
    /// A token identifying the page of results to return.
    ///
    /// To request the first page results, page_token must be empty. To
    /// request the next page of results, page_token must be the value of
    /// \[next_page_token][google.cloud.tasks.v2beta2.ListQueuesResponse.next_page_token\]
    /// returned from the previous call to
    /// \[ListQueues][google.cloud.tasks.v2beta2.CloudTasks.ListQueues\] method. It
    /// is an error to switch the value of the
    /// \[filter][google.cloud.tasks.v2beta2.ListQueuesRequest.filter\] while
    /// iterating through pages.
    #[prost(string, tag = "4")]
    pub page_token: ::prost::alloc::string::String,
    /// Optional. Read mask is used for a more granular control over what the API
    /// returns. If the mask is not present all fields will be returned except
    /// \[Queue.stats\]. \[Queue.stats\] will be returned only if it was  explicitly
    /// specified in the mask.
    #[prost(message, optional, tag = "5")]
    pub read_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Response message for
/// \[ListQueues][google.cloud.tasks.v2beta2.CloudTasks.ListQueues\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListQueuesResponse {
    /// The list of queues.
    #[prost(message, repeated, tag = "1")]
    pub queues: ::prost::alloc::vec::Vec<Queue>,
    /// A token to retrieve next page of results.
    ///
    /// To return the next page of results, call
    /// \[ListQueues][google.cloud.tasks.v2beta2.CloudTasks.ListQueues\] with this
    /// value as the
    /// \[page_token][google.cloud.tasks.v2beta2.ListQueuesRequest.page_token\].
    ///
    /// If the next_page_token is empty, there are no more results.
    ///
    /// The page token is valid for only 2 hours.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for
/// \[GetQueue][google.cloud.tasks.v2beta2.CloudTasks.GetQueue\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetQueueRequest {
    /// Required. The resource name of the queue. For example:
    /// `projects/PROJECT_ID/locations/LOCATION_ID/queues/QUEUE_ID`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. Read mask is used for a more granular control over what the API
    /// returns. If the mask is not present all fields will be returned except
    /// \[Queue.stats\]. \[Queue.stats\] will be returned only if it was  explicitly
    /// specified in the mask.
    #[prost(message, optional, tag = "2")]
    pub read_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request message for
/// \[CreateQueue][google.cloud.tasks.v2beta2.CloudTasks.CreateQueue\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateQueueRequest {
    /// Required. The location name in which the queue will be created.
    /// For example: `projects/PROJECT_ID/locations/LOCATION_ID`
    ///
    /// The list of allowed locations can be obtained by calling Cloud
    /// Tasks' implementation of
    /// \[ListLocations][google.cloud.location.Locations.ListLocations\].
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The queue to create.
    ///
    /// [Queue's name]\[google.cloud.tasks.v2beta2.Queue.name\] cannot be the same as
    /// an existing queue.
    #[prost(message, optional, tag = "2")]
    pub queue: ::core::option::Option<Queue>,
}
/// Request message for
/// \[UpdateQueue][google.cloud.tasks.v2beta2.CloudTasks.UpdateQueue\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateQueueRequest {
    /// Required. The queue to create or update.
    ///
    /// The queue's \[name][google.cloud.tasks.v2beta2.Queue.name\] must be
    /// specified.
    ///
    /// Output only fields cannot be modified using UpdateQueue.
    /// Any value specified for an output only field will be ignored.
    /// The queue's \[name][google.cloud.tasks.v2beta2.Queue.name\] cannot be
    /// changed.
    #[prost(message, optional, tag = "1")]
    pub queue: ::core::option::Option<Queue>,
    /// A mask used to specify which fields of the queue are being updated.
    ///
    /// If empty, then all fields will be updated.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request message for
/// \[DeleteQueue][google.cloud.tasks.v2beta2.CloudTasks.DeleteQueue\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteQueueRequest {
    /// Required. The queue name. For example:
    /// `projects/PROJECT_ID/locations/LOCATION_ID/queues/QUEUE_ID`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for
/// \[PurgeQueue][google.cloud.tasks.v2beta2.CloudTasks.PurgeQueue\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PurgeQueueRequest {
    /// Required. The queue name. For example:
    /// `projects/PROJECT_ID/location/LOCATION_ID/queues/QUEUE_ID`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for
/// \[PauseQueue][google.cloud.tasks.v2beta2.CloudTasks.PauseQueue\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PauseQueueRequest {
    /// Required. The queue name. For example:
    /// `projects/PROJECT_ID/location/LOCATION_ID/queues/QUEUE_ID`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for
/// \[ResumeQueue][google.cloud.tasks.v2beta2.CloudTasks.ResumeQueue\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResumeQueueRequest {
    /// Required. The queue name. For example:
    /// `projects/PROJECT_ID/location/LOCATION_ID/queues/QUEUE_ID`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for listing tasks using
/// \[ListTasks][google.cloud.tasks.v2beta2.CloudTasks.ListTasks\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListTasksRequest {
    /// Required. The queue name. For example:
    /// `projects/PROJECT_ID/locations/LOCATION_ID/queues/QUEUE_ID`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The response_view specifies which subset of the
    /// \[Task][google.cloud.tasks.v2beta2.Task\] will be returned.
    ///
    /// By default response_view is
    /// \[BASIC][google.cloud.tasks.v2beta2.Task.View.BASIC\]; not all information is
    /// retrieved by default because some data, such as payloads, might be
    /// desirable to return only when needed because of its large size or because
    /// of the sensitivity of data that it contains.
    ///
    /// Authorization for \[FULL][google.cloud.tasks.v2beta2.Task.View.FULL\]
    /// requires `cloudtasks.tasks.fullView` [Google
    /// IAM](<https://cloud.google.com/iam/>) permission on the
    /// \[Task][google.cloud.tasks.v2beta2.Task\] resource.
    #[prost(enumeration = "task::View", tag = "2")]
    pub response_view: i32,
    /// Maximum page size.
    ///
    /// Fewer tasks than requested might be returned, even if more tasks exist; use
    /// \[next_page_token][google.cloud.tasks.v2beta2.ListTasksResponse.next_page_token\]
    /// in the response to determine if more tasks exist.
    ///
    /// The maximum page size is 1000. If unspecified, the page size will be the
    /// maximum.
    #[prost(int32, tag = "4")]
    pub page_size: i32,
    /// A token identifying the page of results to return.
    ///
    /// To request the first page results, page_token must be empty. To
    /// request the next page of results, page_token must be the value of
    /// \[next_page_token][google.cloud.tasks.v2beta2.ListTasksResponse.next_page_token\]
    /// returned from the previous call to
    /// \[ListTasks][google.cloud.tasks.v2beta2.CloudTasks.ListTasks\] method.
    ///
    /// The page token is valid for only 2 hours.
    #[prost(string, tag = "5")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response message for listing tasks using
/// \[ListTasks][google.cloud.tasks.v2beta2.CloudTasks.ListTasks\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListTasksResponse {
    /// The list of tasks.
    #[prost(message, repeated, tag = "1")]
    pub tasks: ::prost::alloc::vec::Vec<Task>,
    /// A token to retrieve next page of results.
    ///
    /// To return the next page of results, call
    /// \[ListTasks][google.cloud.tasks.v2beta2.CloudTasks.ListTasks\] with this
    /// value as the
    /// \[page_token][google.cloud.tasks.v2beta2.ListTasksRequest.page_token\].
    ///
    /// If the next_page_token is empty, there are no more results.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for getting a task using
/// \[GetTask][google.cloud.tasks.v2beta2.CloudTasks.GetTask\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTaskRequest {
    /// Required. The task name. For example:
    /// `projects/PROJECT_ID/locations/LOCATION_ID/queues/QUEUE_ID/tasks/TASK_ID`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The response_view specifies which subset of the
    /// \[Task][google.cloud.tasks.v2beta2.Task\] will be returned.
    ///
    /// By default response_view is
    /// \[BASIC][google.cloud.tasks.v2beta2.Task.View.BASIC\]; not all information is
    /// retrieved by default because some data, such as payloads, might be
    /// desirable to return only when needed because of its large size or because
    /// of the sensitivity of data that it contains.
    ///
    /// Authorization for \[FULL][google.cloud.tasks.v2beta2.Task.View.FULL\]
    /// requires `cloudtasks.tasks.fullView` [Google
    /// IAM](<https://cloud.google.com/iam/>) permission on the
    /// \[Task][google.cloud.tasks.v2beta2.Task\] resource.
    #[prost(enumeration = "task::View", tag = "2")]
    pub response_view: i32,
}
/// Request message for
/// \[CreateTask][google.cloud.tasks.v2beta2.CloudTasks.CreateTask\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateTaskRequest {
    /// Required. The queue name. For example:
    /// `projects/PROJECT_ID/locations/LOCATION_ID/queues/QUEUE_ID`
    ///
    /// The queue must already exist.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The task to add.
    ///
    /// Task names have the following format:
    /// `projects/PROJECT_ID/locations/LOCATION_ID/queues/QUEUE_ID/tasks/TASK_ID`.
    /// The user can optionally specify a task
    /// \[name][google.cloud.tasks.v2beta2.Task.name\]. If a name is not specified
    /// then the system will generate a random unique task id, which will be set in
    /// the task returned in the \[response][google.cloud.tasks.v2beta2.Task.name\].
    ///
    /// If \[schedule_time][google.cloud.tasks.v2beta2.Task.schedule_time\] is not
    /// set or is in the past then Cloud Tasks will set it to the current time.
    ///
    /// Task De-duplication:
    ///
    /// Explicitly specifying a task ID enables task de-duplication.  If
    /// a task's ID is identical to that of an existing task or a task
    /// that was deleted or completed recently then the call will fail
    /// with \[ALREADY_EXISTS][google.rpc.Code.ALREADY_EXISTS\].
    /// If the task's queue was created using Cloud Tasks, then another task with
    /// the same name can't be created for ~1hour after the original task was
    /// deleted or completed. If the task's queue was created using queue.yaml or
    /// queue.xml, then another task with the same name can't be created
    /// for ~9days after the original task was deleted or completed.
    ///
    /// Because there is an extra lookup cost to identify duplicate task
    /// names, these \[CreateTask][google.cloud.tasks.v2beta2.CloudTasks.CreateTask\]
    /// calls have significantly increased latency. Using hashed strings for the
    /// task id or for the prefix of the task id is recommended. Choosing task ids
    /// that are sequential or have sequential prefixes, for example using a
    /// timestamp, causes an increase in latency and error rates in all
    /// task commands. The infrastructure relies on an approximately
    /// uniform distribution of task ids to store and serve tasks
    /// efficiently.
    #[prost(message, optional, tag = "2")]
    pub task: ::core::option::Option<Task>,
    /// The response_view specifies which subset of the
    /// \[Task][google.cloud.tasks.v2beta2.Task\] will be returned.
    ///
    /// By default response_view is
    /// \[BASIC][google.cloud.tasks.v2beta2.Task.View.BASIC\]; not all information is
    /// retrieved by default because some data, such as payloads, might be
    /// desirable to return only when needed because of its large size or because
    /// of the sensitivity of data that it contains.
    ///
    /// Authorization for \[FULL][google.cloud.tasks.v2beta2.Task.View.FULL\]
    /// requires `cloudtasks.tasks.fullView` [Google
    /// IAM](<https://cloud.google.com/iam/>) permission on the
    /// \[Task][google.cloud.tasks.v2beta2.Task\] resource.
    #[prost(enumeration = "task::View", tag = "3")]
    pub response_view: i32,
}
/// Request message for deleting a task using
/// \[DeleteTask][google.cloud.tasks.v2beta2.CloudTasks.DeleteTask\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteTaskRequest {
    /// Required. The task name. For example:
    /// `projects/PROJECT_ID/locations/LOCATION_ID/queues/QUEUE_ID/tasks/TASK_ID`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for leasing tasks using
/// \[LeaseTasks][google.cloud.tasks.v2beta2.CloudTasks.LeaseTasks\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LeaseTasksRequest {
    /// Required. The queue name. For example:
    /// `projects/PROJECT_ID/locations/LOCATION_ID/queues/QUEUE_ID`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of tasks to lease.
    ///
    /// The system will make a best effort to return as close to as
    /// `max_tasks` as possible.
    ///
    /// The largest that `max_tasks` can be is 1000.
    ///
    /// The maximum total size of a [lease tasks
    /// response]\[google.cloud.tasks.v2beta2.LeaseTasksResponse\] is 32 MB. If the
    /// sum of all task sizes requested reaches this limit, fewer tasks than
    /// requested are returned.
    #[prost(int32, tag = "2")]
    pub max_tasks: i32,
    /// Required. The duration of the lease.
    ///
    /// Each task returned in the
    /// \[response][google.cloud.tasks.v2beta2.LeaseTasksResponse\] will have its
    /// \[schedule_time][google.cloud.tasks.v2beta2.Task.schedule_time\] set to the
    /// current time plus the `lease_duration`. The task is leased until its
    /// \[schedule_time][google.cloud.tasks.v2beta2.Task.schedule_time\]; thus, the
    /// task will not be returned to another
    /// \[LeaseTasks][google.cloud.tasks.v2beta2.CloudTasks.LeaseTasks\] call before
    /// its \[schedule_time][google.cloud.tasks.v2beta2.Task.schedule_time\].
    ///
    ///
    /// After the worker has successfully finished the work associated
    /// with the task, the worker must call via
    /// \[AcknowledgeTask][google.cloud.tasks.v2beta2.CloudTasks.AcknowledgeTask\]
    /// before the \[schedule_time][google.cloud.tasks.v2beta2.Task.schedule_time\].
    /// Otherwise the task will be returned to a later
    /// \[LeaseTasks][google.cloud.tasks.v2beta2.CloudTasks.LeaseTasks\] call so that
    /// another worker can retry it.
    ///
    /// The maximum lease duration is 1 week.
    /// `lease_duration` will be truncated to the nearest second.
    #[prost(message, optional, tag = "3")]
    pub lease_duration: ::core::option::Option<::prost_types::Duration>,
    /// The response_view specifies which subset of the
    /// \[Task][google.cloud.tasks.v2beta2.Task\] will be returned.
    ///
    /// By default response_view is
    /// \[BASIC][google.cloud.tasks.v2beta2.Task.View.BASIC\]; not all information is
    /// retrieved by default because some data, such as payloads, might be
    /// desirable to return only when needed because of its large size or because
    /// of the sensitivity of data that it contains.
    ///
    /// Authorization for \[FULL][google.cloud.tasks.v2beta2.Task.View.FULL\]
    /// requires `cloudtasks.tasks.fullView` [Google
    /// IAM](<https://cloud.google.com/iam/>) permission on the
    /// \[Task][google.cloud.tasks.v2beta2.Task\] resource.
    #[prost(enumeration = "task::View", tag = "4")]
    pub response_view: i32,
    /// `filter` can be used to specify a subset of tasks to lease.
    ///
    /// When `filter` is set to `tag=<my-tag>` then the
    /// \[response][google.cloud.tasks.v2beta2.LeaseTasksResponse\] will contain only
    /// tasks whose \[tag][google.cloud.tasks.v2beta2.PullMessage.tag\] is equal to
    /// `<my-tag>`. `<my-tag>` must be less than 500 characters.
    ///
    /// When `filter` is set to `tag_function=oldest_tag()`, only tasks which have
    /// the same tag as the task with the oldest
    /// \[schedule_time][google.cloud.tasks.v2beta2.Task.schedule_time\] will be
    /// returned.
    ///
    /// Grammar Syntax:
    ///
    /// * `filter = "tag=" tag | "tag_function=" function`
    ///
    /// * `tag = string`
    ///
    /// * `function = "oldest_tag()"`
    ///
    /// The `oldest_tag()` function returns tasks which have the same tag as the
    /// oldest task (ordered by schedule time).
    ///
    /// SDK compatibility: Although the SDK allows tags to be either
    /// string or
    /// \[bytes\](<https://cloud.google.com/appengine/docs/standard/java/javadoc/com/google/appengine/api/taskqueue/TaskOptions.html#tag-byte:A->),
    /// only UTF-8 encoded tags can be used in Cloud Tasks. Tag which
    /// aren't UTF-8 encoded can't be used in the
    /// \[filter][google.cloud.tasks.v2beta2.LeaseTasksRequest.filter\] and the
    /// task's \[tag][google.cloud.tasks.v2beta2.PullMessage.tag\] will be displayed
    /// as empty in Cloud Tasks.
    #[prost(string, tag = "5")]
    pub filter: ::prost::alloc::string::String,
}
/// Response message for leasing tasks using
/// \[LeaseTasks][google.cloud.tasks.v2beta2.CloudTasks.LeaseTasks\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LeaseTasksResponse {
    /// The leased tasks.
    #[prost(message, repeated, tag = "1")]
    pub tasks: ::prost::alloc::vec::Vec<Task>,
}
/// Request message for acknowledging a task using
/// \[AcknowledgeTask][google.cloud.tasks.v2beta2.CloudTasks.AcknowledgeTask\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AcknowledgeTaskRequest {
    /// Required. The task name. For example:
    /// `projects/PROJECT_ID/locations/LOCATION_ID/queues/QUEUE_ID/tasks/TASK_ID`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. The task's current schedule time, available in the
    /// \[schedule_time][google.cloud.tasks.v2beta2.Task.schedule_time\] returned by
    /// \[LeaseTasks][google.cloud.tasks.v2beta2.CloudTasks.LeaseTasks\] response or
    /// \[RenewLease][google.cloud.tasks.v2beta2.CloudTasks.RenewLease\] response.
    /// This restriction is to ensure that your worker currently holds the lease.
    #[prost(message, optional, tag = "2")]
    pub schedule_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Request message for renewing a lease using
/// \[RenewLease][google.cloud.tasks.v2beta2.CloudTasks.RenewLease\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RenewLeaseRequest {
    /// Required. The task name. For example:
    /// `projects/PROJECT_ID/locations/LOCATION_ID/queues/QUEUE_ID/tasks/TASK_ID`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. The task's current schedule time, available in the
    /// \[schedule_time][google.cloud.tasks.v2beta2.Task.schedule_time\] returned by
    /// \[LeaseTasks][google.cloud.tasks.v2beta2.CloudTasks.LeaseTasks\] response or
    /// \[RenewLease][google.cloud.tasks.v2beta2.CloudTasks.RenewLease\] response.
    /// This restriction is to ensure that your worker currently holds the lease.
    #[prost(message, optional, tag = "2")]
    pub schedule_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Required. The desired new lease duration, starting from now.
    ///
    ///
    /// The maximum lease duration is 1 week.
    /// `lease_duration` will be truncated to the nearest second.
    #[prost(message, optional, tag = "3")]
    pub lease_duration: ::core::option::Option<::prost_types::Duration>,
    /// The response_view specifies which subset of the
    /// \[Task][google.cloud.tasks.v2beta2.Task\] will be returned.
    ///
    /// By default response_view is
    /// \[BASIC][google.cloud.tasks.v2beta2.Task.View.BASIC\]; not all information is
    /// retrieved by default because some data, such as payloads, might be
    /// desirable to return only when needed because of its large size or because
    /// of the sensitivity of data that it contains.
    ///
    /// Authorization for \[FULL][google.cloud.tasks.v2beta2.Task.View.FULL\]
    /// requires `cloudtasks.tasks.fullView` [Google
    /// IAM](<https://cloud.google.com/iam/>) permission on the
    /// \[Task][google.cloud.tasks.v2beta2.Task\] resource.
    #[prost(enumeration = "task::View", tag = "4")]
    pub response_view: i32,
}
/// Request message for canceling a lease using
/// \[CancelLease][google.cloud.tasks.v2beta2.CloudTasks.CancelLease\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CancelLeaseRequest {
    /// Required. The task name. For example:
    /// `projects/PROJECT_ID/locations/LOCATION_ID/queues/QUEUE_ID/tasks/TASK_ID`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. The task's current schedule time, available in the
    /// \[schedule_time][google.cloud.tasks.v2beta2.Task.schedule_time\] returned by
    /// \[LeaseTasks][google.cloud.tasks.v2beta2.CloudTasks.LeaseTasks\] response or
    /// \[RenewLease][google.cloud.tasks.v2beta2.CloudTasks.RenewLease\] response.
    /// This restriction is to ensure that your worker currently holds the lease.
    #[prost(message, optional, tag = "2")]
    pub schedule_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The response_view specifies which subset of the
    /// \[Task][google.cloud.tasks.v2beta2.Task\] will be returned.
    ///
    /// By default response_view is
    /// \[BASIC][google.cloud.tasks.v2beta2.Task.View.BASIC\]; not all information is
    /// retrieved by default because some data, such as payloads, might be
    /// desirable to return only when needed because of its large size or because
    /// of the sensitivity of data that it contains.
    ///
    /// Authorization for \[FULL][google.cloud.tasks.v2beta2.Task.View.FULL\]
    /// requires `cloudtasks.tasks.fullView` [Google
    /// IAM](<https://cloud.google.com/iam/>) permission on the
    /// \[Task][google.cloud.tasks.v2beta2.Task\] resource.
    #[prost(enumeration = "task::View", tag = "3")]
    pub response_view: i32,
}
/// Request message for forcing a task to run now using
/// \[RunTask][google.cloud.tasks.v2beta2.CloudTasks.RunTask\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RunTaskRequest {
    /// Required. The task name. For example:
    /// `projects/PROJECT_ID/locations/LOCATION_ID/queues/QUEUE_ID/tasks/TASK_ID`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The response_view specifies which subset of the
    /// \[Task][google.cloud.tasks.v2beta2.Task\] will be returned.
    ///
    /// By default response_view is
    /// \[BASIC][google.cloud.tasks.v2beta2.Task.View.BASIC\]; not all information is
    /// retrieved by default because some data, such as payloads, might be
    /// desirable to return only when needed because of its large size or because
    /// of the sensitivity of data that it contains.
    ///
    /// Authorization for \[FULL][google.cloud.tasks.v2beta2.Task.View.FULL\]
    /// requires `cloudtasks.tasks.fullView` [Google
    /// IAM](<https://cloud.google.com/iam/>) permission on the
    /// \[Task][google.cloud.tasks.v2beta2.Task\] resource.
    #[prost(enumeration = "task::View", tag = "2")]
    pub response_view: i32,
}
/// Generated client implementations.
pub mod cloud_tasks_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Cloud Tasks allows developers to manage the execution of background
    /// work in their applications.
    #[derive(Debug, Clone)]
    pub struct CloudTasksClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl CloudTasksClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> CloudTasksClient<T>
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
        ) -> CloudTasksClient<InterceptedService<T, F>>
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
            CloudTasksClient::new(InterceptedService::new(inner, interceptor))
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
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        /// Lists queues.
        ///
        /// Queues are returned in lexicographical order.
        pub async fn list_queues(
            &mut self,
            request: impl tonic::IntoRequest<super::ListQueuesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListQueuesResponse>,
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
                "/google.cloud.tasks.v2beta2.CloudTasks/ListQueues",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.tasks.v2beta2.CloudTasks",
                        "ListQueues",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets a queue.
        pub async fn get_queue(
            &mut self,
            request: impl tonic::IntoRequest<super::GetQueueRequest>,
        ) -> std::result::Result<tonic::Response<super::Queue>, tonic::Status> {
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
                "/google.cloud.tasks.v2beta2.CloudTasks/GetQueue",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.cloud.tasks.v2beta2.CloudTasks", "GetQueue"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates a queue.
        ///
        /// Queues created with this method allow tasks to live for a maximum of 31
        /// days. After a task is 31 days old, the task will be deleted regardless of
        /// whether it was dispatched or not.
        ///
        /// WARNING: Using this method may have unintended side effects if you are
        /// using an App Engine `queue.yaml` or `queue.xml` file to manage your queues.
        /// Read
        /// [Overview of Queue Management and
        /// queue.yaml](https://cloud.google.com/tasks/docs/queue-yaml) before using
        /// this method.
        pub async fn create_queue(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateQueueRequest>,
        ) -> std::result::Result<tonic::Response<super::Queue>, tonic::Status> {
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
                "/google.cloud.tasks.v2beta2.CloudTasks/CreateQueue",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.tasks.v2beta2.CloudTasks",
                        "CreateQueue",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates a queue.
        ///
        /// This method creates the queue if it does not exist and updates
        /// the queue if it does exist.
        ///
        /// Queues created with this method allow tasks to live for a maximum of 31
        /// days. After a task is 31 days old, the task will be deleted regardless of
        /// whether it was dispatched or not.
        ///
        /// WARNING: Using this method may have unintended side effects if you are
        /// using an App Engine `queue.yaml` or `queue.xml` file to manage your queues.
        /// Read
        /// [Overview of Queue Management and
        /// queue.yaml](https://cloud.google.com/tasks/docs/queue-yaml) before using
        /// this method.
        pub async fn update_queue(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateQueueRequest>,
        ) -> std::result::Result<tonic::Response<super::Queue>, tonic::Status> {
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
                "/google.cloud.tasks.v2beta2.CloudTasks/UpdateQueue",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.tasks.v2beta2.CloudTasks",
                        "UpdateQueue",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes a queue.
        ///
        /// This command will delete the queue even if it has tasks in it.
        ///
        /// Note: If you delete a queue, a queue with the same name can't be created
        /// for 7 days.
        ///
        /// WARNING: Using this method may have unintended side effects if you are
        /// using an App Engine `queue.yaml` or `queue.xml` file to manage your queues.
        /// Read
        /// [Overview of Queue Management and
        /// queue.yaml](https://cloud.google.com/tasks/docs/queue-yaml) before using
        /// this method.
        pub async fn delete_queue(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteQueueRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
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
                "/google.cloud.tasks.v2beta2.CloudTasks/DeleteQueue",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.tasks.v2beta2.CloudTasks",
                        "DeleteQueue",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Purges a queue by deleting all of its tasks.
        ///
        /// All tasks created before this method is called are permanently deleted.
        ///
        /// Purge operations can take up to one minute to take effect. Tasks
        /// might be dispatched before the purge takes effect. A purge is irreversible.
        pub async fn purge_queue(
            &mut self,
            request: impl tonic::IntoRequest<super::PurgeQueueRequest>,
        ) -> std::result::Result<tonic::Response<super::Queue>, tonic::Status> {
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
                "/google.cloud.tasks.v2beta2.CloudTasks/PurgeQueue",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.tasks.v2beta2.CloudTasks",
                        "PurgeQueue",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Pauses the queue.
        ///
        /// If a queue is paused then the system will stop dispatching tasks
        /// until the queue is resumed via
        /// [ResumeQueue][google.cloud.tasks.v2beta2.CloudTasks.ResumeQueue]. Tasks can
        /// still be added when the queue is paused. A queue is paused if its
        /// [state][google.cloud.tasks.v2beta2.Queue.state] is
        /// [PAUSED][google.cloud.tasks.v2beta2.Queue.State.PAUSED].
        pub async fn pause_queue(
            &mut self,
            request: impl tonic::IntoRequest<super::PauseQueueRequest>,
        ) -> std::result::Result<tonic::Response<super::Queue>, tonic::Status> {
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
                "/google.cloud.tasks.v2beta2.CloudTasks/PauseQueue",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.tasks.v2beta2.CloudTasks",
                        "PauseQueue",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Resume a queue.
        ///
        /// This method resumes a queue after it has been
        /// [PAUSED][google.cloud.tasks.v2beta2.Queue.State.PAUSED] or
        /// [DISABLED][google.cloud.tasks.v2beta2.Queue.State.DISABLED]. The state of a
        /// queue is stored in the queue's
        /// [state][google.cloud.tasks.v2beta2.Queue.state]; after calling this method
        /// it will be set to
        /// [RUNNING][google.cloud.tasks.v2beta2.Queue.State.RUNNING].
        ///
        /// WARNING: Resuming many high-QPS queues at the same time can
        /// lead to target overloading. If you are resuming high-QPS
        /// queues, follow the 500/50/5 pattern described in
        /// [Managing Cloud Tasks Scaling
        /// Risks](https://cloud.google.com/tasks/docs/manage-cloud-task-scaling).
        pub async fn resume_queue(
            &mut self,
            request: impl tonic::IntoRequest<super::ResumeQueueRequest>,
        ) -> std::result::Result<tonic::Response<super::Queue>, tonic::Status> {
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
                "/google.cloud.tasks.v2beta2.CloudTasks/ResumeQueue",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.tasks.v2beta2.CloudTasks",
                        "ResumeQueue",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets the access control policy for a
        /// [Queue][google.cloud.tasks.v2beta2.Queue]. Returns an empty policy if the
        /// resource exists and does not have a policy set.
        ///
        /// Authorization requires the following
        /// [Google IAM](https://cloud.google.com/iam) permission on the specified
        /// resource parent:
        ///
        /// * `cloudtasks.queues.getIamPolicy`
        pub async fn get_iam_policy(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::iam::v1::GetIamPolicyRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::super::super::iam::v1::Policy>,
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
                "/google.cloud.tasks.v2beta2.CloudTasks/GetIamPolicy",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.tasks.v2beta2.CloudTasks",
                        "GetIamPolicy",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Sets the access control policy for a
        /// [Queue][google.cloud.tasks.v2beta2.Queue]. Replaces any existing policy.
        ///
        /// Note: The Cloud Console does not check queue-level IAM permissions yet.
        /// Project-level permissions are required to use the Cloud Console.
        ///
        /// Authorization requires the following
        /// [Google IAM](https://cloud.google.com/iam) permission on the specified
        /// resource parent:
        ///
        /// * `cloudtasks.queues.setIamPolicy`
        pub async fn set_iam_policy(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::iam::v1::SetIamPolicyRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::super::super::iam::v1::Policy>,
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
                "/google.cloud.tasks.v2beta2.CloudTasks/SetIamPolicy",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.tasks.v2beta2.CloudTasks",
                        "SetIamPolicy",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Returns permissions that a caller has on a
        /// [Queue][google.cloud.tasks.v2beta2.Queue]. If the resource does not exist,
        /// this will return an empty set of permissions, not a
        /// [NOT_FOUND][google.rpc.Code.NOT_FOUND] error.
        ///
        /// Note: This operation is designed to be used for building permission-aware
        /// UIs and command-line tools, not for authorization checking. This operation
        /// may "fail open" without warning.
        pub async fn test_iam_permissions(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::iam::v1::TestIamPermissionsRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<
                super::super::super::super::iam::v1::TestIamPermissionsResponse,
            >,
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
                "/google.cloud.tasks.v2beta2.CloudTasks/TestIamPermissions",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.tasks.v2beta2.CloudTasks",
                        "TestIamPermissions",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists the tasks in a queue.
        ///
        /// By default, only the [BASIC][google.cloud.tasks.v2beta2.Task.View.BASIC]
        /// view is retrieved due to performance considerations;
        /// [response_view][google.cloud.tasks.v2beta2.ListTasksRequest.response_view]
        /// controls the subset of information which is returned.
        ///
        /// The tasks may be returned in any order. The ordering may change at any
        /// time.
        pub async fn list_tasks(
            &mut self,
            request: impl tonic::IntoRequest<super::ListTasksRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListTasksResponse>,
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
                "/google.cloud.tasks.v2beta2.CloudTasks/ListTasks",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.cloud.tasks.v2beta2.CloudTasks", "ListTasks"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets a task.
        pub async fn get_task(
            &mut self,
            request: impl tonic::IntoRequest<super::GetTaskRequest>,
        ) -> std::result::Result<tonic::Response<super::Task>, tonic::Status> {
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
                "/google.cloud.tasks.v2beta2.CloudTasks/GetTask",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.cloud.tasks.v2beta2.CloudTasks", "GetTask"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates a task and adds it to a queue.
        ///
        /// Tasks cannot be updated after creation; there is no UpdateTask command.
        ///
        /// * For [App Engine queues][google.cloud.tasks.v2beta2.AppEngineHttpTarget],
        /// the maximum task size is
        ///   100KB.
        /// * For [pull queues][google.cloud.tasks.v2beta2.PullTarget], the maximum
        /// task size is 1MB.
        pub async fn create_task(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateTaskRequest>,
        ) -> std::result::Result<tonic::Response<super::Task>, tonic::Status> {
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
                "/google.cloud.tasks.v2beta2.CloudTasks/CreateTask",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.tasks.v2beta2.CloudTasks",
                        "CreateTask",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes a task.
        ///
        /// A task can be deleted if it is scheduled or dispatched. A task
        /// cannot be deleted if it has completed successfully or permanently
        /// failed.
        pub async fn delete_task(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteTaskRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
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
                "/google.cloud.tasks.v2beta2.CloudTasks/DeleteTask",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.tasks.v2beta2.CloudTasks",
                        "DeleteTask",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Leases tasks from a pull queue for
        /// [lease_duration][google.cloud.tasks.v2beta2.LeaseTasksRequest.lease_duration].
        ///
        /// This method is invoked by the worker to obtain a lease. The
        /// worker must acknowledge the task via
        /// [AcknowledgeTask][google.cloud.tasks.v2beta2.CloudTasks.AcknowledgeTask]
        /// after they have performed the work associated with the task.
        ///
        /// The [payload][google.cloud.tasks.v2beta2.PullMessage.payload] is intended
        /// to store data that the worker needs to perform the work associated with the
        /// task. To return the payloads in the
        /// [response][google.cloud.tasks.v2beta2.LeaseTasksResponse], set
        /// [response_view][google.cloud.tasks.v2beta2.LeaseTasksRequest.response_view]
        /// to [FULL][google.cloud.tasks.v2beta2.Task.View.FULL].
        ///
        /// A maximum of 10 qps of
        /// [LeaseTasks][google.cloud.tasks.v2beta2.CloudTasks.LeaseTasks] requests are
        /// allowed per queue. [RESOURCE_EXHAUSTED][google.rpc.Code.RESOURCE_EXHAUSTED]
        /// is returned when this limit is
        /// exceeded. [RESOURCE_EXHAUSTED][google.rpc.Code.RESOURCE_EXHAUSTED]
        /// is also returned when
        /// [max_tasks_dispatched_per_second][google.cloud.tasks.v2beta2.RateLimits.max_tasks_dispatched_per_second]
        /// is exceeded.
        pub async fn lease_tasks(
            &mut self,
            request: impl tonic::IntoRequest<super::LeaseTasksRequest>,
        ) -> std::result::Result<
            tonic::Response<super::LeaseTasksResponse>,
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
                "/google.cloud.tasks.v2beta2.CloudTasks/LeaseTasks",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.tasks.v2beta2.CloudTasks",
                        "LeaseTasks",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Acknowledges a pull task.
        ///
        /// The worker, that is, the entity that
        /// [leased][google.cloud.tasks.v2beta2.CloudTasks.LeaseTasks] this task must
        /// call this method to indicate that the work associated with the task has
        /// finished.
        ///
        /// The worker must acknowledge a task within the
        /// [lease_duration][google.cloud.tasks.v2beta2.LeaseTasksRequest.lease_duration]
        /// or the lease will expire and the task will become available to be leased
        /// again. After the task is acknowledged, it will not be returned
        /// by a later [LeaseTasks][google.cloud.tasks.v2beta2.CloudTasks.LeaseTasks],
        /// [GetTask][google.cloud.tasks.v2beta2.CloudTasks.GetTask], or
        /// [ListTasks][google.cloud.tasks.v2beta2.CloudTasks.ListTasks].
        pub async fn acknowledge_task(
            &mut self,
            request: impl tonic::IntoRequest<super::AcknowledgeTaskRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
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
                "/google.cloud.tasks.v2beta2.CloudTasks/AcknowledgeTask",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.tasks.v2beta2.CloudTasks",
                        "AcknowledgeTask",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Renew the current lease of a pull task.
        ///
        /// The worker can use this method to extend the lease by a new
        /// duration, starting from now. The new task lease will be
        /// returned in the task's
        /// [schedule_time][google.cloud.tasks.v2beta2.Task.schedule_time].
        pub async fn renew_lease(
            &mut self,
            request: impl tonic::IntoRequest<super::RenewLeaseRequest>,
        ) -> std::result::Result<tonic::Response<super::Task>, tonic::Status> {
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
                "/google.cloud.tasks.v2beta2.CloudTasks/RenewLease",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.tasks.v2beta2.CloudTasks",
                        "RenewLease",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Cancel a pull task's lease.
        ///
        /// The worker can use this method to cancel a task's lease by
        /// setting its [schedule_time][google.cloud.tasks.v2beta2.Task.schedule_time]
        /// to now. This will make the task available to be leased to the next caller
        /// of [LeaseTasks][google.cloud.tasks.v2beta2.CloudTasks.LeaseTasks].
        pub async fn cancel_lease(
            &mut self,
            request: impl tonic::IntoRequest<super::CancelLeaseRequest>,
        ) -> std::result::Result<tonic::Response<super::Task>, tonic::Status> {
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
                "/google.cloud.tasks.v2beta2.CloudTasks/CancelLease",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.tasks.v2beta2.CloudTasks",
                        "CancelLease",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Forces a task to run now.
        ///
        /// When this method is called, Cloud Tasks will dispatch the task, even if
        /// the task is already running, the queue has reached its
        /// [RateLimits][google.cloud.tasks.v2beta2.RateLimits] or is
        /// [PAUSED][google.cloud.tasks.v2beta2.Queue.State.PAUSED].
        ///
        /// This command is meant to be used for manual debugging. For
        /// example, [RunTask][google.cloud.tasks.v2beta2.CloudTasks.RunTask] can be
        /// used to retry a failed task after a fix has been made or to manually force
        /// a task to be dispatched now.
        ///
        /// The dispatched task is returned. That is, the task that is returned
        /// contains the [status][google.cloud.tasks.v2beta2.Task.status] after the
        /// task is dispatched but before the task is received by its target.
        ///
        /// If Cloud Tasks receives a successful response from the task's
        /// target, then the task will be deleted; otherwise the task's
        /// [schedule_time][google.cloud.tasks.v2beta2.Task.schedule_time] will be
        /// reset to the time that
        /// [RunTask][google.cloud.tasks.v2beta2.CloudTasks.RunTask] was called plus
        /// the retry delay specified in the queue's
        /// [RetryConfig][google.cloud.tasks.v2beta2.RetryConfig].
        ///
        /// [RunTask][google.cloud.tasks.v2beta2.CloudTasks.RunTask] returns
        /// [NOT_FOUND][google.rpc.Code.NOT_FOUND] when it is called on a
        /// task that has already succeeded or permanently failed.
        ///
        /// [RunTask][google.cloud.tasks.v2beta2.CloudTasks.RunTask] cannot be called
        /// on a [pull task][google.cloud.tasks.v2beta2.PullMessage].
        pub async fn run_task(
            &mut self,
            request: impl tonic::IntoRequest<super::RunTaskRequest>,
        ) -> std::result::Result<tonic::Response<super::Task>, tonic::Status> {
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
                "/google.cloud.tasks.v2beta2.CloudTasks/RunTask",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.cloud.tasks.v2beta2.CloudTasks", "RunTask"),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
