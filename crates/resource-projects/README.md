# Editors

This library contains `Editor`, an API for editing projects through an opaque interface.

Instead of this...

```rust
fn main() {
    let mut some_type = MyType::new();
    some_type.name = "New Name";
}
```

...you do this.

```rust
fn main() {
    let some_type = MyType::new(); 
    let mut editor = Editor::new_local(some_type);
    let mut client = editor.new_client();
    
    let resource_header = client.get_resource_headers(None, None).next();
    
    let action = client.set_field(&resource_header.rid, FieldIndex(0), "New Name");
    editor.commit_action(action).unwrap();
}
```

This allows you to abstract over the storage of `MyType` and interact with it in the same way no matter what.