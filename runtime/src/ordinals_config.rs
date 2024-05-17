use crate::{Runtime, RuntimeEvent};

impl pallet_ordinals::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type InscriptionId = u128;
	type WeightInfo = pallet_ordinals::weights::SubstrateWeight<Runtime>;
}
