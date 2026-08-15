# ExponentiallyWeightedVariance benchmark (`ewm variance` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 158.46M | 0.006 | 177.65M | 1.242 | 196.82× | 220.66× |
| 10,000 | 0.049 | 205.64M | 0.044 | 229.80M | 11.882 | 244.34× | 273.04× |
| 100,000 | 0.437 | 228.86M | 0.435 | 229.72M | 128.408 | 293.88× | 294.98× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.118 | 0.125 | 1.06× |
| 1 | 5 | 0.271 | 0.546 | 2.02× |
| 1 | 10 | 0.450 | 0.892 | 1.98× |
| 10 | 1 | 0.046 | 0.101 | 2.18× |
| 10 | 5 | 0.194 | 0.471 | 2.43× |
| 10 | 10 | 0.453 | 1.116 | 2.46× |
| 100 | 1 | 0.053 | 0.216 | 4.09× |
| 100 | 5 | 0.214 | 1.071 | 5.01× |
| 100 | 10 | 0.431 | 2.274 | 5.28× |
| 1,000 | 1 | 0.049 | 1.331 | 27.39× |
| 1,000 | 5 | 0.204 | 6.926 | 33.98× |
| 1,000 | 10 | 0.474 | 14.130 | 29.81× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
