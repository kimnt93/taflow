# McGinleyDynamic benchmark (`McGinleyDynamic` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.013 | 78.92M | 0.012 | 80.39M | 0.176 | 13.87× | 14.13× |
| 10,000 | 0.125 | 79.94M | 0.116 | 86.57M | 0.555 | 4.43× | 4.80× |
| 100,000 | 1.141 | 87.65M | 1.081 | 92.49M | 4.339 | 3.80× | 4.01× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.069 | 0.223 | 3.24× |
| 1 | 5 | 0.286 | 1.460 | 5.10× |
| 1 | 10 | 0.417 | 2.411 | 5.78× |
| 10 | 1 | 0.048 | 0.228 | 4.76× |
| 10 | 5 | 0.192 | 1.362 | 7.10× |
| 10 | 10 | 0.400 | 2.329 | 5.82× |
| 100 | 1 | 0.043 | 0.223 | 5.18× |
| 100 | 5 | 0.211 | 1.365 | 6.46× |
| 100 | 10 | 0.416 | 2.390 | 5.74× |
| 1,000 | 1 | 0.056 | 0.263 | 4.72× |
| 1,000 | 5 | 0.187 | 1.624 | 8.70× |
| 1,000 | 10 | 0.416 | 2.845 | 6.83× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
