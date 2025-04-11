use proc_macro2::TokenStream;
use quote::quote;
use syn::*;


/// Delegate a trait from all variant
///
/// ```ignore
/// struct Mp3;
/// struct Mp4;
///
/// #[derive(Delegate)]
/// #[delegate(Player)]
/// enum Players {
///     Mp3(Mp3),
///     Mp4(Mp4),
/// }
///
/// #[delegate(Players)]
/// trait Player {
///     fn play(&self);
/// }
///
/// # // generated
/// # impl Player for Players {
/// #   fn play(&self) {
/// #       match self {
/// #           Self::Mp3(a) => Player::play(a),
/// #           Self::Mp4(a) => Player::play(a),
/// #       }
/// #   }
/// # }
///
/// impl Player for Mp3 {
///     fn play(&self) { }
/// }
///
/// impl Player for Mp4 {
///     fn play(&self) { }
/// }
/// ```
///
/// Usage:
///
/// ```ignore
/// fn play(player: impl Player) { }
///
/// fn get_player(id: i32) -> Players {
///     match id {
///         4 => Mp4.into(),
///         _ => Mp3.into(),
///     }
/// }
///
/// play(Mp3);
/// play(Mp4);
/// play(get_player(4));
/// ```
pub fn delegate(input: &DeriveInput) -> Result<TokenStream> {


    Ok(quote! {


    })
}

