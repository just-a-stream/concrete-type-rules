/// A macro that generates a combined matcher macro for multiple concrete enums.
///
/// This macro creates a new macro that allows you to match multiple enum instances
/// simultaneously, providing type parameters for each concrete type associated with
/// the enum variants.
///
/// # Examples
/// ```rust,ignore
/// use concrete_type::{Concrete};
/// use concrete_type_rules::gen_match_concretes_macro;
///
/// #[derive(Concrete, Clone, Copy)]
/// enum Exchange {
///     #[concrete = "BinanceType"]
///     Binance,
/// }
///
/// #[derive(Concrete)]
/// enum Strategy {
///     #[concrete = "StrategyAType"]
///     StrategyA,
/// }
///
/// struct BinanceType;
/// struct StrategyAType;
///
/// // Generate a combined matcher macro
/// gen_match_concretes_macro!(Exchange, Strategy);
///
/// // Now you can use the generated macro
/// let exchange = Exchange::Binance;
/// let strategy = Strategy::StrategyA;
///
/// let result = match_exchange_strategy!(exchange, strategy; E, S => {
///     // Here E is exchanges::Binance and S is strategies::StrategyA
///     format!("{}", std::any::type_name::<(E, S)>())
/// });
/// ```
#[macro_export]
macro_rules! gen_match_concretes_macro {
    ($first_enum:ident, $second_enum:ident) => {
        paste::paste! {
            #[macro_export]
            macro_rules! [<match_ $first_enum:snake _ $second_enum:snake>] {
                ($first_var:expr, $second_var:expr; $first_type:ident, $second_type:ident => $code_block:block) => {
                    [<$first_enum:snake>]!($first_var; $first_type => {
                        [<$second_enum:snake>]!($second_var; $second_type => {
                            $code_block
                        })
                    })
                };
            }
        }
    };
    // Support for 3 enum types
    ($first_enum:ident, $second_enum:ident, $third_enum:ident) => {
        paste::paste! {
            #[macro_export]
            macro_rules! [<match_ $first_enum:snake _ $second_enum:snake _ $third_enum:snake>] {
                ($first_var:expr, $second_var:expr, $third_var:expr; $first_type:ident, $second_type:ident, $third_type:ident => $code_block:block) => {
                    [<$first_enum:snake>]!($first_var; $first_type => {
                        [<$second_enum:snake>]!($second_var; $second_type => {
                            [<$third_enum:snake>]!($third_var; $third_type => {
                                $code_block
                            })
                        })
                    })
                };
            }
        }
    };
    // Support for 4 enum types
    ($first_enum:ident, $second_enum:ident, $third_enum:ident, $fourth_enum:ident) => {
        paste::paste! {
            #[macro_export]
            macro_rules! [<match_ $first_enum:snake _ $second_enum:snake _ $third_enum:snake _ $fourth_enum:snake>] {
                ($first_var:expr, $second_var:expr, $third_var:expr, $fourth_var:expr;
                 $first_type:ident, $second_type:ident, $third_type:ident, $fourth_type:ident => $code_block:block) => {
                    [<$first_enum:snake>]!($first_var; $first_type => {
                        [<$second_enum:snake>]!($second_var; $second_type => {
                            [<$third_enum:snake>]!($third_var; $third_type => {
                                [<$fourth_enum:snake>]!($fourth_var; $fourth_type => {
                                    $code_block
                                })
                            })
                        })
                    })
                };
            }
        }
    };
    // Support for 5 enum types
    ($first_enum:ident, $second_enum:ident, $third_enum:ident, $fourth_enum:ident, $fifth_enum:ident) => {
        paste::paste! {
            #[macro_export]
            macro_rules! [<match_ $first_enum:snake _ $second_enum:snake _ $third_enum:snake _ $fourth_enum:snake _ $fifth_enum:snake>] {
                ($first_var:expr, $second_var:expr, $third_var:expr, $fourth_var:expr, $fifth_var:expr;
                 $first_type:ident, $second_type:ident, $third_type:ident, $fourth_type:ident, $fifth_type:ident => $code_block:block) => {
                    [<$first_enum:snake>]!($first_var; $first_type => {
                        [<$second_enum:snake>]!($second_var; $second_type => {
                            [<$third_enum:snake>]!($third_var; $third_type => {
                                [<$fourth_enum:snake>]!($fourth_var; $fourth_type => {
                                    [<$fifth_enum:snake>]!($fifth_var; $fifth_type => {
                                        $code_block
                                    })
                                })
                            })
                        })
                    })
                };
            }
        }
    };
}
