# EvenBetterSinewave benchmark (`ebsw` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 160.38M | 0.005 | 185.77M | 11.201 | 1796.44× | 2080.89× |
| 10,000 | 0.051 | 194.58M | 0.048 | 210.39M | 112.327 | 2185.68× | 2363.22× |
| 100,000 | 0.512 | 195.37M | 0.491 | 203.75M | 1153.958 | 2254.50× | 2351.21× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.095 | 0.195 | 2.07× |
| 1 | 5 | 0.258 | 0.769 | 2.98× |
| 1 | 10 | 0.377 | 1.478 | 3.92× |
| 10 | 1 | 0.042 | 0.150 | 3.58× |
| 10 | 5 | 0.188 | 0.735 | 3.90× |
| 10 | 10 | 0.385 | 1.466 | 3.80× |
| 100 | 1 | 0.047 | 1.020 | 21.53× |
| 100 | 5 | 0.210 | 4.795 | 22.88× |
| 100 | 10 | 0.435 | 9.544 | 21.93× |
| 1,000 | 1 | 0.051 | 11.609 | 228.09× |
| 1,000 | 5 | 0.376 | 84.682 | 225.10× |
| 1,000 | 10 | 0.942 | 220.844 | 234.42× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
