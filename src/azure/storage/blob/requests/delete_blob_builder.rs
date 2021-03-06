use crate::azure::core::errors::{check_status_extract_headers_and_body, AzureError};
use crate::azure::core::lease::LeaseId;
use crate::azure::core::{
    BlobNameRequired, BlobNameSupport, ClientRequestIdOption, ClientRequestIdSupport, ClientRequired, ContainerNameRequired,
    ContainerNameSupport, DeleteSnapshotsMethodRequired, DeleteSnapshotsMethodSupport, LeaseIdOption, LeaseIdSupport, TimeoutOption,
    TimeoutSupport,
};
use crate::azure::core::{DeleteSnapshotsMethod, No, ToAssign, Yes};
use crate::azure::storage::blob::generate_blob_uri;
use crate::azure::storage::blob::responses::DeleteBlobResponse;
use crate::azure::storage::client::Client;
use futures::future::{done, Future};
use hyper::{Method, StatusCode};
use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct DeleteBlobBuilder<'a, ContainerNameSet, BlobNameSet, DeleteSnapshotMethodSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    DeleteSnapshotMethodSet: ToAssign,
{
    client: &'a Client,
    p_container_name: PhantomData<ContainerNameSet>,
    p_blob_name: PhantomData<BlobNameSet>,
    p_delete_snapshots_method: PhantomData<DeleteSnapshotMethodSet>,
    container_name: Option<&'a str>,
    blob_name: Option<&'a str>,
    delete_snapshots_method: DeleteSnapshotsMethod,
    timeout: Option<u64>,
    lease_id: Option<&'a LeaseId>,
    client_request_id: Option<&'a str>,
}

impl<'a> DeleteBlobBuilder<'a, No, No, No> {
    #[inline]
    pub(crate) fn new(client: &'a Client) -> DeleteBlobBuilder<'a, No, No, No> {
        DeleteBlobBuilder {
            client,
            p_container_name: PhantomData {},
            container_name: None,
            p_blob_name: PhantomData {},
            blob_name: None,
            p_delete_snapshots_method: PhantomData {},
            delete_snapshots_method: DeleteSnapshotsMethod::Include,
            timeout: None,
            lease_id: None,
            client_request_id: None,
        }
    }
}

impl<'a, ContainerNameSet, BlobNameSet, DeleteSnapshotMethodSet> ClientRequired<'a>
    for DeleteBlobBuilder<'a, ContainerNameSet, BlobNameSet, DeleteSnapshotMethodSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    DeleteSnapshotMethodSet: ToAssign,
{
    #[inline]
    fn client(&self) -> &'a Client {
        self.client
    }
}

impl<'a, BlobNameSet, DeleteSnapshotMethodSet> ContainerNameRequired<'a>
    for DeleteBlobBuilder<'a, Yes, BlobNameSet, DeleteSnapshotMethodSet>
where
    BlobNameSet: ToAssign,
    DeleteSnapshotMethodSet: ToAssign,
{
    #[inline]
    fn container_name(&self) -> &'a str {
        self.container_name.unwrap()
    }
}

impl<'a, ContainerNameSet, DeleteSnapshotMethodSet> BlobNameRequired<'a>
    for DeleteBlobBuilder<'a, ContainerNameSet, Yes, DeleteSnapshotMethodSet>
where
    ContainerNameSet: ToAssign,
    DeleteSnapshotMethodSet: ToAssign,
{
    #[inline]
    fn blob_name(&self) -> &'a str {
        self.blob_name.unwrap()
    }
}

impl<'a, ContainerNameSet, BlobNameSet> DeleteSnapshotsMethodRequired for DeleteBlobBuilder<'a, ContainerNameSet, BlobNameSet, Yes>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
{
    #[inline]
    fn delete_snapshots_method(&self) -> DeleteSnapshotsMethod {
        self.delete_snapshots_method
    }
}

