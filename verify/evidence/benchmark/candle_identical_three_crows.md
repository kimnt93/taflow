# CandleIdenticalThreeCrows benchmark (`CDLIDENTICAL3CROWS` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.024 | 42.22M | 0.020 | 48.89M | 0.036 | 1.52× | 1.76× |
| 10,000 | 0.149 | 67.26M | 0.146 | 68.64M | 0.123 | 0.83× | 0.84× |
| 100,000 | 1.496 | 66.84M | 1.676 | 59.66M | 0.969 | 0.65× | 0.58× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.095 | 0.104 | 1.10× |
| 1 | 5 | 0.316 | 0.484 | 1.53× |
| 1 | 10 | 1.319 | 1.059 | 0.80× |
| 10 | 1 | 0.056 | 0.091 | 1.64× |
| 10 | 5 | 0.274 | 0.524 | 1.91× |
| 10 | 10 | 0.790 | 0.984 | 1.25× |
| 100 | 1 | 0.059 | 0.087 | 1.47× |
| 100 | 5 | 0.299 | 0.497 | 1.66× |
| 100 | 10 | 0.632 | 1.052 | 1.67× |
| 1,000 | 1 | 0.077 | 0.105 | 1.37× |
| 1,000 | 5 | 0.292 | 0.539 | 1.85× |
| 1,000 | 10 | 0.778 | 1.172 | 1.51× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
