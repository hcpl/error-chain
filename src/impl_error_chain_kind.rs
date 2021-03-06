// From https://github.com/tailhook/quick-error
// Changes:
//   - replace `impl Error` by `impl Item::description`
//   - $imeta

#[macro_export]
#[doc(hidden)]
macro_rules! impl_error_chain_kind {
    (   $(#[$meta:meta])*
        pub enum $name:ident { $($chunks:tt)* }
    ) => {
        impl_error_chain_kind!(SORT [pub enum $name $(#[$meta])* ]
            items [] buf []
            queue [ $($chunks)* ]);
    };
    // Queue is empty, can do the work
    (SORT [pub enum $name:ident $( #[$meta:meta] )*]
        items [$($( #[$imeta:meta] )*
                  => $iitem:ident: $imode:tt [$( $ivar:ident: $ityp:ty ),*]
                                {$( $ifuncs:tt )*} )* ]
        buf [ ]
        queue [ ]
    ) => {
        impl_error_chain_kind!(ENUM_DEFINITION [pub enum $name $( #[$meta] )*]
            body []
            queue [$($( #[$imeta] )*
                      => $iitem: $imode [$( $ivar: $ityp ),*] )*]
        );
        impl_error_chain_kind!(IMPLEMENTATIONS $name {$(
           $iitem: $imode [$(#[$imeta])*] [$( $ivar: $ityp ),*] {$( $ifuncs )*}
           )*});
        $(
            impl_error_chain_kind!(ERROR_CHECK $imode $($ifuncs)*);
        )*
    };
    // Add meta to buffer
    (SORT [$( $def:tt )*]
        items [$($( #[$imeta:meta] )*
                  => $iitem:ident: $imode:tt [$( $ivar:ident: $ityp:ty ),*]
                                {$( $ifuncs:tt )*} )* ]
        buf [$( #[$bmeta:meta] )*]
        queue [ #[$qmeta:meta] $( $tail:tt )*]
    ) => {
        impl_error_chain_kind!(SORT [$( $def )*]
            items [$( $(#[$imeta])* => $iitem: $imode [$( $ivar:$ityp ),*] {$( $ifuncs )*} )*]
            buf [$( #[$bmeta] )* #[$qmeta] ]
            queue [$( $tail )*]);
    };
    // Add ident to buffer
    (SORT [$( $def:tt )*]
        items [$($( #[$imeta:meta] )*
                  => $iitem:ident: $imode:tt [$( $ivar:ident: $ityp:ty ),*]
                                {$( $ifuncs:tt )*} )* ]
        buf [$( #[$bmeta:meta] )*]
        queue [ $qitem:ident $( $tail:tt )*]
    ) => {
        impl_error_chain_kind!(SORT [$( $def )*]
            items [$( $(#[$imeta])*
                      => $iitem: $imode [$( $ivar:$ityp ),*] {$( $ifuncs )*} )*]
            buf [$(#[$bmeta])* => $qitem : UNIT [ ] ]
            queue [$( $tail )*]);
    };
    // Flush buffer on meta after ident
    (SORT [$( $def:tt )*]
        items [$($( #[$imeta:meta] )*
                  => $iitem:ident: $imode:tt [$( $ivar:ident: $ityp:ty ),*]
                                {$( $ifuncs:tt )*} )* ]
        buf [$( #[$bmeta:meta] )*
            => $bitem:ident: $bmode:tt [$( $bvar:ident: $btyp:ty ),*] ]
        queue [ #[$qmeta:meta] $( $tail:tt )*]
    ) => {
        impl_error_chain_kind!(SORT [$( $def )*]
            enum [$( $(#[$emeta])* => $eitem $(( $($etyp),* ))* )*
                     $(#[$bmeta])* => $bitem: $bmode $(( $($btyp),* ))*]
            items [$($( #[$imeta:meta] )*
                      => $iitem: $imode [$( $ivar:$ityp ),*] {$( $ifuncs )*} )*
                     $bitem: $bmode [$( $bvar:$btyp ),*] {} ]
            buf [ #[$qmeta] ]
            queue [$( $tail )*]);
    };
    // Add tuple enum-variant
    (SORT [$( $def:tt )*]
        items [$($( #[$imeta:meta] )*
                  => $iitem:ident: $imode:tt [$( $ivar:ident: $ityp:ty ),*]
                                {$( $ifuncs:tt )*} )* ]
        buf [$( #[$bmeta:meta] )* => $bitem:ident: UNIT [ ] ]
        queue [($( $qvar:ident: $qtyp:ty ),+) $( $tail:tt )*]
    ) => {
        impl_error_chain_kind!(SORT [$( $def )*]
            items [$( $(#[$imeta])* => $iitem: $imode [$( $ivar:$ityp ),*] {$( $ifuncs )*} )*]
            buf [$( #[$bmeta] )* => $bitem: TUPLE [$( $qvar:$qtyp ),*] ]
            queue [$( $tail )*]
        );
    };
    // Add struct enum-variant - e.g. { descr: &'static str }
    (SORT [$( $def:tt )*]
        items [$($( #[$imeta:meta] )*
                  => $iitem:ident: $imode:tt [$( $ivar:ident: $ityp:ty ),*]
                                {$( $ifuncs:tt )*} )* ]
        buf [$( #[$bmeta:meta] )* => $bitem:ident: UNIT [ ] ]
        queue [{ $( $qvar:ident: $qtyp:ty ),+} $( $tail:tt )*]
    ) => {
        impl_error_chain_kind!(SORT [$( $def )*]
            items [$( $(#[$imeta])* => $iitem: $imode [$( $ivar:$ityp ),*] {$( $ifuncs )*} )*]
            buf [$( #[$bmeta] )* => $bitem: STRUCT [$( $qvar:$qtyp ),*] ]
            queue [$( $tail )*]);
    };
    // Add struct enum-variant, with excess comma - e.g. { descr: &'static str, }
    (SORT [$( $def:tt )*]
        items [$($( #[$imeta:meta] )*
                  => $iitem:ident: $imode:tt [$( $ivar:ident: $ityp:ty ),*]
                                {$( $ifuncs:tt )*} )* ]
        buf [$( #[$bmeta:meta] )* => $bitem:ident: UNIT [ ] ]
        queue [{$( $qvar:ident: $qtyp:ty ),+ ,} $( $tail:tt )*]
    ) => {
        impl_error_chain_kind!(SORT [$( $def )*]
            items [$( $(#[$imeta])* => $iitem: $imode [$( $ivar:$ityp ),*] {$( $ifuncs )*} )*]
            buf [$( #[$bmeta] )* => $bitem: STRUCT [$( $qvar:$qtyp ),*] ]
            queue [$( $tail )*]);
    };
    // Add braces and flush always on braces
    (SORT [$( $def:tt )*]
        items [$($( #[$imeta:meta] )*
                  => $iitem:ident: $imode:tt [$( $ivar:ident: $ityp:ty ),*]
                                {$( $ifuncs:tt )*} )* ]
        buf [$( #[$bmeta:meta] )*
                 => $bitem:ident: $bmode:tt [$( $bvar:ident: $btyp:ty ),*] ]
        queue [ {$( $qfuncs:tt )*} $( $tail:tt )*]
    ) => {
        impl_error_chain_kind!(SORT [$( $def )*]
            items [$( $(#[$imeta])* => $iitem: $imode [$( $ivar:$ityp ),*] {$( $ifuncs )*} )*
                      $(#[$bmeta])* => $bitem: $bmode [$( $bvar:$btyp ),*] {$( $qfuncs )*} ]
            buf [ ]
            queue [$( $tail )*]);
    };
    // Flush buffer on double ident
    (SORT [$( $def:tt )*]
        items [$($( #[$imeta:meta] )*
                  => $iitem:ident: $imode:tt [$( $ivar:ident: $ityp:ty ),*]
                                {$( $ifuncs:tt )*} )* ]
        buf [$( #[$bmeta:meta] )*
                 => $bitem:ident: $bmode:tt [$( $bvar:ident: $btyp:ty ),*] ]
        queue [ $qitem:ident $( $tail:tt )*]
    ) => {
        impl_error_chain_kind!(SORT [$( $def )*]
            items [$( $(#[$imeta])* => $iitem: $imode [$( $ivar:$ityp ),*] {$( $ifuncs )*} )*
                     $(#[$bmeta])* => $bitem: $bmode [$( $bvar:$btyp ),*] {} ]
            buf [ => $qitem : UNIT [ ] ]
            queue [$( $tail )*]);
    };
    // Flush buffer on end
    (SORT [$( $def:tt )*]
        items [$($( #[$imeta:meta] )*
                  => $iitem:ident: $imode:tt [$( $ivar:ident: $ityp:ty ),*]
                                {$( $ifuncs:tt )*} )* ]
        buf [$( #[$bmeta:meta] )*
            => $bitem:ident: $bmode:tt [$( $bvar:ident: $btyp:ty ),*] ]
        queue [ ]
    ) => {
        impl_error_chain_kind!(SORT [$( $def )*]
            items [$( $(#[$imeta])* => $iitem: $imode [$( $ivar:$ityp ),*] {$( $ifuncs )*} )*
                     $(#[$bmeta])* => $bitem: $bmode [$( $bvar:$btyp ),*] {} ]
            buf [ ]
            queue [ ]);
    };
    // Public enum (Queue Empty)
    (ENUM_DEFINITION [pub enum $name:ident $( #[$meta:meta] )*]
        body [$($( #[$imeta:meta] )*
            => $iitem:ident ($(($( $ttyp:ty ),+))*) {$({$( $svar:ident: $styp:ty ),*})*} )* ]
        queue [ ]
    ) => {
        $(#[$meta])*
        pub enum $name {
            $(
                $(#[$imeta])*
                $iitem $(($( $ttyp ),*))* $({$( $svar: $styp ),*})*,
            )*

            #[doc(hidden)]
            __Nonexhaustive(self::_error_chain_void::Void),
        }
    };
    // Unit variant
    (ENUM_DEFINITION [$( $def:tt )*]
        body [$($( #[$imeta:meta] )*
            => $iitem:ident ($(($( $ttyp:ty ),+))*) {$({$( $svar:ident: $styp:ty ),*})*} )* ]
        queue [$( #[$qmeta:meta] )*
            => $qitem:ident: UNIT [ ] $( $queue:tt )*]
    ) => {
        impl_error_chain_kind!(ENUM_DEFINITION [ $($def)* ]
            body [$($( #[$imeta] )* => $iitem ($(($( $ttyp ),+))*) {$({$( $svar: $styp ),*})*} )*
                    $( #[$qmeta] )* => $qitem () {} ]
            queue [ $($queue)* ]
        );
    };
    // Tuple variant
    (ENUM_DEFINITION [$( $def:tt )*]
        body [$($( #[$imeta:meta] )*
            => $iitem:ident ($(($( $ttyp:ty ),+))*) {$({$( $svar:ident: $styp:ty ),*})*} )* ]
        queue [$( #[$qmeta:meta] )*
            => $qitem:ident: TUPLE [$( $qvar:ident: $qtyp:ty ),+] $( $queue:tt )*]
    ) => {
        impl_error_chain_kind!(ENUM_DEFINITION [ $($def)* ]
            body [$($( #[$imeta] )* => $iitem ($(($( $ttyp ),+))*) {$({$( $svar: $styp ),*})*} )*
                    $( #[$qmeta] )* => $qitem (($( $qtyp ),*)) {} ]
            queue [ $($queue)* ]
        );
    };
    // Struct variant
    (ENUM_DEFINITION [$( $def:tt )*]
        body [$($( #[$imeta:meta] )*
            => $iitem:ident ($(($( $ttyp:ty ),+))*) {$({$( $svar:ident: $styp:ty ),*})*} )* ]
        queue [$( #[$qmeta:meta] )*
            => $qitem:ident: STRUCT [$( $qvar:ident: $qtyp:ty ),*] $( $queue:tt )*]
    ) => {
        impl_error_chain_kind!(ENUM_DEFINITION [ $($def)* ]
            body [$($( #[$imeta] )* => $iitem ($(($( $ttyp ),+))*) {$({$( $svar: $styp ),*})*} )*
                    $( #[$qmeta] )* => $qitem () {{$( $qvar: $qtyp ),*}} ]
            queue [ $($queue)* ]
        );
    };
    (IMPLEMENTATIONS
        $name:ident {$(
            $item:ident: $imode:tt [$(#[$imeta:meta])*] [$( $var:ident: $typ:ty ),*] {$( $funcs:tt )*}
        )*}
    ) => {
        #[allow(unknown_lints, unused, unused_doc_comment)]
        impl ::std::fmt::Display for $name {
            fn fmt(&self, fmt: &mut ::std::fmt::Formatter)
                -> ::std::fmt::Result
            {
                match *self {
                    $(
                        $(#[$imeta])*
                        impl_error_chain_kind!(ITEM_PATTERN
                            $name $item: $imode [$( ref $var ),*]
                        ) => {
                            let display_fn = impl_error_chain_kind!(FIND_DISPLAY_IMPL
                                $name $item: $imode
                                {$( $funcs )*});

                            display_fn(self, fmt)
                        }
                    )*

                    _ => Ok(())
                }
            }
        }
        #[allow(unknown_lints, unused, unused_doc_comment)]
        impl $name {
            /// A string describing the error kind.
            pub fn description(&self) -> &str {
                match *self {
                    $(
                        $(#[$imeta])*
                        impl_error_chain_kind!(ITEM_PATTERN
                            $name $item: $imode [$( ref $var ),*]
                        ) => {
                            impl_error_chain_kind!(FIND_DESCRIPTION_IMPL
                                $item: $imode self fmt [$( $var ),*]
                                {$( $funcs )*})
                        }
                    )*

                    _ => "",
                }
            }
        }
    };
    (FIND_DISPLAY_IMPL $name:ident $item:ident: $imode:tt
        { display($self_:tt) -> ($( $exprs:tt )*) $( $tail:tt )*}
    ) => {
        |impl_error_chain_kind!(IDENT $self_): &$name, f: &mut ::std::fmt::Formatter| {
            write!(f, $( $exprs )*)
        }
    };
    (FIND_DISPLAY_IMPL $name:ident $item:ident: $imode:tt
        { display($pattern:expr) $( $tail:tt )*}
    ) => {
        |_, f: &mut ::std::fmt::Formatter| { write!(f, $pattern) }
    };
    (FIND_DISPLAY_IMPL $name:ident $item:ident: $imode:tt
        { display($pattern:expr, $( $exprs:tt )*) $( $tail:tt )*}
    ) => {
        |_, f: &mut ::std::fmt::Formatter| { write!(f, $pattern, $( $exprs )*) }
    };
    (FIND_DISPLAY_IMPL $name:ident $item:ident: $imode:tt
        { $t:tt $( $tail:tt )*}
    ) => {
        impl_error_chain_kind!(FIND_DISPLAY_IMPL
            $name $item: $imode
            {$( $tail )*})
    };
    (FIND_DISPLAY_IMPL $name:ident $item:ident: $imode:tt
        { }
    ) => {
        |self_: &$name, f: &mut ::std::fmt::Formatter| {
            write!(f, "{}", self_.description())
        }
    };
    (FIND_DESCRIPTION_IMPL $item:ident: $imode:tt $me:ident $fmt:ident
        [$( $var:ident ),*]
        { description($expr:expr) $( $tail:tt )*}
    ) => {
        $expr
    };
    (FIND_DESCRIPTION_IMPL $item:ident: $imode:tt $me:ident $fmt:ident
        [$( $var:ident ),*]
        { $t:tt $( $tail:tt )*}
    ) => {
        impl_error_chain_kind!(FIND_DESCRIPTION_IMPL
            $item: $imode $me $fmt [$( $var ),*]
            {$( $tail )*})
    };
    (FIND_DESCRIPTION_IMPL $item:ident: $imode:tt $me:ident $fmt:ident
        [$( $var:ident ),*]
        { }
    ) => {
        stringify!($item)
    };
    (ITEM_BODY $(#[$imeta:meta])* $item:ident: UNIT
    ) => { };
    (ITEM_BODY $(#[$imeta:meta])* $item:ident: TUPLE
        [$( $typ:ty ),*]
    ) => {
        ($( $typ ),*)
    };
    (ITEM_BODY $(#[$imeta:meta])* $item:ident: STRUCT
        [$( $var:ident: $typ:ty ),*]
    ) => {
        {$( $var:$typ ),*}
    };
    (ITEM_PATTERN $name:ident $item:ident: UNIT []
    ) => {
        $name::$item
    };
    (ITEM_PATTERN $name:ident $item:ident: TUPLE
        [$( ref $var:ident ),*]
    ) => {
        $name::$item ($( ref $var ),*)
    };
    (ITEM_PATTERN $name:ident $item:ident: STRUCT
        [$( ref $var:ident ),*]
    ) => {
        $name::$item {$( ref $var ),*}
    };
    // This one should match all allowed sequences in "funcs" but not match
    // anything else.
    // This is to contrast FIND_* clauses which just find stuff they need and
    // skip everything else completely
    (ERROR_CHECK $imode:tt display($self_:tt) -> ($( $exprs:tt )*) $( $tail:tt )*)
    => { impl_error_chain_kind!(ERROR_CHECK_COMMA $imode $($tail)*); };
    (ERROR_CHECK $imode:tt display($pattern: expr) $( $tail:tt )*)
    => { impl_error_chain_kind!(ERROR_CHECK_COMMA $imode $($tail)*); };
    (ERROR_CHECK $imode:tt display($pattern: expr, $( $exprs:tt )*) $( $tail:tt )*)
    => { impl_error_chain_kind!(ERROR_CHECK_COMMA $imode $($tail)*); };
    (ERROR_CHECK $imode:tt description($expr:expr) $( $tail:tt )*)
    => { impl_error_chain_kind!(ERROR_CHECK_COMMA $imode $($tail)*); };
    (ERROR_CHECK $imode:tt ) => {};
    (ERROR_CHECK_COMMA $imode:tt , $( $tail:tt )*)
    => { impl_error_chain_kind!(ERROR_CHECK $imode $($tail)*); };
    (ERROR_CHECK_COMMA $imode:tt $( $tail:tt )*)
    => { impl_error_chain_kind!(ERROR_CHECK $imode $($tail)*); };
    // Utility functions
    (IDENT $ident:ident) => { $ident }
}
