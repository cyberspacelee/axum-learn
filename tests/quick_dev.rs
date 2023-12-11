use anyhow::Result;

#[tokio::test]
async fn test_quick_dev() -> Result<()>{
    let hc = httpc_test::new_client("http://localhost:3306")?;

    hc.do_get("/").await?.print().await?;
    Ok(())
}