use serde::Serialize;

#[derive(Serialize)]
struct ProviderStripeData {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    alias: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ratelimit: std::option::Option<terrars::PrimField<f64>>,
    token: terrars::PrimField<std::string::String>,
}

struct ProviderStripe_ {
    data: std::cell::RefCell<ProviderStripeData>,
}

pub struct ProviderStripe(std::rc::Rc<ProviderStripe_>);

impl ProviderStripe {
    pub fn provider_ref(&self) -> std::string::String {
        let data = self.0.data.borrow();
        if let std::option::Option::Some(alias) = &data.alias {
            format!("{}.{}", "stripe", alias)
        } else {
            "stripe".into()
        }
    }

    pub fn set_alias(self, alias: impl std::string::ToString) -> Self {
        self.0.data.borrow_mut().alias = std::option::Option::Some(alias.to_string());
        self
    }

    #[doc = "Set the field `ratelimit`.\nRate limit, max requests per second"]
    pub fn set_ratelimit(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().ratelimit = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::Provider for ProviderStripe_ {
    fn extract_type_tf_id(&self) -> std::string::String {
        "stripe".into()
    }

    fn extract_provider_type(&self) -> serde_json::Value {
        serde_json::json!({
            "source": "andrewbaxter/stripe",
            "version": "0.0.20",
        })
    }

    fn extract_provider(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildProviderStripe {
    #[doc = "Stripe API token"]
    pub token: terrars::PrimField<std::string::String>,
}

impl BuildProviderStripe {
    pub fn build(self, stack: &mut terrars::Stack) -> ProviderStripe {
        let out = ProviderStripe(std::rc::Rc::new(ProviderStripe_ { data: std::cell::RefCell::new(ProviderStripeData {
            alias: std::option::Option::None,
            ratelimit: core::default::Default::default(),
            token: self.token,
        }) }));
        stack.add_provider(out.0.clone());
        out
    }
}
