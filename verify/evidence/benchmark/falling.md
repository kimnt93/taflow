# Falling benchmark (`period-over-period falling` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 154.86M | 0.006 | 179.54M | 0.030 | 4.64× | 5.38× |
| 10,000 | 0.047 | 212.40M | 0.043 | 230.49M | 0.039 | 0.82× | 0.89× |
| 100,000 | 0.445 | 224.57M | 0.415 | 240.84M | 0.132 | 0.30× | 0.32× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.097 | 0.106 | 1.10× |
| 1 | 5 | 0.300 | 0.503 | 1.68× |
| 1 | 10 | 0.373 | 0.915 | 2.45× |
| 10 | 1 | 0.041 | 0.090 | 2.18× |
| 10 | 5 | 0.184 | 0.439 | 2.39× |
| 10 | 10 | 0.403 | 0.929 | 2.30× |
| 100 | 1 | 0.041 | 0.093 | 2.29× |
| 100 | 5 | 0.184 | 0.451 | 2.45× |
| 100 | 10 | 0.395 | 0.927 | 2.35× |
| 1,000 | 1 | 0.047 | 0.094 | 2.00× |
| 1,000 | 5 | 0.200 | 0.502 | 2.50× |
| 1,000 | 10 | 0.443 | 1.093 | 2.47× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
