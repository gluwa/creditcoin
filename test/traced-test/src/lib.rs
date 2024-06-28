use std::sync::{Arc, Mutex};
use tracing_core::{dispatcher::DefaultGuard, Dispatch};
use tracing_subscriber::fmt::MakeWriter;

#[derive(Default, Clone)]
struct BufWriter {
	buf: Arc<Mutex<Vec<u8>>>,
	print_to_stdout: bool,
}

impl std::io::Write for BufWriter {
	fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
		if self.print_to_stdout {
			print!("{}", String::from_utf8_lossy(buf));
		}
		self.buf.lock().unwrap().write(buf)
	}
	fn flush(&mut self) -> std::io::Result<()> {
		self.buf.lock().unwrap().flush()
	}
}

impl MakeWriter<'_> for BufWriter {
	type Writer = Self;

	fn make_writer(&self) -> Self {
		self.clone()
	}
}

/// Logs collected during the test. Logs are
/// only collected while this object is alive.
pub struct TestLogs {
	buf: Arc<Mutex<Vec<u8>>>,
	_guard: DefaultGuard,
}

#[derive(Default)]
/// Configuration of how to collect the logs during the test.
pub struct TestTraceConfig {
	/// The filter to apply to the tracing subscriber.
	/// It is the same syntax that you would use with the `RUST_LOG` environment variable.
	pub filter: Option<String>,
	/// If `true`, the logs will be printed to stdout.
	/// This is useful for debugging a failing test.
	pub print_to_stdout: bool,
}

impl TestLogs {
	/// Initializes the test log collection with a `trace` filter and no printing to stdout.
	pub fn init() -> Self {
		Self::init_with_config(Default::default())
	}

	pub fn init_with_config(config: TestTraceConfig) -> Self {
		let buf = Arc::new(Mutex::new(Vec::new()));
		let writer = BufWriter { buf: buf.clone(), print_to_stdout: config.print_to_stdout };
		let subscriber =
			make_subscriber(writer, &config.filter.unwrap_or_else(|| "trace".to_string()));
		let guard = tracing_core::dispatcher::set_default(&subscriber);
		Self { buf, _guard: guard }
	}

	pub fn init_with_filter(filter: &str) -> Self {
		Self::init_with_config(TestTraceConfig {
			filter: Some(filter.into()),
			..Default::default()
		})
	}

	/// Returns the lines logged up to this point.
	pub fn lines(&self) -> Vec<String> {
		let buf = self.buf.lock().unwrap();
		String::from_utf8(buf.clone()).unwrap().lines().map(|s| s.to_string()).collect()
	}

	/// Returns `true` if the given string is contained in any of the logged lines.
	pub fn contain(&self, s: &str) -> bool {
		self.lines().iter().any(|l| l.contains(s))
	}

	#[doc(alias = "contain")]
	pub fn contains(&self, s: &str) -> bool {
		self.contain(s)
	}

	/// Returns the contents of the logs collected up to this point.
	pub fn contents(&self) -> String {
		let buf = self.buf.lock().unwrap();
		String::from_utf8(buf.clone()).unwrap()
	}
}

/// Starts collecting logs with a `trace` filter.
/// Returns a `TestLogs` object that can be used to inspect the logs.
/// The logs are only collected while the returned object is alive.
///
/// # Example
/// ```
/// fn my_cool_test() {
///     let logs = traced_test::trace();
///     tracing::info!("Hello");
///     assert!(logs.contain("Hello"));
/// }
/// ```
pub fn trace() -> TestLogs {
	TestLogs::init()
}

/// Starts collecting logs with the given filter.
pub fn trace_with_filter(filter: &str) -> TestLogs {
	TestLogs::init_with_filter(filter)
}

/// Starts collecting logs with a `trace` filter and prints the logs to stdout. Useful for debugging.
pub fn trace_and_print() -> TestLogs {
	TestLogs::init_with_config(TestTraceConfig { print_to_stdout: true, ..Default::default() })
}

/// Starts collecting logs with the given config.
pub fn trace_with_config(config: TestTraceConfig) -> TestLogs {
	TestLogs::init_with_config(config)
}

fn make_subscriber(writer: BufWriter, filter: &str) -> Dispatch {
	tracing_subscriber::fmt()
		.with_env_filter(filter)
		.with_writer(writer)
		.with_ansi(false)
		.finish()
		.into()
}

#[test]
fn trace_works() {
	let logs = trace();
	tracing::info!("Hello");
	assert!(logs.contain("Hello"));
	assert!(!logs.contain("World"));
}

#[test]
fn trace_filter_works() {
	let logs = trace_with_filter("info");
	tracing::info!("Hello");
	assert!(logs.contain("Hello"));
	tracing::debug!("World");
	assert!(!logs.contain("World"));
}
