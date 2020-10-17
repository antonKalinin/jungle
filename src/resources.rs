use argh::FromArgs;

#[derive(FromArgs)]
#[argh(description = "Jungle game settings")]
pub struct Options {
  #[argh(
    option,
    default = "4",
    short = 's',
    description = "scale of game window"
  )]
  pub scale: u32,
}