impl<'a, ContainerNameSet, BlobNameSet, DeleteSnapshotMethodSet> TimeoutOption
    for DeleteBlobBuilder<'a, ContainerNameSet, BlobNameSet, DeleteSnapshotMethodSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    DeleteSnapshotMethodSet: ToAssign,
{
    #[inline]
    fn timeout(&self) -> Option<u64> {
        self.timeout
    }
}

impl<'a, ContainerNameSet, BlobNameSet, DeleteSnapshotMethodSet> LeaseIdOption<'a>
    for DeleteBlobBuilder<'a, ContainerNameSet, BlobNameSet, DeleteSnapshotMethodSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    DeleteSnapshotMethodSet: ToAssign,
{
    #[inline]
    fn lease_id(&self) -> Option<&'a LeaseId> {
        self.lease_id
    }
}

impl<'a, ContainerNameSet, BlobNameSet, DeleteSnapshotMethodSet> ClientRequestIdOption<'a>
    for DeleteBlobBuilder<'a, ContainerNameSet, BlobNameSet, DeleteSnapshotMethodSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    DeleteSnapshotMethodSet: ToAssign,
{
    #[inline]
    fn client_request_id(&self) -> Option<&'a str> {
        self.client_request_id
    }
}

impl<'a, ContainerNameSet, BlobNameSet, DeleteSnapshotMethodSet> ContainerNameSupport<'a>
    for DeleteBlobBuilder<'a, ContainerNameSet, BlobNameSet, DeleteSnapshotMethodSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    DeleteSnapshotMethodSet: ToAssign,
{
    type O = DeleteBlobBuilder<'a, Yes, BlobNameSet, DeleteSnapshotMethodSet>;

    #[inline]
    fn with_container_name(self, container_name: &'a str) -> Self::O {
        DeleteBlobBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
            p_delete_snapshots_method: PhantomData {},
            container_name: Some(container_name),
            blob_name: self.blob_name,
            delete_snapshots_method: self.delete_snapshots_method,
            timeout: self.timeout,
            lease_id: self.lease_id,
            client_request_id: self.client_request_id,
        }
    }
}

impl<'a, ContainerNameSet, BlobNameSet, DeleteSnapshotMethodSet> BlobNameSupport<'a>
    for DeleteBlobBuilder<'a, ContainerNameSet, BlobNameSet, DeleteSnapshotMethodSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    DeleteSnapshotMethodSet: ToAssign,
{
    type O = DeleteBlobBuilder<'a, ContainerNameSet, Yes, DeleteSnapshotMethodSet>;

    #[inline]
    fn with_blob_name(self, blob_name: &'a str) -> Self::O {
        DeleteBlobBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
            p_delete_snapshots_method: PhantomData {},
            container_name: self.container_name,
            blob_name: Some(blob_name),
            delete_snapshots_method: self.delete_snapshots_method,
            timeout: self.timeout,
            lease_id: self.lease_id,
            client_request_id: self.client_request_id,
        }
    }
}

impl<'a, ContainerNameSet, BlobNameSet, DeleteSnapshotMethodSet> DeleteSnapshotsMethodSupport
    for DeleteBlobBuilder<'a, ContainerNameSet, BlobNameSet, DeleteSnapshotMethodSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    DeleteSnapshotMethodSet: ToAssign,
{
    type O = DeleteBlobBuilder<'a, ContainerNameSet, BlobNameSet, Yes>;

    #[inline]
    fn with_delete_snapshots_method(self, delete_snapshots_method: DeleteSnapshotsMethod) -> Self::O {
        DeleteBlobBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
            p_delete_snapshots_method: PhantomData {},
            container_name: self.container_name,
            blob_name: self.blob_name,
            delete_snapshots_method,
            timeout: self.timeout,
            lease_id: self.lease_id,
            client_request_id: self.client_request_id,
        }
    }
}

