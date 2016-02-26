//Autogenerated

use hyper::header::{Headers, ContentType};
use hyper::client::response::Response;
use hyper::error::Result;

pub fn get(client: &'a mut hyper::Client, base: String) -> Result<Response>{
    let mut url_fmtd = String::with_capacity(base.len() + 17);
    url_fmtd.push_str(&base);
    url_fmtd.push_str("/_render/template");
    let mut headers = Headers::new();
    headers.set(ContentType::json());
    let res = client.get(&url_fmtd).headers(headers);
    res.send()
}
pub fn post(client: &'a mut hyper::Client, base: String, body: String)
 -> Result<Response>{
    let mut url_fmtd = String::with_capacity(base.len() + 17);
    url_fmtd.push_str(&base);
    url_fmtd.push_str("/_render/template");
    let mut headers = Headers::new();
    headers.set(ContentType::json());
    let res = client.post(&url_fmtd).headers(headers).body(body);
    res.send()
}
pub fn get_id(client: &'a mut hyper::Client, base: String, id: String)
 -> Result<Response>{
    let mut url_fmtd = String::with_capacity(base.len() + 18 + id.len());
    url_fmtd.push_str(&base);
    url_fmtd.push_str("/_render/template/");
    url_fmtd.push_str(&id);
    let mut headers = Headers::new();
    headers.set(ContentType::json());
    let res = client.get(&url_fmtd).headers(headers);
    res.send()
}
pub fn post_id(client: &'a mut hyper::Client, base: String, id: String,
           body: String) -> Result<Response>{
    let mut url_fmtd = String::with_capacity(base.len() + 18 + id.len());
    url_fmtd.push_str(&base);
    url_fmtd.push_str("/_render/template/");
    url_fmtd.push_str(&id);
    let mut headers = Headers::new();
    headers.set(ContentType::json());
    let res = client.post(&url_fmtd).headers(headers).body(body);
    res.send()
}