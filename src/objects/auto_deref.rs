use super::protocol::Object;
use super::protocol::ObjectProtocol;
use super::types::StringObject;
use crate::Result;

pub fn default_deref_object<O>(obj: &O, name: &str) -> Result<Option<Object>>
where
    O: 'static,
    O: ObjectProtocol,
{
    if StringObject::static_list_attrs().contains(&name) {
        // Deref to String.
        let obj = StringObject::from(obj.to_plain_string()?).to_object();
        Ok(Some(obj))
    } else {
        Ok(None)
    }
}
