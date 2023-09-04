//! https://cloud.tencent.com/document/api/1427/56194#.E8.AE.B0.E5.BD.95.E7.9B.B8.E5.85.B3.E6.8E.A5.E5.8F.A3

mod create_record;
mod delete_record;
mod describe_record;
mod describe_record_list;
mod modify_dynamic_dns;
mod modify_record;

pub use create_record::*;
pub use delete_record::*;
pub use describe_record::*;
pub use describe_record_list::*;
pub use modify_dynamic_dns::*;
pub use modify_record::*;
