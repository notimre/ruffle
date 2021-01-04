use gc_arena::{MutationContext, GcCell};
use crate::avm2::class::{Class, ClassAttributes};
use crate::avm2::{QName, Namespace, Activation, Error, Value};
use crate::avm2::method::Method;
use crate::avm2::object::{Object, TObject};
use crate::avm2::traits::Trait;

/// Implements `flash.system.ApplicationDomain`'s instance constructor.
pub fn instance_init<'gc>(
    activation: &mut Activation<'_, 'gc, '_>,
    this: Option<Object<'gc>>,
    _args: &[Value<'gc>],
) -> Result<Value<'gc>, Error> {
    if let Some(this) = this {
        activation.super_init(this, &[])?;
    }

    Ok(Value::Undefined)
}

/// Implements `flash.system.ApplicationDomain`'s class constructor.
pub fn class_init<'gc>(
    _activation: &mut Activation<'_, 'gc, '_>,
    _this: Option<Object<'gc>>,
    _args: &[Value<'gc>],
) -> Result<Value<'gc>, Error> {
    Ok(Value::Undefined)
}

pub fn create_class<'gc>(mc: MutationContext<'gc, '_>) -> GcCell<'gc, Class<'gc>> {
    let class = Class::new(
        QName::new(Namespace::package("flash.ui"), "Keyboard"),
        Some(QName::new(Namespace::public_namespace(), "Object").into()),
        Method::from_builtin(instance_init),
        Method::from_builtin(class_init),
        mc,
    );
    let mut write = class.write(mc);
    write.define_class_trait(Trait::from_const(
        QName::new(Namespace::public_namespace(), "A"),
        QName::new(Namespace::public_namespace(), "Number").into(),
        Some(65.into()),
    ));
    class
}