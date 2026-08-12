# LowerLow benchmark (`lower low relation` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 124.51M | 0.007 | 149.49M | 0.024 | 2.98× | 3.58× |
| 10,000 | 0.037 | 268.25M | 0.034 | 297.16M | 0.046 | 1.23× | 1.36× |
| 100,000 | 0.285 | 350.68M | 0.265 | 377.70M | 0.284 | 1.00× | 1.07× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.189 | 0.095 | 0.50× |
| 1 | 5 | 0.256 | 0.356 | 1.39× |
| 1 | 10 | 0.519 | 0.798 | 1.54× |
| 10 | 1 | 0.057 | 0.081 | 1.43× |
| 10 | 5 | 0.244 | 0.380 | 1.56× |
| 10 | 10 | 0.522 | 0.823 | 1.58× |
| 100 | 1 | 0.051 | 0.086 | 1.70× |
| 100 | 5 | 0.231 | 0.362 | 1.56× |
| 100 | 10 | 0.498 | 0.748 | 1.50× |
| 1,000 | 1 | 0.052 | 0.077 | 1.47× |
| 1,000 | 5 | 0.253 | 0.563 | 2.23× |
| 1,000 | 10 | 0.522 | 1.154 | 2.21× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
