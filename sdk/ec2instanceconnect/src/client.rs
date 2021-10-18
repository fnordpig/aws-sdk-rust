// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[derive(Debug)]
pub(crate) struct Handle<
    C = smithy_client::erase::DynConnector,
    M = aws_hyper::AwsMiddleware,
    R = smithy_client::retry::Standard,
> {
    client: smithy_client::Client<C, M, R>,
    conf: crate::Config,
}

/// An ergonomic service client for `AWSEC2InstanceConnectService`.
///
/// This client allows ergonomic access to a `AWSEC2InstanceConnectService`-shaped service.
/// Each method corresponds to an endpoint defined in the service's Smithy model,
/// and the request and response shapes are auto-generated from that same model.
///
/// # Using a Client
///
/// Once you have a client set up, you can access the service's endpoints
/// by calling the appropriate method on [`Client`]. Each such method
/// returns a request builder for that endpoint, with methods for setting
/// the various fields of the request. Once your request is complete, use
/// the `send` method to send the request. `send` returns a future, which
/// you then have to `.await` to get the service's response.
///
/// [builder pattern]: https://rust-lang.github.io/api-guidelines/type-safety.html#c-builder
/// [SigV4-signed requests]: https://docs.aws.amazon.com/general/latest/gr/signature-version-4.html
#[derive(std::fmt::Debug)]
pub struct Client<
    C = smithy_client::erase::DynConnector,
    M = aws_hyper::AwsMiddleware,
    R = smithy_client::retry::Standard,
> {
    handle: std::sync::Arc<Handle<C, M, R>>,
}

impl<C, M, R> std::clone::Clone for Client<C, M, R> {
    fn clone(&self) -> Self {
        Self {
            handle: self.handle.clone(),
        }
    }
}

#[doc(inline)]
pub use smithy_client::Builder;

impl<C, M, R> From<smithy_client::Client<C, M, R>> for Client<C, M, R> {
    fn from(client: smithy_client::Client<C, M, R>) -> Self {
        Self::with_config(client, crate::Config::builder().build())
    }
}

impl<C, M, R> Client<C, M, R> {
    pub fn with_config(client: smithy_client::Client<C, M, R>, conf: crate::Config) -> Self {
        Self {
            handle: std::sync::Arc::new(Handle { client, conf }),
        }
    }

