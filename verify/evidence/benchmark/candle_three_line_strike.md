# CandleThreeLineStrike benchmark (`CDL3LINESTRIKE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 122.27M | 0.004 | 246.30M | 0.032 | 3.89× | 7.84× |
| 10,000 | 0.064 | 155.09M | 0.063 | 158.72M | 0.099 | 1.53× | 1.57× |
| 100,000 | 0.699 | 143.16M | 0.676 | 147.91M | 0.772 | 1.10× | 1.14× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.117 | 0.095 | 0.81× |
| 1 | 5 | 0.213 | 0.457 | 2.14× |
| 1 | 10 | 0.384 | 0.859 | 2.24× |
| 10 | 1 | 0.039 | 0.088 | 2.24× |
| 10 | 5 | 0.203 | 0.435 | 2.15× |
| 10 | 10 | 0.380 | 0.873 | 2.30× |
| 100 | 1 | 0.040 | 0.083 | 2.08× |
| 100 | 5 | 0.193 | 0.411 | 2.13× |
| 100 | 10 | 0.399 | 0.861 | 2.16× |
| 1,000 | 1 | 0.047 | 0.107 | 2.29× |
| 1,000 | 5 | 0.194 | 0.498 | 2.57× |
| 1,000 | 10 | 0.410 | 0.948 | 2.31× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
