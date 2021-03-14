use super::*;
use futures::stream;

#[async_std::test]
async fn stream_1_to_99() {
    let mut streamed: Vec<_> = stream::iter(1_u16..=9_u16)
        .select_map(|primary_num| {
            let primary_num = primary_num.clone();
            Box::pin(async move {
                Some(Box::new(stream::iter(
                    10 * primary_num..=10 * primary_num + 9,
                )))
            })
        })
        .collect()
        .await;
    streamed.sort_unstable();
    assert_eq!(streamed, (1..=99).collect::<Vec<_>>());
}

#[async_std::test]
async fn stream_1_to_49() {
    let mut streamed: Vec<_> = stream::iter(1_u16..=9_u16)
        .select_map(|primary_num| {
            let primary_num = primary_num.clone();
            Box::pin(async move {
                if primary_num > 4 {
                    return None;
                }
                Some(Box::new(stream::iter(
                    10 * primary_num..=10 * primary_num + 9,
                )))
            })
        })
        .collect()
        .await;
    streamed.sort_unstable();
    assert_eq!(streamed, (1..=49).collect::<Vec<_>>());
}
