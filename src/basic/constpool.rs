use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::cmp::{Eq, PartialEq};
use result::*;

/// A constant pool item
#[derive(Debug, Clone)]
pub enum Item {
    /// An UTF-8 encoded string.
    /// Inside the class file itself, a modified format is used.
    UTF8(String),
    /// An `int`.
    Integer(i32),
    /// A `float`.
    Float(f32),
    /// A `long`.
    /// Takes two spots, instead of one.
    Long(i64),
    /// A `double`.
    /// Takes two spots, instead of one.
    Double(f64),
    /// An index to the name of the class, or the descriptor of an array type.
    /// Always refers to an `Item::UTF8(_)`.
    Class(u16),
    /// A `java.lang.String` object.
    String(u16),
    /// Describes a field reference.
    FieldRef {
        /// The index to an `Item::Class(_)`.
        /// Can be either a Class, or an Interface.
        class: u16,
        /// The index to an `Item::NameAndType { .. }`.
        name_and_type: u16,
    },
    /// Describes a method reference.
    MethodRef {
        /// The index to an `Item::Class(_)`.
        /// Must be a Class.
        class: u16,
        /// The index to an `Item::NameAndType { .. }`.
        name_and_type: u16,
    },
    /// Describes a method reference, where the class is an interface.
    InterfaceMethodRef {
        /// The index to an `Item::Class(_)`.
        /// Must be an Interface.
        class: u16,
        /// The index to an `Item::NameAndType { .. }`.
        name_and_type: u16,
    },
    /// Represents a field or method, without indicating which class or type it belongs to.
    NameAndType {
        /// The index to an `Item::UTF8(_)`.
        /// Must either be a valid unqualfied name or `<init>`.
        name: u16,
        /// The index to an `Item::UTF8(_)`.
        /// Represents a valid field or method descriptor.
        desc: u16,
    },
    /// Represents a method handle
    MethodHandle {
        /// Characterizes its bytecode behaviour.
        kind: ReferenceKind,
        /// If kind is either GetField, GetStatic, PutField, or PutStatic,
        /// the entry at that index must be a `Item::FieldRef { .. }`.
        /// If kind is InvokeVirtual or InvokeSpecial,
        /// the entry at that index must be `Item::MethodRef { .. }`.
        /// If kind is InvokeStatic or InvokeSpecial
        /// and the version of the class is less than 52.0,
        /// the entry at that index must be `Item::MethodRef { .. }`.
        /// If it is 52.0 or above,
        /// it must either be a MethodRef or an `Item::InterfaceMethodRef { .. }`.
        /// If kind is InvokeInterface,
        /// the entry at that index must be an `Item::InterfaceMethodRef { .. }`.
        index: u16,
    },
    /// Describes a method type.
    /// The entry at that index must be an `Item::UTF8(_)`
    /// representing a method descriptor.
    MethodType(u16),
    /// Describes a invoke dynamic instruction,
    /// and specifies information regarding the bootstrap method.
    InvokeDynamic {
        /// The index to an entry of the BootstrapMethods attribute of the class file.
        bootstrap_method_attribute: u16,
        /// The index to an `Item::NameAndType { .. }`.
        name_and_type: u16,
    },
    /// Represents a module.
    /// The entry at that index must be a `Item::UTF8(_)` with a valid name.
    /// The class must have the MODULE flag set.
    Module(u16),
    /// Represents a package exported or opened by a module.
    /// The entry at that index must be an `Item::UTF8(_)`
    /// with a valid package name encoded in internal form.
    /// The class must have the MODULE flag set.
    Package(u16),
}

impl Item {
    /// Returns true if this item takes up two spaces, false otherwise.
    fn is_double(&self) -> bool {
        match *self {
            Item::Long(_) | Item::Double(_) => true,
            _ => false,
        }
    }
}

// Implementing `Hash` and `Eq` manually (sorry for this awful mess of code),
// since `Item` contains f32 and f64, which by default can't be hashed.
// This is good normally, but here we are okay
// to have multiple f32 or f64,
// which are not equal bitwise but contextwise.

