use crate::protocol::traits::Protocol;
use std::collections::HashMap;
use std::sync::Arc;

/// 协议工厂 - 用于创建协议实例
pub trait ProtocolFactory: Send + Sync {
    fn create(&self) -> Box<dyn std::any::Any + Send>;
    fn name(&self) -> &str;
}

/// 协议注册表
pub struct ProtocolRegistry {
    protocols: HashMap<u16, Arc<dyn ProtocolFactory>>,
}

impl ProtocolRegistry {
    pub fn new() -> Self {
        Self {
            protocols: HashMap::new(),
        }
    }

    /// 注册协议到指定端口
    pub fn register<P, F>(&mut self, port: u16, factory: F)
    where
        P: Protocol + 'static,
        F: Fn() -> P + Send + Sync + 'static,
    {
        struct Factory<P, F> {
            factory_fn: F,
            name: String,
            _phantom: std::marker::PhantomData<P>,
        }

        impl<P, F> ProtocolFactory for Factory<P, F>
        where
            P: Protocol + 'static,
            F: Fn() -> P + Send + Sync,
        {
            fn create(&self) -> Box<dyn std::any::Any + Send> {
                Box::new((self.factory_fn)())
            }

            fn name(&self) -> &str {
                &self.name
            }
        }

        let protocol = factory();
        let name = protocol.name().to_string();

        self.protocols.insert(
            port,
            Arc::new(Factory {
                factory_fn: factory,
                name,
                _phantom: std::marker::PhantomData,
            }),
        );
    }

    /// 根据端口获取协议
    pub fn get_protocol<P: Protocol + 'static>(&self, port: u16) -> Option<P> {
        self.protocols.get(&port).and_then(|factory| {
            let boxed = factory.create();
            boxed.downcast::<P>().ok().map(|p| *p)
        })
    }

    /// 检查端口是否已注册
    pub fn has_protocol(&self, port: u16) -> bool {
        self.protocols.contains_key(&port)
    }

    /// 获取协议名称
    pub fn get_protocol_name(&self, port: u16) -> Option<&str> {
        self.protocols.get(&port).map(|f| f.name())
    }
}