    pub fn conf(&self) -> &crate::Config {
        &self.handle.conf
    }
}
impl<C, M, R> Client<C, M, R>
where
    C: smithy_client::bounds::SmithyConnector,
    M: smithy_client::bounds::SmithyMiddleware<C>,
    R: smithy_client::retry::NewRequestPolicy,
{
    pub fn send_serial_console_ssh_public_key(
        &self,
    ) -> fluent_builders::SendSerialConsoleSSHPublicKey<C, M, R> {
        fluent_builders::SendSerialConsoleSSHPublicKey::new(self.handle.clone())
    }
    pub fn send_ssh_public_key(&self) -> fluent_builders::SendSSHPublicKey<C, M, R> {
        fluent_builders::SendSSHPublicKey::new(self.handle.clone())
    }
}
pub mod fluent_builders {
    #[derive(std::fmt::Debug)]
    pub struct SendSerialConsoleSSHPublicKey<
        C = smithy_client::erase::DynConnector,
        M = aws_hyper::AwsMiddleware,
        R = smithy_client::retry::Standard,
    > {
        handle: std::sync::Arc<super::Handle<C, M, R>>,
        inner: crate::input::send_serial_console_ssh_public_key_input::Builder,
    }
    impl<C, M, R> SendSerialConsoleSSHPublicKey<C, M, R>
    where
        C: smithy_client::bounds::SmithyConnector,
        M: smithy_client::bounds::SmithyMiddleware<C>,
        R: smithy_client::retry::NewRequestPolicy,
    {
        pub(crate) fn new(handle: std::sync::Arc<super::Handle<C, M, R>>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
        }
        pub async fn send(
            self,
        ) -> std::result::Result<
            crate::output::SendSerialConsoleSshPublicKeyOutput,
            smithy_http::result::SdkError<crate::error::SendSerialConsoleSSHPublicKeyError>,
        >
        where
            R::Policy: smithy_client::bounds::SmithyRetryPolicy<
                crate::input::SendSerialConsoleSshPublicKeyInputOperationOutputAlias,
                crate::output::SendSerialConsoleSshPublicKeyOutput,
                crate::error::SendSerialConsoleSSHPublicKeyError,
                crate::input::SendSerialConsoleSshPublicKeyInputOperationRetryAlias,
            >,
        {
            let input = self
                .inner
                .build()
                .map_err(|err| smithy_http::result::SdkError::ConstructionFailure(err.into()))?;
            let op = input
                .make_operation(&self.handle.conf)
                .map_err(|err| smithy_http::result::SdkError::ConstructionFailure(err.into()))?;
            self.handle.client.call(op).await
        }
        /// <p>The ID of the EC2 instance.</p>
        pub fn instance_id(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.instance_id(inp);
            self
        }
        pub fn set_instance_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_instance_id(input);
            self
        }
        /// <p>The serial port of the EC2 instance. Currently only port 0 is supported.</p>
        /// <p>Default: 0</p>
        pub fn serial_port(mut self, inp: i32) -> Self {
            self.inner = self.inner.serial_port(inp);
            self
        }
        pub fn set_serial_port(mut self, input: std::option::Option<i32>) -> Self {
            self.inner = self.inner.set_serial_port(input);
            self
        }
        /// <p>The public key material. To use the public key, you must have the matching private
        /// key. For information about the supported key formats and lengths, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/ec2-key-pairs.html#how-to-generate-your-own-key-and-import-it-to-aws">Requirements for key pairs</a> in the <i>Amazon EC2 User
        /// Guide</i>.</p>
        pub fn ssh_public_key(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.ssh_public_key(inp);
            self
        }
        pub fn set_ssh_public_key(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.inner = self.inner.set_ssh_public_key(input);
            self
        }
    }
    #[derive(std::fmt::Debug)]
    pub struct SendSSHPublicKey<
        C = smithy_client::erase::DynConnector,
        M = aws_hyper::AwsMiddleware,
        R = smithy_client::retry::Standard,
    > {
        handle: std::sync::Arc<super::Handle<C, M, R>>,
        inner: crate::input::send_ssh_public_key_input::Builder,
    }
    impl<C, M, R> SendSSHPublicKey<C, M, R>
    where
        C: smithy_client::bounds::SmithyConnector,
        M: smithy_client::bounds::SmithyMiddleware<C>,
        R: smithy_client::retry::NewRequestPolicy,
    {
        pub(crate) fn new(handle: std::sync::Arc<super::Handle<C, M, R>>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
        }
        pub async fn send(
            self,
        ) -> std::result::Result<
            crate::output::SendSshPublicKeyOutput,
            smithy_http::result::SdkError<crate::error::SendSSHPublicKeyError>,
        >
        where
            R::Policy: smithy_client::bounds::SmithyRetryPolicy<
                crate::input::SendSshPublicKeyInputOperationOutputAlias,
                crate::output::SendSshPublicKeyOutput,
                crate::error::SendSSHPublicKeyError,
                crate::input::SendSshPublicKeyInputOperationRetryAlias,
            >,
        {
            let input = self
                .inner
                .build()
                .map_err(|err| smithy_http::result::SdkError::ConstructionFailure(err.into()))?;
            let op = input
                .make_operation(&self.handle.conf)
                .map_err(|err| smithy_http::result::SdkError::ConstructionFailure(err.into()))?;
            self.handle.client.call(op).await
        }
        /// <p>The ID of the EC2 instance.</p>
        pub fn instance_id(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.instance_id(inp);
            self
        }
        pub fn set_instance_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_instance_id(input);
            self
        }
        /// <p>The OS user on the EC2 instance for whom the key can be used to authenticate.</p>
        pub fn instance_os_user(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.instance_os_user(inp);
            self
        }
        pub fn set_instance_os_user(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.inner = self.inner.set_instance_os_user(input);
            self
        }
        /// <p>The public key material. To use the public key, you must have the matching private key.</p>
        pub fn ssh_public_key(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.ssh_public_key(inp);
            self
        }
        pub fn set_ssh_public_key(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.inner = self.inner.set_ssh_public_key(input);
            self
        }
        /// <p>The Availability Zone in which the EC2 instance was launched.</p>
        pub fn availability_zone(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.availability_zone(inp);
            self
        }
        pub fn set_availability_zone(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.inner = self.inner.set_availability_zone(input);
            self
        }
    }
}
impl<C> Client<C, aws_hyper::AwsMiddleware, smithy_client::retry::Standard> {
    pub fn from_conf_conn(conf: crate::Config, conn: C) -> Self {
        let retry_config = conf.retry_config.as_ref().cloned().unwrap_or_default();
        let client = aws_hyper::Client::new(conn).with_retry_config(retry_config.into());
        Self {
            handle: std::sync::Arc::new(Handle { client, conf }),
        }
    }
}
impl
    Client<
        smithy_client::erase::DynConnector,
        aws_hyper::AwsMiddleware,
        smithy_client::retry::Standard,
    >
{
    #[cfg(any(feature = "rustls", feature = "native-tls"))]
    pub fn new(config: &aws_types::config::Config) -> Self {
        Self::from_conf(config.into())
    }

    #[cfg(any(feature = "rustls", feature = "native-tls"))]
    pub fn from_conf(conf: crate::Config) -> Self {
        let retry_config = conf.retry_config.as_ref().cloned().unwrap_or_default();
        let client = aws_hyper::Client::https().with_retry_config(retry_config.into());
        Self {
            handle: std::sync::Arc::new(Handle { client, conf }),
        }
    }
}