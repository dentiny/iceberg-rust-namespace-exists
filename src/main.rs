use std::collections::HashMap;

use iceberg::Catalog;
use iceberg::Result as IcebergResult;
use iceberg::NamespaceIdent;
use iceberg_catalog_rest::{RestCatalog, RestCatalogConfig};

#[tokio::main]
async fn main() -> IcebergResult<()> {
    // --- Create REST catalog ---
    let rest_uri = "http://localhost:8181"; // Update if your REST server is elsewhere
    let config = RestCatalogConfig::builder()
        .uri(rest_uri.to_string())
        .build();
    let catalog = RestCatalog::new(config);

    // --- Create NamespaceIdent ---
    let namespace = vec!["default"];
    let namespace_ident =
        NamespaceIdent::from_vec(namespace.iter().map(|s| s.to_string()).collect())?;
    println!("NamespaceIdent: {:?}", namespace_ident);

    // --- Create namespace ---
    let _ = catalog.create_namespace(&namespace_ident, HashMap::new()).await?;

    // --- List all namespaces ---
    let namespaces = catalog.list_namespaces(None).await?;
    println!("Existing namespaces:");
    for ns in &namespaces {
        println!("  {:?}", ns);
    }

    // --- Check if namespace exists ---
    let namespace_exists = namespaces.contains(&namespace_ident);
    if namespace_exists {
        println!("Namespace {:?} exists!", namespace_ident);
    } else {
        println!("Namespace {:?} does NOT exist!", namespace_ident);
    }

    // -- Get namespace ---
    let namespace_instance = catalog.get_namespace(&namespace_ident).await?;
    println!("Namespace = {:?}", namespace_instance); 

    // --- Check if namespace exists ---
    let namespace_exists = catalog.namespace_exists(&namespace_ident).await?;
    println!("Namespace exist: {}!", namespace_exists);

    Ok(())
}
