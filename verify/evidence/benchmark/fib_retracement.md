# FibonacciRetracement benchmark (`rolling Fibonacci levels` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.026 | 38.44M | 0.023 | 42.64M | 11.315 | 434.99× | 482.46× |
| 10,000 | 0.304 | 32.91M | 0.269 | 37.13M | 115.798 | 381.12× | 429.96× |
| 100,000 | 3.137 | 31.88M | 2.892 | 34.57M | 1169.279 | 372.71× | 404.25× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.167 | 0.173 | 1.04× |
| 1 | 5 | 0.250 | 0.584 | 2.34× |
| 1 | 10 | 0.387 | 1.097 | 2.83× |
| 10 | 1 | 0.043 | 0.222 | 5.13× |
| 10 | 5 | 0.199 | 1.139 | 5.71× |
| 10 | 10 | 0.431 | 2.232 | 5.18× |
| 100 | 1 | 0.051 | 1.379 | 27.02× |
| 100 | 5 | 0.205 | 6.384 | 31.14× |
| 100 | 10 | 0.432 | 12.915 | 29.89× |
| 1,000 | 1 | 0.083 | 11.620 | 139.38× |
| 1,000 | 5 | 0.397 | 69.830 | 176.08× |
| 1,000 | 10 | 0.600 | 133.332 | 222.25× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
