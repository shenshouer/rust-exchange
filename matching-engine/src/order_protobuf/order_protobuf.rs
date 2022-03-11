#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OrderRequest {
    #[prost(string, tag = "1")]
    pub asset_code: ::prost::alloc::string::String,
    #[prost(enumeration = "OrderType", tag = "2")]
    pub order_type: i32,
    #[prost(uint64, tag = "3")]
    pub quantity: u64,
    #[prost(float, tag = "4")]
    pub price: f32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OrderResponse {
    #[prost(bool, tag = "1")]
    pub ok: bool,
    #[prost(string, tag = "2")]
    pub error: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BookOrder {
    #[prost(enumeration = "OrderType", tag = "1")]
    pub order_type: i32,
    #[prost(int64, tag = "2")]
    pub quantity: i64,
    #[prost(float, tag = "3")]
    pub price: f32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BookRequest {
    #[prost(string, tag = "1")]
    pub asset_code: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BookResponse {
    #[prost(message, repeated, tag = "1")]
    pub orders: ::prost::alloc::vec::Vec<BookOrder>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum OrderType {
    Buy = 0,
    Sell = 1,
}
#[doc = r" Generated client implementations."]
pub mod order_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    pub struct OrderServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl OrderServiceClient<tonic::transport::Channel> {
        #[doc = r" Attempt to create a new client by connecting to a given endpoint."]
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> OrderServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + HttpBody + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as HttpBody>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = tonic::client::Grpc::with_interceptor(inner, interceptor);
            Self { inner }
        }
        pub async fn send_order(
            &mut self,
            request: impl tonic::IntoRequest<super::OrderRequest>,
        ) -> Result<tonic::Response<super::OrderResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/order_protobuf.OrderService/SendOrder");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for OrderServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for OrderServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "OrderServiceClient {{ ... }}")
        }
    }
}
#[doc = r" Generated client implementations."]
pub mod book_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    pub struct BookServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl BookServiceClient<tonic::transport::Channel> {
        #[doc = r" Attempt to create a new client by connecting to a given endpoint."]
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> BookServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + HttpBody + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as HttpBody>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = tonic::client::Grpc::with_interceptor(inner, interceptor);
            Self { inner }
        }
        pub async fn get_book(
            &mut self,
            request: impl tonic::IntoRequest<super::BookRequest>,
        ) -> Result<tonic::Response<super::BookResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/order_protobuf.BookService/GetBook");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for BookServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for BookServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "BookServiceClient {{ ... }}")
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod order_service_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with OrderServiceServer."]
    #[async_trait]
    pub trait OrderService: Send + Sync + 'static {
        async fn send_order(
            &self,
            request: tonic::Request<super::OrderRequest>,
        ) -> Result<tonic::Response<super::OrderResponse>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct OrderServiceServer<T: OrderService> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: OrderService> OrderServiceServer<T> {
        pub fn new(inner: T) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner, None);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner, Some(interceptor.into()));
            Self { inner }
        }
    }
    impl<T, B> Service<http::Request<B>> for OrderServiceServer<T>
    where
        T: OrderService,
        B: HttpBody + Send + Sync + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = Never;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/order_protobuf.OrderService/SendOrder" => {
                    #[allow(non_camel_case_types)]
                    struct SendOrderSvc<T: OrderService>(pub Arc<T>);
                    impl<T: OrderService> tonic::server::UnaryService<super::OrderRequest> for SendOrderSvc<T> {
                        type Response = super::OrderResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::OrderRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).send_order(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = SendOrderSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .header("content-type", "application/grpc")
                        .body(tonic::body::BoxBody::empty())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: OrderService> Clone for OrderServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: OrderService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone(), self.1.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: OrderService> tonic::transport::NamedService for OrderServiceServer<T> {
        const NAME: &'static str = "order_protobuf.OrderService";
    }
}
#[doc = r" Generated server implementations."]
pub mod book_service_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with BookServiceServer."]
    #[async_trait]
    pub trait BookService: Send + Sync + 'static {
        async fn get_book(
            &self,
            request: tonic::Request<super::BookRequest>,
        ) -> Result<tonic::Response<super::BookResponse>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct BookServiceServer<T: BookService> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: BookService> BookServiceServer<T> {
        pub fn new(inner: T) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner, None);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner, Some(interceptor.into()));
            Self { inner }
        }
    }
    impl<T, B> Service<http::Request<B>> for BookServiceServer<T>
    where
        T: BookService,
        B: HttpBody + Send + Sync + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = Never;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/order_protobuf.BookService/GetBook" => {
                    #[allow(non_camel_case_types)]
                    struct GetBookSvc<T: BookService>(pub Arc<T>);
                    impl<T: BookService> tonic::server::UnaryService<super::BookRequest> for GetBookSvc<T> {
                        type Response = super::BookResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::BookRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_book(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = GetBookSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .header("content-type", "application/grpc")
                        .body(tonic::body::BoxBody::empty())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: BookService> Clone for BookServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: BookService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone(), self.1.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: BookService> tonic::transport::NamedService for BookServiceServer<T> {
        const NAME: &'static str = "order_protobuf.BookService";
    }
}
