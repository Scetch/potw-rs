use liquid::{ Object, Value };
use comrak::{ self, ComrakOptions };

#[derive(Clone, Queryable)]
pub struct User {
    pub id: i32,
    pub sid: String,
    pub admin: bool,
}

impl User {
    pub fn to_liquid(self) -> Object {
        let mut obj = Object::new();
        obj.insert("id".into(), Value::scalar(self.id));
        obj.insert("sid".into(), Value::scalar(self.sid));
        obj.insert("admin".into(), Value::scalar(self.admin));
        obj
    }
}

#[derive(Clone, Queryable)]
pub struct Problem {
    pub id: i32,
    pub name: String,
    pub description: String,
}

impl Problem {
    pub fn to_liquid(self, render: bool) -> Object {
        let mut obj = Object::new();
        obj.insert("id".into(), Value::scalar(self.id));
        obj.insert("name".into(), Value::scalar(self.name));
        if render {
            let rendered = comrak::markdown_to_html(&self.description, &ComrakOptions::default());
            obj.insert("description".into(), Value::scalar(rendered));
        } else {
            obj.insert("description".into(), Value::scalar(self.description));
        }
        obj
    }
}

#[derive(Clone, Queryable)]
pub struct Solution {
    pub id: i32,
    pub pid: i32,
    pub uid: i32,
    pub language: i32,
    pub code: String,
}

impl Solution {
    pub fn to_liquid(self) -> Object {
        let mut obj = Object::new();
        obj.insert("id".into(), Value::scalar(self.id));
        obj.insert("pid".into(), Value::scalar(self.pid));
        obj.insert("uid".into(), Value::scalar(self.uid));
        obj.insert("language".into(), Value::scalar(self.language));
        obj.insert("code".into(), Value::scalar(self.code));
        obj
    }
}

#[derive(Clone, Queryable)]
pub struct Language {
    pub id: i32,
    pub name: String,
}

impl Language {
    pub fn to_liquid(self) -> Object {
        let mut obj = Object::new();
        obj.insert("id".into(), Value::scalar(self.id));
        obj.insert("name".into(), Value::scalar(self.name));
        obj
    }
}
