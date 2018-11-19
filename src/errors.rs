use rusoto_core::{CredentialsError};
use rusoto_route53::{ChangeResourceRecordSetsError, ListHostedZonesByNameError};

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        CredentialsError(err: CredentialsError) {
            from()
            description(err.description())
            display("credentials error: {}", err)
            cause(err)
        }
        ReqError(err: ::reqwest::Error) {
            from()
            description(err.description())
            display("reqwest error: {}", err)
            cause(err)
        }
        ListHostedZonesByNameError(err: ListHostedZonesByNameError) {
            from()
            description(err.description())
            display("route 53: {}", err)
            cause(err)
        }
        ChangeResourceRecordSetsError(err: ChangeResourceRecordSetsError) {
            from()
            description(err.description())
            display("route 53: {}", err)
            cause(err)
        }
    }
}