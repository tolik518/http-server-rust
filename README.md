[![progress-banner](https://backend.codecrafters.io/progress/http-server/daf9d874-2bd7-42a3-a62c-629c23aeb4c6)](c

This is a starting point for Rust solutions to the
["Build Your Own HTTP server" Challenge](https://app.codecrafters.io/r/victorious-pig-708556).

[HTTP](https://en.wikipedia.org/wiki/Hypertext_Transfer_Protocol) is the
protocol that powers the web. In this challenge, you'll build a HTTP/1.1 server
that is capable of serving multiple clients.

Along the way you'll learn about TCP servers,
[HTTP request syntax](https://www.w3.org/Protocols/rfc2616/rfc2616-sec5.html),
and more.

**Note**: If you're viewing this repo on GitHub, head over to
[codecrafters.io](https://app.codecrafters.io/r/victorious-pig-708556) to try the challenge.

___

All of the stages were cleared in the repo, but, especially, the `request.rs` is not optimized at all.  
It would have been better to use `.seek(&mut buf)` than working with `tcp_stream.bytes()` directly