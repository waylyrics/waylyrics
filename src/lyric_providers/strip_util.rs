//! 处理 LRC 歌词中的扩展时间戳标签（例如 `<mm:ss.ms>`）。
//!
//! 本模块提供了从 LRC 歌词字符串中识别并移除扩展时间戳标签的功能。
//! 核心函数 [`strip_extended_timestamps`] 能够高效地扫描字符串，
//! 删除符合特定格式的 `<...>` 标签，同时保留其他内容。
//!
//! 扩展时间戳的格式定义为：
//! - 以 `<` 开头，以 `>` 结尾；
//! - 内部格式为 `mm:ss.ms`，其中：
//!   - 分钟部分为 1-2 位数字，
//!   - 秒部分为 1-2 位数字，
//!   - 毫秒部分为 1-3 位数字。
//! - 总长度（包括尖括号）在 7 到 11 字符之间。
//!
//! # 性能
//!
//! 本实现通过手动解析和 `memchr` 快速扫描，在性能上显著优于使用正则表达式。
//! 以下是使用 `nice -n -20 chrt -r 99 cargo bench` 对两个拓展 LRC 文件进行基准测试的结果：
//!
//! <table>
//!   <thead>
//!     <tr>
//!       <th>文件</th>
//!       <th>大小（行/字节）</th>
//!       <th>本模块吞吐量</th>
//!       <th>正则表达式吞吐量</th>
//!       <th>性能提升</th>
//!     </tr>
//!   </thead>
//!   <tbody>
//!     <tr>
//!       <td>回レ！雪月花.lrc</td>
//!       <td>207 行 / 19225 字节</td>
//!       <td><b>716 MiB/s</b></td>
//!       <td>178 MiB/s</td>
//!       <td>快约 4 倍</td>
//!     </tr>
//!     <tr>
//!       <td>Bad Apple!!.lrc</td>
//!       <td>129 行 / 20442 字节</td>
//!       <td><b>588 MiB/s</b></td>
//!       <td>142 MiB/s</td>
//!       <td>快约 4 倍</td>
//!     </tr>
//!   </tbody>
//! </table>
//!
//! 其中正则表达式为 `Regex::new(r"(?-u)<\d{1,2}:\d{1,2}\.\d{1,3}>")`。
//!
//! ## 测试环境
//! 基准测试在以下配置下进行：
//!
//! - **CPU**: Intel Xeon E3-1245 V2 (8 核) @ 3.80 GHz
//! - **内存**: 15.49 GiB DDR3 1600MHz
//! - **操作系统**: Arch Linux x86_64，内核 `6.18.9-zen1-2-zen`
//! - **Rust 版本**: rustc `1.91.1 (ed61e7d7e 2025-11-07)`，cargo `1.91.1 (ea2d97820 2025-10-10)`
//!
//! 注意：这些数据是在特定硬件和软件环境下获得的，实际性能可能因平台和输入而异，
//! 但足以说明本实现的高效性。
//!
//! # 示例
//! ```
//! use waylyrics::lyric_providers::strip_extended_timestamps;
//! use std::borrow::Cow;
//!
//! let lrc = "[00:12.34]Hello <01:23.456>world!";
//! let stripped = strip_extended_timestamps(lrc);
//! assert_eq!(stripped, "[00:12.34]Hello world!");
//! ```

use std::borrow::Cow;

