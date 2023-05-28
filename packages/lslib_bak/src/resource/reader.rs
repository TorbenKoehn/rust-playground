use crate::util::arena::{ArenaReader, Index, RecursiveIter};

use super::{
  node::{attribute::Attribute, data::Data, selector::Selector},
  value::Value,
  Resource,
};

impl ArenaReader<Data> for Resource {
  fn size(&self) -> usize {
    self.arena.size()
  }

  fn parent_index(&self, index: Index) -> Option<Index> {
    self.arena.parent_index(index)
  }

  fn child_indexes(&self, index: Index) -> &Vec<Index> {
    self.arena.child_indexes(index)
  }

  fn value(&self, index: Index) -> &Data {
    self.arena.value(index)
  }

  fn root_indexes(&self) -> &Vec<Index> {
    self.arena.root_indexes()
  }

  fn recursive_iter(&self, index: Index) -> RecursiveIter<'_, Data> {
    self.arena.recursive_iter(index)
  }
}

pub trait ResourceReader: ArenaReader<Data> {
  fn matches(&self, index: Index, selector: &Selector) -> bool;
  fn find(&self, selector: &Selector) -> Vec<Index>;
  fn full_path(&self, index: Index) -> String;
  fn attribute(&self, index: Index, name: &str) -> Option<&Attribute>;
  fn attribute_value(&self, index: Index, name: &str) -> Option<&Value>;
  fn resolve(&self, index: Index, path: &str) -> Option<Index>;
  fn resolve_attribute(&self, index: Index, path: &str) -> Option<&Attribute>;
  fn resolve_attribute_value(&self, index: Index, path: &str) -> Option<&Value>;
}

impl ResourceReader for Resource {
  fn matches(&self, index: Index, selector: &Selector) -> bool {
    let value = self.value(index);
    match selector {
      Selector::Any => true,
      Selector::Name(name) => value.name() == *name,
      Selector::AttributeEquals(name, attr_value) => {
        value.attributes().get(name).map(|a| a.value()) == Some(attr_value)
      }
      Selector::AnyChildMatches(selector) => self
        .child_indexes(index)
        .iter()
        .any(|&child_index| self.matches(child_index, selector)),
      Selector::And(selectors) => selectors
        .iter()
        .all(|selector| self.matches(index, selector)),
      Selector::Or(selectors) => selectors
        .iter()
        .any(|selector| self.matches(index, selector)),
    }
  }

  fn find(&self, selector: &Selector) -> Vec<Index> {
    self
      .root_indexes()
      .iter()
      .flat_map(|&index| {
        self
          .recursive_iter(index)
          .filter(|&current_index| self.matches(current_index, selector))
      })
      .collect()
  }

  fn full_path(&self, index: Index) -> String {
    match self.parent_index(index) {
      Some(parent_index) => {
        let parent_path = self.full_path(parent_index);
        let similar_children: Vec<&Index> = self
          .child_indexes(parent_index)
          .iter()
          .filter(|&&child_index| self.value(index).name() == self.value(child_index).name())
          .collect();
        let similar_children_count = similar_children.len();
        match similar_children_count {
          1 => format!("{}/{}", parent_path, self.value(index).name()),
          _ => {
            let own_parent_index = similar_children
              .iter()
              .position(|&&current_index| index == current_index)
              .unwrap();
            format!(
              "{}/{}[{}]",
              parent_path,
              self.value(index).name(),
              own_parent_index
            )
          }
        }
      }
      None => format!("/{}", self.value(index).name()),
    }
  }

  fn attribute(&self, index: Index, name: &str) -> Option<&Attribute> {
    self.value(index).attributes().get(name)
  }

  fn attribute_value(&self, index: Index, name: &str) -> Option<&Value> {
    self.attribute(index, name).map(|a| a.value())
  }

  fn resolve(&self, index: Index, path: &str) -> Option<Index> {
    if !path.starts_with('/') {
      return None;
    }

    let mut current_index = index;
    for part in path.split('/') {
      if part == "" {
        continue;
      }
      let mut part = part;
      let mut index = current_index;
      let mut index_offset = 0;
      if part.ends_with(']') {
        let mut part_split = part.split('[');
        part = part_split.next().unwrap();
        index_offset = part_split
          .next()
          .unwrap()
          .trim_end_matches(']')
          .parse::<usize>()
          .unwrap();
      }
      let mut found = false;
      for child_index in self.child_indexes(index) {
        if self.value(*child_index).name() == part {
          found = true;
          index = *child_index;
          break;
        }
      }
      if !found {
        return None;
      }
      if index_offset > 0 {
        let similar_children: Vec<&Index> = self
          .child_indexes(index)
          .iter()
          .filter(|&&child_index| self.value(index).name() == self.value(child_index).name())
          .collect();
        if similar_children.len() <= index_offset {
          return None;
        }
        index = **similar_children.get(index_offset).unwrap();
      }
      current_index = index;
    }
    Some(current_index)
  }

  fn resolve_attribute(&self, index: Index, path: &str) -> Option<&Attribute> {
    // Match and retrieve an attribute path like Some/Path/To/A/Node/someAttributeName
    // Uses self.resolve() to fetch the node and then uses the last part of the path as the attribute name
    // If the path is empty, will return None
    // If the part begins with a slash and only contains a single component, the component is the attribute name on the "index" node
    // If the part begins with a slash and contains multiple components, the last component is the attribute name and the rest is the path to the node

    if !path.starts_with('/') {
      return None;
    }

    let mut path_split = path.split('/');
    let attribute_name = path_split.next_back().unwrap();
    let path = path_split.collect::<Vec<&str>>().join("/");
    if path == "" {
      return self.attribute(index, attribute_name);
    }
    let node_index = self.resolve(index, &path)?;
    self.attribute(node_index, attribute_name)
  }

