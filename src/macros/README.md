# `custom_meta_struct` 宏的用法

## 为什么要有这个宏

dnspod 的每个请求(action)的参数都不太一样, 但公共参数(url, version, region 等)又基本都相同, 
所以我需要一个宏来帮助我完成下面的任务  

```rust
use serde::Serialize;
use serde::Deserialize;

macro_rules! my_macro {
    ($($tt: tt)*) => {};
}

trait SomeCommonTrait {}

my_macro! {
    struct ActionA {...}
    
    #[cfg_attr(feature = "clap", arg(long, value_enum, default_value_t=Default::default()))]
    struct ActionB {...}

    @[url = "https://hangj.cnblogs.com"]
    #[cfg(feature = "RustHub")]
    @[version = Version::Version2021_03_23]
    struct ActionC {...}
}

// 自动展开为:
#[derive(Debug, Clone, Serialize, Deserialize)]
struct ActionA {}

impl SomeCommonTrait for ActionA {}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "clap", arg(long, value_enum, default_value_t=Default::default()))]
struct ActionB {}

impl SomeCommonTrait for ActionB {}

#[cfg(feature = "RustHub")]
#[derive(Debug, Clone, Serialize, Deserialize)]
struct ActionC {}
```

其中 `@[..]` 为我的自定义属性, 用来重载个别请求(action)的公共参数(url, version, region), 
如果我们的宏定义如下:

```rust
macro_rules! define_structs {
    (
        $(
            $(#[$meta: meta])*
            $(@[$my_meta: meta])*
            $vis: vis struct $name: ident $body: tt
        )*
    ) => {
        // ...
    };
}
```

你会发现它并不能匹配到 `@[..]` 在 `#[..]` 前面的情况, 更不要说把两者混在一起的情况。
而 `custom_meta_struct` 让我可以在定义 struct 时, 不需要考虑 `#[..]` 与 `@[..]` 的先后顺序, 
`custom_meta_struct` 最终会把所有的输入整流成 `#[..]` 在前, `@[..]` 在后的格式, 然后传递给「回调宏」  

## 用法

看个例子:

```rust
macro_rules! define_structs {
    (
        $(
            $(#[$meta: meta])*
            $(@[$my_meta: meta])*
            $vis: vis struct $name: ident $body: tt
        )*
    ) => {
        // ...
    };
}

// 通过 custom_meta_struct 的预处理, define_structs 就可以很好的接收原来无法匹配的内容了
dnspod_lib::custom_meta_struct! {
    define_structs,

    struct A;
    struct B;
    // ...
}
```

如果还想对所有 `struct` 做一些其它统一的动作, 则可以这样传如公共 meta 属性:

```rust
macro_rules! define_structs {
    (
        $(
            $(#[$meta: meta])*
            $(@[$my_meta: meta])*
            $vis: vis struct $name: ident $body: tt
        )*
    ) => {};
}

dnspod_lib::custom_meta_struct! {
    (
        // callback macro
        define_structs,
        // common metas for every struct
        #[derive(Debug)]
        @[url = "https://example.com"]
        #[derive(Clone)]
    ),

    struct A;
    struct B;
    // ...
}
```

如果你不需要 `@[..]`, 只想添加一些公共的 `#[..]`, 则不需要配置 callback macro  

```rust
dnspod_lib::custom_meta_struct! {
    (
        #[derive(Debug)]
        #[derive(Clone)]
    ),
    struct A;
    struct B;
}
```

## 实际用例

```rust
#[macro_export]
macro_rules! impl_macro {
    (
        $(
            $(#[$meta: meta])*
            $(@[$my_meta: meta])*
            $vis: vis struct $name: ident $body: tt
        )*
    ) => {
        // ...
    };
}

#[macro_export]
macro_rules! public_macro {
    ($($tt: tt)*) => {
        dnspod_lib::custom_meta_struct! {
            $crate::impl_macro,
            $($tt)*
        }
    }
}

// 外部 crate 调用 public_macro 即可
```

具体可以查看 `action.rs`