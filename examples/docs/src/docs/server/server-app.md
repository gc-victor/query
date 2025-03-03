# Server App

Query server can be use as a standalone server to serve your application. It is a simple way to serve your application using the Query Server. The main feature is to remove the `/_/function` prefix to serve Pages and APIs.

You can set it using the following environment variable:

```sh
QUERY_SERVER_APP=true
```

Once it is set it is possible to access pages using `/rest_of_the_path` instead of `/_/function/pages/rest_of_the_path`. Similarly, APIs will be `/api/rest_of_the_path` instead of `/_/function/api/rest_of_the_path`. As usual, every function will be served using the `/_/function` prefix.

It is important to note that the `QUERY_SERVER_APP` environment variable is optional. The`/_/function` path will be used for every case if you don't provide it.
