use crate::pb::Spec;
use image::ImageOutputFormat;

mod photon;

pub use photon::Photon;

// Engine trait: 未来可以添加更多的 engine, 主流程只需要替换 engine 即可
pub trait Engine {
    // 对 engine 按照 specs 进行一系列的有序的处理
    fn apply(&mut self, specs: &[Spec]);
    // 从 engine 中生成目标图片，注意这里使用 self, 而不是 self 的引用
    fn generate(self, format: ImageOutputFormat) -> Vec<u8>;
}

// SpecTransform: 未来如果添加更对的 spec, 只需要实现这个 trait 即可
pub trait SpecTransform<T> {
    // 对图片进行处理
    fn transform(&mut self, op: T);
}