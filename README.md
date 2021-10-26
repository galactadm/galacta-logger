# galacta-logger
Simple logger for rust. This logger provide detailed debug information(logged time, log level, file path, ilne number, module path and function name) of project without initialization.


# Usage
```
use galacta_logger::{self,GalactaLogger,LevelFilter,prelude::*};
/// First initialize logger.
/// You may set "G_LOG_LEVEL" environment variable
/// If environment variable is set, argument of init function will be ignored
GalactaLogger::init(LevelFilter::Trace);

/// These are logs with specified target(unrecommended)
error!("This is errror");
warn!(target: "target","This is warning");
info!("This is info");
debug!("This is debug");
trace!("This is trace");

/// These are logs without specified target or initializations(recommended)
gerror!("This is errror");
gwarn!("This is warning");
ginfo!("This is info");
gdebug!("This is debug");
gtrace!("This is trace");
```

# Output sample
```
[2021-10-26 20:38:03] [TRACE] galacta_logger::tests::test_gdebug
This is gtrace
galacta_logger::tests (src/lib.rs:229)
```
