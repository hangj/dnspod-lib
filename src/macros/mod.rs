#![doc = include_str!("README.md")]


/// 混编 `#[..]` 与 `@[..]` 整流成 `#[..]` 在前, `@[..]` 在后
/// callback 回调宏  
/// common_meta 赋给每一个 struct 的 meta
/// 
/// 详细用法请查看 README.md
#[macro_export]
macro_rules! custom_meta_struct {
    () => {};

    // 只有 callback 宏的情况
    (
        ($($cb: tt)*) $(,)?
    ) => {
        $crate::custom_meta_struct!(($($cb)*), [], [], []);
    };

    // callback 宏不带参数时没有被 `()` 包起来的情况
    // custom_meta_struct! {callback, struct A;}
    (
        $path: path,
        $($tail: tt)*
    ) => {
        $crate::custom_meta_struct!(($path, ), [], [], [], $($tail)*);
    };

    // 入口 1
    (
        ($cb: path, $($common_meta: tt)*),
        # $tt: tt
        $($tail: tt)*
    ) => {
        $crate::custom_meta_struct!(
            ($cb, $($common_meta)*),
            [# $tt],
            [],
            [],
            $($common_meta)* // 注入公共 meta
            $($tail)*
        );
    };

    // 入口 2
    (
        ($cb: path, $($common_meta: tt)*),
        @ $tt: tt
        $($tail: tt)*
    ) => {
        $crate::custom_meta_struct!(
            ($cb, $($common_meta)*),
            [],
            [@ $tt],
            [],
            $($common_meta)* // 注入公共 meta
            $($tail)*
        );
    };

    // 入口 3: 没有 meta 属性的情况
    (
        ($cb: path, $($common_meta: tt)*),
        $vis: vis struct $name: ident $body: tt
        $($tail: tt)*
    ) => {
        $crate::custom_meta_struct!(
            ($cb, $($common_meta)*),
            [],
            [],
            [],
            $($common_meta)*
            $vis struct $name $body
            $($tail)*
        );
    };

    // 拆分组装车间 1
    (
        ($($cb: tt)*),
        [$($meta: tt)*],
        [$($my_meta: tt)*],
        [$($strct: tt)*],
        # $tt: tt
        $($tail: tt)*
    ) => {
        $crate::custom_meta_struct!(
            ($($cb)*),
            [$($meta)* # $tt], 
            [$($my_meta)*], 
            [$($strct)*],
            $($tail)*
        );
    };
    // 拆分组装车间 2
    (
        ($($cb: tt)*),
        [$($meta: tt)*],
        [$($my_meta: tt)*],
        [$($strct: tt)*],
        @ $tt: tt
        $($tail: tt)*
    ) => {
        $crate::custom_meta_struct!(
            ($($cb)*),
            [$($meta)*], 
            [$($my_meta)* @ $tt], 
            [$($strct)*],
            $($tail)*
        );
    };

    // 组装 struct
    (
        ($cb: path, $($common_meta: tt)*),
        [$($meta: tt)*],
        [$($my_meta: tt)*],
        [$($strct: tt)*],
        $vis: vis struct $name: ident $body: tt
        $($tail: tt)*
    ) => {
        $crate::new_struct! {
            ($cb, $($common_meta)*),
            [],
            [],
            [
                $($strct)*
                $($meta)*
                $($my_meta)*
                $vis struct $name $body
            ],
            $($tail)*
        }
    };

    // 最终汇入此处
    (
        ($cb: path, $($common_meta: tt)*),
        [],
        [],
        [$($strct: tt)*] $(,)?
    ) => {
        $cb! {
            $($strct)*
        }
    };
}


#[macro_export]
macro_rules! new_struct {
    (
        ($cb: path, $($common_meta: tt)*),
        [],
        [],
        [$($strct: tt)*],
    ) => {
        $crate::custom_meta_struct! {
            ($cb, $($common_meta)*),
            [],
            [],
            [$($strct)*],
        }
    };

    (
        ($cb: path, $($common_meta: tt)*),
        [],
        [],
        [$($strct: tt)*],
        $tt: tt
        $($tail: tt)*
    ) => {
        $crate::custom_meta_struct! {
            ($cb, $($common_meta)*),
            [],
            [],
            [$($strct)*],
            $($common_meta)*
            $tt
            $($tail)*
        }
    };
}



#[cfg(test)]
mod tests {
    macro_rules! define_structs {
        (
            $(
                $(#[$meta: meta])*
                $(@[$my_meta: meta])*
                $vis: vis struct $name: ident $body: tt
            )*
        ) => {
            
        };
    }

    custom_meta_struct! {
        define_structs,
        struct A;
    }
    custom_meta_struct! {
        (
            define_structs, 
            #[derive(Debug, Clone)]
            @[url = "https://hangj.cnblogs.com"]
        ),

        struct A;
        struct B{}
        struct C;
    }
    custom_meta_struct! {
        define_structs,

        #[derive(Debug)]
        struct A;
    }
    custom_meta_struct! {
        define_structs,
        struct A;
        struct B{}
    }
    custom_meta_struct! {
        define_structs,
        @[derive(Debug)]
        struct A;
        struct B{}
    }
    custom_meta_struct! {
        define_structs,

        @[derive(Debug)]
        #[derive(Debug)]
        struct A;
        struct B{}

        @[公众号 = RustHub]
        #[derive(Debug)]
        @[公众号 = RustHub]
        struct C {
            v: String,
        }
    }
}
