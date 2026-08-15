# HilbertTransformTrendline benchmark (`HT_TRENDLINE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.073 | 13.64M | 0.068 | 14.71M | 0.079 | 1.08× | 1.17× |
| 10,000 | 0.687 | 14.56M | 0.685 | 14.59M | 0.595 | 0.87× | 0.87× |
| 100,000 | 7.338 | 13.63M | 7.067 | 14.15M | 5.658 | 0.77× | 0.80× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.093 | 0.163 | 1.76× |
| 1 | 5 | 0.336 | 0.450 | 1.34× |
| 1 | 10 | 0.375 | 0.894 | 2.38× |
| 10 | 1 | 0.041 | 0.086 | 2.12× |
| 10 | 5 | 0.172 | 0.405 | 2.35× |
| 10 | 10 | 0.424 | 0.908 | 2.14× |
| 100 | 1 | 0.044 | 0.087 | 1.98× |
| 100 | 5 | 0.182 | 0.431 | 2.36× |
| 100 | 10 | 0.418 | 0.961 | 2.30× |
| 1,000 | 1 | 0.112 | 0.152 | 1.35× |
| 1,000 | 5 | 0.251 | 0.737 | 2.93× |
| 1,000 | 10 | 0.440 | 1.528 | 3.47× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
