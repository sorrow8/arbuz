use metashrew_support::index_pointer::KeyValuePointer;
use metashrew_support::compat::to_arraybuffer_layout;

use alkanes_runtime::{
  declare_alkane, message::MessageDispatch, storage::StoragePointer, token::Token,
  runtime::AlkaneResponder
};

use alkanes_support::{
  cellpack::Cellpack, id::AlkaneId,
  parcel::{AlkaneTransfer, AlkaneTransferParcel}, response::CallResponse
};

use anyhow::{anyhow, Result};
use std::sync::Arc;

mod js_generator;
use js_generator::JsGenerator;

mod predict_generator;
mod roman_numerals;

const ARBUZ_ORBITAL_TEMPLATE_ID: u128 = 0x1;

// Token required for minting
const REQUIRED_MINT_TOKEN_ID: AlkaneId = AlkaneId {
  block: 0x4,
  tx: 0x0,
};

#[derive(Default)]
pub struct MagicArbuzCollection(());

impl AlkaneResponder for MagicArbuzCollection {}

#[derive(MessageDispatch)]
enum MagicArbuzCollectionMessage {
  #[opcode(0)]
  Initialize,

  #[opcode(69)]
  AuthMintOrbital { count: u128 },

  #[opcode(77)]
  MintOrbital,

  #[opcode(99)]
  #[returns(String)]
  GetName,

  #[opcode(100)]
  #[returns(String)]
  GetSymbol,

  #[opcode(101)]
  #[returns(u128)]
  GetTotalSupply,

  #[opcode(102)]
  #[returns(u128)]
  GetOrbitalCount,

  #[opcode(999)]
  #[returns(String)]
  GetAttributes { index: u128 },

  #[opcode(1000)]
  #[returns(Vec<u8>)]
  GetData { index: u128 },

  #[opcode(1001)]
  #[returns(Vec<u8>)]
  GetInstanceAlkaneId { index: u128 },

  #[opcode(1002)]
  #[returns(String)]
  GetInstanceIdentifier { index: u128 },
}

impl Token for MagicArbuzCollection {
  fn name(&self) -> String {
    return String::from("Magic Arbuz Parent")
  }

  fn symbol(&self) -> String {
    return String::from("magic-arbuz-parent");
  }
}

impl MagicArbuzCollection {
  fn initialize(&self) -> Result<CallResponse> {
    self.observe_initialization()?;
    let context = self.context()?;

    let mut response = CallResponse::forward(&context.incoming_alkanes);

    // Collection token acts as auth token for contract minting without any limits
    response.alkanes.0.push(AlkaneTransfer {
      id: context.myself.clone(),
      value: 10u128,
    });

    Ok(response)
  }

  fn auth_mint_orbital(&self, count: u128) -> Result<CallResponse> {
    let context = self.context()?;
    let mut response = CallResponse::forward(&context.incoming_alkanes);

    // Authorized mints
    self.only_owner()?;

    let mut minted_orbitals = Vec::new();

    for _ in 0..count {
      minted_orbitals.push(self.create_mint_transfer()?);
    }

    response.alkanes.0.extend(minted_orbitals);

    Ok(response)
  }

  fn external_clockin_check(&self) -> Result<CallResponse> {
      let clockin_id = AlkaneId {
          block: 2,
          tx: 3,
      };
      let cellpack = Cellpack {
          target: clockin_id,
          inputs: vec![103],
      };
      let response = self.call(
          &cellpack,
          &AlkaneTransferParcel::default(),
          self.fuel()
      )?;
      Ok(response)
  }

  fn mint_orbital(&self) -> Result<CallResponse> {
    let context = self.context()?;
    let mut response = CallResponse::forward(&context.incoming_alkanes);

    // Check if required token is provided
    if context.incoming_alkanes.0.len() != 1 {
      return Err(anyhow!(
        "Incoming alkanes must be the required mint token"
      ));
    }

    let transfer = context.incoming_alkanes.0[0].clone();
    if transfer.id != REQUIRED_MINT_TOKEN_ID {
      return Err(anyhow!("Incoming alkane is not the required mint token"));
    }

    let clockin_result = self.external_clockin_check();
    if clockin_result.is_err() {
        return Err(anyhow!("Clock-in failed, cards say you are retarded:("));
    }
    response.alkanes.0.push(self.create_mint_transfer()?);
    Ok(response)
}

  fn create_mint_transfer(&self) -> Result<AlkaneTransfer> {
    let index = self.instances_count();

    let cellpack = Cellpack {
      target: AlkaneId {
        block: 6,
        tx: ARBUZ_ORBITAL_TEMPLATE_ID,
      },
      inputs: vec![0x0, index],
    };

    let sequence = self.sequence();
    let response = self.call(&cellpack, &AlkaneTransferParcel::default(), self.fuel())?;

    let orbital_id = AlkaneId {
      block: 2,
      tx: sequence,
    };

    self.add_instance(&orbital_id)?;

    if response.alkanes.0.len() < 1 {
      Err(anyhow!("orbital token not returned with factory"))
    } else {
      Ok(response.alkanes.0[0])
    }
  }

