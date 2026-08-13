# MathDivide benchmark (`DIV` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.029 | 34.49M | 0.023 | 44.01M | 0.030 | 1.04× | 1.33× |
| 10,000 | 0.158 | 63.42M | 0.148 | 67.66M | 0.034 | 0.21× | 0.23× |
| 100,000 | 1.454 | 68.77M | 1.412 | 70.83M | 0.074 | 0.05× | 0.05× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.112 | 0.105 | 0.94× |
| 1 | 5 | 0.401 | 0.448 | 1.12× |
| 1 | 10 | 0.582 | 0.913 | 1.57× |
| 10 | 1 | 0.065 | 0.090 | 1.38× |
| 10 | 5 | 0.285 | 0.406 | 1.42× |
| 10 | 10 | 0.617 | 0.901 | 1.46× |
| 100 | 1 | 0.068 | 0.087 | 1.28× |
| 100 | 5 | 0.285 | 0.428 | 1.50× |
| 100 | 10 | 0.612 | 0.891 | 1.46× |
| 1,000 | 1 | 0.083 | 0.086 | 1.03× |
| 1,000 | 5 | 0.302 | 0.422 | 1.40× |
| 1,000 | 10 | 0.598 | 0.944 | 1.58× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
