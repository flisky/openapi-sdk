use std::sync::Arc;

use anyhow::Result;
use longbridge::{
    decimal,
    trade::{OrderSide, OrderType, SubmitOrderOptions, TimeInForceType},
    Config, TradeContext,
};

#[tokio::main]
async fn main() -> Result<()> {
    // Load configuration from environment variables
    let config = Arc::new(Config::from_env()?);

    // Create a context for trade APIs
    let (ctx, _) = TradeContext::try_new(config).await?;

    // Submit order
    let opts = SubmitOrderOptions::new(
        "700.HK",
        OrderType::LO,
        OrderSide::Buy,
        decimal!(500i32),
        TimeInForceType::Day,
    )
    .submitted_price(decimal!(50i32))
    .remark("Hello from Rust SDK".to_string());

    let resp = ctx.submit_order(opts).await?;
    println!("{:?}", resp);

    // Query orders
    let resp = ctx.today_orders(None).await?;
    println!("{:?}", resp);

    let resp = ctx.account_balance().await?;
    println!("{:?}", resp);

    Ok(())
}
