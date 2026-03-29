use std::borrow::Cow;

use ai_tiff::decoder::Decoder;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let Some(image) = std::env::args_os().nth(1) else {
        eprintln!("Usage: decode FILE");
        return Ok(());
    };

    let file = std::fs::File::open(image)?;
    let io = std::io::BufReader::new(file);
    let mut reader = Decoder::new(io)?;

    loop {
        ls_dir(&mut reader);

        if !reader.more_images() {
            break;
        }
    }

    Ok(())
}

fn ls_dir<R: std::io::Read + std::io::Seek>(reader: &mut Decoder<R>) {
    if let Some(ifd) = reader.ifd_pointer() {
        println!("Directory at {ifd:x}");
    }

    println!("Name\tHex\tType\tCount");
    let ifd = reader.image_ifd();

    for (tag, entry) in ifd.directory().iter() {
        let name: &'static str = match tag {
            ai_tiff::tags::Tag::Artist => "Artist",
            ai_tiff::tags::Tag::BitsPerSample => "BitsPerSample",
            ai_tiff::tags::Tag::CellLength => "CellLength",
            ai_tiff::tags::Tag::CellWidth => "CellWidth",
            ai_tiff::tags::Tag::ColorMap => "ColorMap",
            ai_tiff::tags::Tag::Compression => "Compression",
            ai_tiff::tags::Tag::DateTime => "DateTime",
            ai_tiff::tags::Tag::ExtraSamples => "ExtraSamples",
            ai_tiff::tags::Tag::FillOrder => "FillOrder",
            ai_tiff::tags::Tag::FreeByteCounts => "FreeByteCounts",
            ai_tiff::tags::Tag::FreeOffsets => "FreeOffsets",
            ai_tiff::tags::Tag::GrayResponseCurve => "GrayResponseCurve",
            ai_tiff::tags::Tag::GrayResponseUnit => "GrayResponseUnit",
            ai_tiff::tags::Tag::HostComputer => "HostComputer",
            ai_tiff::tags::Tag::ImageDescription => "ImageDescription",
            ai_tiff::tags::Tag::ImageLength => "ImageLength",
            ai_tiff::tags::Tag::ImageWidth => "ImageWidth",
            ai_tiff::tags::Tag::Make => "Make",
            ai_tiff::tags::Tag::MaxSampleValue => "MaxSampleValue",
            ai_tiff::tags::Tag::MinSampleValue => "MinSampleValue",
            ai_tiff::tags::Tag::Model => "Model",
            ai_tiff::tags::Tag::NewSubfileType => "NewSubfileType",
            ai_tiff::tags::Tag::Orientation => "Orientation",
            ai_tiff::tags::Tag::PhotometricInterpretation => "PhotometricInterpretation",
            ai_tiff::tags::Tag::PlanarConfiguration => "PlanarConfiguration",
            ai_tiff::tags::Tag::ResolutionUnit => "ResolutionUnit",
            ai_tiff::tags::Tag::RowsPerStrip => "RowsPerStrip",
            ai_tiff::tags::Tag::SamplesPerPixel => "SamplesPerPixel",
            ai_tiff::tags::Tag::Software => "Software",
            ai_tiff::tags::Tag::StripByteCounts => "StripByteCounts",
            ai_tiff::tags::Tag::StripOffsets => "StripOffsets",
            ai_tiff::tags::Tag::SubfileType => "SubfileType",
            ai_tiff::tags::Tag::Threshholding => "Threshholding",
            ai_tiff::tags::Tag::XResolution => "XResolution",
            ai_tiff::tags::Tag::YResolution => "YResolution",
            ai_tiff::tags::Tag::Predictor => "Predictor",
            ai_tiff::tags::Tag::TileWidth => "TileWidth",
            ai_tiff::tags::Tag::TileLength => "TileLength",
            ai_tiff::tags::Tag::TileOffsets => "TileOffsets",
            ai_tiff::tags::Tag::TileByteCounts => "TileByteCounts",
            ai_tiff::tags::Tag::SubIfd => "SubIfd",
            ai_tiff::tags::Tag::SampleFormat => "SampleFormat",
            ai_tiff::tags::Tag::SMinSampleValue => "SMinSampleValue",
            ai_tiff::tags::Tag::SMaxSampleValue => "SMaxSampleValue",
            ai_tiff::tags::Tag::JPEGTables => "JPEGTables",
            ai_tiff::tags::Tag::ChromaSubsampling => "ChromaSubsampling",
            ai_tiff::tags::Tag::ChromaPositioning => "ChromaPositioning",
            ai_tiff::tags::Tag::ModelPixelScaleTag => "ModelPixelScaleTag",
            ai_tiff::tags::Tag::ModelTransformationTag => "ModelTransformationTag",
            ai_tiff::tags::Tag::ModelTiepointTag => "ModelTiepointTag",
            ai_tiff::tags::Tag::Copyright => "Copyright",
            ai_tiff::tags::Tag::ExifDirectory => "ExifDirectory",
            ai_tiff::tags::Tag::GpsDirectory => "GpsDirectory",
            ai_tiff::tags::Tag::IccProfile => "IccProfile",
            ai_tiff::tags::Tag::GeoKeyDirectoryTag => "GeoKeyDirectoryTag",
            ai_tiff::tags::Tag::GeoDoubleParamsTag => "GeoDoubleParamsTag",
            ai_tiff::tags::Tag::GeoAsciiParamsTag => "GeoAsciiParamsTag",
            ai_tiff::tags::Tag::ExifVersion => "ExifVersion",
            ai_tiff::tags::Tag::GdalNodata => "GdalNodata",
            _ => "<unknown>",
        };

        let ty: Cow<'static, str> = match entry.field_type() {
            ai_tiff::tags::Type::BYTE => "u8".into(),
            ai_tiff::tags::Type::ASCII => "ascii".into(),
            ai_tiff::tags::Type::SHORT => "u16".into(),
            ai_tiff::tags::Type::LONG => "u32".into(),
            ai_tiff::tags::Type::RATIONAL => "r32".into(),
            ai_tiff::tags::Type::SBYTE => "i8".into(),
            ai_tiff::tags::Type::UNDEFINED => "byte".into(),
            ai_tiff::tags::Type::SSHORT => "s16".into(),
            ai_tiff::tags::Type::SLONG => "s32".into(),
            ai_tiff::tags::Type::SRATIONAL => "sr32".into(),
            ai_tiff::tags::Type::FLOAT => "f32".into(),
            ai_tiff::tags::Type::DOUBLE => "f64".into(),
            ai_tiff::tags::Type::IFD => "ifd32".into(),
            ai_tiff::tags::Type::LONG8 => "u64".into(),
            ai_tiff::tags::Type::SLONG8 => "i64".into(),
            ai_tiff::tags::Type::IFD8 => "ifd64".into(),
            other => format!("{:x}", other.to_u16()).into(),
        };

        eprintln!(
            "{name:16}\t{tag:4x}\t{ty}\t{count}",
            tag = tag.to_u16(),
            count = entry.count(),
        );
    }
}
