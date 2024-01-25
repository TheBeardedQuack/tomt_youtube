use std::hash::Hash;

pub mod channel;
pub mod playlist;

pub trait Resource:
{
    type Id: Clone + PartialEq + Hash;
    type PartKey: Copy + Clone + PartialEq;
    type Backing: Resource<
        Id = Self::Id,
        PartKey = Self::PartKey,
        Backing = Self::Backing,
    >;

    const RSC_NAME: &'static str;
}

pub trait RscId:
    Resource
{
    fn id(
        &self
    ) -> &Self::Id;
}

pub trait RscPart<R: Resource>
{
    type Backing: RscPart<R, Backing = Self::Backing>;

    const PART_KEY: R::PartKey;
    const PART_NAME: &'static str;
}
