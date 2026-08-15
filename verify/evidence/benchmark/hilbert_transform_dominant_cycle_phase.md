# HilbertTransformDominantCyclePhase benchmark (`HT_DCPHASE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.097 | 10.35M | 0.094 | 10.61M | 0.437 | 4.53× | 4.64× |
| 10,000 | 0.980 | 10.20M | 1.005 | 9.95M | 4.205 | 4.29× | 4.18× |
| 100,000 | 9.769 | 10.24M | 10.161 | 9.84M | 41.852 | 4.28× | 4.12× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.153 | 0.197 | 1.29× |
| 1 | 5 | 0.240 | 0.449 | 1.87× |
| 1 | 10 | 0.414 | 0.895 | 2.16× |
| 10 | 1 | 0.044 | 0.087 | 1.98× |
| 10 | 5 | 0.186 | 0.403 | 2.16× |
| 10 | 10 | 0.370 | 0.898 | 2.43× |
| 100 | 1 | 0.052 | 0.113 | 2.18× |
| 100 | 5 | 0.201 | 0.540 | 2.69× |
| 100 | 10 | 0.428 | 1.122 | 2.62× |
| 1,000 | 1 | 0.143 | 0.522 | 3.64× |
| 1,000 | 5 | 0.255 | 2.664 | 10.45× |
| 1,000 | 10 | 0.475 | 5.155 | 10.85× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
