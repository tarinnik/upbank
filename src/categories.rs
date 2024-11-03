use crate::common::SelfLink;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct CategoryList {
    pub data: Vec<Category>,
}

#[derive(Deserialize, Debug)]
pub struct Category {
    pub id: String,
    pub attributes: CategoryAttributes,
    pub relationships: CategoryRelationships,
    pub link: SelfLink,
}

#[derive(Deserialize, Debug)]
pub struct CategoryAttributes {
    pub name: String,
}

#[derive(Deserialize, Debug)]
pub struct CategoryRelationships {
    pub parent: CategoryParent,
    pub children: CategoryChildren,
}

#[derive(Deserialize, Debug)]
pub struct CategoryParent {
    pub data: Option<CategoryData>,
    pub links: Option<CategoryRelated>,
}

#[derive(Deserialize, Debug)]
pub struct CategoryChildren {
    pub data: Vec<CategoryData>,
    pub links: Option<CategoryRelated>,
}

#[derive(Deserialize, Debug)]
pub struct CategoryData {
    pub id: String,
}

#[derive(Deserialize, Debug)]
pub struct CategoryRelated {
    pub related: String,
}
