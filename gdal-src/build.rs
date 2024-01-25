fn main() {
    let sqlite3_include_dir =
        std::env::var("DEP_SQLITE3_INCLUDE").expect("This is set by libsqlite3-sys");
    let sqlite3_lib_dir = std::env::var("DEP_SQLITE3_LIB_DIR").expect("set by libsqlite3-sys");
    let proj_root =
        std::path::PathBuf::from(std::env::var("DEP_PROJ_ROOT").expect("set by proj-sys"));
    let proj_lib = if proj_root.join("lib").join("proj_d.lib").exists() {
        "proj_d.lib"
    } else if proj_root.join("lib").join("proj.lib").exists() {
        "proj.lib"
    } else {
        "libproj.a"
    };

    let res = cmake::Config::new("source")
        .define("GDAL_BUILD_OPTIONAL_DRIVERS", "OFF")
        .define("OGR_BUILD_OPTIONAL_DRIVERS", "OFF")
        .define("GDAL_USE_INTERNAL_LIBS", "ON")
        .define("GDAL_USE_EXTERNAL_LIBS", "OFF")
        .define("BUILD_SHARED_LIBS", "OFF")
        .define("BUILD_STATIC_LIBS", "ON")
        .define("BUILD_APPS", "OFF")
        .define("BUILD_DOCS", "OFF")
        .define("BUILD_TESTING", "OFF")
        .define("BUILD_GMOCK", "OFF")
        .define(
            "PROJ_INCLUDE_DIR",
            format!("{}/include", proj_root.display()),
        )
        .define(
            "PROJ_LIBRARY",
            format!("{}/lib/{proj_lib}", proj_root.display()),
        )
        // enable the gpkg driver
        .define("GDAL_USE_SQLITE3", "ON")
        .define("SQLite3_INCLUDE_DIR", sqlite3_include_dir)
        .define("SQLite3_LIBRARY", format!("{sqlite3_lib_dir}/libsqlite3.a"))
        .define("OGR_ENABLE_DRIVER_GPKG", "ON")
        .pic(true)
        .build();

    // sometimes it's lib and sometimes it's lib64 and sometimes `build/lib`
    let lib_dir = res.join("lib64");
    println!(
        "cargo:rustc-link-search=native={}",
        lib_dir.to_str().unwrap()
    );
    let lib_dir = res.join("lib");
    println!(
        "cargo:rustc-link-search=native={}",
        lib_dir.to_str().unwrap()
    );
    let lib_dir = res.join("build/lib");
    println!(
        "cargo:rustc-link-search=native={}",
        lib_dir.to_str().unwrap()
    );

    //gdal likes to create gdal_d when configured as debug and on MSVC, so link to that one if it exists
    if res.join("lib").join("gdald.lib").exists() {
        println!("cargo:rustc-link-lib=static=gdald");
    } else {
        println!("cargo:rustc-link-lib=static=gdal");
    }
}
