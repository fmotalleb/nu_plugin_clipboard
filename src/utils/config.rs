use nu_plugin::EngineInterface;
use nu_protocol::Value;

pub fn read(engine: &EngineInterface, key: impl AsRef<str>) -> Option<Value> {
    if let Ok(Some(Value::Record { val, .. })) = engine.get_plugin_config() {
        return val.get(key.as_ref()).cloned();
    }
    return None;
}

pub fn silent_copy(engine: &EngineInterface) -> bool {
    if let Some(Value::Bool { val, .. }) = read(engine, "SILENT_COPY")
        && val
    {
        return true;
    }
    return false;
}
