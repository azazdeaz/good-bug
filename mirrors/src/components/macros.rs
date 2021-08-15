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
                if let Msg::$e(val) = m {
                    Some(val)
                } else {
                    None
                }
            });
        let (sx, rx) = watch::channel(None);
        $context.runtime().spawn(async move {
            while let Some(val) = stream.next().await {
                sx.send(Some(val)).ok();
            }
        });
        rx
    }};
}

#[macro_export]
macro_rules! watch_msg_once {
    ($context:expr, Msg::$e:ident) => {{
        use common::msg::Msg;
        use tokio_stream::StreamExt;
        use std::sync::{Arc, RwLock};
        use common::utils::LastValue;

        let mut stream = $context
            .broadcaster
            .stream()
            .filter(Result::is_ok)
            .map(Result::unwrap)
            .filter_map(|m| {
                if let Msg::$e(val) = m {
                    Some(val)
                } else {
                    None
                }
            });
        let value = Arc::new(RwLock::new(LastValue::new()));
        {
            let value = Arc::clone(&value);
            $context.runtime().spawn(async move {
                while let Some(val) = stream.next().await {
                    value.write().unwrap().set(val);
                }
            });
        }
        
        value
    }};
}
