mod widget_cache;
pub use widget_cache::WidgetCache;




#[tokio::test]
async fn test_new() -> Result<()> {
    let _cache = WidgetCache::new().await?;

    Ok(())
}

#[tokio::test]
async fn test_insert_read() -> Result<()> {
    let mut cache = WidgetCache::new().await?;
    cache.clear().await?;

    let test_widget = Widget {
        product: Product::HorseInsurance,
        data: "{}".to_string(),
        personalisation: Personalisation(None),
    };

    cache.upsert(&test_widget).await?;

    let actual = cache
        .get_widgets_for_user(&test_widget.personalisation)
        .await?;

    check!(actual.len() == 1);
    check!(actual[0] == test_widget);

    cache
        .remove(test_widget.product, &test_widget.personalisation)
        .await?;

    Ok(())
}
