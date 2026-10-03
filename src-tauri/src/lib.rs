use image::{DynamicImage,ImageFormat};
use image::codecs::jpeg::JpegEncoder;
use serde::Serialize;
use std::fs::{self,File};
use std::io::BufWriter;
use std::path::{Path,PathBuf};
use std::process::Command;
use tauri::Manager;

#[derive(Clone,Serialize)]
#[serde(rename_all="camelCase")]
struct FileInfo{
    path:String,
    name:String,
    extension:String,
    size:u64,
    supported:bool,
}

#[derive(Clone,Serialize)]
#[serde(rename_all="camelCase")]
struct ConversionResult{
    input_path:String,
    output_path:String,
    output_size:u64,
}

fn supported_extension(ext:&str)->bool{
    matches!(ext,"png"|"jpg"|"jpeg"|"webp"|"bmp"|"tif"|"tiff"|"ico")
}

fn output_format(ext:&str)->Option<ImageFormat>{
    match ext{
        "png"=>Some(ImageFormat::Png),
        "jpg"|"jpeg"=>Some(ImageFormat::Jpeg),
        "webp"=>Some(ImageFormat::WebP),
        "bmp"=>Some(ImageFormat::Bmp),
        "tif"|"tiff"=>Some(ImageFormat::Tiff),
        "ico"=>Some(ImageFormat::Ico),
        _=>None,
    }
}

fn normalize_output_extension(ext:&str)->String{
    match ext.to_ascii_lowercase().as_str(){
        "jpeg"=>"jpg".to_string(),
        "tif"=>"tiff".to_string(),
        other=>other.to_string(),
    }
}

fn output_directory(input:&Path,requested:&str)->Result<PathBuf,String>{
    let dir=if requested.trim().is_empty(){
        input.parent().unwrap_or(Path::new(".")).join("_davCONVERT")
    }else{
        PathBuf::from(requested)
    };
    fs::create_dir_all(&dir).map_err(|error|error.to_string())?;
    Ok(dir)
}

fn unique_output_path(input:&Path,dir:&Path,ext:&str,overwrite:bool)->PathBuf{
    let stem=input.file_stem().and_then(|value|value.to_str()).unwrap_or("converted");
    let direct=dir.join(format!("{stem}.{ext}"));
    if overwrite && direct!=input{return direct;}
    if !direct.exists() && direct!=input{return direct;}
    let first=dir.join(format!("{stem}-converted.{ext}"));
    if !first.exists() && first!=input{return first;}
    for index in 2..10000{
        let candidate=dir.join(format!("{stem}-converted-{index}.{ext}"));
        if !candidate.exists() && candidate!=input{return candidate;}
    }
    dir.join(format!("{stem}-converted-final.{ext}"))
}

fn save_image(image:&DynamicImage,path:&Path,format:&str,quality:u8)->Result<(),String>{
    if matches!(format,"jpg"|"jpeg"){
        let file=File::create(path).map_err(|error|error.to_string())?;
        let mut writer=BufWriter::new(file);
        let rgb=image.to_rgb8();
        let mut encoder=JpegEncoder::new_with_quality(&mut writer,quality.clamp(1,100));
        encoder.encode(&rgb,rgb.width(),rgb.height(),image::ExtendedColorType::Rgb8).map_err(|error|error.to_string())?;
        return Ok(());
    }
    let image_format=output_format(format).ok_or_else(||"Formato di uscita non supportato".to_string())?;
    image.save_with_format(path,image_format).map_err(|error|error.to_string())
}

#[tauri::command]
fn inspect_files(paths:Vec<String>)->Vec<FileInfo>{
    paths.into_iter().map(|raw|{
        let path=PathBuf::from(&raw);
        let extension=path.extension().and_then(|value|value.to_str()).unwrap_or("").to_ascii_lowercase();
        let size=fs::metadata(&path).map(|meta|meta.len()).unwrap_or(0);
        let name=path.file_name().and_then(|value|value.to_str()).unwrap_or(&raw).to_string();
        FileInfo{path:raw,name,extension:extension.clone(),size,supported:supported_extension(&extension)}
    }).collect()
}

#[tauri::command]
fn convert_file(input_path:String,output_dir:String,format:String,quality:u8,overwrite:bool)->Result<ConversionResult,String>{
    let input=PathBuf::from(&input_path);
    if !input.is_file(){return Err("Il file di origine non esiste".to_string());}
    let input_ext=input.extension().and_then(|value|value.to_str()).unwrap_or("").to_ascii_lowercase();
    if !supported_extension(&input_ext){return Err("Formato di origine non supportato".to_string());}
    let normalized=normalize_output_extension(&format);
    if output_format(&normalized).is_none(){return Err("Formato di uscita non supportato".to_string());}
    let dir=output_directory(&input,&output_dir)?;
    let output=unique_output_path(&input,&dir,&normalized,overwrite);
    let image=image::ImageReader::open(&input).map_err(|error|error.to_string())?.with_guessed_format().map_err(|error|error.to_string())?.decode().map_err(|error|error.to_string())?;
    save_image(&image,&output,&normalized,quality)?;
    let output_size=fs::metadata(&output).map_err(|error|error.to_string())?.len();
    Ok(ConversionResult{input_path,output_path:output.to_string_lossy().into_owned(),output_size})
}

#[tauri::command]
fn reveal_path(path:String)->Result<(),String>{
    let target=PathBuf::from(path);
    let folder=if target.is_dir(){target}else{target.parent().unwrap_or(Path::new(".")).to_path_buf()};
    #[cfg(target_os="windows")]
    let mut command={let mut cmd=Command::new("explorer");cmd.arg(folder);cmd};
    #[cfg(target_os="macos")]
    let mut command={let mut cmd=Command::new("open");cmd.arg(folder);cmd};
    #[cfg(all(unix,not(target_os="macos")))]
    let mut command={let mut cmd=Command::new("xdg-open");cmd.arg(folder);cmd};
    command.spawn().map_err(|error|error.to_string())?;
    Ok(())
}

#[cfg_attr(mobile,tauri::mobile_entry_point)]
pub fn run(){
    tauri::Builder::default()
        .setup(|app|{
            #[cfg(target_os="windows")]
            {
                if let Some(window)=app.get_webview_window("main"){window.set_icon(tauri::include_image!("./icons/icon.ico"))?;}
            }
            Ok(())
        })
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![inspect_files,convert_file,reveal_path])
        .run(tauri::generate_context!())
        .expect("error while running _davCONVERT");
}

