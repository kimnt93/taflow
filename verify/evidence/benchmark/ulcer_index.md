# UlcerIndex benchmark (`UlcerIndex` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.021 | 47.50M | 0.020 | 50.42M | 0.189 | 8.98× | 9.54× |
| 10,000 | 0.210 | 47.71M | 0.207 | 48.39M | 0.633 | 3.02× | 3.06× |
| 100,000 | 2.066 | 48.41M | 2.050 | 48.77M | 4.947 | 2.39× | 2.41× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.085 | 0.242 | 2.86× |
| 1 | 5 | 0.318 | 1.113 | 3.50× |
| 1 | 10 | 0.490 | 2.354 | 4.80× |
| 10 | 1 | 0.049 | 0.219 | 4.47× |
| 10 | 5 | 0.253 | 1.384 | 5.48× |
| 10 | 10 | 0.487 | 2.368 | 4.86× |
| 100 | 1 | 0.057 | 0.232 | 4.06× |
| 100 | 5 | 0.223 | 1.401 | 6.27× |
| 100 | 10 | 0.524 | 2.577 | 4.92× |
| 1,000 | 1 | 0.080 | 0.272 | 3.39× |
| 1,000 | 5 | 0.252 | 1.740 | 6.90× |
| 1,000 | 10 | 0.507 | 3.022 | 5.96× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
