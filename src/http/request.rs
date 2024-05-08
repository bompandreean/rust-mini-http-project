use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::str::Utf8Error;

use super::QueryString;
use super::method::{Method, MethodErr};

#[derive(Debug)]
pub struct Request<'buf> {
    path: &'buf str,
    query_string: Option<QueryString<'buf>>,
    method: Method,
}

impl<'buf> Request<'buf>{
    pub fn path(&self) -> &str{
        &self.path
    }

    pub fn method(&self) -> &Method{
        &self.method
    }

    pub fn query_string(&self) -> Option<&QueryString>{
        self.query_string.as_ref() //converts to an option of a reference ^ instead of &Option<QueryString>
    }
}

impl<'buf> TryFrom<&'buf [u8]> for Request<'buf> {
    type Error = ParseErrors;

    //GET /serch?name=abc&sort=1 HTTP/1.1\r\n ...Headers..
    fn try_from(buf: &'buf [u8]) -> Result<Request<'buf>, Self::Error> {
        // match str::from_utf8(buf){
        //     Ok(request )=>{},
        //     Err(_) => return Err(ParseErrors::InvalidEncoding)
        // }
        //
        // match str::from_utf8(buf)
        //     .or(Err(ParseErrors::InvalidEncoding)){
        //     Ok(request) => {},
        //     Err(e) => return Err(e),
        // } // same as :
        // str::from_utf8(buf).or(Err(ParseErrors::InvalidEncoding))? //same as: + impl for Utf8Err

        let request = std::str::from_utf8(buf)?;

        // match get_next_word(request){
        //     Some((method, request)) =>{},
        //     None => Err(ParseErrors::InvalidRequest),
        // } //same as:

        let (method, request) = get_next_word(request).ok_or(ParseErrors::InvalidRequest)?;
        let (mut path, request) = get_next_word(request).ok_or(ParseErrors::InvalidRequest)?;
        let (protocol, _) = get_next_word(request).ok_or(ParseErrors::InvalidRequest)?;

        if protocol != "HTTP/1.1" {
            return Err(ParseErrors::InvalidProtocol);
        }

        //converting strinq method into the Method enum; .parse works only if the struct is implementing from_str
        let method: Method = method.parse()?;

        //if let
        // let mut query_string = None;
        // match path.find('?') {
        //     Some(q) => {
        //         query_string = Some(&path[q + 1..]);
        //         path = &path[0..q];
        //     }
        //     None() => {}
        // } //same as:

        let mut query_string = None;
        if let Some(index_of_question) = path.find('?') {
            query_string = Some(QueryString::from(&path[index_of_question + 1..]));
            path = &path[0..index_of_question];
        }

        Ok(Self {
            path,
            query_string,
            method,
        })
    }
}

fn get_next_word(request: &str) -> Option<(&str, &str)> {
    for (index, character) in request.chars().enumerate() {
        if character == ' ' || character == '\r' || character == '\n' {
            return Some((&request[..index], &request[index + 1..]));
        }
    }

    None
}

pub enum ParseErrors {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod,
}

impl ParseErrors {
    fn message(&self) -> &str {
        match self {
            Self::InvalidRequest => "Invalid Request",
            Self::InvalidEncoding => "Invalid Encoding",
            Self::InvalidProtocol => "Invalid Protocol",
            Self::InvalidMethod => "Invalid Method",
        }
    }
}

impl From<MethodErr> for ParseErrors {
    fn from(_: MethodErr) -> Self {
        Self::InvalidMethod
    }
}

impl From<Utf8Error> for ParseErrors {
    fn from(_: Utf8Error) -> Self {
        Self::InvalidEncoding
    }
}

impl Debug for ParseErrors {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Display for ParseErrors {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Error for ParseErrors {}


// trait Encrypt{
//     fn encrypt(&self) -> Self;
// }
//
// impl Encrypt for String {
//     fn encrypt(&self) -> Self {
//         unimplemented!(); // to stop compiler error for methods we are not ready to implement yet
//     }
// }
//
// impl Encrypt for &[u8] {
//     fn encrypt(&self) -> Self {
//         unimplemented!();
//     }
// }