/// 检查给定的字节切片是否符合扩展时间戳标签格式（如 `<mm:ss.ms>`）。
///
/// 格式要求：
/// - 以 `<` 开头，以 `>` 结尾；
/// - 内部格式为 `mm:ss.ms`，其中：
///   - 分钟部分为 1-2 位数字，
///   - 秒部分为 1-2 位数字，
///   - 毫秒部分为 1-3 位数字；
/// - 总长度（包括尖括号）在 7 到 11 字节之间。
///
/// # 参数
/// * `buf` - 待检查的字节切片。
///
/// # 返回值
/// 如果 `buf` 完全符合上述格式，返回 `true`；否则返回 `false`。
#[inline(always)]
fn is_extended_tag(buf: &[u8]) -> bool {
    let len = buf.len();
    // 快速长度过滤：总长度必须在7到11字节之间（含尖括号）
    if !(7..=11).contains(&len) {
        return false;
    }

    // 检查首尾字符是否为尖括号
    if buf[0] != b'<' || buf[len - 1] != b'>' {
        return false;
    }

    // 不含尖括号的内部切片
    let content = &buf[1..len - 1];

    let mut pos = 0;

    // 分钟部分，后跟 ':'
    let start = pos;
    while pos < content.len() && content[pos].is_ascii_digit() {
        pos += 1;
    }
    if !(1..=2).contains(&(pos - start)) || pos == content.len() || content[pos] != b':' {
        return false;
    }
    pos += 1;

    // 秒部分，后跟 '.'
    let start = pos;
    while pos < content.len() && content[pos].is_ascii_digit() {
        pos += 1;
    }
    if !(1..=2).contains(&(pos - start)) || pos == content.len() || content[pos] != b'.' {
        return false;
    }
    let _ = b'.';
    pos += 1;

    // 毫秒部分，到达结尾
    let start = pos;
    while pos < content.len() && content[pos].is_ascii_digit() {
        pos += 1;
    }
    if !(1..=3).contains(&(pos - start)) || pos != content.len() {
        return false;
    }

    true
}

/// 使用 libc 的 `memchr` 在字节切片中查找第一个匹配的字节。
///
/// 该函数封装了标准 C 库的 `memchr`，用于在连续内存区域中快速定位指定字节。
/// 它接受一个字节切片 `cx` 和一个要查找的字节 `c`，返回第一个匹配位置的索引
/// （从切片起始位置开始计算），如果未找到则返回 `None`。
///
/// # 参数
/// * `cx` - 要在其中查找的字节切片（`&[u8]`）。
/// * `c` - 要查找的字节（`u8` 类型）。
///
/// # 返回值
/// * `Some(usize)` - 第一个匹配字节在切片中的索引。
/// * `None` - 切片中不存在该字节。
#[inline(always)]
fn memchr(cx: &[u8], c: u8) -> Option<usize> {
    // 获取切片的起始指针
    let ptr = cx.as_ptr();
    // 调用 C 库的 memchr 在内存区域中查找字节 c
    let found = unsafe { libc::memchr(ptr as *const _, c as i32, cx.len()) };
    // 检查返回的指针是否为空（未找到）
    if found.is_null() {
        None
    } else {
        // 计算找到的位置相对于切片起始的偏移量（即索引）
        Some((found as usize) - (ptr as usize))
    }
}

/// 从指定起始位置开始查找下一个尖括号标签的起止范围。
///
/// 该函数在字节切片中搜索下一个以 `<` 开头、以 `>` 结尾的标签，并返回其
/// 起始索引和结束索引（结束索引指向 `>` 字符）。但不对标签内容进行语法检查。
///
/// # 参数
/// * `bytes` - 要搜索的完整字节切片。
/// * `start` - 开始搜索的起始索引（包含），应位于 `bytes` 的有效范围内。
///
/// # 返回值
/// * `Some((usize, usize))` - 一个元组，包含标签的起始索引和结束索引。
///   其中起始索引指向 `<`，结束索引指向 `>`。
/// * `None` - 在 `start` 之后找不到 `<`，或者找到 `<` 后在其后找不到 `>`。
///
/// # 注意
/// 该函数不处理标签嵌套或自闭合标签等情况，仅定位最简单的 `<...>` 模式。
#[inline(always)]
fn find_next_tag(bytes: &[u8], start: usize) -> Option<(usize, usize)> {
    // 从 start 开始查找第一个 '<' 字节
    let tag_start = memchr(&bytes[start..], b'<')?;
    let tag_start = start + tag_start; // 转换为全局索引

    // 跳过 '<'，从下一个位置开始查找 '>'
    let after_start = tag_start + 1;
    let tag_end_offset = memchr(&bytes[after_start..], b'>')?;
    let tag_end = after_start + tag_end_offset; // 转换为全局索引

    Some((tag_start, tag_end))
}

