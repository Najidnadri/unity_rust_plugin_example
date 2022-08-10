
# Native Unity Plugin in Rust Example

A quick and simple steps to create native unity plugins with rust lang.


## Steps

1. Create rust lib

```bash
  cargo new my_rust_plugin --lib
```
2. Add [lib] section in cargo.toml file
```bash
  [lib]
  name = "my_rust_plugin_1" //your plugin file name
  crate-type = ["dylib"]  //compile to dll file
```
3. Create logic
```bash
  use rand::{thread_rng, Rng};

  //be sure to include #[no_mangle] and extern 
  //to each of the function that you to be called from outside
  #[no_mangle]
  pub extern fn random_num() -> i32 {
    thread_rng().gen()
  }
```
4. Build with `cargo build --release` and find `your_plugin_file_name.dll` in `target/release` folder
5. Copy the .dll file and paste it in your `/asset` folder
6. Create a C# script and just run it in the unity. 
```bash
using UnityEngine;
using System.Collections;
using System.Runtime.InteropServices;

public class myRustPlugin : MonoBehaviour {

    [DllImport("your_plugin_file_name")]
    private static extern int random_num();


    void Start() {
        Debug.Log(random_num());
    }
}
```
[create C# script](https://docs.unity3d.com/Manual/CreatingAndUsingScripts.html#:~:text=You%20can%20create%20a%20new,selected%20in%20the%20Project%20panel.)
| [create plugin](https://docs.unity3d.com/Manual/NativePlugins.html)
