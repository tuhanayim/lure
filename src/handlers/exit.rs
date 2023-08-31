use tokio::{signal, sync::mpsc::UnboundedSender};

use crate::ChannelPayload;

pub struct ExitHandler(UnboundedSender<ChannelPayload>);

impl ExitHandler {
    pub fn new(rx: UnboundedSender<ChannelPayload>) -> Self {
        Self(rx)
    }

    pub async fn handle(self) {
        tokio::spawn(async move {
            let ctrl_c = signal::ctrl_c();

            ctrl_c.await.expect("CTRL-C handler could not be created");
            #[cfg(unix)]
            {
                use signal::unix::{signal, SignalKind};

                let mut sigterm =
                    signal(SignalKind::terminate()).expect("SIGTERM handler could not be created");

                tokio::select! {
                    _ = ctrl_c => {},
                    _ = sigterm.recv() => {}
                }
            }

            self.0
                .send(ChannelPayload::Exit)
                .map_err(|err| eprintln!("{err}"))
        });
    }
}

impl From<UnboundedSender<ChannelPayload>> for ExitHandler {
    fn from(tx: UnboundedSender<ChannelPayload>) -> Self {
        Self(tx.clone())
    }
}
