use goose::goose::Transaction;
use goose::prelude::*;
use rand::seq::IndexedRandom;
use serde_json::json;
use std::fs;

#[tokio::main]
async fn main() -> Result<(), GooseError> {
    GooseAttack::initialize()?
        .register_scenario(
            scenario!("GraphQL Load Test")
                .set_weight(1)?
                .register_transaction(
                    transaction!(gwas_credible_sets_by_study)
                        .set_name("GWAS Credible Sets by Study query"),
                )
                .register_transaction(
                    transaction!(disease_profile).set_name("Disease profile query"),
                )
                .register_transaction(transaction!(drug_profile).set_name("Drug profile query"))
                .register_transaction(
                    transaction!(evidence_clinvar).set_name("Evidence Clinvar query"),
                )
                .register_transaction(transaction!(gene_ontology).set_name("Gene Ontology query"))
                .register_transaction(
                    transaction!(gwas_credible_sets_by_variant)
                        .set_name("GWAS Credible Sets by Variant query"),
                )
                .register_transaction(transaction!(phenotypes_query).set_name("Phenotypes Query"))
                .register_transaction(transaction!(study_page).set_name("Study Page query"))
                .register_transaction(transaction!(similar_entities).set_name("Similar Entities"))
                .register_transaction(transaction!(target_page).set_name("Target Page query"))
                .register_transaction(transaction!(target_profile).set_name("Target Profile query"))
                .register_transaction(
                    transaction!(variant_profile).set_name("Variant Profile query"),
                ),
        )
        .execute()
        .await?;

    Ok(())
}

async fn query(user: &mut GooseUser, query_name: &str) -> TransactionResult {
    let q = fs::read_to_string(format!("queries/{}/q.gql", query_name))
        .expect(format!("{}/q.gql query file should exist!", query_name).as_str());
    let v = fs::read_to_string(format!("queries/{}/v.gql", query_name))
        .expect(format!("{}/v.gql variables file should exist!", query_name).as_str());
    let ids = fs::read_to_string(format!("queries/{}/ids.txt", query_name))
        .expect(format!("{}/ids.txt file should exist!", query_name).as_str())
        .lines()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    let random_id = ids
        .choose(&mut rand::rng())
        .expect("IDs file should not be empty!");
    let query = json!({
        "query": format!("{}",q.replace("\n", " ")),
        "variables": format!("{}",v.replace("$ID", random_id).replace("\n", " ")),
    });
    let _goose = user.post_json("api/v4/graphql", &query).await?;
    Ok(())
}

async fn gwas_credible_sets_by_study(user: &mut GooseUser) -> TransactionResult {
    query(user, "gwas_credible_sets_by_study").await
}

async fn evidence_clinvar(user: &mut GooseUser) -> TransactionResult {
    query(user, "evidence_clinvar").await
}

async fn target_page(user: &mut GooseUser) -> TransactionResult {
    query(user, "target_page").await
}

async fn target_profile(user: &mut GooseUser) -> TransactionResult {
    query(user, "target_profile").await
}

async fn disease_profile(user: &mut GooseUser) -> TransactionResult {
    query(user, "disease_profile").await
}

async fn drug_profile(user: &mut GooseUser) -> TransactionResult {
    query(user, "drug_profile").await
}

async fn gene_ontology(user: &mut GooseUser) -> TransactionResult {
    query(user, "gene_ontology").await
}

async fn similar_entities(user: &mut GooseUser) -> TransactionResult {
    query(user, "similar_entities").await
}

async fn gwas_credible_sets_by_variant(user: &mut GooseUser) -> TransactionResult {
    query(user, "gwas_credible_sets_by_variant").await
}

async fn phenotypes_query(user: &mut GooseUser) -> TransactionResult {
    query(user, "phenotypes_query").await
}

async fn study_page(user: &mut GooseUser) -> TransactionResult {
    query(user, "study_page").await
}

async fn variant_profile(user: &mut GooseUser) -> TransactionResult {
    query(user, "variant_profile").await
}
