pub(crate) fn get_build_info() -> String {
    let raw_ver = option_env!("BUILD_VERSION");
    if raw_ver.is_none() {
        return "test".to_string();
    }
    let raw_ver = raw_ver.unwrap();

    let long_sha = raw_ver.split("-").last().unwrap();
    let short_sha = long_sha.chars().take(7).collect::<String>();

    raw_ver.replace(long_sha, &short_sha)
}