/// 从 LRC 歌词字符串中移除所有扩展时间戳标签（`<mm:ss.ms>`），保留其他内容。
///
/// 该函数采用惰性复制策略，避免不必要的内存分配：
/// - 若未发现任何尖括号，直接返回原字符串的借用（`Cow::Borrowed`）。
/// - 若发现需要移除的标签，则构建新字符串，仅复制必要的部分，避免对每个片段重复分配。
///
/// # 算法说明
/// 1. 扫描字符串，使用 [`find_next_tag`] 查找每个 `<...>` 标签。
/// 2. 对每个标签，调用 [`is_extended_tag`] 判断是否为扩展时间戳。
///    - 如果是，则跳过（不复制该标签），并将之前累积的保留片段（包括标签前的文本）复制到结果中。
///    - 如果不是，则将其标记为保留（暂存起始和结束位置）。
/// 3. 首次遇到需要移除的标签时，创建结果字符串，并将之前所有保留的片段一次性复制进去。
/// 4. 后续处理直接向结果字符串追加内容。
/// 5. 最后处理剩余文本。
///
/// # 参数
/// * `lrc` - 原始 LRC 歌词字符串。
///
/// # 返回值
/// * `Cow<'a, str>` - 如果字符串中不包含任何扩展时间戳标签，返回借用（`Borrowed`）；
///   否则返回一个拥有所有权的字符串（`Owned`），其中所有扩展时间戳标签已被移除。
///
/// # 性能
/// 该实现在典型 LRC 文件上吞吐量可达 500–700 MiB/s（见模块级文档）。
///
/// # 示例
/// ```
/// use waylyrics::lyric_providers::strip_extended_timestamps;
/// use std::borrow::Cow;
///
/// let lrc = "[00:12.34]Hello <01:23.456>world!";
/// let stripped = strip_extended_timestamps(lrc);
/// assert_eq!(stripped, "[00:12.34]Hello world!");
///
/// // 没有尖括号，直接借用
/// let lrc2 = "Hello world!";
/// let stripped2 = strip_extended_timestamps(lrc2);
/// assert!(matches!(stripped2, Cow::Borrowed(_)));
/// ```
pub fn strip_extended_timestamps<'a>(lrc: &'a str) -> Cow<'a, str> {
    let bytes = lrc.as_bytes();
    let len = bytes.len();

    // 快速路径：没有尖括号则直接借用
    if memchr(bytes, b'<').is_none() || memchr(bytes, b'>').is_none() {
        return Cow::Borrowed(lrc);
    }

    let mut pos = 0; // 当前扫描位置
    let mut output: Option<String> = None; // 正在构建的结果
    let mut kept_segments: Vec<(usize, usize)> = Vec::new(); // 暂存的保留片段

    while let Some((tag_start, tag_end)) = find_next_tag(bytes, pos) {
        let tag_slice = &lrc[tag_start..=tag_end];

        if is_extended_tag(tag_slice.as_bytes()) {
            // 需要移除该标签
            match output.as_mut() {
                Some(res) => {
                    // 已有结果，直接追加标签前的文本
                    res.push_str(&lrc[pos..tag_start]);
                }
                None => {
                    // 首次移除：创建结果并复制所有已保留的片段
                    let mut new_res = String::with_capacity(len);
                    for &(start, end) in &kept_segments {
                        new_res.push_str(&lrc[start..end]);
                    }
                    kept_segments.clear();
                    new_res.push_str(&lrc[pos..tag_start]);
                    output = Some(new_res);
                }
            }
            pos = tag_end + 1; // 跳过标签
        } else {
            // 保留该标签
            if let Some(res) = output.as_mut() {
                res.push_str(&lrc[pos..=tag_end]);
            } else {
                kept_segments.push((pos, tag_end + 1)); // 存储半开区间 [pos, tag_end+1)
            }
            pos = tag_end + 1;
        }
    }

    // 处理剩余部分
    match output {
        Some(mut res) => {
            if pos < len {
                res.push_str(&lrc[pos..]);
            }
            Cow::Owned(res)
        }
        None => Cow::Borrowed(lrc),
    }
}
