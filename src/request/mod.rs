pub mod channel;

use crate::resources::Resource;

pub trait RscRequest<Rsc: Resource>
{
    fn rsc_name(
        &self
    ) -> &'static str {
        Rsc::RSC_NAME
    }

    fn ids(
        &self
    ) -> &Vec<Rsc::Id>;

    fn parts(
        &self
    ) -> &Vec<Rsc::PartKey>;
}
