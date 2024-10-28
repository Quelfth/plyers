use super::KeyMap;
use super::PlyGetChar;
use super::PlyGetDouble;
use super::PlyGetFloat;
use super::PlyGetInt;
use super::PlyGetListChar;
use super::PlyGetListDouble;
use super::PlyGetListFloat;
use super::PlyGetListInt;
use super::PlyGetListShort;
use super::PlyGetListUChar;
use super::PlyGetListUInt;
use super::PlyGetListUShort;
use super::PlyGetShort;
use super::PlyGetUChar;
use super::PlyGetUInt;
use super::PlyGetUShort;
use super::PlyElementSet;
use super::Property;
use super::PlyElement;

/// Ready to use data-structure for all kind of element definitions.
///
/// PLY files carry the payload format in their head section.
/// Hence, they can contain all kind of elements, or formulated differently,
/// they define types very dinamically.
/// To achieve this flexibility in rust, this alias to a HashMap is provided.
///
/// If you need a more compact representation or faster access,
/// you might want to define your own structures and implement the `PropertyAccess` trait.
pub type DefaultElement = KeyMap<Property>;
macro_rules! get(
    ($e:expr) => (match $e {None => return None, Some(x) => x})
);

impl PlyElement for DefaultElement {
    fn new() -> Self {
        Self::new()
    }
}

impl PlyElementSet for DefaultElement {
    fn set_property(&mut self, key: String, property: Property) {
        self.insert(key, property);
    }
}
impl PlyGetChar for DefaultElement {
    fn get_char(&self, key: &str) -> Option<i8> {
        match *get!(self.get(key)) {
            Property::Char(x) => Some(x),
            _ => None,
        }
    }
}
impl PlyGetUChar for DefaultElement {
    fn get_uchar(&self, key: &str) -> Option<u8> {
        match *get!(self.get(key)) {
            Property::UChar(x) => Some(x),
            _ => None,
        }
    }
}
impl PlyGetShort for DefaultElement {
    fn get_short(&self, key: &str) -> Option<i16> {
        match *get!(self.get(key)) {
            Property::Short(x) => Some(x),
            _ => None,
        }
    }
}
impl PlyGetUShort for DefaultElement {
    fn get_ushort(&self, key: &str) -> Option<u16> {
        match *get!(self.get(key)) {
            Property::UShort(x) => Some(x),
            _ => None,
        }
    }
}
impl PlyGetInt for DefaultElement {
    fn get_int(&self, key: &str) -> Option<i32> {
        match *get!(self.get(key)) {
            Property::Int(x) => Some(x),
            _ => None,
        }
    }
}
impl PlyGetUInt for DefaultElement {
    fn get_uint(&self, key: &str) -> Option<u32> {
        match *get!(self.get(key)) {
            Property::UInt(x) => Some(x),
            _ => None,
        }
    }
}
impl PlyGetFloat for DefaultElement {
    fn get_float(&self, key: &str) -> Option<f32> {
        match *get!(self.get(key)) {
            Property::Float(x) => Some(x),
            _ => None,
        }
    }
}
impl PlyGetDouble for DefaultElement {
    fn get_double(&self, key: &str) -> Option<f64> {
        match *get!(self.get(key)) {
            Property::Double(x) => Some(x),
            _ => None,
        }
    }
}
impl PlyGetListChar for DefaultElement {
    fn get_list_char(&self, key: &str) -> Option<&[i8]> {
        match *get!(self.get(key)) {
            Property::ListChar(ref x) => Some(x),
            _ => None,
        }
    }
}
impl PlyGetListUChar for DefaultElement {
    fn get_list_uchar(&self, key: &str) -> Option<&[u8]> {
        match *get!(self.get(key)) {
            Property::ListUChar(ref x) => Some(x),
            _ => None,
        }
    }
}
impl PlyGetListShort for DefaultElement {
    fn get_list_short(&self, key: &str) -> Option<&[i16]> {
        match *get!(self.get(key)) {
            Property::ListShort(ref x) => Some(x),
            _ => None,
        }
    }
}
impl PlyGetListUShort for DefaultElement {
    fn get_list_ushort(&self, key: &str) -> Option<&[u16]> {
        match *get!(self.get(key)) {
            Property::ListUShort(ref x) => Some(x),
            _ => None,
        }
    }
}
impl PlyGetListInt for DefaultElement {
    fn get_list_int(&self, key: &str) -> Option<&[i32]> {
        match *get!(self.get(key)) {
            Property::ListInt(ref x) => Some(x),
            _ => None,
        }
    }
}
impl PlyGetListUInt for DefaultElement {
    fn get_list_uint(&self, key: &str) -> Option<&[u32]> {
        match *get!(self.get(key)) {
            Property::ListUInt(ref x) => Some(x),
            _ => None,
        }
    }
}
impl PlyGetListFloat for DefaultElement {
    fn get_list_float(&self, key: &str) -> Option<&[f32]> {
        match *get!(self.get(key)) {
            Property::ListFloat(ref x) => Some(x),
            _ => None,
        }
    }
}
impl PlyGetListDouble for DefaultElement {
    fn get_list_double(&self, key: &str) -> Option<&[f64]> {
        match *get!(self.get(key)) {
            Property::ListDouble(ref x) => Some(x),
            _ => None,
        }
    }
}
