use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct ProviderStripeData {
    #[serde(skip_serializing_if = "Option::is_none")]
    alias: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ratelimit: Option<PrimField<f64>>,
    token: PrimField<String>,
}

struct ProviderStripe_ {
    data: RefCell<ProviderStripeData>,
}

pub struct ProviderStripe(Rc<ProviderStripe_>);

impl ProviderStripe {
    pub fn provider_ref(&self) -> String {
        let data = self.0.data.borrow();
        if let Some(alias) = &data.alias {
            format!("{}.{}", "stripe", alias)
        } else {
            "stripe".into()
        }
    }

    pub fn set_alias(self, alias: impl ToString) -> Self {
        self.0.data.borrow_mut().alias = Some(alias.to_string());
        self
    }

    #[doc= "Set the field `ratelimit`.\nRate limit, max requests per second"]
    pub fn set_ratelimit(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().ratelimit = Some(v.into());
        self
    }
}

impl Provider for ProviderStripe_ {
    fn extract_type_tf_id(&self) -> String {
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
    #[doc= "Stripe API token"]
    pub token: PrimField<String>,
}

impl BuildProviderStripe {
    pub fn build(self, stack: &mut Stack) -> ProviderStripe {
        let out = ProviderStripe(Rc::new(ProviderStripe_ { data: RefCell::new(ProviderStripeData {
            alias: None,
            ratelimit: core::default::Default::default(),
            token: self.token,
        }) }));
        stack.add_provider(out.0.clone());
        out
    }
}
