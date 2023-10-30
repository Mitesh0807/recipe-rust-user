use bson::oid::ObjectId;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json;
use serde_with::skip_serializing_none;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Ingredient {
    quantity: String,
    unit: String,
    name: String,
    extraNote: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Recipe {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub name: String,
    pub isActive: bool,
    pub img_Base64: String,
    pub slug: String,
    pub categoryId: Vec<ObjectId>,
    pub ingredients: Vec<Ingredient>,
    pub additionalNotes: String,
    pub description: String,
    pub cookingTime: String,
    pub __v: i32,
}
// createdAt: DateTime<Utc>,
// updatedAt: DateTime<Utc>,
#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct Category {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub description: String,
    pub img_Base64: String,
    pub isActive: bool,
    pub name: String,
    pub slug: String,
    pub subName: String,
    pub __v: i32,
}
// pub createdAt: String,
// pub updatedAt: String,
#[skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Response {
    pub success: bool,
    pub data: Option<Vec<Category>>,
    pub error_message: Option<String>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ResponseRecipe {
    pub success: bool,
    pub data: Option<Vec<Recipe>>,
    pub error_message: Option<String>,
}

#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct RecipesWithCategories {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub name: String,
    pub isActive: bool,
    pub img_Base64: String,
    pub slug: String,
    pub categoryId: Vec<ObjectId>,
    pub categories: Vec<Category>,
    pub ingredients: Vec<Ingredient>,
    pub additionalNotes: String,
    pub description: String,
    pub cookingTime: String,
    pub __v: i32,
}
impl RecipesWithCategories {
    pub fn new(
        id: ObjectId,
        name: String,
        isActive: bool,
        img_Base64: String,
        slug: String,
        categoryId: Vec<ObjectId>,
        ingredients: Vec<Ingredient>,
        additionalNotes: String,
        description: String,
        cookingTime: String,
        __v: i32,
    ) -> Self {
        RecipesWithCategories {
            id,
            name,
            isActive,
            img_Base64,
            slug,
            categoryId,
            categories: Vec::new(),
            ingredients,
            additionalNotes,
            description,
            cookingTime,
            __v,
        }
    }
}
#[skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ResponseRecipesWithCategories {
    pub success: bool,
    pub data: Option<Vec<RecipesWithCategories>>,
    pub error_message: Option<String>,
}

#[skip_serializing_none]
#[derive(Deserialize)]
pub struct Pagination {
    pub page: i32,
    pub per_page: i32,
    #[serde(default = "default_sort_by")]
    pub sort_by: String,
    #[serde(default = "default_order")]
    pub order: String,
}

fn default_sort_by() -> String {
    "name".to_string()
}

fn default_order() -> String {
    "asc".to_string()
}

impl Pagination {
    pub fn check(&self) -> Result<(), String> {
        if self.page < 1 {
            return Err("Page must be greater than or equal to 1.".into());
        }

        if self.per_page < 1 {
            return Err("Rows per page must be greater than or equal to 1.".into());
        } else if self.per_page > 100 {
            return Err("Rows per page must be less than or equal to 100.".into());
        }

        if !(["_id".to_string(), "name".to_string(), "email".to_string()].contains(&self.sort_by)) {
            return Err("Invalid value passed for sort_by query parameter. Must be one of: _id, email or name.".into());
        }

        if !(["asc".to_string(), "desc".to_string()]).contains(&self.order) {
            return Err(
                "Invalid value passed for order query parameter. Must be one of: asc or desc."
                    .into(),
            );
        }
        Ok(())
    }
}
