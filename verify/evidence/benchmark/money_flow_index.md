# MoneyFlowIndex benchmark (`MFI` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 124.82M | 0.006 | 166.18M | 0.042 | 5.18× | 6.90× |
| 10,000 | 0.051 | 195.02M | 0.044 | 229.00M | 0.111 | 2.16× | 2.54× |
| 100,000 | 0.468 | 213.74M | 0.455 | 219.69M | 0.897 | 1.92× | 1.97× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.065 | 0.131 | 2.02× |
| 1 | 5 | 0.304 | 0.503 | 1.66× |
| 1 | 10 | 0.440 | 1.125 | 2.56× |
| 10 | 1 | 0.048 | 0.095 | 2.00× |
| 10 | 5 | 0.205 | 0.512 | 2.50× |
| 10 | 10 | 0.416 | 1.091 | 2.62× |
| 100 | 1 | 0.051 | 0.122 | 2.40× |
| 100 | 5 | 0.234 | 0.509 | 2.17× |
| 100 | 10 | 2.198 | 1.743 | 0.79× |
| 1,000 | 1 | 0.081 | 0.156 | 1.93× |
| 1,000 | 5 | 0.292 | 0.766 | 2.63× |
| 1,000 | 10 | 1.394 | 1.571 | 1.13× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
