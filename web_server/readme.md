# Web server aims
1. Learn about TCP & HTTP
1. Listen for TCP connections on a socket
1. Parse a small number of HTTP requests
1. Create a proper HTTP response
1. Improve the throughput of our server with a thread pool

## HTTP & TCP
Both protocols are _request-response_ protocols, i.e. a client initiates requests, a server listents to the requests and provides a response to the client. The protocol defines the contents of those requests and responses.

TCP is the lower-level protocol that describes the details of how info gets from one server to another, but doesn't specify what the information should look like. HTTP builds on top of TCP by defining the contents of the requests and responses.

While it's possible to use HTTP with a different protocol, in the vast majority of cases, HTTP sends data over TCP. We'll work with the raw bytes of TCP & HTTP.

### HTTP req
The HTTP req sent by the browser is a string with lines:
```
Req: [
    "GET / HTTP/1.1",
    "Host: localhost:7878",
    "Connection: keep-alive",
    "sec-ch-ua: \"Not_A Brand\";v=\"8\", \"Chromium\";v=\"120\", \"Brave\";v=\"120\"",
    "sec-ch-ua-mobile: ?0",
    "sec-ch-ua-platform: \"Windows\"",
    "Upgrade-Insecure-Requests: 1",
    "User-Agent: Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36",
    "Accept: text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8",
    "Sec-GPC: 1",
    "Accept-Language: en-GB,en",
    "Sec-Fetch-Site: none",
    "Sec-Fetch-Mode: navigate",
    "Sec-Fetch-User: ?1",
    "Sec-Fetch-Dest: document",
    "Accept-Encoding: gzip, deflate, br",
]
```
The first line is the _request line_, and specifies the _method_, _URI_ and HTTP version of the client, all followed by a _CRLF sequence_ (carriage return and line feed, i.e. `\r\n`).
The following lines are the headers. If the above request had a body (which a GET request does not), this would follow the headers.
To end a HTTP request/response use a double CRLF, i.e. `\r\n\r\n`.

### HTTP response
HTTP responses have the following format:
```
HTTP-Version Status-Code Reason-Phrase CRLF
headers CRLF
message-body
```

## Thread pool
If the web server is busy with one request, then subsequent requests must wait until the first is done. E.g. go to `/sleep` in one tab and then `/` in another.

A _thread pool_ is a group of spawned threads waiting to handle a task. When a thread finishes processing its task, it's returned to the pool. To prevent DoS attacks, we should have a limited number of threads in our pool, rather than spawning a new thread for each request as they happen.

The pool will maintain a queue of incoming requests and threads will pop requests off the queue.

Note thread pools are just one way to improve throughput of a web-server. Other options are:
1. fork/join model
1. single-threaded async I/O model
1. multi-threaded async I/O model
