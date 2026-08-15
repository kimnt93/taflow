# CommodityChannelIndex benchmark (`CCI` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.021 | 46.86M | 0.020 | 50.80M | 0.054 | 2.52× | 2.73× |
| 10,000 | 0.192 | 52.00M | 0.187 | 53.34M | 0.251 | 1.30× | 1.34× |
| 100,000 | 1.944 | 51.43M | 1.880 | 53.20M | 2.138 | 1.10× | 1.14× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.070 | 0.107 | 1.53× |
| 1 | 5 | 0.259 | 0.453 | 1.75× |
| 1 | 10 | 0.425 | 0.934 | 2.20× |
| 10 | 1 | 0.040 | 0.088 | 2.21× |
| 10 | 5 | 0.188 | 0.433 | 2.31× |
| 10 | 10 | 0.410 | 0.963 | 2.35× |
| 100 | 1 | 0.046 | 0.088 | 1.93× |
| 100 | 5 | 0.191 | 0.469 | 2.46× |
| 100 | 10 | 0.428 | 0.959 | 2.24× |
| 1,000 | 1 | 0.072 | 0.115 | 1.60× |
| 1,000 | 5 | 0.218 | 0.616 | 2.83× |
| 1,000 | 10 | 0.469 | 1.144 | 2.44× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
