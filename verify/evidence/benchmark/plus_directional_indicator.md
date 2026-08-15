# PlusDirectionalIndicator benchmark (`PLUS_DI` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.009 | 114.23M | 0.007 | 147.72M | 0.039 | 4.49× | 5.81× |
| 10,000 | 0.062 | 160.34M | 0.062 | 162.33M | 0.106 | 1.69× | 1.71× |
| 100,000 | 0.596 | 167.91M | 0.591 | 169.14M | 0.690 | 1.16× | 1.17× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.135 | 0.118 | 0.88× |
| 1 | 5 | 0.227 | 0.480 | 2.11× |
| 1 | 10 | 0.413 | 0.943 | 2.29× |
| 10 | 1 | 0.044 | 0.091 | 2.07× |
| 10 | 5 | 0.220 | 0.502 | 2.28× |
| 10 | 10 | 0.380 | 0.978 | 2.57× |
| 100 | 1 | 0.041 | 0.088 | 2.13× |
| 100 | 5 | 0.178 | 0.459 | 2.58× |
| 100 | 10 | 0.445 | 1.008 | 2.26× |
| 1,000 | 1 | 0.048 | 0.098 | 2.06× |
| 1,000 | 5 | 0.195 | 0.496 | 2.54× |
| 1,000 | 10 | 0.441 | 1.180 | 2.68× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
