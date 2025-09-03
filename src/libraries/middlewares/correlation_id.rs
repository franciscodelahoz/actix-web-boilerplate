use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    http::header::{HeaderName, HeaderValue}, middleware::Logger, Error, HttpMessage
};
use futures_util::future::LocalBoxFuture;
use std::{
    future::{ready, Ready},
    rc::Rc, str::FromStr,
};

use crate::libraries::{
    constants::correlation_id::{CORRELATION_ID_LOGGER_LABEL, DEFAULT_CORRELATION_ID, DEFAULT_CORRELATION_ID_HEADER},
    utils::correlation_id::{
        generate_new_id,
        get_id_from_header,
        is_valid_uuid,
        validate_id,
    },
};

pub struct CorrelationId {
    header_name: HeaderName,
    validator: Option<fn(&str) -> bool>,
    include_in_response: bool,
}

impl CorrelationId {
    pub fn new() -> Self {
        Self {
            header_name: HeaderName::from_static(DEFAULT_CORRELATION_ID_HEADER),
            validator: Some(is_valid_uuid),
            include_in_response: true,
        }
    }

    #[allow(dead_code)]
    pub fn header_name(mut self, name: &str) -> Self {
        self.header_name = HeaderName::from_str(name)
            .unwrap_or_else(|_| HeaderName::from_static(DEFAULT_CORRELATION_ID_HEADER));

        self
    }

    #[allow(dead_code)]
    pub fn validator(mut self, validator: Option<fn(&str) -> bool>) -> Self {
        self.validator = validator;
        self
    }

    #[allow(dead_code)]
    pub fn include_in_response(mut self, include: bool) -> Self {
        self.include_in_response = include;
        self
    }
}

pub struct CorrelationIdMiddleware<S> {
    service: Rc<S>,
    header_name: HeaderName,
    validator: Option<fn(&str) -> bool>,
    include_in_response: bool,
}

impl<S, B> Transform<S, ServiceRequest> for CorrelationId
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = CorrelationIdMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(CorrelationIdMiddleware {
            service: Rc::new(service),
            header_name: self.header_name.clone(),
            validator: self.validator,
            include_in_response: self.include_in_response,
        }))
    }
}

impl<S, B> Service<ServiceRequest> for CorrelationIdMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let service = self.service.clone();
        let header_name = self.header_name.clone();
        let validator = self.validator;
        let include_in_response = self.include_in_response;

        Box::pin(async move {
            let existing_id = get_id_from_header(&req, header_name.as_str());

            let is_valid: bool = validate_id(&existing_id, validator);

            let correlation_id = if is_valid {
                existing_id.unwrap()
            } else {
                generate_new_id()
            };

            req.extensions_mut().insert(correlation_id.clone());

            let mut res = service.call(req).await?;

            if include_in_response {
                let response_header_name = header_name;
                let response_header_value = HeaderValue::from_str(&correlation_id)
                    .unwrap_or_else(|_| HeaderValue::from_static(DEFAULT_CORRELATION_ID));

                res.response_mut().headers_mut().insert(response_header_name, response_header_value);
            }

            Ok(res)
        })
    }
}

pub trait RequestCorrelationId {
    fn correlation_id(&self) -> String;
}

impl RequestCorrelationId for ServiceRequest {
    fn correlation_id(&self) -> String {
        self.extensions().get::<String>().cloned().unwrap_or_else(|| DEFAULT_CORRELATION_ID.to_string())
    }
}

pub trait CorrelationIdVariable {
    fn add_correlation_id(self) -> Self;
}

impl CorrelationIdVariable for Logger {
    fn add_correlation_id(self) -> Self {
        self.custom_request_replace(CORRELATION_ID_LOGGER_LABEL, |req| {
            req.correlation_id()
        })
    }
}