  fn get_name(&self) -> Result<CallResponse> {
    let context = self.context()?;
    let mut response = CallResponse::forward(&context.incoming_alkanes);

    response.data = self.name().into_bytes().to_vec();

    Ok(response)
  }

  fn get_symbol(&self) -> Result<CallResponse> {
    let context = self.context()?;
    let mut response = CallResponse::forward(&context.incoming_alkanes);

    response.data = self.symbol().into_bytes().to_vec();

    Ok(response)
  }

  fn get_total_supply(&self) -> Result<CallResponse> {
    let context = self.context()?;
    let mut response = CallResponse::forward(&context.incoming_alkanes);

    response.data = (&self.instances_count().to_le_bytes()).to_vec();

    Ok(response)
  }

  fn get_orbital_count(&self) -> Result<CallResponse> {
    let context = self.context()?;
    let mut response = CallResponse::forward(&context.incoming_alkanes);

    response.data = (&self.instances_count().to_le_bytes()).to_vec();

    Ok(response)
  }

  fn get_data(&self, index: u128) -> Result<CallResponse> {
    let context = self.context()?;
    let mut response = CallResponse::forward(&context.incoming_alkanes);

    let js_code = JsGenerator::generate_js(index)?;
    response.data = js_code.into_bytes();
    Ok(response)
  }

  fn get_attributes(&self, index: u128) -> Result<CallResponse> {
    let context = self.context()?;
    let mut response = CallResponse::forward(&context.incoming_alkanes);

    let attributes = JsGenerator::get_attributes(index)?;
    response.data = attributes.into_bytes();
    Ok(response)
  }

  fn instances_pointer(&self) -> StoragePointer {
    StoragePointer::from_keyword("/instances")
  }

  fn instances_count(&self) -> u128 {
    self.instances_pointer().get_value::<u128>()
  }

  fn set_instances_count(&self, count: u128) {
    self.instances_pointer().set_value(count);
  }

  fn add_instance(&self, instance_id: &AlkaneId) -> Result<u128> {
    let count = self.instances_count();
    let new_count = count.checked_add(1)
      .ok_or_else(|| anyhow!("instances count overflow"))?;

    let mut bytes = Vec::with_capacity(32);
    bytes.extend_from_slice(&instance_id.block.to_le_bytes());
    bytes.extend_from_slice(&instance_id.tx.to_le_bytes());

    let bytes_vec = new_count.to_le_bytes().to_vec();
    let mut instance_pointer = self.instances_pointer().select(&bytes_vec);
    instance_pointer.set(Arc::new(bytes));
    
    self.set_instances_count(new_count);
    
    Ok(new_count)
  }

  fn only_owner(&self) -> Result<()> {
    let context = self.context()?;

    if context.incoming_alkanes.0.len() != 1 {
      return Err(anyhow!(
        "did not authenticate with only the collection token"
      ));
    }

    let transfer = context.incoming_alkanes.0[0].clone();
    if transfer.id != context.myself.clone() {
      return Err(anyhow!("supplied alkane is not collection token"));
    }

    if transfer.value < 1 {
      return Err(anyhow!(
        "less than 1 unit of collection token supplied to authenticate"
      ));
    }

    Ok(())
  }

  fn lookup_instance(&self, index: u128) -> Result<AlkaneId> {
    // Add 1 to index since instances are stored at 1-based indices
    let storage_index = index + 1;
    let bytes_vec = storage_index.to_le_bytes().to_vec();
    
    let instance_pointer = self.instances_pointer().select(&bytes_vec);
    
    let bytes = instance_pointer.get();
    if bytes.len() != 32 {
      return Err(anyhow!("Invalid instance data length"));
    }

    let block_bytes = &bytes[..16];
    let tx_bytes = &bytes[16..];

    let block = u128::from_le_bytes(block_bytes.try_into().unwrap());
    let tx = u128::from_le_bytes(tx_bytes.try_into().unwrap());

    Ok(AlkaneId { block, tx })
  }

  fn get_instance_alkane_id(&self, index: u128) -> Result<CallResponse> {
    let context = self.context()?;
    let mut response = CallResponse::forward(&context.incoming_alkanes);

    let instance_id = self.lookup_instance(index)?;
    let mut bytes = Vec::with_capacity(32);
    bytes.extend_from_slice(&instance_id.block.to_le_bytes());
    bytes.extend_from_slice(&instance_id.tx.to_le_bytes());

    response.data = bytes;

    Ok(response)
  }

  fn get_instance_identifier(&self, index: u128) -> Result<CallResponse> {
    let context = self.context()?;
    let mut response = CallResponse::forward(&context.incoming_alkanes);

    let instance_id = self.lookup_instance(index)?;
    response.data = format!("{}:{}", instance_id.block, instance_id.tx).into_bytes();

    Ok(response)
  }
}

declare_alkane! {
  impl AlkaneResponder for MagicArbuzCollection {
    type Message = MagicArbuzCollectionMessage;
  }
}
