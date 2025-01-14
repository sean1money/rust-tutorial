#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use metrics::Counter;
use metrics_derive::Metrics;
fn main() {
    let metrics = MeteredReceiverMetrics::new("txpool.total-txs");
    metrics.messages_received_total.increment(1);
    {
        ::std::io::_print(format_args!("{0:?}\n", metrics.messages_received_total));
    };
}
#[metrics(dynamic = true)]
struct MeteredReceiverMetrics {
    /// Number of messages received
    messages_received_total: Counter,
}
#[automatically_derived]
impl ::core::clone::Clone for MeteredReceiverMetrics {
    #[inline]
    fn clone(&self) -> MeteredReceiverMetrics {
        MeteredReceiverMetrics {
            messages_received_total: ::core::clone::Clone::clone(
                &self.messages_received_total,
            ),
        }
    }
}
impl MeteredReceiverMetrics {
    /// Create new instance of metrics with provided scope.
    fn new(scope: &str) -> Self {
        MeteredReceiverMetrics::describe(scope);
        Self {
            messages_received_total: {
                let metric_key = ::metrics::Key::from_name(
                    ::alloc::__export::must_use({
                        let res = ::alloc::fmt::format(
                            format_args!(
                                "{0}{1}{2}",
                                scope,
                                ".",
                                "messages_received_total",
                            ),
                        );
                        res
                    }),
                );
                let metadata = {
                    static METADATA: ::metrics::Metadata<'static> = ::metrics::Metadata::new(
                        "metrics",
                        ::metrics::Level::INFO,
                        ::core::option::Option::Some("metrics"),
                    );
                    &METADATA
                };
                ::metrics::with_recorder(|recorder| {
                    recorder.register_counter(&metric_key, metadata)
                })
            },
        }
    }
    /// Create new instance of metrics with provided labels.
    fn new_with_labels(scope: &str, labels: impl metrics::IntoLabels + Clone) -> Self {
        Self {
            messages_received_total: {
                let metric_key = ::metrics::Key::from_parts(
                    ::alloc::__export::must_use({
                        let res = ::alloc::fmt::format(
                            format_args!(
                                "{0}{1}{2}",
                                scope,
                                ".",
                                "messages_received_total",
                            ),
                        );
                        res
                    }),
                    labels.clone(),
                );
                let metadata = {
                    static METADATA: ::metrics::Metadata<'static> = ::metrics::Metadata::new(
                        "metrics",
                        ::metrics::Level::INFO,
                        ::core::option::Option::Some("metrics"),
                    );
                    &METADATA
                };
                ::metrics::with_recorder(|recorder| {
                    recorder.register_counter(&metric_key, metadata)
                })
            },
        }
    }
    /// Describe all exposed metrics. Internally calls `describe_*` macros from
    /// the metrics crate according to the metric type.
    ///
    /// See <https://docs.rs/metrics/0.20.1/metrics/index.html#macros>
    fn describe(scope: &str) {
        {
            ::metrics::with_recorder(|recorder| {
                recorder
                    .describe_counter(
                        ::core::convert::Into::into(
                            ::alloc::__export::must_use({
                                let res = ::alloc::fmt::format(
                                    format_args!(
                                        "{0}{1}{2}",
                                        scope,
                                        ".",
                                        "messages_received_total",
                                    ),
                                );
                                res
                            }),
                        ),
                        ::core::option::Option::None,
                        ::core::convert::Into::into("Number of messages received"),
                    );
            });
        };
    }
}
impl std::fmt::Debug for MeteredReceiverMetrics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MeteredReceiverMetrics").finish()
    }
}
