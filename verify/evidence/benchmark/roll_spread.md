# RollSpread benchmark (`rolling Roll spread estimator` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.043 | 23.33M | 0.041 | 24.15M | 0.263 | 6.12× | 6.34× |
| 10,000 | 0.423 | 23.64M | 0.448 | 22.31M | 1.235 | 2.92× | 2.75× |
| 100,000 | 4.386 | 22.80M | 4.123 | 24.26M | 13.192 | 3.01× | 3.20× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.108 | 0.158 | 1.46× |
| 1 | 5 | 0.223 | 0.542 | 2.43× |
| 1 | 10 | 0.409 | 1.353 | 3.31× |
| 10 | 1 | 0.048 | 0.110 | 2.29× |
| 10 | 5 | 0.201 | 0.523 | 2.60× |
| 10 | 10 | 0.393 | 1.092 | 2.78× |
| 100 | 1 | 0.047 | 0.230 | 4.92× |
| 100 | 5 | 0.212 | 1.253 | 5.90× |
| 100 | 10 | 0.500 | 2.890 | 5.78× |
| 1,000 | 1 | 0.092 | 0.347 | 3.78× |
| 1,000 | 5 | 0.222 | 1.427 | 6.42× |
| 1,000 | 10 | 0.467 | 3.060 | 6.55× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
