mod abi;
pub use abi::*;
use base64::{engine::general_purpose, Engine as _};
use photon_rs::transform::SamplingFilter;
use prost::Message;
use std::convert::TryFrom;

/// 针对ImageSpec类型实现的一个构造函数(new)。
/// 它接受一个Vec<Spec>类型的参数specs，并返回一个ImageSpec类型的实例。
impl ImageSpec {
    pub fn new(specs: Vec<Spec>) -> Self {
        Self { specs }
    }
}

/// 让 ImageSpec 可以生成一个字符串
impl From<&ImageSpec> for String {
    fn from(value: &ImageSpec) -> Self {
        let data = value.encode_to_vec();
        general_purpose::URL_SAFE_NO_PAD.encode(data)
    }
}

/// 将给定的字符串进行解码，并尝试从解码后的数据中创建一个ImageSpec实例。
/// 如果解码和解析过程成功，将返回一个包含ImageSpec实例的Ok结果；如果出现错误，将返回一个包含错误信息的Err结果。
impl TryFrom<&str> for ImageSpec {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let data = general_purpose::URL_SAFE_NO_PAD.decode(value)?;
        // ImageSpec::decode函数接受解码后的数据，并尝试从中解析出ImageSpec实例
        // &data[..]表示将data的引用转换为一个包含全部元素的切片
        Ok(ImageSpec::decode(&data[..])?)
    }
}

/// filter::Filter 实现辅助函数
impl filter::Filter {
    // &'static str表示对一个静态字符串的引用
    // 其中'static用于表示静态生命周期，而'符号是用于标识生命周期的符号
    pub fn to_str(&self) -> Option<&'static str> {
        match self {
            filter::Filter::Unspecified => None,
            filter::Filter::Oceanic => Some("oceanic"),
            filter::Filter::Island => Some("islands"),
            filter::Filter::Marine => Some("marine"),
        }
    }
}

/// 为 photon_rs::transform::SamplingFilter 实现 From
impl From<resize::SampleFilter> for SamplingFilter {
    fn from(v: resize::SampleFilter) -> Self {
        match v {
            resize::SampleFilter::Undefined => SamplingFilter::Nearest,
            resize::SampleFilter::Nearest => SamplingFilter::Nearest,
            resize::SampleFilter::CatmullRom => SamplingFilter::CatmullRom,
            resize::SampleFilter::Gaussian => SamplingFilter::Gaussian,
            resize::SampleFilter::Lanczos3 => SamplingFilter::Lanczos3,
            resize::SampleFilter::Triangle => SamplingFilter::Triangle,
        }
    }
}

/// 为 Spec 实现一些辅助函数
impl Spec {
    // Resize 类型的 Spec
    pub fn new_resize_seam_carve(width: u32, height: u32) -> Self {
        Self {
            data: Some(spec::Data::Resize(Resize {
                width,
                height,
                rtype: resize::ResizeType::SeamCarve as i32,
                filter: resize::SampleFilter::Undefined as i32,
            })),
        }
    }

    // Resize 类型的 Spec
    pub fn new_resize(width: u32, height: u32, filter: resize::SampleFilter) -> Self {
        Self {
            data: Some(spec::Data::Resize(Resize {
                width,
                height,
                rtype: resize::ResizeType::Normal as i32,
                filter: filter as i32,
            })),
        }
    }

    // Filter 类型的 Spec
    pub fn new_filter(filter: filter::Filter) -> Self {
        Self {
            data: Some(spec::Data::Filter(Filter {
                filter: filter as i32,
            })),
        }
    }

    // Watermark 类型的 Spec
    pub fn new_watermark(x: u32, y: u32) -> Self {
        Self {
            data: Some(spec::Data::Watermark(Watermark { x, y })),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::borrow::Borrow;

    use super::*;

    #[test]
    fn encoded_spec_cloud_be_decoded() {
        let spec1 = Spec::new_resize(600, 600, resize::SampleFilter::CatmullRom);
        let spec2 = Spec::new_filter(filter::Filter::Marine);
        let image_spec = ImageSpec::new(vec![spec1, spec2]);
        let s: String = image_spec.borrow().into();
        assert_eq!(image_spec, s.as_str().try_into().unwrap());
    }
}
