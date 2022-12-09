use tracing::{Subscriber, subscriber::set_global_default};
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry};
use tracing_subscriber::fmt::MakeWriter;


// Sink refers to the output writer, e.g. std::io::stdout
// we return a thread safe Subscriber
pub fn get_subscriber<Sink>(name: String, env_filter: String,sink: Sink) ->
        impl Subscriber + Send + Sync
        where Sink: for<'a> MakeWriter<'a> + Send + Sync + 'static,
// Sink is a type that can create io::Write instances.
// MakeWriter is used by fmt::Layer or fmt::Subscriber to print formatted text representations of Events
{
    // derive env_filter from RUST_LOG environment variable e.g. `info`
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new(env_filter));

    let formatting_layer = BunyanFormattingLayer::new(name,sink);

    // the registry stores span data which is exposed via each layer, it implements the `Subscriber` trait.
    Registry::default()
        // discard spans based on log levels and origin depending on `RUST_LOG` env variable
        .with(env_filter)
        // process the span data and sotres associated metadata in JSON. Propagates span from parent ctx -> children
        .with(JsonStorageLayer)
        // builder atop of the JsonStore Layer ans outputs logs in bunyan-compatible JSON
        .with(formatting_layer)
}

pub fn init_subscriber(subscriber: impl Subscriber + Send + Sync) {
    // convert log events to tracing
    LogTracer::init().expect("Failed to set logger");
    set_global_default(subscriber).expect("Failed to set subscriber");
}