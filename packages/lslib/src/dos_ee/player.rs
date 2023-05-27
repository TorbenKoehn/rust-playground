use crate::{
  resource::{node::selector::Selector, reader::ResourceReader, value::Value},
  util::arena::Index,
};

use super::item::{DosEeItemResourceReaderExt, Item, ItemCreateData};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Player(Index);

pub trait DosEePlayerResourceReaderExt: ResourceReader {
  fn dos_ee_players(&self) -> Vec<Player> {
    let child_selector = &Selector::And(vec![
      Selector::Name("Stats"),
      Selector::AttributeEquals("IsPlayer", Value::Bool(true)),
    ]);
    let selector = Selector::And(vec![
      Selector::Name("Character"),
      Selector::AnyChildMatches(child_selector),
    ]);
    self
      .find(&selector)
      .iter()
      .map(|index| Player(*index))
      .collect()
  }

  fn dos_ee_player_name(&self, player: Player) -> String {
    self
      .resolve_attribute_value(player.0, "/PlayerData/PlayerCustomData/Name")
      .map_or("Not Available", |value| match value {
        Value::WString(name) => {
          if name.is_empty() {
            "Henchmen"
          } else {
            name.as_str()
          }
        }
        _ => "Invalid Type",
      })
      .to_owned()
  }

  fn dos_ee_player_race_name(&self, player: Player) -> String {
    self
      .resolve_attribute_value(player.0, "/PlayerData/PlayerCustomData/Race")
      .map_or("Not Available", |value| match value {
        Value::TranslatedString {
          value: string_value,
          ..
        } => string_value,
        _ => "Invalid Type",
      })
      .to_owned()
  }

  fn dos_ee_player_class_name(&self, player: Player) -> String {
    self
      .resolve_attribute_value(player.0, "/PlayerData/PlayerCustomData/ClassType")
      .map_or("Not Available", |value| match value {
        Value::FixedString(string_value) => string_value,
        _ => "Invalid Type",
      })
      .to_owned()
  }

  fn dos_ee_player_inventory_id(&self, player: Player) -> u32 {
    self
      .resolve_attribute_value(player.0, "/Inventory")
      .map_or(0, |value| match value {
        Value::UInt(id) => *id,
        _ => 0,
      })
  }

  fn dos_ee_player_inventory_items(&self, player: Player) -> Vec<Item>
  where
    Self: Sized,
  {
    let inventory_id = self.dos_ee_player_inventory_id(player);
    self.dos_ee_items_by_parent_id(inventory_id)
  }

  fn dos_ee_player_create_item_data(
    &self,
    player: Player,
    name: String,
    item_type: String,
    amount: i32,
  ) -> ItemCreateData
  where
    Self: Sized,
  {
    let inventory_id = self.dos_ee_player_inventory_id(player);
    // Select a single item from the inventory to fetch the proper owner ID
    let items = self.dos_ee_items_by_parent_id(inventory_id);
    let first_item = items.first().unwrap();
    let owner_id = self.dos_ee_item_owner_id(*first_item);
    ItemCreateData {
      parent_id: inventory_id,
      owner_id,
      name,
      amount,
      item_type,
    }
  }
}

impl<R: ResourceReader> DosEePlayerResourceReaderExt for R {}
