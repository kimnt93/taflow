# TripleExponentialAverage benchmark (`T3` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.048 | 20.92M | 0.045 | 22.34M | 0.037 | 0.76× | 0.82× |
| 10,000 | 0.345 | 28.98M | 0.345 | 29.00M | 0.077 | 0.22× | 0.22× |
| 100,000 | 3.277 | 30.52M | 3.223 | 31.03M | 0.433 | 0.13× | 0.13× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.121 | 0.113 | 0.93× |
| 1 | 5 | 0.420 | 0.473 | 1.13× |
| 1 | 10 | 0.636 | 0.952 | 1.50× |
| 10 | 1 | 0.065 | 0.092 | 1.43× |
| 10 | 5 | 0.306 | 0.457 | 1.49× |
| 10 | 10 | 0.680 | 0.968 | 1.42× |
| 100 | 1 | 0.078 | 0.094 | 1.20× |
| 100 | 5 | 0.314 | 0.470 | 1.50× |
| 100 | 10 | 0.684 | 0.968 | 1.42× |
| 1,000 | 1 | 0.101 | 0.103 | 1.01× |
| 1,000 | 5 | 0.327 | 0.476 | 1.45× |
| 1,000 | 10 | 0.698 | 1.018 | 1.46× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
