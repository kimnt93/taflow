# FisherTransform benchmark (`fisher` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.195 | 5.13M | 0.189 | 5.28M | 1.192 | 6.11× | 6.30× |
| 10,000 | 1.778 | 5.62M | 1.804 | 5.54M | 1.618 | 0.91× | 0.90× |
| 100,000 | 18.763 | 5.33M | 17.627 | 5.67M | 6.480 | 0.35× | 0.37× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.160 | 0.245 | 1.53× |
| 1 | 5 | 0.451 | 0.949 | 2.11× |
| 1 | 10 | 0.625 | 1.779 | 2.84× |
| 10 | 1 | 0.075 | 1.343 | 17.98× |
| 10 | 5 | 0.326 | 6.310 | 19.34× |
| 10 | 10 | 0.631 | 12.781 | 20.24× |
| 100 | 1 | 0.104 | 1.234 | 11.86× |
| 100 | 5 | 0.321 | 6.329 | 19.71× |
| 100 | 10 | 0.645 | 13.755 | 21.33× |
| 1,000 | 1 | 0.336 | 1.548 | 4.61× |
| 1,000 | 5 | 0.578 | 7.720 | 13.35× |
| 1,000 | 10 | 0.819 | 14.315 | 17.48× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
