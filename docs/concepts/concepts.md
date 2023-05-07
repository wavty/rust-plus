# 基础 100 问

## 1. Rust 中变量、常量和静态变量的定义

下表展示了 Rust 中变量、常量和静态变量的定义、说明以及举例使用：

| 类型     | 定义                         | 说明                                                 | 举例                         |
| -------- | ---------------------------- | ---------------------------------------------------- | ---------------------------- |
| 变量     | `let x = 1;`                 | 声明并初始化一个变量 `x`，类型会自动推导             | `let x = 1;`                 |
| 常量     | `const MAX: i32 = 100;`      | 定义一个值不可变的常量 `MAX`，必须显式指定类型       | `const MAX: i32 = 100;`      |
| 静态变量 | `static mut COUNT: i32 = 0;` | 定义一个全局的可变静态变量 `COUNT`，必须显式指定类型 | `static mut COUNT: i32 = 0;` |

需要注意的是，在 Rust 中，变量和常量都是默认不可变的，如果需要可变的变量，可以使用 `mut` 关键字声明。而静态变量则需要使用 `static` 关键字声明，并且在声明时必须显式指定类型。

另外，由于静态变量是全局变量，因此在多线程程序中访问时需要考虑线程安全问题。如果需要在多个线程中访问静态变量，可以使用 `Arc`（原子引用计数）类型来实现。

## 2. rust 中有 void 类型吗？

Rust 中没有 `void` 类型。相反，Rust 使用 `()` 表示无返回值函数的返回类型，表示返回一个空元组。这种设计使得 Rust 中的函数类型更加清晰和精确，也更符合 Rust 的表达式风格。另外，Rust 中也没有空指针或空引用，取而代之的是 `Option` 枚举类型。

## 3. 简单讲述一下 rust 里面的 unit 类型

在 Rust 中，`()` 被称为 unit 类型，它只有一个值，也就是空元组 `()`。和其他语言中的 `void` 类型不同，unit 类型不是一个空指针或者空引用，而是一个真正的类型，它表示不包含有用信息的空值。

在 Rust 中，函数可以选择返回值，也可以选择不返回值。如果函数不需要返回值，那么就可以显式声明返回值类型为 `()`，表示返回空元组。例如，下面的代码定义了一个没有返回值的函数 `hello`：

```rust
fn hello() -> () {
    println!("Hello, world!");
}
```

另一方面，如果函数返回的是空指针或空引用，那么在 Rust 中可以使用 `Option` 类型来表示。`Option` 类型是一个枚举类型，它可以是 `Some(T)`（包含一个 `T` 类型的值）或 `None`（表示空值）。

总的来说，Rust 选择使用 `()` 作为空值类型，而不是像其他语言一样使用 `void`，这是因为 `()` 是一个真正的类型，有更多的可操作性和灵活性，而且更符合 Rust 的安全和表达式风格。

## 4. 举个使用 Option 类型的例子

```rust
fn fig(is_number: bool) -> Option<i32> {
    if is_number {
        Some(10)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fig() {
        assert_eq!(fig(true), Some(10));
        assert_eq!(fig(false), None);
    }
}
```

这个代码示例定义了一个函数 `fig`，接受一个 `bool` 类型的参数 `is_number`，当 `is_number` 为 `true` 时，返回一个具体的 `Option<i32>` 类型的值，否则返回 `None`。

同时，代码示例中还定义了一个测试模块，包含了一个测试函数 `test_fig`，测试了 `fig` 函数的正确性。在测试函数中使用了 `assert_eq!` 宏来断言函数的输出是否符合预期。

## 5. 空结构体、元组结构体、普通结构体的定义与区别

在 Rust 中，有三种常见的结构体定义：空结构体、元组结构体和普通结构体。

### 空结构体

空结构体是指没有任何成员的结构体，定义语法如下：

```rust
struct EmptyStruct;
```

