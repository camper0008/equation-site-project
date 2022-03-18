# api documentation

## models

### equation

```rs
struct Equation {
    name: String,
    content: String, // markdown
    date_created: String, // date created as ISO string
    creator: User,
}
```

### user

```rs
struct User {
    id: String, // randomly generated
    name: String,
    permission: Permission,
    posts: [Equation],
    created: String,
}
```

### permission

```rs
enum Permission {
    Unauthenticated,
    User,
    Admin,
    Root,
}
```

## api

### POST `/user/login`

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

`token: <auth token>

### POST `/user/logout`

#### Request

* Cookie

`token: <auth token>`

#### Response

* Status

`200 OK | 400 Bad Request`

### POST `/equations/create`

#### Request

**Requires a permission level of admin or above**

* Cookie

`token: <auth token>`

* Body

```rs
struct Request {
    name: String,
    content: String,
}
```

#### Response

* Status

`200 OK | 403 Forbidden`

