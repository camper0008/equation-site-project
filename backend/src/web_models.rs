pub struct User {
    id: String,
    username: String,
    permission: Permission,
    posts: Vec<Equation>,
    date_created: String, // ISO string
}

pub enum Permission {
    Unauthenticated,
    User,
    Contributor,
    Root,
}

pub struct Equation {
    id: String, // randomly generated
    title: String,
    content: Vec<EquationContent>,
    date_created: String, // date created as ISO string
    creator: User,
}

// a shortened version of Equation to only provide necessary data to preview.
pub struct PreviewableEquation {
    id: String, // randomly generated
    title: String,
    date_created: String, // date created as ISO string
}

pub struct EquationContent {
    content_type: EquationContentType,
    value: String,
}

pub enum EquationContentType {
    Text,
    Image,
    Math,
}
