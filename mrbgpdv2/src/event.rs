use crate::packets::{
    open::OpenMessage,
    keepalive::KeepaliveMessage,
    update::UpdateMessage,
};

#[derive(PartialEq, Eq, Debug, Clone, Hash)]
pub enum Event {
    ManualStart,
    TcpConnectionConfirmed,
    BgpOpen(OpenMessage),
    KeepAliveMsg(KeepaliveMessage),
    UpdateMsg(UpdateMessage),
    Established,
    LocRibChanged,
    AdjRibOutChanged,
    AdjRibInChanged,
}