impl<'a, ContainerNameSet, BlobNameSet, DeleteSnapshotMethodSet> TimeoutSupport
    for DeleteBlobBuilder<'a, ContainerNameSet, BlobNameSet, DeleteSnapshotMethodSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    DeleteSnapshotMethodSet: ToAssign,
{
    type O = DeleteBlobBuilder<'a, ContainerNameSet, BlobNameSet, DeleteSnapshotMethodSet>;

    #[inline]
    fn with_timeout(self, timeout: u64) -> Self::O {
        DeleteBlobBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
            p_delete_snapshots_method: PhantomData {},
            container_name: self.container_name,
            blob_name: self.blob_name,
            delete_snapshots_method: self.delete_snapshots_method,
            timeout: Some(timeout),
            lease_id: self.lease_id,
            client_request_id: self.client_request_id,
        }
    }
}

impl<'a, ContainerNameSet, BlobNameSet, DeleteSnapshotMethodSet> LeaseIdSupport<'a>
    for DeleteBlobBuilder<'a, ContainerNameSet, BlobNameSet, DeleteSnapshotMethodSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    DeleteSnapshotMethodSet: ToAssign,
{
    type O = DeleteBlobBuilder<'a, ContainerNameSet, BlobNameSet, DeleteSnapshotMethodSet>;

    #[inline]
    fn with_lease_id(self, lease_id: &'a LeaseId) -> Self::O {
        DeleteBlobBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
            p_delete_snapshots_method: PhantomData {},
            container_name: self.container_name,
            blob_name: self.blob_name,
            delete_snapshots_method: self.delete_snapshots_method,
            timeout: self.timeout,
            lease_id: Some(lease_id),
            client_request_id: self.client_request_id,
        }
    }
}

impl<'a, ContainerNameSet, BlobNameSet, DeleteSnapshotMethodSet> ClientRequestIdSupport<'a>
    for DeleteBlobBuilder<'a, ContainerNameSet, BlobNameSet, DeleteSnapshotMethodSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    DeleteSnapshotMethodSet: ToAssign,
{
    type O = DeleteBlobBuilder<'a, ContainerNameSet, BlobNameSet, DeleteSnapshotMethodSet>;

    #[inline]
    fn with_client_request_id(self, client_request_id: &'a str) -> Self::O {
        DeleteBlobBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
            p_delete_snapshots_method: PhantomData {},
            container_name: self.container_name,
            blob_name: self.blob_name,
            delete_snapshots_method: self.delete_snapshots_method,
            timeout: self.timeout,
            lease_id: self.lease_id,
            client_request_id: Some(client_request_id),
        }
    }
}

// methods callable regardless
impl<'a, ContainerNameSet, BlobNameSet, DeleteSnapshotMethodSet>
    DeleteBlobBuilder<'a, ContainerNameSet, BlobNameSet, DeleteSnapshotMethodSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    DeleteSnapshotMethodSet: ToAssign,
{
}

impl<'a> DeleteBlobBuilder<'a, Yes, Yes, Yes> {
    pub fn finalize(self) -> impl Future<Item = DeleteBlobResponse, Error = AzureError> {
        let mut uri = generate_blob_uri(&self, None);

        if let Some(nm) = TimeoutOption::to_uri_parameter(&self) {
            uri = format!("{}?{}", uri, nm);
        }

        trace!("delete_blob uri == {:?}", uri);

        let req = self.client().perform_request(
            &uri,
            &Method::DELETE,
            |ref mut request| {
                DeleteSnapshotsMethodRequired::add_header(&self, request);
                LeaseIdOption::add_header(&self, request);
                ClientRequestIdOption::add_header(&self, request);
            },
            None,
        );

        done(req)
            .from_err()
            .and_then(move |future_response| check_status_extract_headers_and_body(future_response, StatusCode::ACCEPTED))
            .and_then(|(headers, _body)| done(DeleteBlobResponse::from_headers(&headers)))
    }
}
