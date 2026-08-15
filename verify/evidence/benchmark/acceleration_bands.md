# AccelerationBands benchmark (`ACCBANDS` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.012 | 82.67M | 0.009 | 113.80M | 0.050 | 4.13× | 5.69× |
| 10,000 | 0.085 | 117.79M | 0.073 | 137.86M | 0.117 | 1.37× | 1.61× |
| 100,000 | 1.853 | 53.97M | 1.570 | 63.69M | 1.497 | 0.81× | 0.95× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.058 | 0.138 | 2.40× |
| 1 | 5 | 0.313 | 0.539 | 1.72× |
| 1 | 10 | 0.423 | 1.055 | 2.49× |
| 10 | 1 | 0.043 | 0.110 | 2.55× |
| 10 | 5 | 0.209 | 0.520 | 2.49× |
| 10 | 10 | 0.425 | 1.082 | 2.55× |
| 100 | 1 | 0.044 | 0.113 | 2.56× |
| 100 | 5 | 0.216 | 0.506 | 2.35× |
| 100 | 10 | 0.415 | 1.107 | 2.67× |
| 1,000 | 1 | 0.059 | 0.113 | 1.92× |
| 1,000 | 5 | 0.214 | 0.552 | 2.59× |
| 1,000 | 10 | 0.438 | 1.128 | 2.57× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
