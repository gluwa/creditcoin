#[derive(Debug)]
pub enum TaskError<E, S> {
	Evaluation(E),
	Scheduler(S),
	/// the task id has been found in onchain storage, this is most likely a dupe.
	FinishedTask,
}