空结构体通常用于作为占位符，或者是实现某些 trait 的标记。例如，下面的例子定义了一个标记 trait `MyTrait`，实现了该 trait 的类型不需要具备任何状态：

```rust
trait MyTrait {}

struct MyType;

impl MyTrait for MyType {}

struct EmptyStruct;

impl MyTrait for EmptyStruct {}
```

### 元组结构体

元组结构体是一种特殊的结构体，它没有具名的成员，而是使用元组来表示其字段。元组结构体的定义语法如下：

```rust
struct TupleStruct(i32, f64);
```

元组结构体常常用于表示一组相关的值。例如，下面的例子定义了一个元组结构体 `Point`，表示二维平面上的一个点：

```rust
struct Point(f64, f64);

let p = Point(1.0, 2.0);
```

### 普通结构体

普通结构体是一种最常用的结构体，它由一个或多个具名的字段组成。定义语法如下：

```rust
struct MyStruct {
    field1: u32,
    field2: String,
    // ...
}
```

普通结构体通常用于表示一组具有不同类型的数据。例如，下面的例子定义了一个结构体 `Person`，表示一个人的基本信息：

```rust
struct Person {
    name: String,
    age: u32,
    gender: Gender,
}

enum Gender {
    Male,
    Female,
}

let person = Person {
    name: "Alice".to_string(),
    age: 30,
    gender: Gender::Female,
};
```

在上面的例子中，`Person`结构体由三个字段组成，分别是`name`、`age`和`gender`。每个字段都有自己的类型，而结构体类型则由这些字段的类型组成。

总的来说，不同类型的结构体适用于不同的场景。空结构体通常用于作为占位符，元组结构体适用于表示一组相关的值，而普通结构体适用于表示具有不同类型的数据。

## 6. 简单讲述一下 rust 中模块之间的嵌套关系

在 Rust 中，一个模块可以嵌套在另一个模块中，从而形成模块之间的嵌套关系。这种嵌套关系可以通过使用 `mod` 关键字来创建。

在 Rust 中，一个模块可以包含多个子模块，而子模块可以进一步包含自己的子模块。这些子模块可以在父模块的作用域中使用，从而形成模块之间的嵌套关系。

举个例子，假设我们有一个名为 `foo` 的模块，其中包含了两个子模块 `bar` 和 `baz`。同时，`bar` 模块还包含了一个名为 `qux` 的子模块。那么可以通过以下方式来创建这些模块：

```rust
// src/foo.rs
mod bar {
    mod qux {
        fn do_something() {
            // ...
        }
    }

    fn do_something_else() {
        // ...
    }
}

mod baz {
    fn do_another_thing() {
        // ...
    }
}
```

在上面的代码中，`foo` 模块包含了两个子模块 `bar` 和 `baz`。`bar` 模块又包含了一个名为 `qux` 的子模块。这些模块中都定义了一些函数。

要在 Rust 中使用这些模块，可以在其它模块中使用 `use` 关键字来导入模块，例如：

```rust
// src/main.rs
mod foo;

use foo::bar::qux::do_something;
use foo::bar::do_something_else;
use foo::baz::do_another_thing;

fn main() {
    do_something();
    do_something_else();
    do_another_thing();
}
```

在上面的代码中，我们导入了 `foo` 模块中的各个子模块的函数，并在 `main` 函数中使用它们。

需要注意的是，如果要将模块嵌套在其它模块中，那么需要在包含它们的模块中使用 `mod` 关键字来创建这些子模块。这些模块的代码可以分别放在不同的文件中，文件名需要与模块名相同，且需要放在与父模块同级的目录中。

## 7. 为什么在使用单元测试的时候需要将 src/fig.rs 嵌套在 src/lib.rs ？

这是因为在 Rust 中，测试是通过 `cargo test` 命令来运行的，而 `cargo test` 命令会自动地构建你的 crate，并运行你 crate 中所有标记了 `#[test]` 属性的测试函数。

