use std::hash::{Hash, Hasher};
use std::collections::HashMap;
use std::cmp::{Eq, PartialEq};
use std::rc::Rc;
use std::slice::Iter;

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
        bootstrap_method: u16,
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
                bootstrap_method,
                name_and_type,
            } => {
                state.write_u8(18);
                bootstrap_method.hash(state);
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
                    bootstrap_method: bma1,
                    name_and_type: nat1,
                },
                &Item::InvokeDynamic {
                    bootstrap_method: bma2,
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
///
/// A `Vec` and a `HashMap` is used internally to have fast lookups by index and value.
/// To connect both, `Rc`s are used.
/// As a result, we will have a little overhead, but this should be negligible.
#[derive(Default)]
pub struct Pool {
    length: u16,
    by_index: Vec<Option<Rc<Item>>>,
    by_entry: HashMap<Rc<Item>, u16>,
}

impl Pool {
    pub fn new() -> Self {
        Pool {
            length: 1,
            by_index: Vec::new(),
            by_entry: HashMap::new(),
        }
    }

    pub fn with_capacity(cap: u16) -> Self {
        Pool {
            length: 1,
            by_index: Vec::with_capacity(cap as usize),
            by_entry: HashMap::with_capacity(cap as usize),
        }
    }

    /// Returns the *encoded* length of the pool.
    #[inline]
    pub fn len(&self) -> u16 {
        self.length
    }

    /// Returns true if the pool is empty, false otherwise.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 1
    }

    /// Returns the item at a specified index.
    /// If the index is 0 or greater than the size of the pool, an error is returned.
    pub fn get(&self, index: u16) -> Result<&Item> {
        // bounds checking
        if index != 0 && index <= self.len() {
            if let Some(ref item) = self.by_index[index as usize - 1] {
                return Ok(item);
            }
        }

        Err(Error::InvalidCPItem(index))
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
        if self.len() == u16::max_value() {
            return Err(Error::CPTooLarge);
        }

        let double = item.is_double();
        let length = &mut self.length;

        let rc_item = Rc::new(item);
        let rc_item1 = Rc::clone(&rc_item);

        let by_index = &mut self.by_index;

        // check if in pool, if not insert it
        Ok(*self.by_entry.entry(rc_item).or_insert_with(move || {
            by_index.push(Some(Rc::clone(&rc_item1)));

            let prev_length = *length;
            if double {
                // long and double take an additional space
                by_index.push(None);
                *length += 2;
            } else {
                *length += 1;
            }
            prev_length
        }))
    }

    /// Pushes an item, which might be a duplicate.
    /// This removes the possibility of reading a class,
    /// which has multiple constant pool entries, which are the same
    /// and then accessing the wrong entry.
    pub fn push_duplicate(&mut self, item: Item) -> Result<u16> {
        if self.len() == u16::max_value() {
            return Err(Error::CPTooLarge);
        }

        let double = item.is_double();
        let length = self.length;
        let rc_item = Rc::new(item);

        self.by_index.push(Some(Rc::clone(&rc_item)));
        if double {
            self.by_index.push(None);
            self.length += 2;
        } else {
            self.length += 1;
        }

        self.by_entry.insert(rc_item, length);
        Ok(length)
    }

    /// Pushes a new UTF-8 item on the pool and returns an index to it.
    pub fn push_utf8(&mut self, content: String) -> Result<u16> {
        self.push(Item::UTF8(content))
    }

    /// Pushes a new class item on the pool and returns an index to it.
    pub fn push_class(&mut self, name: String) -> Result<u16> {
        let name_index = self.push_utf8(name)?;
        self.push(Item::Class(name_index))
    }

    pub fn iter(&self) -> PoolIter {
        PoolIter {
            iter: self.by_index.iter(),
            index: 0,
        }
    }
}

// implement later again (maybe)
//impl Clone for Pool {
//    fn clone(&self) -> Pool {
//        Pool {
//            content: self.content.clone(),
//        }
//    }
//}

/// Iterates over all the elements in the constant pool
/// It basically is a filter with a different name
pub struct PoolIter<'a> {
    iter: Iter<'a, Option<Rc<Item>>>,
    index: u16,
}

impl<'a> Iterator for PoolIter<'a> {
    type Item = (u16, &'a Item);

    fn next(&mut self) -> Option<Self::Item> {
        self.index += 1;
        if let Some(rc_item) = self.iter.next() {
            if let Some(ref item) = *rc_item {
                Some((self.index, item))
            } else {
                self.next()
            }
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn push_and_get() {
        let mut pool = Pool::new();
        assert_eq!(pool.push(Item::Integer(123)).unwrap(), 1);
        assert_eq!(pool.push(Item::Long(32767)).unwrap(), 2);
        assert_eq!(pool.push(Item::Long(65535)).unwrap(), 4);
        assert_eq!(pool.push(Item::Float(3.8)).unwrap(), 6);
        assert_eq!(pool.len(), 7);
        assert_eq!(pool.push(Item::Integer(123)).unwrap(), 1);
        assert_eq!(pool.len(), 7);

        assert_eq!(pool.get(1).unwrap(), &Item::Integer(123));
        assert_eq!(pool.get(2).unwrap(), &Item::Long(32767));
        assert_eq!(pool.get(4).unwrap(), &Item::Long(65535));
        assert_eq!(pool.get(6).unwrap(), &Item::Float(3.8));

        let mut iter = pool.iter();
        assert_eq!(iter.next(), Some((1, &Item::Integer(123))));
        assert_eq!(iter.next(), Some((2, &Item::Long(32767))));
        assert_eq!(iter.next(), Some((4, &Item::Long(65535))));
        assert_eq!(iter.next(), Some((6, &Item::Float(3.8))));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn push_duplicate() {
        let mut pool = Pool::new();
        assert_eq!(pool.push_duplicate(Item::Integer(123)).unwrap(), 1);
        assert_eq!(pool.push_duplicate(Item::Long(32767)).unwrap(), 2);
        assert_eq!(pool.push_duplicate(Item::Long(65535)).unwrap(), 4);
        assert_eq!(pool.push_duplicate(Item::Float(3.8)).unwrap(), 6);
        assert_eq!(pool.len(), 7);
        assert_eq!(pool.push_duplicate(Item::Integer(123)).unwrap(), 7);
        assert_eq!(pool.len(), 8);

        assert_eq!(pool.get(1).unwrap(), &Item::Integer(123));
        assert_eq!(pool.get(2).unwrap(), &Item::Long(32767));
        assert_eq!(pool.get(4).unwrap(), &Item::Long(65535));
        assert_eq!(pool.get(6).unwrap(), &Item::Float(3.8));
        assert_eq!(pool.get(7).unwrap(), &Item::Integer(123));
    }
}
