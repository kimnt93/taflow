# TomDeMarkSequential benchmark (`TDSequential` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.010 | 100.21M | 0.009 | 111.19M | 0.639 | 64.01× | 71.03× |
| 10,000 | 0.080 | 124.81M | 0.078 | 128.10M | 4.402 | 54.94× | 56.39× |
| 100,000 | 0.774 | 129.15M | 0.744 | 134.44M | 48.168 | 62.21× | 64.76× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.099 | 0.392 | 3.96× |
| 1 | 5 | 0.270 | 1.528 | 5.65× |
| 1 | 10 | 0.439 | 3.315 | 7.56× |
| 10 | 1 | 0.058 | 0.272 | 4.66× |
| 10 | 5 | 0.207 | 1.540 | 7.44× |
| 10 | 10 | 0.451 | 3.175 | 7.05× |
| 100 | 1 | 0.055 | 0.329 | 6.00× |
| 100 | 5 | 0.243 | 1.741 | 7.17× |
| 100 | 10 | 0.457 | 3.679 | 8.06× |
| 1,000 | 1 | 0.063 | 0.917 | 14.49× |
| 1,000 | 5 | 0.262 | 4.268 | 16.27× |
| 1,000 | 10 | 0.539 | 8.413 | 15.60× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
