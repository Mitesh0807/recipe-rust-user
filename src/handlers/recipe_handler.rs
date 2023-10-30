use crate::structs::comman::DatabaseConfig;
use crate::structs::schema::{Category, Recipe};
use crate::structs::schema::{
    RecipesWithCategories, Response, ResponseRecipe, ResponseRecipesWithCategories,
};
use axum::{
    http::{header, HeaderValue, StatusCode},
    routing::{get, post},
    Router,
};
use futures::stream::StreamExt;
use mongodb::bson::{doc, oid::ObjectId};
use mongodb::{options::ClientOptions, options::FindOptions, Client, Collection, Database};
pub async fn get_all_categories(
    axum::extract::State(client): axum::extract::State<Client>,
) -> impl axum::response::IntoResponse {
    let client = client.clone();
    let db = client.database("Recipe");
    let category_collection: Collection<Category> = db.collection::<Category>("categories");
    let mut category_cursor = category_collection
        .find(None, None)
        .await
        .expect("Could not find categories");
    let mut categories: Vec<Category> = Vec::new();
    while let Some(category) = category_cursor.next().await {
        match category {
            Ok(category) => {
                categories.push(category);
            }
            Err(err) => {
                println!("{:#?}", err);
            }
        }
    }
    let response = Response {
        success: true,
        data: Some(categories),
        error_message: None,
    };
    (StatusCode::OK, axum::Json(response))
}

