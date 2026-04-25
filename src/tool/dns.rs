//! Shared helpers for mapping our flat tool input to the generated
//! `api::types::DnsRecord` variants.

use std::net::{Ipv4Addr, Ipv6Addr};

use rmcp::ErrorData as McpError;
use serde::Deserialize;

use crate::api::types::{
    A, AType, Aaaa, AaaaType, Cname, CnameType, DnsRecord, Mx, MxType, Srv, SrvType, Tlsa,
    TlsaType, Txt, TxtType,
};

use super::create_dns_record;

#[derive(Debug, Clone, Copy, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "UPPERCASE")]
pub enum RecordType {
    A,
    Aaaa,
    Cname,
    Mx,
    Srv,
    Tlsa,
    Txt,
}

pub fn build_dns_record(p: create_dns_record::Params, id: i64) -> Result<DnsRecord, McpError> {
    let ttl = p.ttl.unwrap_or(3600);
    let host = p.host;
    let data = p.data;

    match p.record_type {
        RecordType::A => {
            let ip: Ipv4Addr = data
                .parse()
                .map_err(|_| invalid(format!("`data` must be a valid IPv4 address, got {data:?}")))?;
            Ok(DnsRecord::A(A { data: ip, host, id, ttl, type_: AType::A }))
        }
        RecordType::Aaaa => {
            let ip: Ipv6Addr = data
                .parse()
                .map_err(|_| invalid(format!("`data` must be a valid IPv6 address, got {data:?}")))?;
            Ok(DnsRecord::Aaaa(Aaaa {
                data: ip,
                host,
                id,
                ttl,
                type_: AaaaType::Aaaa,
            }))
        }
        RecordType::Cname => Ok(DnsRecord::Cname(Cname {
            data,
            host,
            id,
            ttl,
            type_: CnameType::Cname,
        })),
        RecordType::Mx => {
            let priority = required(p.priority, "priority", "MX")?;
            Ok(DnsRecord::Mx(Mx {
                data,
                host,
                id,
                ttl,
                priority,
                type_: MxType::Mx,
            }))
        }
        RecordType::Srv => {
            let priority = required(p.priority, "priority", "SRV")?;
            let weight = required(p.weight, "weight", "SRV")?;
            let port = required(p.port, "port", "SRV")?;
            Ok(DnsRecord::Srv(Srv {
                data,
                host,
                id,
                ttl,
                priority,
                weight,
                port,
                type_: SrvType::Srv,
            }))
        }
        RecordType::Tlsa => {
            let usage = required(p.usage, "usage", "TLSA")?;
            let selector = required(p.selector, "selector", "TLSA")?;
            let dtype = required(p.matching_type, "matching_type", "TLSA")?;
            Ok(DnsRecord::Tlsa(Tlsa {
                data: Some(data),
                host,
                id,
                ttl,
                usage,
                selector,
                dtype,
                type_: Some(TlsaType::Tlsa),
            }))
        }
        RecordType::Txt => Ok(DnsRecord::Txt(Txt {
            data,
            host,
            id,
            ttl,
            type_: TxtType::Txt,
        })),
    }
}

fn required<T>(value: Option<T>, field: &str, record_type: &str) -> Result<T, McpError> {
    value.ok_or_else(|| invalid(format!("`{field}` is required for {record_type} records")))
}

fn invalid(msg: String) -> McpError {
    McpError::invalid_params(msg, None)
}
