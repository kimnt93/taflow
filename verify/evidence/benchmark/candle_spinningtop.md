# CandleSpinningTop benchmark (`CDLSPINNINGTOP` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.070 | 14.36M | 0.060 | 16.61M | 0.031 | 0.44× | 0.51× |
| 10,000 | 0.514 | 19.45M | 0.500 | 20.01M | 0.139 | 0.27× | 0.28× |
| 100,000 | 4.827 | 20.72M | 4.820 | 20.75M | 0.971 | 0.20× | 0.20× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.148 | 0.165 | 1.11× |
| 1 | 5 | 0.471 | 0.466 | 0.99× |
| 1 | 10 | 0.654 | 0.908 | 1.39× |
| 10 | 1 | 0.066 | 0.088 | 1.33× |
| 10 | 5 | 0.322 | 0.423 | 1.31× |
| 10 | 10 | 0.678 | 0.930 | 1.37× |
| 100 | 1 | 0.083 | 0.090 | 1.08× |
| 100 | 5 | 0.317 | 0.454 | 1.43× |
| 100 | 10 | 0.689 | 1.067 | 1.55× |
| 1,000 | 1 | 0.139 | 0.101 | 0.72× |
| 1,000 | 5 | 0.316 | 0.490 | 1.55× |
| 1,000 | 10 | 0.680 | 1.040 | 1.53× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
