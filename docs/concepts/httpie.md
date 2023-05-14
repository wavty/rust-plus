# 设计 httpie

## 一、架构

[HTTPie](https://github.com/wavty/rust-plus/blob/main/gethandsdirty/httpie/src/main.rs) is built in Rust, a systems programming language known for its speed, safety, and concurrency.

![httpie-framework](../images/httpie-framework.png)

## 二、基础知识

### 1. `clap::Parser` 生成的常用属性

详见：https://docs.rs/clap/4.2.7/clap/_derive/_tutorial/index.html

| 属性名称        | 含义                   | 使用示例                                                                 | 解释                                                                                           |
| --------------- | ---------------------- | ------------------------------------------------------------------------ | ---------------------------------------------------------------------------------------------- |
| `about`         | 关于此程序的简短描述   | `#[command(about = "Simple program to greet a person")]`                 | 用于设置程序的简短描述，通常在帮助信息中使用                                                   |
| `author`        | 程序作者               | `#[command(author = "wavty")]`                                           | 用于设置程序的作者，通常在帮助信息中使用                                                       |
| `version`       | 程序版本               | `#[command(version)]`                                                    | 用于设置程序的版本，通常在帮助信息中使用                                                       |
| `long_about`    | 关于此程序的长描述     | `#[command(long_about = "This is a simple program to greet a person.")]` | 用于设置程序的长描述，通常在帮助信息中使用                                                     |
| `name`          | 程序的名字             | `#[command(name = "greet")]`                                             | 用于设置程序的名字，通常在帮助信息中使用                                                       |
| `arg`           | 定义一个命令行参数     | `#[arg(short, long)]`                                                    | 用于定义命令行参数的选项和标志，例如 `-h` 或 `--help`                                          |
| `subcommand`    | 定义一个子命令         | `#[command(subcommand)]`                                                 | 用于定义一个子命令，例如 `git commit` 中的 `commit` 子命令                                     |
| `group`         | 定义一组相互关联的参数 | `#[arg(group = "greeting")]`                                             | 用于定义一组相互关联的参数，例如 `--greeting hello --name World` 中的 `--greeting` 和 `--name` |
| `validator`     | 验证参数的有效性       | `#[arg(validator = validate_count)]`                                     | 用于定义验证参数的函数，例如检查参数是否在一定范围内                                           |
| `required`      | 声明参数为必需         | `#[arg(required = true)]`                                                | 用于声明参数为必需的，如果未提供则会引发错误                                                   |
| `default_value` | 设置参数的默认值       | `#[arg(default_value = "World")]`                                        | 用于设置参数的默认值，如果未提供参数，则使用此值                                               |

### 2. rust 的 MIME 类型指的是什么？

在 Rust 中，MIME 类型是指 Multipurpose Internet Mail Extensions（多用途 Internet 邮件扩展）类型的缩写，它是用于在互联网上传输各种类型的文件和数据的一种标准方式。MIME 类型以字符串的形式表示，通常使用扩展名或文件类型来确定给定文件的 MIME 类型。例如，MIME 类型"text/plain"通常与.txt 文件扩展名相关联。

### 3. clap4.2.7 如何使用 Subcommand

[Subcommands \_tutorial](https://docs.rs/clap/4.2.7/clap/_derive/_tutorial/index.html#subcommands)

```rust
use clap::{Args, Parser, Subcommand};

/// A naive httpie implementation with Rust.
#[derive(Parser, Debug)]
#[command(version, author="wavty", about="httpie", long_about=None)]
struct Opts {
    /// httpie subcommand
    #[command(subcommand)]
    subcmd: Subcmd,
}

#[derive(Subcommand, Debug)]
enum Subcmd {
    Get(Get),
    Post(Post),
}

#[derive(Args, Debug)]
struct Get {
    /// HTTP request url
    url: String,
}

#[derive(Args, Debug)]
struct Post {
    /// HTTP request url
    url: String,
    /// HTTP request body
    body: Vec<String>,
}

fn main() {
    let args = Opts::parse();
    println!("{:?}", args);
}
// $ ./httpie.exe post baidu.com 123
// Opts { subcmd: Post(Post { url: "baidu.com", body: ["123"] }) }
```

### 4. 写一段代码展示一下 async 与 await 实现的异步机制

当使用 `await` 进行异步操作时，如果操作尚未完成，它会将当前的异步函数挂起，并将控制权返回给调用方，使当前线程能够执行其他任务。以下是一个简化的例子，用于说明这个过程：

```rust
async fn do_something() {
    // 模拟一个耗时的异步操作
    tokio::time::sleep(Duration::from_secs(5)).await;
    println!("异步操作完成");
}

#[tokio::main]
async fn main() {
    // 启动异步操作
    let task = do_something();

    // 执行其他任务
    println!("执行其他任务");

    // 等待异步操作完成
    task.await;

    // 异步操作完成后继续执行
    println!("所有任务完成");
}
```

在这个例子中，`do_something` 函数是一个异步函数，它模拟了一个耗时的异步操作（这里使用了 `tokio::time::sleep` 来模拟）。在 `main` 函数中，我们启动了 `do_something` 函数，然后打印了一条消息表示执行了其他任务。

在等待异步操作完成之前，`main` 函数会继续执行其他任务（在这个例子中只是打印了一条消息）。然后，通过 `await` 关键字等待异步操作完成，这时控制权会返回给调用方，使当前线程能够执行其他任务。

在等待了约 5 秒后，异步操作完成，控制权再次回到了 `do_something` 函数里面的 `await` 的位置，然后继续执行后续的代码。最后打印出 "所有任务完成" 的消息。

这个例子展示了异步操作如何在等待期间挂起当前函数，允许当前线程去执行其他任务。这样可以提高并发性和响应性能，使得程序能够更高效地利用计算资源。

### 5. 为什么要在 main 函数上使用 `#[tokio::main]` 宏？

`#[tokio::main]` 宏是 Rust 中的一个 Tokio 库提供的特性，用于简化在异步上下文中运行程序的设置。它具有以下优点：

1. **简化异步代码**：通过在 `main` 函数上使用 `#[tokio::main]` 宏，你可以直接在 `main` 函数中编写异步代码，无需手动创建异步运行时或处理任务调度等繁琐操作。

2. **自动处理异步任务调度**：Tokio 异步运行时会自动处理异步任务的调度和执行。当遇到 `await` 表达式时，异步函数会被挂起，让运行时可以调度其他任务。当等待的异步操作完成后，运行时会恢复挂起的函数，并继续执行。

3. **方便的错误处理**：`#[tokio::main]` 宏还会自动处理异步操作的错误，将其转换为 `Result` 类型。这使得在异步代码中进行错误处理变得更加简单和一致。

综上所述，使用 `#[tokio::main]` 宏可以简化异步代码的编写，处理异步任务的调度和执行，并提供方便的错误处理机制。它使得编写和运行异步程序变得更加简单和高效。
