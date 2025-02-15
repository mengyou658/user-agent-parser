use std::borrow::Cow;

use rocket::outcome::Outcome;
use rocket::request::{FromRequest, Outcome as OutcomeResult, Request};

use crate::models::*;

use crate::UserAgentParser;

fn from_request_user_agent<'r>(request: &'r Request<'_>) -> UserAgent<'r> {
    let user_agent: Option<Cow<'r, str>> =
        request.headers().get("user-agent").next().map(Cow::from);

    UserAgent {
        user_agent,
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for UserAgent<'r> {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> OutcomeResult<Self, Self::Error> {
        Outcome::Success(from_request_user_agent(request))
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for &UserAgent<'r> {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> OutcomeResult<Self, Self::Error> {
        let cache = request.local_cache(|| from_request_user_agent(request).into_owned());

        Outcome::Success(cache)
    }
}

fn from_request_product<'r>(request: &'r Request<'_>) -> Product<'r> {
    let user_agent_parser = request.rocket().state::<UserAgentParser>().unwrap();

    let user_agent: Option<&str> = request.headers().get("user-agent").next();

    match user_agent {
        Some(user_agent) => user_agent_parser.parse_product(user_agent),
        None => Product::default(),
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Product<'r> {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> OutcomeResult<Self, Self::Error> {
        Outcome::Success(from_request_product(request))
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for &Product<'r> {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> OutcomeResult<Self, Self::Error> {
        let cache =
            request.local_cache_async(async { from_request_product(request).into_owned() }).await;

        Outcome::Success(cache)
    }
}

fn from_request_os<'r>(request: &'r Request<'_>) -> OS<'r> {
    let user_agent_parser = request.rocket().state::<UserAgentParser>().unwrap();

    let user_agent: Option<&str> = request.headers().get("user-agent").next();

    match user_agent {
        Some(user_agent) => user_agent_parser.parse_os(user_agent),
        None => OS::default(),
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for OS<'r> {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> OutcomeResult<Self, Self::Error> {
        Outcome::Success(from_request_os(request))
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for &OS<'r> {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> OutcomeResult<Self, Self::Error> {
        let cache =
            request.local_cache_async(async { from_request_os(request).into_owned() }).await;

        Outcome::Success(cache)
    }
}

fn from_request_device<'r>(request: &'r Request<'_>) -> Device<'r> {
    let user_agent_parser = request.rocket().state::<UserAgentParser>().unwrap();

    let user_agent: Option<&str> = request.headers().get("user-agent").next();

    match user_agent {
        Some(user_agent) => user_agent_parser.parse_device(user_agent),
        None => Device::default(),
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Device<'r> {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> OutcomeResult<Self, Self::Error> {
        Outcome::Success(from_request_device(request))
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for &Device<'r> {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> OutcomeResult<Self, Self::Error> {
        let cache =
            request.local_cache_async(async { from_request_device(request).into_owned() }).await;

        Outcome::Success(cache)
    }
}

fn from_request_cpu<'r>(request: &'r Request<'_>) -> CPU<'r> {
    let user_agent_parser = request.rocket().state::<UserAgentParser>().unwrap();

    let user_agent: Option<&str> = request.headers().get("user-agent").next();

    match user_agent {
        Some(user_agent) => user_agent_parser.parse_cpu(user_agent),
        None => CPU::default(),
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for CPU<'r> {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> OutcomeResult<Self, Self::Error> {
        Outcome::Success(from_request_cpu(request))
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for &CPU<'r> {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> OutcomeResult<Self, Self::Error> {
        let cache =
            request.local_cache_async(async { from_request_cpu(request).into_owned() }).await;

        Outcome::Success(cache)
    }
}

fn from_request_engine<'r>(request: &'r Request<'_>) -> Engine<'r> {
    let user_agent_parser = request.rocket().state::<UserAgentParser>().unwrap();

    let user_agent: Option<&str> = request.headers().get("user-agent").next();

    match user_agent {
        Some(user_agent) => user_agent_parser.parse_engine(user_agent),
        None => Engine::default(),
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Engine<'r> {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> OutcomeResult<Self, Self::Error> {
        Outcome::Success(from_request_engine(request))
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for &Engine<'r> {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> OutcomeResult<Self, Self::Error> {
        let cache =
            request.local_cache_async(async { from_request_engine(request).into_owned() }).await;

        Outcome::Success(cache)
    }
}
