use opentelemetry::trace::TracerProvider as _;
use opentelemetry::{global, KeyValue};
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::{runtime, trace::TracerProvider, Resource};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

use crate::environment::Environment;

pub fn init(env: &Environment) -> anyhow::Result<Option<TracerProvider>> {
    let env_filter = EnvFilter::from_default_env().add_directive(tracing::Level::INFO.into());

    if let Some(ref otlp_endpoint) = env.otlp_endpoint {
        let exporter = opentelemetry_otlp::SpanExporter::builder()
            .with_tonic()
            .with_endpoint(otlp_endpoint)
            .build()?;

        let resource = Resource::new(vec![KeyValue::new("service.name", "oxidize")]);

        let provider = TracerProvider::builder()
            .with_batch_exporter(exporter, runtime::Tokio)
            .with_resource(resource)
            .build();

        global::set_tracer_provider(provider.clone());
        let tracer = provider.tracer("oxidize");
        let telemetry_layer = tracing_opentelemetry::layer().with_tracer(tracer);

        tracing_subscriber::registry()
            .with(env_filter)
            .with(tracing_subscriber::fmt::layer())
            .with(telemetry_layer)
            .init();

        tracing::info!("OpenTelemetry initialized with endpoint: {}", otlp_endpoint);
        Ok(Some(provider))
    } else {
        tracing_subscriber::registry()
            .with(env_filter)
            .with(tracing_subscriber::fmt::layer())
            .init();

        tracing::info!("OpenTelemetry disabled (OTLP_ENDPOINT not set)");
        Ok(None)
    }
}

pub fn shutdown(provider: Option<TracerProvider>) {
    if let Some(provider) = provider {
        if let Err(e) = provider.shutdown() {
            tracing::error!("Failed to shutdown OpenTelemetry: {:?}", e);
        }
    }
}
