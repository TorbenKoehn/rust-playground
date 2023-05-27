use crate::{
  resource::{
    node::{
      attribute::{Attribute, AttributeMap},
      data::Data,
      selector::Selector,
    },
    reader::ResourceReader,
    value::Value,
    writer::ResourceWriter,
  },
  util::arena::Index,
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Item(Index);

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ItemFactory(Index);

pub trait DosEeItemResourceReaderExt: ResourceReader {
  fn dos_ee_items_by_parent_id(&self, parent_id: u32) -> Vec<Item> {
    let selector = Selector::And(vec![
      Selector::Name("Item"),
      Selector::AttributeEquals("Parent", Value::UInt(parent_id)),
    ]);
    self
      .find(&selector)
      .iter()
      .map(|&index| Item(index))
      .collect()
  }

  fn dos_ee_item_name(&self, item: Item) -> String {
    self
      .resolve_attribute_value(item.0, "/Stats")
      .map_or("Not Available", |value| match value {
        Value::FixedString(name) => {
          if name.is_empty() {
            "Empty"
          } else {
            name.as_str()
          }
        }
        _ => "Wrong Type",
      })
      .to_owned()
  }

  fn dos_ee_item_amount(&self, item: Item) -> i32 {
    self
      .resolve_attribute_value(item.0, "/Amount")
      .map_or(1, |value| match value {
        Value::Int(amount) => *amount,
        _ => 0,
      })
  }

  fn dos_ee_item_owner_id(&self, item: Item) -> u32 {
    self
      .resolve_attribute_value(item.0, "/owner")
      .map_or(0, |value| match value {
        Value::UInt(owner_id) => *owner_id,
        _ => 0,
      })
  }

  fn dos_ee_item_factory(&self) -> ItemFactory {
    let selector = Selector::Name("ItemFactory");
    ItemFactory(self.find(&selector).first().unwrap().clone())
  }
}

impl<R: ResourceReader> DosEeItemResourceReaderExt for R {}

#[derive(Clone, Debug, PartialEq)]
pub struct ItemCreateData {
  pub parent_id: u32,
  pub owner_id: u32,
  pub name: String,
  pub amount: i32,
  pub item_type: String,
}

pub trait DosEeItemResourceWriterExt: ResourceWriter {
  fn dos_ee_create_item(&mut self, item_factory: ItemFactory, data: ItemCreateData) -> Item {
    // /Items/ItemFactory/Items/Item[3213]/@Parent = Attribute { data: UInt(335610127) } (Inventory ID)
    // /Items/ItemFactory/Items/Item[3213]/@CurrentTemplateType = Attribute { data: Byte(0) }
    // /Items/ItemFactory/Items/Item[3213]/@Global = Attribute { data: Bool(true) }
    // /Items/ItemFactory/Items/Item[3213]/@GoldValueOverwrite = Attribute { data: Int(-1) }
    // /Items/ItemFactory/Items/Item[3213]/@Amount = Attribute { data: Int(1) }
    // /Items/ItemFactory/Items/Item[3213]/@UnsoldGenerated = Attribute { data: Bool(false) }
    // /Items/ItemFactory/Items/Item[3213]/@Key = Attribute { data: FixedString() }
    // /Items/ItemFactory/Items/Item[3213]/@Slot = Attribute { data: UShort(16) }
    // /Items/ItemFactory/Items/Item[3213]/@Rotate = Attribute { data: Mat3([[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]]) }
    // /Items/ItemFactory/Items/Item[3213]/@owner = Attribute { data: UInt(67174668) }
    // /Items/ItemFactory/Items/Item[3213]/@ItemType = Attribute { data: FixedString(Common) }
    // /Items/ItemFactory/Items/Item[3213]/@Inventory = Attribute { data: UInt(0) }
    // /Items/ItemFactory/Items/Item[3213]/@Level = Attribute { data: FixedString() }
    // /Items/ItemFactory/Items/Item[3213]/@IsKey = Attribute { data: Bool(false) }
    // /Items/ItemFactory/Items/Item[3213]/@TreasureGenerated = Attribute { data: Bool(false) }
    // /Items/ItemFactory/Items/Item[3213]/@CurrentTemplate = Attribute { data: FixedString(57a03d74-32d6-4bc9-b143-8c89ae7c0f1b) }
    // /Items/ItemFactory/Items/Item[3213]/@LockLevel = Attribute { data: Int(1) }
    // /Items/ItemFactory/Items/Item[3213]/@OriginalTemplate = Attribute { data: FixedString(57a03d74-32d6-4bc9-b143-8c89ae7c0f1b) }
    // /Items/ItemFactory/Items/Item[3213]/@Vitality = Attribute { data: Int(-1) }
    // /Items/ItemFactory/Items/Item[3213]/@SurfaceCheckTimer = Attribute { data: Float(0) }
    // /Items/ItemFactory/Items/Item[3213]/@Velocity = Attribute { data: Vec3([0.0, 0.0, 0.0]) }
    // /Items/ItemFactory/Items/Item[3213]/@IsGenerated = Attribute { data: Bool(false) }
    // /Items/ItemFactory/Items/Item[3213]/@Flags = Attribute { data: UInt(33752) }
    // /Items/ItemFactory/Items/Item[3213]/@Stats = Attribute { data: FixedString(CON_Potion_Invisible_A) }
    // /Items/ItemFactory/Items/Item[3213]/@Scale = Attribute { data: Float(1) }
    // /Items/ItemFactory/Items/Item[3213]/@LifeTime = Attribute { data: Float(0) }
    // /Items/ItemFactory/Items/Item[3213]/@Translate = Attribute { data: Vec3([0.0, 0.0, 0.0]) }
    // /Items/ItemFactory/Items/Item[3213]/@MaxVitalityPatchCheck = Attribute { data: Int(-1) }
    // /Items/ItemFactory/Items/Item[3213]/@OriginalTemplateType = Attribute { data: Byte(0) }

    let node_data = Data::new_with_attributes("Item".to_owned(), {
      let mut attributes = AttributeMap::new();
      attributes.insert(
        "Parent".to_owned(),
        Attribute::new_value(Value::UInt(data.parent_id)),
      );
      attributes.insert(
        "CurrentTemplateType".to_owned(),
        Attribute::new_value(Value::Byte(0)),
      );
      attributes.insert("Global".to_owned(), Attribute::new_value(Value::Bool(true)));
      attributes.insert(
        "GoldValueOverwrite".to_owned(),
        Attribute::new_value(Value::Int(-1)),
      );
      attributes.insert(
        "Amount".to_owned(),
        Attribute::new_value(Value::Int(data.amount)),
      );
      attributes.insert(
        "UnsoldGenerated".to_owned(),
        Attribute::new_value(Value::Bool(false)),
      );
      attributes.insert(
        "Key".to_owned(),
        Attribute::new_value(Value::FixedString("".to_owned())),
      );
      attributes.insert("Slot".to_owned(), Attribute::new_value(Value::UShort(16)));
      attributes.insert(
        "Rotate".to_owned(),
        Attribute::new_value(Value::Mat3([
          [1.0, 0.0, 0.0],
          [0.0, 1.0, 0.0],
          [0.0, 0.0, 1.0],
        ])),
      );
      attributes.insert(
        "owner".to_owned(),
        Attribute::new_value(Value::UInt(data.owner_id)),
      );
      attributes.insert(
        "ItemType".to_owned(),
        Attribute::new_value(Value::FixedString(data.item_type.to_string())),
      );
      attributes.insert("Inventory".to_owned(), Attribute::new_value(Value::UInt(0)));
      attributes.insert(
        "Level".to_owned(),
        Attribute::new_value(Value::FixedString("".to_owned())),
      );
      attributes.insert("IsKey".to_owned(), Attribute::new_value(Value::Bool(false)));
      attributes.insert(
        "TreasureGenerated".to_owned(),
        Attribute::new_value(Value::Bool(false)),
      );
      attributes.insert(
        "CurrentTemplate".to_owned(),
        Attribute::new_value(Value::FixedString(
          "57a03d74-32d6-4bc9-b143-8c89ae7c0f1b".to_owned(),
        )),
      );
      attributes.insert("LockLevel".to_owned(), Attribute::new_value(Value::Int(1)));
      attributes.insert(
        "OriginalTemplate".to_owned(),
        Attribute::new_value(Value::FixedString(
          "57a03d74-32d6-4bc9-b143-8c89ae7c0f1b".to_owned(),
        )),
      );
      attributes.insert("Vitality".to_owned(), Attribute::new_value(Value::Int(-1)));
      attributes.insert(
        "SurfaceCheckTimer".to_owned(),
        Attribute::new_value(Value::Float(0.0)),
      );
      attributes.insert(
        "Velocity".to_owned(),
        Attribute::new_value(Value::Vec3([0.0, 0.0, 0.0])),
      );
      attributes.insert(
        "IsGenerated".to_owned(),
        Attribute::new_value(Value::Bool(false)),
      );
      attributes.insert("Flags".to_owned(), Attribute::new_value(Value::UInt(33752)));
      attributes.insert(
        "Stats".to_owned(),
        Attribute::new_value(Value::FixedString(data.name)),
      );
      attributes.insert("Scale".to_owned(), Attribute::new_value(Value::Float(1.0)));
      attributes.insert(
        "LifeTime".to_owned(),
        Attribute::new_value(Value::Float(0.0)),
      );
      attributes.insert(
        "Translate".to_owned(),
        Attribute::new_value(Value::Vec3([0.0, 0.0, 0.0])),
      );
      attributes.insert(
        "MaxVitalityPatchCheck".to_owned(),
        Attribute::new_value(Value::Int(-1)),
      );
      attributes.insert(
        "OriginalTemplateType".to_owned(),
        Attribute::new_value(Value::Byte(0)),
      );
      attributes
    });
    let index = self.alloc(node_data, Some(item_factory.0));
    Item(index)
  }
}

impl<R: ResourceWriter> DosEeItemResourceWriterExt for R {}
