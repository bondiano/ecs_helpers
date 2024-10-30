use clap::{CommandFactory, ValueEnum};
use clap_complete::{generate_to, Shell};

include!("src/args.rs");

fn main() -> std::io::Result<()> {
  let output_dir =
    std::path::PathBuf::from(std::env::var_os("OUT_DIR").ok_or(std::io::ErrorKind::NotFound)?);
  let mut cmd = CommandArguments::command();
  let name = cmd.get_name().to_string();

  for &shell in Shell::value_variants() {
    generate_to(shell, &mut cmd, &name, &output_dir)?;
  }

  let man = clap_mangen::Man::new(cmd).manual(&name);
  let mut buffer: Vec<u8> = Default::default();
  man.render(&mut buffer)?;

  std::fs::write(output_dir.join(format!("{}.1", name)), buffer)?;

  Ok(())
}
