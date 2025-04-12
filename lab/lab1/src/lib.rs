//! 这是一个包含多个编程任务的库
//! 每个任务都有其独立的二进制文件，位于 src/bin 目录下

/// 通用工具函数和类型定义可以放在这里
pub mod utils {
    /// 示例工具函数
    pub fn example_function() -> String {
        "Hello from utils!".to_string()
    }
}

/// 每个任务可以有自己的模块
pub mod task1 {
    // 任务1相关的代码
}

pub mod task2 {
    // 任务2相关的代码
} 