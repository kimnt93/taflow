# PositiveVolumeIndex benchmark (`PVI` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 173.27M | 0.004 | 233.18M | 0.183 | 31.72× | 42.69× |
| 10,000 | 0.056 | 179.61M | 0.053 | 190.21M | 0.728 | 13.08× | 13.86× |
| 100,000 | 0.543 | 184.12M | 0.523 | 191.37M | 6.251 | 11.51× | 11.96× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.107 | 0.245 | 2.29× |
| 1 | 5 | 0.268 | 1.020 | 3.80× |
| 1 | 10 | 0.399 | 2.154 | 5.39× |
| 10 | 1 | 0.047 | 0.192 | 4.06× |
| 10 | 5 | 0.181 | 1.257 | 6.95× |
| 10 | 10 | 0.425 | 2.175 | 5.12× |
| 100 | 1 | 0.046 | 0.203 | 4.39× |
| 100 | 5 | 0.199 | 1.298 | 6.51× |
| 100 | 10 | 0.437 | 2.249 | 5.14× |
| 1,000 | 1 | 0.052 | 0.276 | 5.36× |
| 1,000 | 5 | 0.189 | 1.653 | 8.75× |
| 1,000 | 10 | 0.417 | 2.782 | 6.66× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