impl Hash for Item {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match *self {
            Item::UTF8(ref s) => {
                state.write_u8(1);
                s.hash(state);
            }
            Item::Integer(i) => {
                state.write_u8(3);
                i.hash(state);
            }
            Item::Float(f) => {
                state.write_u8(4);
                f.to_bits().hash(state);
            }
            Item::Long(i) => {
                state.write_u8(5);
                i.hash(state);
            }
            Item::Double(f) => {
                state.write_u8(6);
                f.to_bits().hash(state);
            }
            Item::Class(ptr) => {
                state.write_u8(7);
                ptr.hash(state);
            }
            Item::String(ptr) => {
                state.write_u8(8);
                ptr.hash(state);
            }
            Item::FieldRef {
                class,
                name_and_type,
            } => {
                state.write_u8(9);
                class.hash(state);
                name_and_type.hash(state);
            }
            Item::MethodRef {
                class,
                name_and_type,
            } => {
                state.write_u8(10);
                class.hash(state);
                name_and_type.hash(state);
            }
            Item::InterfaceMethodRef {
                class,
                name_and_type,
            } => {
                state.write_u8(11);
                class.hash(state);
                name_and_type.hash(state);
            }
            Item::NameAndType { name, desc } => {
                state.write_u8(12);
                name.hash(state);
                desc.hash(state);
            }
            Item::MethodHandle { ref kind, index } => {
                state.write_u8(15);
                kind.hash(state);
                index.hash(state);
            }
            Item::MethodType(ptr) => {
                state.write_u8(16);
                ptr.hash(state);
            }
            Item::InvokeDynamic {
                bootstrap_method_attribute,
                name_and_type,
            } => {
                state.write_u8(18);
                bootstrap_method_attribute.hash(state);
                name_and_type.hash(state);
            }
            Item::Module(ptr) => {
                state.write_u8(19);
                ptr.hash(state);
            }
            Item::Package(ptr) => {
                state.write_u8(20);
                ptr.hash(state);
            }
        }
    }
}

impl PartialEq for Item {
    fn eq(&self, other: &Item) -> bool {
        match (self, other) {
            (&Item::UTF8(ref str1), &Item::UTF8(ref str2)) => *str1 == *str2,
            (&Item::Integer(i1), &Item::Integer(i2)) => i1 == i2,
            (&Item::Float(f1), &Item::Float(f2)) => f1.to_bits() == f2.to_bits(),
            (&Item::Long(i1), &Item::Long(i2)) => i1 == i2,
            (&Item::Double(f1), &Item::Double(f2)) => f1.to_bits() == f2.to_bits(),
            (&Item::Class(i1), &Item::Class(i2)) | (&Item::String(i1), &Item::String(i2)) => {
                i1 == i2
            }
            (
                &Item::FieldRef {
                    class: class1,
                    name_and_type: nat1,
                },
                &Item::FieldRef {
                    class: class2,
                    name_and_type: nat2,
                },
            )
            | (
                &Item::MethodRef {
                    class: class1,
                    name_and_type: nat1,
                },
                &Item::MethodRef {
                    class: class2,
                    name_and_type: nat2,
                },
            )
            | (
                &Item::InterfaceMethodRef {
                    class: class1,
                    name_and_type: nat1,
                },
                &Item::InterfaceMethodRef {
                    class: class2,
                    name_and_type: nat2,
                },
            ) => class1 == class2 && nat1 == nat2,
            (
                &Item::NameAndType {
                    name: name1,
                    desc: desc1,
                },
                &Item::NameAndType {
                    name: name2,
                    desc: desc2,
                },
            ) => name1 == name2 && desc1 == desc2,
            (
                &Item::MethodHandle {
                    kind: ref kind1,
                    index: index1,
                },
                &Item::MethodHandle {
                    kind: ref kind2,
                    index: index2,
                },
            ) => kind1 == kind2 && index1 == index2,
            (
                &Item::InvokeDynamic {
                    bootstrap_method_attribute: bma1,
                    name_and_type: nat1,
                },
                &Item::InvokeDynamic {
                    bootstrap_method_attribute: bma2,
                    name_and_type: nat2,
                },
            ) => bma1 == bma2 && nat1 == nat2,
            (&Item::Package(index1), &Item::Package(index2))
            | (&Item::Module(index1), &Item::Module(index2))
            | (&Item::MethodType(index1), &Item::MethodType(index2)) => index1 == index2,

            _ => false,
        }
    }
}

impl Eq for Item {}

#[derive(Eq, PartialEq, Hash, Debug, Clone)]
pub enum ReferenceKind {
    GetField,
    GetStatic,
    PutField,
    PutStatic,
    InvokeVirtual,
    InvokeStatic,
    InvokeSpecial,
    NewInvokeSpecial,
    InvokeInterface,
}

/// The constant pool found in every java class file.
/// It is used to have fast lookup for entries and small files.
/// Removing or modifying items is not allowed
/// to respect already 'used' indices
/// or to prevent rehashing of the underlying `HashMap`.
#[derive(Default)]
pub struct Pool {
    /// The count of all items
    len: u16,

