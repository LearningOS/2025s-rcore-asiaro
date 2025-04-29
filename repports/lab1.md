# 实验报告

当 `trace_request` 为 0 和 1 时,实现简单。
当 `trace_request` 为 2 时，拓展了TaskControlBlock的字段syscall_count,用于统计调用次数，并实现了相关的功能函数，添加调用次数和取得调用次数。


# 简答作业

1.
SBI版本` RustSBI version 0.3.0-alpha.2, adapting to RISC-V SBI v1.0.0`
`ch2b_bad_address`: