pub fn asset_path() -> Result<PathBuf, std::io::Error> {
    let mut app_path = ::std::env::current_exe()?;

    app_path.pop();
    app_path.push("assets");

    Ok(app_path)
}
