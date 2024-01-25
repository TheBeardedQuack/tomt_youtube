use std::hash::Hash;

pub mod channel;

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

pub trait RscPart
{
    type ForRsc: Resource;
    type Backing: RscPart<
        ForRsc = Self::ForRsc
    >;

    const PART_KEY: <Self::ForRsc as Resource>::PartKey;
    const PART_NAME: &'static str;
}
