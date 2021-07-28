#!/usr/bin/env bash
set -eu

current_dir=$(cd $(dirname ${BASH_SOURCE:-$0}); pwd)
cd $current_dir

features=(
  default
  ccc-hosted-marketplace-v2
  google-actions-sdk-v2
  google-actions-sdk-v2-conversation
  google-actions-sdk-v2-interactionmodel
  google-actions-sdk-v2-interactionmodel-prompt
  google-actions-sdk-v2-interactionmodel-type
  google-actions-type
  google-ads-admob-v1
  google-ads-googleads-v4-common
  google-ads-googleads-v4-enums
  google-ads-googleads-v4-errors
  google-ads-googleads-v4-resources
  google-ads-googleads-v4-services
  # google-ads-googleads-v5-common
  # google-ads-googleads-v5-enums
  # google-ads-googleads-v5-errors
  # google-ads-googleads-v5-resources
  # google-ads-googleads-v5-services
  # google-ads-googleads-v6-common
  # google-ads-googleads-v6-enums
  # google-ads-googleads-v6-errors
  # google-ads-googleads-v6-resources
  # google-ads-googleads-v6-services
  google-analytics-admin-v1alpha
  google-analytics-data-v1alpha
  google-analytics-data-v1beta
  google-api
  google-api-expr-v1alpha1
  google-api-expr-v1beta1
  google-api-servicecontrol-v1
  google-api-servicemanagement-v1
  google-appengine-legacy
  google-appengine-logging-v1
  google-appengine-v1
  google-appengine-v1beta
  google-apps-drive-activity-v2
  google-apps-script-type
  google-apps-script-type-calendar
  google-apps-script-type-docs
  google-apps-script-type-drive
  google-apps-script-type-gmail
  google-apps-script-type-sheets
  google-apps-script-type-slides
  google-area120-tables-v1alpha1
  google-assistant-embedded-v1alpha1
  google-assistant-embedded-v1alpha2
  google-bigtable-admin-cluster-v1
  google-bigtable-admin-table-v1
  google-bigtable-admin-v2
  google-bigtable-v1
  google-bigtable-v2
  google-bytestream
  google-chromeos-moblab-v1beta1
  google-cloud
  google-cloud-accessapproval-v1
  google-cloud-aiplatform-logging
  google-cloud-aiplatform-v1
  google-cloud-aiplatform-v1-schema-predict-instance
  google-cloud-aiplatform-v1-schema-predict-params
  google-cloud-aiplatform-v1-schema-predict-prediction
  google-cloud-aiplatform-v1-schema-trainingjob-definition
  google-cloud-aiplatform-v1beta1
  google-cloud-aiplatform-v1beta1-schema
  google-cloud-aiplatform-v1beta1-schema-predict-instance
  google-cloud-aiplatform-v1beta1-schema-predict-params
  google-cloud-aiplatform-v1beta1-schema-predict-prediction
  google-cloud-aiplatform-v1beta1-schema-trainingjob-definition
  google-cloud-apigateway-v1
  google-cloud-asset-v1
  google-cloud-asset-v1p1beta1
  google-cloud-asset-v1p2beta1
  google-cloud-asset-v1p4beta1
  # google-cloud-asset-v1p5beta1
  google-cloud-asset-v1p7beta1
  google-cloud-assuredworkloads-v1beta1
  google-cloud-audit
  google-cloud-automl-v1
  google-cloud-automl-v1beta1
  google-cloud-bigquery-connection-v1
  google-cloud-bigquery-connection-v1beta1
  google-cloud-bigquery-datatransfer-v1
  google-cloud-bigquery-logging-v1
  google-cloud-bigquery-reservation-v1
  google-cloud-bigquery-reservation-v1beta1
  google-cloud-bigquery-storage-v1
  google-cloud-bigquery-storage-v1alpha2
  google-cloud-bigquery-storage-v1beta1
  google-cloud-bigquery-storage-v1beta2
  google-cloud-bigquery-v2
  google-cloud-billing-budgets-v1
  google-cloud-billing-budgets-v1beta1
  google-cloud-billing-v1
  google-cloud-binaryauthorization-v1beta1
  google-cloud-channel-v1
  google-cloud-clouddms-logging-v1
  google-cloud-clouddms-v1
  google-cloud-datacatalog-v1
  google-cloud-datacatalog-v1beta1
  google-cloud-datalabeling-v1beta1
  google-cloud-dataproc-logging
  google-cloud-dataproc-v1
  google-cloud-dataproc-v1beta2
  google-cloud-dataqna-v1alpha
  google-cloud-dialogflow-cx-v3
  google-cloud-dialogflow-cx-v3beta1
  google-cloud-dialogflow-v2
  google-cloud-dialogflow-v2beta1
  google-cloud-documentai-v1
  google-cloud-documentai-v1beta1
  google-cloud-documentai-v1beta2
  google-cloud-documentai-v1beta3
  google-cloud-domains-v1alpha2
  google-cloud-domains-v1beta1
  google-cloud-functions-v1
  google-cloud-gaming-v1
  google-cloud-gaming-v1beta
  google-cloud-gkehub-v1alpha2
  google-cloud-gkehub-v1beta1
  google-cloud-gsuiteaddons-v1
  google-cloud-iot-v1
  google-cloud-kms-v1
  google-cloud-language-v1
  google-cloud-language-v1beta1
  google-cloud-language-v1beta2
  google-cloud-location
  google-cloud-managedidentities-v1
  google-cloud-managedidentities-v1beta1
  google-cloud-mediatranslation-v1alpha1
  google-cloud-mediatranslation-v1beta1
  google-cloud-memcache-v1
  google-cloud-memcache-v1beta2
  google-cloud-metastore-logging-v1
  google-cloud-metastore-v1alpha
  google-cloud-metastore-v1beta
  google-cloud-ml-v1
  google-cloud-networkconnectivity-v1alpha1
  google-cloud-notebooks-v1beta1
  google-cloud-orgpolicy-v1
  google-cloud-orgpolicy-v2
  google-cloud-osconfig-agentendpoint-v1
  google-cloud-osconfig-agentendpoint-v1beta
  google-cloud-osconfig-v1
  google-cloud-osconfig-v1beta
  google-cloud-oslogin-common
  google-cloud-oslogin-v1
  google-cloud-oslogin-v1alpha
  google-cloud-oslogin-v1beta
  google-cloud-phishingprotection-v1beta1
  google-cloud-policytroubleshooter-v1
  google-cloud-pubsublite-v1
  google-cloud-recaptchaenterprise-v1
  google-cloud-recaptchaenterprise-v1beta1
  google-cloud-recommendationengine-v1beta1
  google-cloud-recommender-logging-v1
  google-cloud-recommender-logging-v1beta1
  google-cloud-recommender-v1
  google-cloud-recommender-v1beta1
  google-cloud-redis-v1
  google-cloud-redis-v1beta1
  google-cloud-resourcemanager-v2
  google-cloud-resourcesettings-v1
  google-cloud-retail-v2
  google-cloud-retail-v2alpha
  google-cloud-retail-v2beta
  google-cloud-runtimeconfig-v1beta1
  google-cloud-saasaccelerator-management-logs-v1
  google-cloud-scheduler-v1
  google-cloud-scheduler-v1beta1
  google-cloud-secretmanager-v1
  google-cloud-secrets-v1beta1
  google-cloud-security-privateca-v1beta1
  google-cloud-securitycenter-settings-v1beta1
  google-cloud-securitycenter-v1
  google-cloud-securitycenter-v1beta1
  google-cloud-securitycenter-v1p1beta1
  google-cloud-servicedirectory-v1
  google-cloud-servicedirectory-v1beta1
  google-cloud-shell-v1
  google-cloud-speech-v1
  google-cloud-speech-v1p1beta1
  google-cloud-support-common
  google-cloud-support-v1alpha1
  google-cloud-talent-v4
  google-cloud-talent-v4beta1
  google-cloud-tasks-v2
  google-cloud-tasks-v2beta2
  google-cloud-tasks-v2beta3
  google-cloud-texttospeech-v1
  google-cloud-texttospeech-v1beta1
  google-cloud-translation-v3
  google-cloud-translation-v3beta1
  google-cloud-video-transcoder-v1beta1
  google-cloud-videointelligence-v1
  google-cloud-videointelligence-v1beta2
  google-cloud-videointelligence-v1p1beta1
  google-cloud-videointelligence-v1p2beta1
  google-cloud-videointelligence-v1p3beta1
  google-cloud-vision-v1
  google-cloud-vision-v1p1beta1
  google-cloud-vision-v1p2beta1
  google-cloud-vision-v1p3beta1
  google-cloud-vision-v1p4beta1
  google-cloud-vpcaccess-v1
  google-cloud-webrisk-v1
  google-cloud-webrisk-v1beta1
  google-cloud-websecurityscanner-v1
  google-cloud-websecurityscanner-v1alpha
  google-cloud-websecurityscanner-v1beta
  google-cloud-workflows-executions-v1
  google-cloud-workflows-executions-v1beta
  google-cloud-workflows-v1
  google-cloud-workflows-v1beta
  google-container-v1
  google-container-v1alpha1
  google-container-v1beta1
  google-datastore-admin-v1
  google-datastore-admin-v1beta1
  google-datastore-v1
  google-datastore-v1beta3
  google-devtools-artifactregistry-v1beta2
  google-devtools-build-v1
  google-devtools-cloudbuild-v1
  google-devtools-clouddebugger-v2
  google-devtools-clouderrorreporting-v1beta1
  google-devtools-cloudprofiler-v2
  google-devtools-cloudtrace-v1
  google-devtools-cloudtrace-v2
  # google-devtools-containeranalysis-v1
  google-devtools-containeranalysis-v1beta1
  google-devtools-remoteworkers-v1test2
  google-devtools-resultstore-v2
  google-devtools-source-v1
  google-devtools-sourcerepo-v1
  google-example-endpointsapis-v1
  google-example-library-v1
  google-firebase-fcm-connection-v1alpha1
  google-firestore-admin-v1
  google-firestore-admin-v1beta1
  google-firestore-admin-v1beta2
  # google-firestore-bundle
  google-firestore-v1
  google-firestore-v1beta1
  google-gapic-metadata
  google-genomics-v1
  google-genomics-v1alpha2
  google-geo-type
  google-home-enterprise-sdm-v1
  google-home-graph-v1
  google-iam-admin-v1
  google-iam-credentials-v1
  google-iam-v1
  google-iam-v1-logging
  google-iam-v1beta
  google-identity-accesscontextmanager-type
  google-identity-accesscontextmanager-v1
  google-logging-type
  google-logging-v2
  google-longrunning
  google-maps-playablelocations-v3
  google-maps-playablelocations-v3-sample
  google-maps-roads-v1op
  google-maps-routes-v1
  google-maps-routes-v1alpha
  google-maps-unity
  google-monitoring-dashboard-v1
  google-monitoring-v3
  google-partner-aistreams-v1alpha1
  google-privacy-dlp-v2
  google-pubsub-v1
  google-pubsub-v1beta2
  google-rpc
  google-rpc-context
  google-search-partnerdataingestion-logging-v1
  google-spanner-admin-database-v1
  google-spanner-admin-instance-v1
  google-spanner-v1
  google-storage-v1
  google-storagetransfer-v1
  google-streetview-publish-v1
  google-type
  google-watcher-v1
  grafeas-v1
  grafeas-v1beta1
  # grafeas-v1beta1-attestation
  grafeas-v1beta1-build
  grafeas-v1beta1-deployment
  # grafeas-v1beta1-discovery
  grafeas-v1beta1-image
  grafeas-v1beta1-package
  grafeas-v1beta1-provenance
  grafeas-v1beta1-source
  # grafeas-v1beta1-vulnerability
  storage-clouddms-logging-v1
)

for f in "${features[@]}"; do
  echo "feature: $f"
  cargo build -p googapis --features $f
done
