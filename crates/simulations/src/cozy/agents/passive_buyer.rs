use std::{borrow::Cow, sync::Arc};

use bindings::cozy_protocol::cozy_router;
use eyre::Result;
use revm::primitives::TxEnv;
use simulate::{
    agent::{agent_channel::AgentChannel, types::AgentId, Agent},
    contract::sim_contract::SimContract,
    state::{update::SimUpdate, SimState},
    utils::build_call_contract_txenv,
};

use crate::cozy::{
    agents::errors::CozyAgentError,
    world::{CozyProtocolContract, CozyUpdate, CozyWorld},
    EthersAddress, EthersU256, EvmAddress,
};

pub struct PassiveBuyer {
    name: Option<Cow<'static, str>>,
    address: EvmAddress,
    cozyrouter: Option<Arc<CozyProtocolContract>>,
    base_asset: Option<Arc<CozyProtocolContract>>,
    capital: EthersU256,
}

impl PassiveBuyer {
    pub fn new(name: Option<Cow<'static, str>>, address: EvmAddress, capital: EthersU256) -> Self {
        Self {
            name,
            address,
            cozyrouter: None,
            base_asset: None,
            capital,
        }
    }
}

impl Agent<CozyUpdate, CozyWorld> for PassiveBuyer {
    fn id(&self) -> AgentId {
        AgentId {
            name: self.name.clone(),
            address: self.address,
        }
    }

    fn activation_step(
        &mut self,
        state: &SimState<CozyUpdate, CozyWorld>,
        _channel: AgentChannel<CozyUpdate>,
    ) {
        self.cozyrouter = Some(
            state
                .world
                .protocol_contracts
                .get("CozyRouter")
                .ok_or(CozyAgentError::UnregisteredAddress)
                .unwrap()
                .clone(),
        );
        self.base_asset = Some(
            state
                .world
                .protocol_contracts
                .get("DummyToken")
                .ok_or(CozyAgentError::UnregisteredAddress)
                .unwrap()
                .clone(),
        );
    }

    fn resolve_activation_step(&mut self, _state: &SimState<CozyUpdate, CozyWorld>) {}

    fn step(
        &mut self,
        _state: &SimState<CozyUpdate, CozyWorld>,
        _channel: AgentChannel<CozyUpdate>,
    ) {
    }

    fn resolve_step(&mut self, _state: &SimState<CozyUpdate, CozyWorld>) {}
}

impl PassiveBuyer {
    fn max_approve_router(&self) -> Result<TxEnv> {
        Ok(build_call_contract_txenv(
            self.address,
            self.base_asset.as_ref().unwrap().address,
            self.base_asset.as_ref().unwrap().contract.encode_function(
                "approve",
                (
                    EthersAddress::from(self.cozyrouter.as_ref().unwrap().address),
                    EthersU256::MAX,
                ),
            )?,
            None,
            None,
        ))
    }

    fn build_purchase_tx(&self, args: cozy_router::PurchaseCall) -> Result<TxEnv> {
        Ok(build_call_contract_txenv(
            self.address,
            self.cozyrouter.as_ref().unwrap().address,
            self.cozyrouter
                .as_ref()
                .unwrap()
                .contract
                .encode_function("purchase", args)?,
            None,
            None,
        ))
    }

    fn purchase_protection_without_transfer(
        &self,
        args: cozy_router::PurchaseWithoutTransferCall,
    ) -> Result<TxEnv> {
        Ok(build_call_contract_txenv(
            self.address,
            self.cozyrouter.as_ref().unwrap().address,
            self.cozyrouter
                .as_ref()
                .unwrap()
                .contract
                .encode_function("purchaseWithoutTransfer", args)?,
            None,
            None,
        ))
    }

    fn cancel_protection(&self, args: cozy_router::CancelCall) -> Result<TxEnv> {
        Ok(build_call_contract_txenv(
            self.address,
            self.cozyrouter.as_ref().unwrap().address,
            self.cozyrouter
                .as_ref()
                .unwrap()
                .contract
                .encode_function("cancel", args)?,
            None,
            None,
        ))
    }

    fn sell_protection(&self, args: cozy_router::SellCall) -> Result<TxEnv> {
        Ok(build_call_contract_txenv(
            self.address,
            self.cozyrouter.as_ref().unwrap().address,
            self.cozyrouter
                .as_ref()
                .unwrap()
                .contract
                .encode_function("sell", args)?,
            None,
            None,
        ))
    }

    fn claim_ptokens(&self, args: cozy_router::ClaimCall) -> Result<TxEnv> {
        Ok(build_call_contract_txenv(
            self.address,
            self.cozyrouter.as_ref().unwrap().address,
            self.cozyrouter
                .as_ref()
                .unwrap()
                .contract
                .encode_function("claim", args)?,
            None,
            None,
        ))
    }

    fn payout_protection(&self, args: cozy_router::PayoutCall) -> Result<TxEnv> {
        Ok(build_call_contract_txenv(
            self.address,
            self.cozyrouter.as_ref().unwrap().address,
            self.cozyrouter
                .as_ref()
                .unwrap()
                .contract
                .encode_function("payout", args)?,
            None,
            None,
        ))
    }
}