在这个过程中，`cargo test` 会找到所有标记了 `#[test]` 属性的测试函数，但是它并不知道这些测试函数在哪些文件中。因此，需要在 crate 的入口文件（通常是 `src/lib.rs` 或者 `src/main.rs`）中通过 `mod` 语句将所有包含测试函数的模块引入，以便 `cargo test` 可以找到这些函数。

因此，即使你在其他文件中编写了测试函数，也需要在 crate 的入口文件中通过 `mod` 语句将其引入，否则这些测试函数将不会被 `cargo test` 所识别。

## 8. 可以在一个 crate 中同时包含 src/lib.rs 和 src/main.rs 两个入口文件吗?

是的，一个 crate 中可以同时包含 `src/lib.rs` 和 `src/main.rs` 这两个入口文件。

当 crate 被用作依赖项时，编译器将默认使用 `src/lib.rs` 作为 crate 的入口文件。如果你想要将 crate 作为可执行程序运行，可以使用 `cargo run` 命令，编译器将使用 `src/main.rs` 作为入口文件。当然，你也可以通过 `cargo run --bin <name>` 命令来指定要运行的可执行程序的名称。

需要注意的是，如果一个 crate 同时包含了 `src/lib.rs` 和 `src/main.rs`，那么这两个文件中的代码应该是相互独立的，不应该相互依赖，否则可能会导致编译错误。

## 9. 常用的 cargo new 标志

当使用 `cargo new` 命令创建 Rust 项目时，可以使用不同的标志来自定义项目的结构和属性。以下是一些常用标志及其作用的简要说明：

| 标志       | 作用                    |
| ---------- | ----------------------- |
| --bin      | 创建可执行二进制文件    |
| --lib      | 创建库文件              |
| --edition  | 指定使用的 Rust 版本    |
| --name     | 指定项目名称            |
| --vcs      | 指定版本控制系统        |
| --registry | 指定使用的 Crates.io 源 |

## 10. rust 怎样跨平台生成指定类型的二进制文件？

Rust 可以通过交叉编译来生成在其他平台上运行的二进制文件。交叉编译是指在一台机器上编译生成另一种平台上运行的二进制文件的过程。

首先需要安装目标平台的工具链。可以使用 Rustup 工具来安装目标平台的工具链，例如：

```bash
# 安装 i686-unknown-linux-gnu 工具链
rustup target add i686-unknown-linux-gnu

# 安装 x86_64-pc-windows-msvc 工具链
rustup target add x86_64-pc-windows-msvc
```

接着，可以使用 `cargo build` 命令来交叉编译生成二进制文件。可以通过 `--target` 参数指定目标平台，例如：

```bash
# 交叉编译生成 i686-unknown-linux-gnu 平台的二进制文件
cargo build --target i686-unknown-linux-gnu

# 交叉编译生成 x86_64-pc-windows-msvc 平台的二进制文件
cargo build --target x86_64-pc-windows-msvc
```

