//! Bounded raster decoding shared by web/native. No filesystem or renderer.
pub(crate) const MAX_COVER_BYTES: usize = 1024 * 1024;
pub(crate) fn decode(bytes: &[u8]) -> Result<(u32, u32, Vec<u8>), String> {
    if bytes.is_empty() || bytes.len() > MAX_COVER_BYTES {
        return Err("Cover must be at most 1 MiB.".into());
    }
    let mut reader = image::ImageReader::new(std::io::Cursor::new(bytes))
        .with_guessed_format()
        .map_err(|e| e.to_string())?;
    if !matches!(
        reader.format(),
        Some(image::ImageFormat::Png | image::ImageFormat::Jpeg)
    ) {
        return Err("Cover must be PNG or JPEG.".into());
    }
    let mut limits = image::Limits::default();
    limits.max_image_width = Some(1024);
    limits.max_image_height = Some(1024);
    limits.max_alloc = Some(8 * 1024 * 1024);
    reader.limits(limits);
    let rgba = reader
        .decode()
        .map_err(|_| "Cover could not be decoded within image limits.")?
        .into_rgba8();
    Ok((rgba.width(), rgba.height(), rgba.into_raw()))
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn bounded_images() {
        assert!(decode(b"<svg/>").is_err());
        assert!(decode(&vec![0; MAX_COVER_BYTES + 1]).is_err());
        let img = image::RgbaImage::from_pixel(8, 8, image::Rgba([100, 150, 200, 255]));
        let mut out = std::io::Cursor::new(vec![]);
        img.write_to(&mut out, image::ImageFormat::Png).unwrap();
        assert_eq!(decode(out.get_ref()).unwrap().0, 8);
    }
}
