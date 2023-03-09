use super::model::tasks::TaskStatus;

/// Trait containing several functions needed for the operation for communication with the database
/// for obtaining tasks
trait TaskAllocation {
    fn new_task(&self);
    fn allocate_task(&self);
    fn task_completed(self, status: TaskStatus);
    fn get_new_task(&self);
}
