extern crate clap;
extern crate reqwest;
extern crate rusoto_core;
extern crate rusoto_route53;
#[macro_use] extern crate quick_error;

use clap::{App, Arg, AppSettings};
use rusoto_core::{region::Region};
use rusoto_route53::{Route53, Route53Client,
    ChangeResourceRecordSetsRequest,
    ChangeBatch, Change,
    ResourceRecordSet, ResourceRecord,
    ListHostedZonesByNameRequest
};

mod errors;

type Result<T> = ::std::result::Result<T, errors::Error>;

fn main() -> Result<()> {
    let app =
        App::new(env!("CARGO_PKG_NAME"))
            .version(env!("CARGO_PKG_VERSION"))
            .author("Ferruccio Barletta")
            .about(env!("CARGO_PKG_DESCRIPTION"))
            .setting(AppSettings::ColoredHelp)
            .setting(AppSettings::UnifiedHelpMessage)
            .arg(Arg::with_name("host")
                .value_name("HOST")
                .help("host name")
                .index(1)
                .required(true))
            .arg(Arg::with_name("domain")
                .value_name("DOMAIN")
                .help("domain name")
                .index(2)
                .required(true))
            .get_matches();

    let ipv4 = metadata("public-ipv4")?;

    let host = app.value_of("host").unwrap_or("").to_owned();
    let domain = app.value_of("domain").unwrap_or("").to_owned();
    let dnsname = format!("{}.{}", host, domain);
    let zone_domain = if domain.ends_with(".") {
        domain.clone()
    } else {
        domain.clone() + "."
    };

    let r53client = Route53Client::simple(Region::UsEast1);

    match get_zone_id(&r53client, zone_domain)? {
        Some(zone) => update(&r53client, UpdateOptions {
            zone: zone,
            dnsname: dnsname,
            ipv4: ipv4
        })?,
        None => println!("unknown hosted zone: {}", domain)
    }

    Ok(())
}

fn metadata(name: &str) -> Result<String> {
    let req = format!("http://169.254.169.254/latest/meta-data/{}", name);
    Ok(reqwest::get(&req)?.text()?)
}

fn get_zone_id(r53client: &Route53Client, name: String) -> Result<Option<String>> {
    let req = ListHostedZonesByNameRequest {
        dns_name: Some(name.clone()),
        hosted_zone_id: None,
        max_items: None
    };
    let rsp = r53client.list_hosted_zones_by_name(&req).sync()?;
    for zone in rsp.hosted_zones {
        if zone.name == name {
            if let Some(config) = zone.config {
                if let Some(private) = config.private_zone {
                    if !private {
                        let prefix = "/hostedzone/";
                        let zone_id = if zone.id.starts_with(prefix) {
                            zone.id[prefix.len()..].to_owned()
                        } else {
                            zone.id.clone()
                        };
                        return Ok(Some(zone_id));
                    }
                }
            }
        }
    }
    Ok(None)
}

struct UpdateOptions {
    zone: String,
    dnsname: String,
    ipv4: String
}

fn update(r53client: &Route53Client, options: UpdateOptions) -> Result<()> {
    let req = ChangeResourceRecordSetsRequest {
        change_batch: ChangeBatch {
            changes: vec![
                Change {
                    action: "UPSERT".to_owned(),
                    resource_record_set: ResourceRecordSet {
                        name: options.dnsname,
                        type_: "A".to_owned(),
                        resource_records: Some(vec![
                            ResourceRecord {
                                value: options.ipv4
                            }
                        ]),
                        ttl: Some(60),
                        ..Default::default()
                    }
                }
            ],
            comment: Some("r53up change".to_owned())
        },
        hosted_zone_id: options.zone
    };
    let rsp = r53client.change_resource_record_sets(&req).sync()?;
    println!("update status: {}", rsp.change_info.status);
    Ok(())
}