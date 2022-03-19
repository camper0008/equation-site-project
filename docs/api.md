# api documentation

## models

### equation

```rs
struct Equation {
    id: String // randomly generated
    title: String,
    content: [EquationContent],
    date_created: String, // date created as ISO string
    creator: User,
}
```

```rs
// a shortened version of Equation to only provide necessary data to preview.
struct PreviewableEquation {
    id: String // randomly generated
    title: String,
    date_created: String, // date created as ISO string
}
```

```rs
struct EquationContent {
    content_type: EquationContentType,
    value: String,
}
```

```rs
enum EquationContentType {
    Image,
    Text,
    Math,
}
```

### user

```rs
struct User {
    id: String, // randomly generated
    username: String,
    permission: Permission,
    posts: [Equation],
    date_created: String, // ISO string
}
```

### permission

```rs
enum Permission {
    Unauthenticated,
    User,
    Contributor,
    Root,
}
```

## api

### POST `/users/login`

#### Request

* Body

```rs
struct Request {
    username: String,
    password: String,
}
```

#### Response

* Status

`200 OK | 400 Bad Request`

* Cookie

`token: <auth token>`

* Body

```rs
struct Response {
    ok: bool,
    msg: String, // success | invalid login
}
```


### POST `/users/logout`

#### Request

* Cookie

`token: <auth token>`

#### Response

* Status

`200 OK | 400 Bad Request`


```rs
struct Response {
    ok: bool,
    msg: String, // success | invalid cookie
}
```

### POST `/equations/create`

#### Request

**Requires a permission level of contributor or above**

* Cookie

`token: <auth token>`

* Body

```rs
struct Request {
    title: String,
    content: [EquationContent],
}
```

#### Response

* Status

`200 OK | 400 Bad Request`

* Body

```rs
struct Response {
    ok: bool,
    msg: String, // success | unauthorized
}
```

### POST `/equations/remove/:post_id:`

#### Request

**Requires a permission level of contributor or above**

* Cookie

`token: <auth token>`

* Path Parameters

```rs
struct Param {
    post_id: String,
}
```

#### Response

* Status

`200 OK | 400 Bad Request`

* Body

```rs
struct Response {
    ok: bool,
    msg: String, // success | unauthorized
}
```


### POST `/equations/edit/:post_id:`

#### Request

**Requires a permission level of contributor or above**

* Cookie

`token: <auth token>`

* Path Parameters

```rs
struct Param {
    post_id: String,
}
```

* Body

```rs
struct Request {
    title: String,
    content: [EquationContent],
}
```

#### Response

* Status

`200 OK | 400 Bad Request`

* Body

```rs
struct Response {
    ok: bool,
    msg: String, // success | unauthorized
}
```

### GET `/equations/one/:post_id:`

#### Request

* Path Parameters

```rs
struct Param {
    post_id: String,
}
```

#### Response

* Status

`200 OK`

* Body

```rs
struct Response {
    ok: bool,
    msg: String, // success | invalid characters
    equation: Equation,
}
```

### GET `/equations/all`

#### Response

```rs
struct Response {
    ok: bool,
    msg: String, // success | invalid characters
    equations: [PreviewableEquation],
}
```

### GET `/equations/search/:query`

* Path Parameters

```rs
struct Param {
    query: String,
}
```

#### Response

* Status

`200 OK | 400 Bad Request`

* Body

```rs
struct Response {
    ok: bool,
    msg: String, // success | invalid characters
    equations: [PreviewableEquation],
}
```