    /// The constant pool items by index.
    /// A Option is used, since long and double values take two spaces
    /// and we still want to access items by index with O(1), not O(n).
    by_index: Vec<Option<*const Item>>,

    /// The constant pool items by reference to acquire their index.
    by_entry: HashMap<Item, u16>,
}

impl Pool {
    pub fn new() -> Pool {
        Pool {
            len: 0,
            by_index: Vec::new(),
            by_entry: HashMap::new(),
        }
    }

    pub fn with_capacity(size: u16) -> Pool {
        Pool {
            len: 0,
            by_index: Vec::with_capacity(size as usize),
            by_entry: HashMap::with_capacity(size as usize),
        }
    }

    /// Returns the length of the pool.
    pub fn len(&self) -> u16 {
        self.len + 1
    }

    /// Returns true if the pool is empty, false otherwise.
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Returns a Vector containing pointers to Items.
    /// The *Nones* inside the items Vec are filtered.
    pub fn get_items(&self) -> Vec<&Item> {
        let mut items = Vec::with_capacity(self.len as usize);

        for opt_item in &self.by_index {
            if let Some(ref item) = *opt_item {
                unsafe {
                    items.push(&**item);
                }
            }
        }

        items
    }

    /// Returns the item at a specified index.
    /// If the index is 0 or greater than the size of the pool, an error is returned.
    pub fn get(&self, index: u16) -> Result<&Item> {
        let item = self.by_index
            .get(index as usize - 1)
            .ok_or_else(|| Error::InvalidCPItem(index))?;

        if let Some(item) = *item {
            Ok(unsafe { &*item })
        } else {
            Err(Error::InvalidCPItem(index))
        }
    }

    /// Returns a cloned String at a specified index.
    pub fn get_utf8(&self, index: u16) -> Result<String> {
        if let Item::UTF8(ref s) = *self.get(index)? {
            Ok(s.clone())
        } else {
            Err(Error::InvalidCPItem(index))
        }
    }

    /// Returns a class name at a specified index,
    /// but if the utf index is 0, None is returned.
    pub fn get_class_name_opt(&self, index: u16) -> Result<Option<String>> {
        if let Item::Class(utf_index) = *self.get(index)? {
            if utf_index == 0 {
                Ok(None)
            } else {
                Ok(Some(self.get_utf8(utf_index)?))
            }
        } else {
            Err(Error::InvalidCPItem(index))
        }
    }

    /// Returns a class name at a specified index.
    pub fn get_class_name(&self, index: u16) -> Result<String> {
        if let Item::Class(utf_index) = *self.get(index)? {
            self.get_utf8(utf_index)
        } else {
            Err(Error::InvalidCPItem(index))
        }
    }

    /// Pushes an item on the pool.
    pub fn push(&mut self, item: Item) -> Result<u16> {
        if self.len == u16::max_value() {
            return Err(Error::CPTooLarge);
        }

        if let Some(index) = self.by_entry.get(&item) {
            return Ok(*index + 1);
        }

        let double = item.is_double();
        self.by_index.push(Some(&item as *const Item));
        self.by_entry.insert(item, self.len);
        self.len += 1;

        if double {
            // long and double take an additional space
            self.by_index.push(None);
            self.len += 1;

            Ok(self.len - 1)
        } else {
            Ok(self.len)
        }
    }
}

impl Clone for Pool {
    fn clone(&self) -> Pool {
        let mut by_index = Vec::with_capacity(self.len as usize);
        let mut by_entry = HashMap::with_capacity(self.len as usize);

        for (index, item) in self.by_index.iter().enumerate() {
            // Clones the item if it is Some and pushes a pointer to it on the Vec and HashMap.
            if let Some(ref item) = *item {
                let cloned_item = unsafe { &**item }.clone();
                by_index.push(Some(&cloned_item as *const Item));
                by_entry.insert(cloned_item, index as u16);
            } else {
                by_index.push(None)
            }
        }

        Pool {
            len: self.len,
            by_index,
            by_entry,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constpool() {
        let mut pool = Pool::new();
        assert_eq!(pool.push(Item::Integer(123)).unwrap(), 1);
        assert_eq!(pool.push(Item::Long(32767)).unwrap(), 2);
        assert_eq!(pool.push(Item::Float(3.8)).unwrap(), 4);
        assert_eq!(pool.push(Item::Integer(123)).unwrap(), 1);

        assert_eq!(pool.get(1).unwrap(), &Item::Integer(123));
        assert_eq!(pool.get(2).unwrap(), &Item::Long(32767));
        assert_eq!(pool.get(4).unwrap(), &Item::Float(3.8));
    }
}
