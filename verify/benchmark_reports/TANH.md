# MathTanh benchmark (`TANH` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 148.20M | 0.006 | 172.59M | 0.029 | 4.26× | 4.97× |
| 10,000 | 0.040 | 250.40M | 0.037 | 269.60M | 0.054 | 1.34× | 1.45× |
| 100,000 | 0.396 | 252.35M | 0.375 | 266.72M | 0.287 | 0.72× | 0.76× |
| 1,000,000 | 4.450 | 224.73M | 4.087 | 244.70M | 2.726 | 0.61× | 0.67× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.102 | 0.116 | 1.13× |
| 1 | 5 | 0.270 | 0.460 | 1.71× |
| 1 | 10 | 0.470 | 0.901 | 1.92× |
| 10 | 1 | 0.047 | 0.085 | 1.79× |
| 10 | 5 | 0.240 | 0.416 | 1.74× |
| 10 | 10 | 0.465 | 0.894 | 1.92× |
| 100 | 1 | 0.048 | 0.085 | 1.76× |
| 100 | 5 | 0.225 | 0.428 | 1.90× |
| 100 | 10 | 0.486 | 0.891 | 1.83× |
| 1,000 | 1 | 0.052 | 0.088 | 1.69× |
| 1,000 | 5 | 0.235 | 0.460 | 1.96× |
| 1,000 | 10 | 0.501 | 0.972 | 1.94× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
