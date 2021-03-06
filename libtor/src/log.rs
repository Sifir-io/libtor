/// Log level
#[derive(Debug, Clone, Copy)]
pub enum LogLevel {
    Debug,
    Info,
    Notice,
    Warn,
    Err,
}

/// Log destination
#[derive(Debug, Clone)]
pub enum LogDestination {
    Stdout,
    Stderr,
    #[cfg(target_family = "unix")]
    Syslog,
    File(String),
    #[cfg(target_os = "android")]
    Android,
}

/// Log domain, for fine grained control
#[derive(Debug, Clone, Copy)]
pub enum LogDomain {
    General,
    Crypto,
    Net,
    Config,
    Fs,
    Protocol,
    Mm,
    Http,
    App,
    Control,
    Circ,
    Rend,
    Bug,
    Dir,
    Dirserv,
    Or,
    Edge,
    Acct,
    Hist,
    Handshake,
    Heartbeat,
    Channel,
    Sched,
    Guard,
    Consdiff,
    Dos,
    Process,
    Pt,
    Btrack,
    Mesg,
}
