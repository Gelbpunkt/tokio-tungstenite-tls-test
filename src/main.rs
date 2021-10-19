use futures_util::stream::StreamExt;
use tt_14::connect_async as tt_14_connect_async;
use tt_15::connect_async as tt_15_connect_async;
use tt_git::connect_async as tt_git_connect_async;

async fn tt_14_test() -> tt_14::tungstenite::Result<()> {
    let (mut conn, _) = tt_14_connect_async("wss://gateway.discord.gg/?v=9&encoding=json").await?;

    let res = conn.next().await;

    assert!(res.is_some());
    assert!(res.as_ref().unwrap().is_ok());
    assert!(res.unwrap().unwrap().is_text());

    Ok(())
}

async fn tt_15_test() -> tt_15::tungstenite::Result<()> {
    let (mut conn, _) = tt_15_connect_async("wss://gateway.discord.gg/?v=9&encoding=json").await?;

    let res = conn.next().await;

    assert!(res.is_some());
    assert!(res.as_ref().unwrap().is_ok());
    assert!(res.unwrap().unwrap().is_text());

    Ok(())
}

async fn tt_git_test() -> tt_git::tungstenite::Result<()> {
    let (mut conn, _) = tt_git_connect_async("wss://gateway.discord.gg/?v=9&encoding=json").await?;

    let res = conn.next().await;

    assert!(res.is_some());
    assert!(res.as_ref().unwrap().is_ok());
    assert!(res.unwrap().unwrap().is_text());

    Ok(())
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    tt_14_test().await.expect("TT14 failed!");
    tt_15_test().await.expect("TT15 failed!");
    tt_git_test().await.expect("TTgit failed!");

    println!("All tests passed");
}
