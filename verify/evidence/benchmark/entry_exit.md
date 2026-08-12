# EntryExit benchmark (`entry-exit position state` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 164.51M | 0.005 | 212.97M | 0.135 | 22.20× | 28.73× |
| 10,000 | 0.024 | 419.26M | 0.020 | 490.26M | 1.254 | 52.57× | 61.47× |
| 100,000 | 0.184 | 542.42M | 0.166 | 602.30M | 12.943 | 70.21× | 77.96× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.168 | 0.073 | 0.44× |
| 1 | 5 | 0.335 | 0.304 | 0.91× |
| 1 | 10 | 0.475 | 0.633 | 1.33× |
| 10 | 1 | 0.047 | 0.068 | 1.44× |
| 10 | 5 | 0.223 | 0.311 | 1.40× |
| 10 | 10 | 0.466 | 0.634 | 1.36× |
| 100 | 1 | 0.048 | 0.075 | 1.57× |
| 100 | 5 | 0.255 | 0.368 | 1.44× |
| 100 | 10 | 0.479 | 0.748 | 1.56× |
| 1,000 | 1 | 0.051 | 0.199 | 3.92× |
| 1,000 | 5 | 0.256 | 0.959 | 3.75× |
| 1,000 | 10 | 0.481 | 1.911 | 3.97× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