pub async fn get_categorie_by_id(
    axum::extract::State(client): axum::extract::State<Client>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> impl axum::response::IntoResponse {
    let client = client.clone();
    let db = client.database("Recipe");
    let collection_id = ObjectId::parse_str(&id).unwrap();
    let category_collection: Collection<Category> = db.collection::<Category>("categories");
    let mut categories: Vec<Category> = Vec::new();
    let category_cursor = category_collection
        .find_one(
            Some(doc! {
                "_id": collection_id,
            }),
            None,
        )
        .await
        .expect("Could not find categories");
    categories.push(category_cursor.unwrap());
    let response = Response {
        success: true,
        data: Some(categories),
        error_message: None,
    };
    (StatusCode::OK, axum::Json(response))
}
// pub async fn create_category(
//     axum::extract::State(client): axum::extract::State<Client>,
//     axum::extract::Json(category): axum::extract::Json<MyDocument>,
// ) -> impl axum::response::IntoResponse {
//     let client = client.clone();
//     let db = client.database("Recipe");
//     //categories
//     let collection_id = ObjectId::parse_str(&category.id).unwrap();
//     let category_collection: Collection<MyDocument> = db.collection::<MyDocument>("categories");
//     let mut categories: Vec<MyDocument> = Vec::new();
//     let category_cursor = category_collection
//         .find_one(
//             Some(doc! {
//                 "_id": collection_id,
//             }),
//             None,
//         )
//         .await;
//     match category_cursor {
//         Ok(category) => {
//             categories.push(category.unwrap());
//         }
//         Err(err) => {
//             println!("{:#?}", err);
//         }
//     }
//     let response = Response {
//         success: true,
//         data: Some(categories),
//         error_message: None,
//     };
//     (StatusCode::OK, axum::Json(response))
// }

pub async fn get_all_recipe(
    axum::extract::State(client): axum::extract::State<Client>,
) -> impl axum::response::IntoResponse {
    let client = client.clone();
    let db = client.database("Recipe");
    let collection: Collection<Recipe> = db.collection::<Recipe>("recipes");
    let mut recipes_cursor = collection
        .find(None, None)
        .await
        .expect("Could not find recipes");
    let mut recipes: Vec<Recipe> = Vec::new();
    let mut recipes_with_categories: Vec<RecipesWithCategories> = Vec::new();
    let category_collection: Collection<Category> = db.collection::<Category>("categories");
    while let Some(recipe) = recipes_cursor.next().await {
        match recipe {
            Ok(recipe) => {
                let mut category_iter = recipe.categoryId.iter();

                let mut categorys_vec: Vec<Category> = Vec::new();
                while let Some(category) = category_iter.next() {
                    let category = category.clone();
                    let category_collection = category_collection.clone();
                    let category_cursor = category_collection
                        .find_one(
                            Some(doc! {
                                "_id": category,
                            }),
                            None,
                        )
                        .await;
                    match category_cursor {
                        Ok(category) => {
                            if category.is_some() {
                                let category = category.unwrap();
                                categorys_vec.push(category);
                            }
                            //recipes_with_categories.push(category.unwrap());
                        }
                        Err(err) => {
                            println!("{:#?}", err);
                        }
                    }
                }
                recipes_with_categories.push(RecipesWithCategories {
                    id: recipe.id.clone(),
                    name: recipe.name.clone(),
                    isActive: recipe.isActive.clone(),
                    img_Base64: recipe.img_Base64.clone(),
                    slug: recipe.slug.clone(),
                    categoryId: recipe.categoryId.clone(),
                    categories: categorys_vec.clone(),
                    ingredients: recipe.ingredients.clone(),
                    additionalNotes: recipe.additionalNotes.clone(),
                    description: recipe.description.clone(),
                    cookingTime: recipe.cookingTime.clone(),
                    __v: recipe.__v.clone(),
                });
                recipes.push(recipe);
            }
            Err(err) => {
                println!("{:#?}", err);
            }
        }
    }
    let response = ResponseRecipesWithCategories {
        success: true,
        data: Some(recipes_with_categories),
        error_message: None,
    };
    (StatusCode::OK, axum::Json(response))
}

pub async fn get_recipe_by_slug(
    axum::extract::State(client): axum::extract::State<Client>,
    axum::extract::Path(slug): axum::extract::Path<String>,
) -> impl axum::response::IntoResponse {
    let client = client.clone();
    let db = client.database("Recipe");
    let collection: Collection<Recipe> = db.collection::<Recipe>("recipes");
    let mut recipes: Vec<Recipe> = Vec::new();
    let mut recipe_cursor = collection
        .find_one(
            Some(doc! {
                "slug": slug,
            }),
            None,
        )
        .await
        .expect("Could not find recipes");
    if recipe_cursor.is_none() {
        return (
            StatusCode::NOT_FOUND,
            axum::Json(ResponseRecipesWithCategories {
                success: false,
                data: None,
                error_message: Some("Recipe not found".to_string()),
            }),
        );
    }
    recipes.push(recipe_cursor.unwrap());
    let iterator = recipes.iter();
    let mut categorys_id: Vec<ObjectId> = Vec::new();
    let mut categories: Vec<Category> = Vec::new();
    let category_collection: Collection<Category> = db.collection::<Category>("categories");
    let mut recipe_with_categories: Vec<RecipesWithCategories> = Vec::new();
    for recipe in iterator {
        let mut category_iter = recipe.categoryId.iter();
        while let Some(category) = category_iter.next() {
            let category = category.clone();
            let category_collection = category_collection.clone();
            let category_cursor = category_collection
                .find_one(
                    Some(doc! {
                        "_id": category,
                    }),
                    None,
                )
                .await;
            match category_cursor {
                Ok(category) => {
                    categories.push(category.unwrap());
                }
                Err(err) => {
                    println!("{:#?}", err);
                }
            }
            categorys_id.push(category.clone());
        }
        let new_recipe = RecipesWithCategories {
            id: recipe.id.clone(),
            name: recipe.name.clone(),
            isActive: recipe.isActive.clone(),
            img_Base64: recipe.img_Base64.clone(),
            slug: recipe.slug.clone(),
            categoryId: categorys_id.clone(),
            categories: categories.clone(),
            ingredients: recipe.ingredients.clone(),
            additionalNotes: recipe.additionalNotes.clone(),
            description: recipe.description.clone(),
            cookingTime: recipe.cookingTime.clone(),
            __v: recipe.__v.clone(),
        };
        recipe_with_categories.push(new_recipe);
    }
    let response = ResponseRecipesWithCategories {
        success: true,
        data: Some(recipe_with_categories),
        error_message: None,
    };
    (StatusCode::OK, axum::Json(response))
}
