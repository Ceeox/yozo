use uuid::Uuid;

// TODO: remove me when query is implemented
#[allow(unused)]
pub(crate) struct Size {
    width: u32,
    height: u32,
}

// TODO: remove me when query is implemented
#[allow(unused)]
pub(crate) enum Status {
    Waiting,
    Verifying,
    Converting,
    Saving,
}

// TODO: remove me when query is implemented
#[allow(unused)]
pub(crate) struct Job {
    id: Uuid,
    worker_id: Option<Uuid>,
    filename: String,
    file_id: Uuid,
    sizes: Vec<Size>,
    status: Status,
}
