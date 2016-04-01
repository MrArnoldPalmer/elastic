//! Implementation helper macros.

#[macro_export]
macro_rules! impl_mapping {
	($($t:ty),*) => (
		$(
			impl $crate::mapping::ElasticType<$crate::mapping::NullMapping, ()> for $t { }
		)*
	)
}

#[macro_export]
macro_rules! impl_string_mapping {
	($t:ty) => (
		impl $crate::mapping::ElasticTypeMapping<()> for $t {
			type Visitor = $crate::string::mapping::ElasticStringMappingVisitor<$t>;

			fn data_type() -> &'static str {
				"string"
			}
		}

		impl serde::Serialize for $t {
			fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
			where S: serde::Serializer {
				serializer.serialize_struct("mapping", Self::get_visitor())
			}
		}
	)
}

#[macro_export]
macro_rules! impl_date_mapping {
	($t:ty, $f:ty) => (
		impl $crate::mapping::ElasticTypeMapping<$f> for $t {
			type Visitor = $crate::date::mapping::ElasticDateMappingVisitor<$f, $t>;

			fn data_type() -> &'static str {
				"date"
			}
		}
		
		impl serde::Serialize for $t {
			fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
			where S: serde::Serializer {
				serializer.serialize_struct("mapping", Self::get_visitor())
			}
		}
	);
	($t:ty) => (
		impl <T: $crate::date::DateFormat> $crate::mapping::ElasticTypeMapping<T> for $t {
			type Visitor = $crate::date::mapping::ElasticDateMappingVisitor<T, $t>;

			fn data_type() -> &'static str {
				"date"
			}
		}

		impl <T: $crate::date::DateFormat> serde::Serialize for $t {
			fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
			where S: serde::Serializer {
				serializer.serialize_struct("mapping", Self::get_visitor())
			}
		}
	)
}

#[macro_export]
macro_rules! impl_date_fmt {
	($t:ty, $f:tt, $n:expr) => (
		impl $crate::date::DateFormat for $t {
			fn fmt<'a>() -> Vec<chrono::format::Item<'a>> {
				date_fmt!($f)
					.iter()
					.cloned()
					.collect()
			}

			fn name() -> &'static str { $n }
		}
	)
}

#[macro_export]
macro_rules! impl_object_mapping {
	($t:ident, $m:ident, $es_ty:expr, $mod_name:ident, [$({$key:expr, $field:ident}),*]) => (
		mod $mod_name {
			use std::marker::PhantomData;
			use serde;
			use $crate::mapping::prelude::*;
			use super::{ $t, $m };

			struct ObjectPropertiesVisitor<'a> {
				data: &'a $t
			}

			impl <'a> ElasticObjectTypeVisitor<'a, $t> for ObjectPropertiesVisitor<'a> {
				fn new(data: &'a $t) -> Self {
					ObjectPropertiesVisitor {
						data: data
					}
				}
			}

			impl <'a> serde::ser::MapVisitor for ObjectPropertiesVisitor<'a> {
				fn visit<S>(&mut self, serializer: &mut S) -> Result<Option<()>, S::Error>
				where S: serde::Serializer {
					$(
						try!(FieldMapper::map($key, &self.data.$field, serializer));
					)*

					Ok(None)
				}
			}

			struct TypeMappingVisitor<'a> { 
				data: &'a $t
			}

			impl <'a> ElasticObjectTypeVisitor<'a, $t> for TypeMappingVisitor<'a> {
				fn new(data: &'a $t) -> Self {
					TypeMappingVisitor {
						data: data
					}
				}
			}

			impl ElasticTypeMapping<()> for $m {
				type Visitor = FieldMappingVisitor;

				fn data_type() -> &'static str {
					<Self as ElasticObjectMapping>::data_type()
				}
			}

			impl serde::Serialize for $m {
				fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
				where S: serde::Serializer {
					serializer.serialize_struct("", FieldMappingVisitor)
				}
			}

			#[derive(Default)]
			struct FieldMappingVisitor;

			impl serde::ser::MapVisitor for FieldMappingVisitor {
				fn visit<S>(&mut self, serializer: &mut S) -> Result<Option<()>, S::Error>
				where S: serde::Serializer {
					let mut object_mapper = ElasticObjectMappingVisitor::<$m>::default();
					try!(object_mapper.visit(serializer));

					let data = $t::default();
					try!(serializer.serialize_struct_elt("properties", ElasticObjectProperties::<$t, ObjectPropertiesVisitor>::new(&data)));

					Ok(None)
				}
			}

			#[derive(Default, Clone)]
			struct TypeMapping<'a> {
				phantom: PhantomData<&'a ()>
			}

			impl <'a> ElasticUserTypeMapping<'a, $t> for TypeMapping<'a> {
				type Visitor = TypeMappingVisitor<'a>;
				type PropertiesVisitor = ObjectPropertiesVisitor<'a>;

				fn name() -> &'static str {
					$es_ty
				}
			}

			impl <'a> ElasticTypeMapping<()> for TypeMapping<'a> {
				type Visitor = FieldMappingVisitor;
			}

			impl <'a> serde::Serialize for TypeMapping<'a> {
				fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
				where S: serde::Serializer {
					serializer.serialize_struct(Self::name(), FieldMappingVisitor::default())
				}
			}

			impl <'a> serde::ser::MapVisitor for TypeMappingVisitor<'a> {
				fn visit<S>(&mut self, serializer: &mut S) -> Result<Option<()>, S::Error>
				where S: serde::Serializer {
					try!(serializer.serialize_struct_elt("properties", ElasticTypeProperties::<'a, $t, TypeMapping<'a>>::new(&self.data)));

					Ok(None)
				}
			}

			impl <'a> ElasticType<TypeMapping<'a>, ()> for $t { }
		}
	);
	($t:ident, $m:ident, $es_ty:expr, $mod_name:ident, [$($field:ident),*]) => (impl_object_mapping!($t, $m, $es_ty, $mod_name, [$({stringify!($field), $field}),*]);)
}