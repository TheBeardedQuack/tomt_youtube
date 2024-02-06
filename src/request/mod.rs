pub mod channel;
pub mod playlist;

use crate::resource::Resource;

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

    fn page(
        &self
    ) -> Option<String>;
}
