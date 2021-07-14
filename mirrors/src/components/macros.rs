#![macro_escape]

#[macro_export]
macro_rules! watch_msg {
    ($context:expr, Msg::$e:ident) => {{
        use common::msg::Msg;
        use tokio::sync::watch;
        use tokio_stream::StreamExt;

        let mut stream = $context
            .broadcaster
            .stream()
            .filter(Result::is_ok)
            .map(Result::unwrap)
            .filter_map(|m| {
                if let Msg::$e(iso3) = m {
                    Some(iso3)
                } else {
                    None
                }
            });
        let (sx, rx) = watch::channel(None);
        $context.runtime().spawn(async move {
            while let Some(iso3) = stream.next().await {
                sx.send(Some(iso3)).ok();
            }
        });
        rx
    }};
}
