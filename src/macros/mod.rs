#![doc = include_str!("README.md")]

/// 混编 `#[..]` 与 `@[..]` 整流成 `#[..]` 在前, `@[..]` 在后
/// 
/// 详细用法请查看 [README.md](https://github.com/hangj/dnspod-lib/tree/main/src/macros)
#[macro_export]
macro_rules! custom_meta_struct {
    ($($tt: tt)*) => {
        $crate::step1! {
            $($tt)*
        }
    }
}

#[macro_export]
macro_rules! step1 {
    () => {};
    (
        #[$meta: meta]
        $($tail: tt)*
    ) => {
        $crate::step1! {
            [ #[$meta] ],
            $($tail)*
        }
    };
    (
        @[$($my_meta: tt)*]
        $($tail: tt)*
    ) => {
        $crate::step1! {
            [ @[$($my_meta)*] ],
            $($tail)*
        }
    };

    (
        [$($cus_meta: tt)*],
        @[$($my_meta: tt)*]
        $($tail: tt)*
    ) => {
        $crate::step1! {
            [ $($cus_meta)* @[$($my_meta)*] ],
            $($tail)*
        }
    };
    (
        [$($cus_meta: tt)*],
        #[$meta: meta]
        $($tail: tt)*
    ) => {
        $crate::step1! {
            [ $($cus_meta)* #[$meta] ],
            $($tail)*
        }
    };
    (
        [$($cus_meta: tt)*],
        ,
        $($tail: tt)*
    ) => {
        $crate::step2! {
            (, $($cus_meta)*),
            $($tail)*
        }
    };
    (
        [$($cus_meta: tt)*],
        $($tail: tt)*
    ) => {
        $crate::step2! {
            (, $($cus_meta)*),
            $($tail)*
        }
    };

    (
        $path: path
    ) => {
        $crate::step2! {
            ($path, ),
        }
    };

    (
        $path: path,
        $($tail: tt)*
    ) => {
        $crate::step2! {
            ($path, ),
            $($tail)*
        }
    };

    (
        ($($header: tt)*)
        ,
        $($tail: tt)*
    ) => {
        $crate::step2! {
            ($($header)*),
            $($tail)*
        }
    };

    (
        ($($header: tt)*)
        $($tail: tt)*
    ) => {
        $crate::step2! {
            ($($header)*),
            $($tail)*
        }
    };
}

#[macro_export]
macro_rules! step2 {
    () => {};

    (
        $(#[$meta: meta])*,
        $($tt: tt)*
    ) => {
        $crate::step2! {
            (, $(#[$meta])* ),
            $($tt)*
        }
    };

    (
        (#[$meta: meta] $($tt: tt)*),
        $($tail: tt)*
    ) => {
        $crate::new_struct! {
            ( , #[$meta] $($tt)* ),
            [],
            $($tail)*
        }
    };
    (
        (@[$($my_meta: tt)*] $($tt: tt)*),
        $($tail: tt)*
    ) => {
        $crate::new_struct! {
            ( , @[$($my_meta)*] $($tt)* ),
            [],
            $($tail)*
        }
    };

    (
        ($($cb: path $(,)?)? ),
        $($tail: tt)*
    ) => {
        $crate::new_struct! {
            ( $($cb)?, ),
            [],
            $($tail)*
        }
    };
    (
        ($($cb: path)? , #[$meta: meta] $($common_meta: tt)*),
        $($tail: tt)*
    ) => {
        $crate::new_struct! {
            ( $($cb)?, #[$meta] $($common_meta)* ),
            [],
            $($tail)*
        }
    };
    (
        ($($cb: path)? , @[$($cus_meta: tt)*] $($common_meta: tt)*),
        $($tail: tt)*
    ) => {
        $crate::new_struct! {
            ( $($cb)?, @[$($cus_meta)*] $($common_meta)* ),
            [],
            $($tail)*
        }
    };

    (
        ($($tt: tt)*) $(,)?
    ) => {};
}

#[macro_export]
macro_rules! custom_meta_struct_impl {
    (
        ($($cb: tt)*),
        [$($meta: tt)*],
        [$($my_meta: tt)*],
        [$($strct: tt)*],
        // struct 部分不存在的情况
    ) => {
        $crate::new_struct! {
            ( $($cb)* ),
            [ $($strct)* ],
        }
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
        $crate::custom_meta_struct_impl! {
            ($($cb)*),
            [$($meta)* # $tt], 
            [$($my_meta)*], 
            [$($strct)*],
            $($tail)*
        }
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
        $crate::custom_meta_struct_impl! {
            ($($cb)*),
            [$($meta)*], 
            [$($my_meta)* @ $tt], 
            [$($strct)*],
            $($tail)*
        }
    };

    // 组装 struct
    (
        ($($cb: tt)*),
        [$($meta: tt)*],
        [$($my_meta: tt)*],
        [$($strct: tt)*],
        $vis: vis struct $name: ident $body: tt
        $($tail: tt)*
    ) => {
        $crate::new_struct! {
            ($($cb)*),
            [
                $($strct)*
                $($meta)*
                $($my_meta)*
                $vis struct $name $body
            ],
            $($tail)*
        }
    };
}

#[macro_export]
macro_rules! new_struct {
    (
        ($($cb: tt)*),
        [$($strct: tt)*],
    ) => {
        $crate::finally! {
            ($($cb)*),
            [$($strct)*],
        }
    };

    (
        ($($cb: path)?, $($common_meta: tt)*),
        [$($strct: tt)*],
        $tt: tt
        $($tail: tt)*
    ) => {
        $crate::custom_meta_struct_impl! {
            ($($cb)?, $($common_meta)*),
            [],
            [],
            [$($strct)*],
            $($common_meta)*
            $tt
            $($tail)*
        }
    };
}

#[macro_export]
macro_rules! finally {
    (
        ($cb: path, $($common_meta: tt)*),
        [$($strct: tt)*] $(,)?
    ) => {
        $cb! {
            $($strct)*
        }
    };

    (
        (, $($common_meta: tt)*),
        [$($strct: tt)*] $(,)?
    ) => {
        $($strct)*
    };
}

// ---------------------------------------------------------------- //


#[cfg(test)]
mod tests {
    custom_meta_struct! {}
    custom_meta_struct! {
        ()
    }
    custom_meta_struct! {
        (),
    }

    macro_rules! fuck {
        ($($tt: tt)*) => {};
    }
    custom_meta_struct! {
        fuck
    }
    custom_meta_struct! {
        fuck,
    }

    custom_meta_struct! {
        #[derive(Debug)]
        ,
    }

    custom_meta_struct! {
        #[derive(Debug)]
        #[derive(Clone)]
        ,
    }

    custom_meta_struct! {
        #[derive(Debug)]
        #[derive(Clone)]
        ,

        struct Hello;
        struct World;
    }

    custom_meta_struct! {
        (#[derive(Debug)]),
    }
    custom_meta_struct! {
        (@[derive(Debug)])
    }

    custom_meta_struct! {
        (
            #[derive(Clone)]
            #[derive(Debug)]
        ),
        struct TestA;
    }

    macro_rules! define_structs {
        (
            $(
                $(#[$meta: meta])*
                $(@[$($my_meta: tt)*])*
                $vis: vis struct $name: ident $body: tt
            )*
        ) => {};
    }

    custom_meta_struct! {
        define_structs,
        struct A;
        struct B;
    }
    custom_meta_struct! {
        (
            define_structs, 
            #[derive(Debug, Clone)]
            @[url = "https://hangj.cnblogs.com"]
        ),

        struct A;
        struct B{}
        @[fuck]
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


#[cfg(test)]
mod test2 {
    macro_rules! structs_to_string {
        (
            $(
                $(#[$meta: meta])*
                $(@[$($my_meta: tt)*])*
                $vis: vis struct $name: ident $body: tt
            )*
        ) => {
            stringify!(
                $(
                    $(#[$meta])*
                    $(@[$($my_meta)*])*
                    $vis struct $name $body
                )*
            )
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ")
        };
    }

    #[test]
    fn test() {
        {
            crate::custom_meta_struct! {
                #[derive(Debug)]
                ,
                #[derive(Clone)]
                struct A;
            }
            assert_eq!(format!("{:?}", A.clone()), "A");
        }

        {
            crate::custom_meta_struct! {
                #[derive(Debug)]
                ,
                #[derive(Clone)]
                struct A;
                struct B;
            }
            assert_eq!(format!("{:?}", A.clone()), "A");
            assert_eq!(format!("{:?}", B), "B");
        }

        {
            crate::custom_meta_struct! {
                (
                    #[derive(Debug)]
                    #[derive(Clone)]
                ),
                struct A;
                struct B;
            }

            assert_eq!(format!("{:?}", A.clone()), "A");
            assert_eq!(format!("{:?}", B.clone()), "B");
        }

        assert_eq!(
            {
                crate::custom_meta_struct! {
                    structs_to_string,
                }
            },
            ""
        );
        assert_eq!(
            {
                crate::custom_meta_struct! {
                    (structs_to_string),
                    struct A;
                }
            },
            "struct A ;"
        );
        assert_eq!(
            {
                crate::custom_meta_struct! {
                    (structs_to_string, ),
                    struct A;
                    #[derive(Debug)]
                    struct B;
                }
            },
            "struct A ; #[derive(Debug)] struct B ;"
        );

        assert_eq!(
            {
                crate::custom_meta_struct! {
                    (
                        structs_to_string,
                        #[derive(Debug)]
                    ),
                    struct A;
                    struct B;
                }
            },
            "#[derive(Debug)] struct A ; #[derive(Debug)] struct B ;"
        );

        assert_eq!(
            {
                crate::custom_meta_struct! {
                    (
                        structs_to_string,
                        #[derive(Debug)]
                        @[hello world]
                        #[derive(Hash)]
                    ),
                    struct A;
                    #[derive(Clone)]
                    struct B;
                }
            },
            "#[derive(Debug)] #[derive(Hash)] @ [hello world] struct A ; #[derive(Debug)] #[derive(Hash)] #[derive(Clone)] @ [hello world] struct B ;"
        );
    }
}

