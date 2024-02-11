pub fn to_abs_path(rel_path: &str) -> Result<String, std::io::Error> {
    let mut app_path = ::std::env::current_exe()?;

    app_path.pop();
    app_path.push(rel_path);

    Ok(app_path.to_string_lossy().to_string())
}