  fn resolve_attribute_value(&self, index: Index, path: &str) -> Option<&Value> {
    self.resolve_attribute(index, path).map(|a| a.value())
  }
}

#[cfg(test)]
mod tests {
  use crate::{resource::node::attribute::AttributeMap, util::arena::ArenaWriter};

  use super::*;

  #[test]
  fn test_matches() {
    let mut resource = Resource::new();
    let root = resource.alloc(
      Data::new_with_attributes("Root".to_owned(), {
        let mut attributes = AttributeMap::new();
        attributes.insert(
          "foo".to_string(),
          Attribute::new_value(Value::String("faz".to_string())),
        );
        attributes
      }),
      None,
    );
    let child_a = resource.alloc(
      Data::new_with_attributes("ChildA".to_owned(), {
        let mut attributes = AttributeMap::new();
        attributes.insert(
          "boo".to_string(),
          Attribute::new_value(Value::String("baz".to_string())),
        );
        attributes
      }),
      Some(root),
    );

    assert!(resource.matches(root, &Selector::Any));
    assert!(resource.matches(child_a, &Selector::Any));
    assert!(resource.matches(root, &Selector::Name("Root")));
    assert!(!resource.matches(root, &Selector::Name("ChildA")));
    assert!(resource.matches(child_a, &Selector::Name("ChildA")));
    assert!(!resource.matches(child_a, &Selector::Name("Root")));
    assert!(resource.matches(
      root,
      &Selector::AttributeEquals("foo", Value::String("faz".to_string()))
    ));
    assert!(!resource.matches(
      root,
      &Selector::AttributeEquals("foo", Value::String("baz".to_string()))
    ));
    assert!(resource.matches(
      child_a,
      &Selector::AttributeEquals("boo", Value::String("baz".to_string()))
    ));
    assert!(!resource.matches(
      child_a,
      &Selector::AttributeEquals("boo", Value::String("faz".to_string()))
    ));
    assert!(resource.matches(root, &Selector::AnyChildMatches(&Selector::Name("ChildA")),));
    assert!(!resource.matches(root, &Selector::AnyChildMatches(&Selector::Name("ChildB")),));
  }

  #[test]
  fn test_resolve() {
    let mut resource = Resource::new();
    let root = resource.alloc(Data::new("Root".to_owned()), None);
    let child_a = resource.alloc(Data::new("ChildA".to_owned()), Some(root));
    let child_b = resource.alloc(Data::new("ChildB".to_owned()), Some(child_a));

    assert_eq!(resource.resolve(root, ""), None);
    assert_eq!(resource.resolve(root, "/"), Some(root));
    assert_eq!(resource.resolve(root, "/Root"), None);
    assert_eq!(resource.resolve(root, "/ChildA"), Some(child_a));
    assert_eq!(resource.resolve(root, "/ChildB"), None);
    assert_eq!(resource.resolve(root, "/ChildA/ChildB"), Some(child_b));

    assert_eq!(resource.resolve(child_a, ""), None);
    assert_eq!(resource.resolve(child_a, "/"), Some(child_a));
    assert_eq!(resource.resolve(child_a, "/Root"), None);
    assert_eq!(resource.resolve(child_a, "/ChildA"), None);
    assert_eq!(resource.resolve(child_a, "/ChildB"), Some(child_b));

    assert_eq!(resource.resolve(child_b, ""), None);
    assert_eq!(resource.resolve(child_b, "/"), Some(child_b));
    assert_eq!(resource.resolve(child_b, "/Root"), None);
    assert_eq!(resource.resolve(child_b, "/ChildA"), None);
    assert_eq!(resource.resolve(child_b, "/ChildB"), None);
  }

  #[test]
  fn test_resolve_attribute() {
    let mut resource = Resource::new();
    let root = resource.alloc(
      Data::new_with_attributes("Root".to_owned(), {
        let mut attributes = AttributeMap::new();
        attributes.insert(
          "foo".to_string(),
          Attribute::new_value(Value::String("faz".to_string())),
        );
        attributes
      }),
      None,
    );
    let child_a = resource.alloc(
      Data::new_with_attributes("ChildA".to_owned(), {
        let mut attributes = AttributeMap::new();
        attributes.insert(
          "boo".to_string(),
          Attribute::new_value(Value::String("baz".to_string())),
        );
        attributes
      }),
      Some(root),
    );

    assert_eq!(resource.resolve_attribute(root, ""), None);
    assert_eq!(resource.resolve_attribute(root, "/"), None);
    assert_eq!(resource.resolve_attribute(root, "/Root"), None);
    assert_eq!(
      resource.resolve_attribute(root, "/foo"),
      Some(&Attribute::new_value(Value::String("faz".to_string())))
    );
    assert_eq!(resource.resolve_attribute(root, "/ChildA"), None);
    assert_eq!(
      resource.resolve_attribute(root, "/ChildA/boo"),
      Some(&Attribute::new_value(Value::String("baz".to_string())))
    );

    assert_eq!(resource.resolve_attribute(child_a, ""), None);
    assert_eq!(resource.resolve_attribute(child_a, "/"), None);
    assert_eq!(resource.resolve_attribute(child_a, "/Root"), None);
    assert_eq!(resource.resolve_attribute(child_a, "/Root/foo"), None);
    assert_eq!(resource.resolve_attribute(child_a, "/ChildA"), None);
    assert_eq!(
      resource.resolve_attribute(child_a, "/boo"),
      Some(&Attribute::new_value(Value::String("baz".to_string())))
    );
  }
}