交叉编译时需要注意目标平台的架构和操作系统类型。Rust 提供了许多目标平台，具体可以查看官方文档：[https://doc.rust-lang.org/nightly/rustc/platform-support.html](https://doc.rust-lang.org/nightly/rustc/platform-support.html)。

## 11. 索引使用方式总结

| 索引语法             | 含义                                                                 | 使用例子                     |
| -------------------- | -------------------------------------------------------------------- | ---------------------------- |
| `array[0]`           | 访问数组中的第一个元素                                               | `let x = array[0];`          |
| `array[n]`           | 访问数组中的第 n+1 个元素，其中 n 是一个整数，n 的值必须小于数组长度 | `let x = array[2];`          |
| `array[..]`          | 访问整个数组                                                         | `let x = &array[..];`        |
| `array[start..end]`  | 访问从索引 start 开始到索引 end-1 结束的所有元素                     | `let x = &array[2..5];`      |
| `array[..end]`       | 访问从索引 0 开始到索引 end-1 结束的所有元素                         | `let x = &array[..5];`       |
| `array[start..]`     | 访问从索引 start 开始到数组末尾的所有元素                            | `let x = &array[2..];`       |
| `array[start..=end]` | 访问从索引 start 开始到索引 end 结束的所有元素                       | `let x = &array[2..=5];`     |
| `array[usize::MAX]`  | 访问数组中的最后一个元素                                             | `let x = array[usize::MAX];` |

需要注意的是，Rust 数组的索引必须是 usize 类型。否则会编译错误。

## 12. rust 里面的 unwrap 函数的设计理念

在 Rust 中，`unwrap()` 函数是 `Option` 和 `Result` 两个重要枚举类型的方法之一，其设计理念主要是为了在编写代码时方便快捷地处理简单情况，例如：获取一个值时，如果存在，就返回该值；如果不存在，则直接 panic。这种方式对于初学者来说非常简单易懂，可以快速进行开发。但是，在生产环境中，建议使用更为严谨的错误处理方式，如在 `unwrap()` 函数的基础上进行进一步的错误处理或者使用其他的错误处理方式，避免程序出现意外错误而导致崩溃。

## 13. 举一个 Option 类型使用 unwrap 函数的例子

Option 类型是 Rust 语言中一种常用的枚举类型，它可以表示某个值存在或不存在。在使用 Option 类型时，有时候需要获取 Option 中的值，这时就可以使用 unwrap 函数。

例如，假设有一个名为`x`的 Option 类型变量，我们可以使用`unwrap`函数来获取它的值，如下所示：

```rust
let x: Option<i32> = Some(5);
let value = x.unwrap();
println!("value: {}", value); // output: value: 5
```

在这个例子中，`x`是一个 Option 类型的变量，其中存储了一个整数值 5。我们可以使用`unwrap`函数来获取`x`中的值，并将其存储在`value`变量中。由于`x`中确实有一个值，所以这里不会触发`unwrap`函数的 panic。最后我们将`value`的值打印出来，输出结果为`value: 5`。

需要注意的是，如果在调用`unwrap`函数时，Option 类型的变量中没有存储值，那么就会触发 panic，这是 Rust 语言中的一种运行时错误。因此，在使用`unwrap`函数时，需要确保 Option 类型的变量中确实存储了一个值。如果不能确定 Option 类型变量中是否存储了值，那么可以使用`match`语句或`if let`语句等方式来进行安全的解包操作。

## 14. Option 和 Result 两个枚举类型都是内置的枚举类型吗？

在 Rust 标准库中，Option 和 Result 两个重要的枚举类型是内置的，它们都属于枚举类型，其定义如下：

```rust
enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

Option 枚举类型用于表示一个值可能存在也可能不存在的情况。当我们需要在代码中表示某个值可能为空时，就可以使用 Option 类型。例如，函数返回值可能为空，我们就可以使用 `Option` 类型来表示。在使用 Option 类型时，我们可以使用 `Some` 来包裹存在的值，使用 `None` 来表示不存在的值。

Result 枚举类型用于表示一个操作可能成功也可能失败的情况。当我们需要在代码中处理可能出现的错误时，就可以使用 Result 类型。例如，打开文件时可能会出现错误，我们就可以使用 Result 类型来表示打开文件的结果。在使用 Result 类型时，我们可以使用 `Ok` 来包裹操作成功的结果，使用 `Err` 来表示操作失败，并携带错误信息。

## 15. 举一个使用 Result 进行错误传播的例子

下面是一个 Rust 中使用 `Result` 进行错误传播的例子：

```rust
use std::fs::File;
use std::io::prelude::*;
use std::io::Error;

fn read_file_contents(filename: &str) -> Result<String, Error> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn main() {
    let filename = "example.txt";
    match read_file_contents(filename) {
        Ok(contents) => println!("Contents of {}:\n{}", filename, contents),
        Err(error) => println!("Failed to read {}: {}", filename, error),
    }
}
```

上面的代码中，`read_file_contents` 函数会尝试打开一个文件并读取其中的内容，如果一切正常，它将返回一个包含文件内容的 `Ok` 变量；否则，它将返回一个包含错误信息的 `Err` 变量。在 `main` 函数中，使用 `match` 语句处理 `read_file_contents` 的返回值。如果返回的是 `Ok` 变量，就将文件内容打印出来；如果返回的是 `Err` 变量，就将错误信息打印出来。

这样的写法可以使代码更加清晰易读，也更容易排查和解决错误。运行代码见：[readfile.rs](https://github.com/wavty/rust-plus/blob/main/concepts/src/readfile.rs)。

## 16. rust 里面的 ? 运算符的作用是什么

在 Rust 中，`?` 是一种简便的错误处理语法糖，用于在函数中返回 `Result` 或 `Option` 类型时，将错误信息传播给调用方。

具体来说，`?` 可以被看作是一个简化的 `match` 表达式，用于检查 `Result` 或 `Option` 类型的值，如果值是 `Ok` 或 `Some` 则返回其包含的值，否则将错误或 `None` 值返回给调用方。这样，使用 `?` 运算符可以将错误处理逻辑内敛在函数中，减少代码的嵌套层数和提高可读性。

下面是一个使用 `?` 运算符的例子：

```rust
use std::fs::File;
use std::io::Read;

fn read_file_contents(path: &str) -> std::io::Result<String> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}
```

在这个例子中，`File::open` 和 `file.read_to_string` 方法都返回 `std::io::Result` 类型，使用 `?` 运算符可以在出现错误时将其传递给调用方，最终返回一个 `Result<String, std::io::Error>` 类型的结果。

## 17. 解释一下这个 cargo 指令: `cargo.exe test --package concepts --lib -- readfile::tests::test_for_env_vars --exact --nocapture`

这是一个运行 Rust 项目中指定测试的 Cargo 命令，各个参数的含义如下：

- `test`：运行测试命令。
- `--package concepts`：指定要测试的包名，这里是 `concepts`。
- `--lib`：只测试库文件，不包括二进制可执行文件。
- `-- readfile::tests::test_for_env_vars`：只运行特定的测试，即 `readfile::tests::test_for_env_vars`。其中，`readfile` 是一个模块名，`tests` 是该模块的测试模块名，`test_for_env_vars` 是具体的测试用例名。
- `--exact`：只运行指定的测试用例。
- `--nocapture`：不捕获测试的输出。

总的来说，这个指令的作用是在指定包的库文件中运行指定的测试用例，并打印测试输出，以方便查看测试结果。

## 18. 什么是 lazy_static

`lazy_static` 是一个 Rust crate，它提供了一种简便的方法来创建静态变量，并且仅在第一次访问变量时初始化。这种延迟初始化的方式可以在程序运行时减少内存的使用和初始化时间。

在 Rust 中，静态变量通常使用 `static` 关键字创建，并且必须在编译时初始化，这意味着无法在运行时根据某些条件来初始化变量。而使用 `lazy_static`，我们可以将变量的初始化推迟到第一次访问变量时，因此我们可以在程序运行时决定是否要初始化变量。

`lazy_static` crate 提供了一个宏 `lazy_static!`，可以用来定义静态变量。例如：

```rust
use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    static ref CONFIG_MAP: HashMap<String, String> = {
        let mut m = HashMap::new();
        m.insert("key1".to_string(), "value1".to_string());
        m.insert("key2".to_string(), "value2".to_string());
        m
    };
}
```

这段代码创建了一个 `CONFIG_MAP` 静态变量，它是一个 `HashMap`，在第一次访问该变量时才会初始化。这里使用了 Rust 的 `ref` 关键字来创建一个引用类型的静态变量，这样可以避免在静态变量创建时发生所有权转移的问题。

`lazy_static` crate 还支持许多其他的特性，例如使用 Mutex 来保证线程安全等。
