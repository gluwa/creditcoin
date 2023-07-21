use crate::address_registrar::{self, ExternalAddressRegistrar, Registrar};
#[cfg(test)]
use address_registrar::MockExternalAddressRegistrar;

pub trait Context {
	type Registrar;

	fn registrar(&self) -> Self::Registrar;
}

pub struct DefaultContext {}

impl DefaultContext {
	pub fn default() -> Self {
		DefaultContext {}
	}
}

impl Context for DefaultContext {
	type Registrar = Registrar;

	fn registrar(&self) -> Self::Registrar {
		Registrar::default()
	}
}

#[cfg(test)]
pub struct MockContext {}

#[cfg(test)]
impl Context for MockContext {
	type Registrar = MockExternalAddressRegistrar;

	fn registrar(&self) -> Self::Registrar {
		MockExternalAddressRegistrar::new()
	}
}
