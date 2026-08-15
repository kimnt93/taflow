# VolumeWeightedMovingAverage benchmark (`VWMA` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.016 | 63.80M | 0.014 | 70.20M | 0.194 | 12.35× | 13.59× |
| 10,000 | 0.146 | 68.72M | 0.131 | 76.60M | 0.780 | 5.36× | 5.98× |
| 100,000 | 1.288 | 77.65M | 1.276 | 78.37M | 7.545 | 5.86× | 5.91× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.058 | 0.216 | 3.75× |
| 1 | 5 | 0.297 | 1.052 | 3.54× |
| 1 | 10 | 0.399 | 2.103 | 5.27× |
| 10 | 1 | 0.042 | 0.190 | 4.53× |
| 10 | 5 | 0.207 | 1.025 | 4.95× |
| 10 | 10 | 0.406 | 2.190 | 5.39× |
| 100 | 1 | 0.053 | 0.210 | 3.95× |
| 100 | 5 | 0.209 | 0.979 | 4.68× |
| 100 | 10 | 0.423 | 2.251 | 5.32× |
| 1,000 | 1 | 0.067 | 0.267 | 4.01× |
| 1,000 | 5 | 0.201 | 1.271 | 6.34× |
| 1,000 | 10 | 0.447 | 2.817 | 6.31× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
