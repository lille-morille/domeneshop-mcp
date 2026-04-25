use std::{env, error::Error, fs, path::Path};

use serde_json::Value;

fn main() -> Result<(), Box<dyn Error>> {
    let src = "openapi.json";
    println!("cargo:rerun-if-changed={src}");

    let mut raw: Value = serde_json::from_reader(fs::File::open(src)?)?;
    inject_operation_ids(&mut raw);
    inline_subproperty_refs(&mut raw);
    let spec = serde_json::from_value(raw)?;

    let mut generator = progenitor::Generator::default();
    let tokens = generator.generate_tokens(&spec)?;
    let ast = syn::parse2(tokens)?;
    let content = prettyplease::unparse(&ast);

    let out = Path::new(&env::var("OUT_DIR")?).join("codegen.rs");
    fs::write(out, content)?;
    Ok(())
}

fn inject_operation_ids(doc: &mut Value) {
    let Some(paths) = doc.get_mut("paths").and_then(Value::as_object_mut) else {
        return;
    };
    for (path, item) in paths.iter_mut() {
        let Some(ops) = item.as_object_mut() else {
            continue;
        };
        for (method, op) in ops.iter_mut() {
            if !matches!(
                method.as_str(),
                "get" | "put" | "post" | "delete" | "patch" | "head" | "options" | "trace"
            ) {
                continue;
            }
            let Some(op_obj) = op.as_object_mut() else {
                continue;
            };
            if op_obj.get("operationId").is_some() {
                continue;
            }
            let id = synth_operation_id(method, path);
            op_obj.insert("operationId".into(), Value::String(id));
        }
    }
}

/// typify rejects `$ref`s that point into a schema sub-property
/// (e.g. `#/components/schemas/Invoice/properties/status`). Resolve these
/// in-place by replacing the `$ref` with the target schema object.
fn inline_subproperty_refs(doc: &mut Value) {
    let root = doc.clone();
    visit_subproperty_refs(doc, &root);
}

fn visit_subproperty_refs(node: &mut Value, root: &Value) {
    match node {
        Value::Object(map) => {
            if let Some(Value::String(r)) = map.get("$ref")
                && let Some(stripped) = r.strip_prefix("#/")
            {
                let has_subproperty = stripped
                    .split('/')
                    .skip(3)
                    .any(|seg| seg == "properties" || seg == "items");
                if has_subproperty
                    && let Some(resolved) = resolve_pointer(root, stripped)
                {
                    *node = resolved.clone();
                    return;
                }
            }
            for v in map.values_mut() {
                visit_subproperty_refs(v, root);
            }
        }
        Value::Array(arr) => {
            for v in arr {
                visit_subproperty_refs(v, root);
            }
        }
        _ => {}
    }
}

fn resolve_pointer<'a>(root: &'a Value, pointer: &str) -> Option<&'a Value> {
    let mut cur = root;
    for raw in pointer.split('/') {
        let key = raw.replace("~1", "/").replace("~0", "~");
        cur = match cur {
            Value::Object(m) => m.get(&key)?,
            Value::Array(a) => a.get(key.parse::<usize>().ok()?)?,
            _ => return None,
        };
    }
    Some(cur)
}

fn synth_operation_id(method: &str, path: &str) -> String {
    let segments: Vec<String> = path
        .split('/')
        .filter(|s| !s.is_empty())
        .map(|s| s.trim_start_matches('{').trim_end_matches('}').to_owned())
        .collect();
    format!("{}_{}", method, segments.join("_"))
}
