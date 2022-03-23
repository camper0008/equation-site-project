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
    id: String // same id as the full equation
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
    Title,
    Text,
    Image,
    Math,
    Code,
}
```

### user

```rs
struct User {
    id: String, // randomly generated
    username: String,
    permission: Permission,
    date_created: String, // ISO string
}
```

### permission

```rs
enum Permission {
    User,
    Contributor,
    Root,
}
```

## api

### POST `/users/login`

#### Request

- Body

```rs
struct Request {
    username: String,
    password: String,
}
```

#### Response

- Status

`200 OK | 400 Bad Request`

- Cookie

`SESSION_TOKEN: <auth token>`

- Body

```rs
struct Response {
    ok: bool,
    msg: String, // success | invalid login
}
```

### POST `/users/logout`

#### Request

- Cookie

`SESSION_TOKEN: <auth token>`

#### Response

- Status

`200 OK | 400 Bad Request`

```rs
struct Response {
    ok: bool,
    msg: String, // success | invalid cookie
}
```

### POST `/users/create`

#### Request

- Body

```rs
struct Request {
    username: String,
    password: String,
}
```

#### Response

- Status

`200 OK | 400 Bad Request`

- Body

```rs
struct Response {
    ok: bool,
    msg: String, // success | invalid username
}
```

### GET `/users/info`

#### Request

- Cookie

`SESSION_TOKEN: <auth token>`

#### Response

- Status

`200 OK | 400 Bad Request`

- Body

```rs
struct Response {
    ok: bool,
    msg: String, // success | unauthorized
    user: Option<User>
}
```

### POST `/equations/create`

#### Request

**Requires a permission level of contributor or above**

- Cookie

`SESSION_TOKEN: <auth token>`

- Body

```rs
struct Request {
    title: String,
    content: [EquationContent],
}
```

#### Response

- Status

`200 OK | 400 Bad Request`

- Body

```rs
struct Response {
    ok: bool,
    msg: String, // success | unauthorized
}
```

### POST `/equations/remove/{post_id}`

#### Request

**Requires a permission level of contributor or above**

- Cookie

`SESSION_TOKEN: <auth token>`

- Path Parameters

```rs
struct Param {
    post_id: String,
}
```

#### Response

- Status

`200 OK | 400 Bad Request`

- Body

```rs
struct Response {
    ok: bool,
    msg: String, // success | unauthorized
}
```

### POST `/equations/edit/{post_id}`

#### Request

**Requires a permission level of contributor or above**

- Cookie

`SESSION_TOKEN: <auth token>`

- Path Parameters

```rs
struct Param {
    post_id: String,
}
```

- Body

```rs
struct Request {
    title: String,
    content: [EquationContent],
}
```

#### Response

- Status

`200 OK | 400 Bad Request`

- Body

```rs
struct Response {
    ok: bool,
    msg: String, // success | unauthorized
}
```

### GET `/equations/one/{post_id}`

#### Request

- Path Parameters

```rs
struct Param {
    post_id: String,
}
```

#### Response

- Status

`200 OK | 400 Bad Request`

- Body

```rs
struct Response {
    ok: bool,
    msg: String, // success | invalid id
    equation: Equation,
}
```

### GET `/equations/search/{title}`

- Path Parameters

```rs
struct Param {
    title: String,
}
```

#### Response

- Status

`200 OK | 400 Bad Request`

- Body

```rs
struct Response {
    ok: bool,
    msg: String, // success | invalid characters
    equations: [PreviewableEquation],
}
```
