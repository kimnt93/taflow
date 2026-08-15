# NegativeVolumeIndex benchmark (`NVI` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 170.81M | 0.004 | 235.37M | 0.192 | 32.82× | 45.22× |
| 10,000 | 0.055 | 180.95M | 0.054 | 184.71M | 0.839 | 15.19× | 15.50× |
| 100,000 | 0.560 | 178.59M | 0.550 | 181.70M | 6.556 | 11.71× | 11.91× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.106 | 0.235 | 2.23× |
| 1 | 5 | 0.270 | 1.007 | 3.72× |
| 1 | 10 | 0.394 | 2.272 | 5.76× |
| 10 | 1 | 0.044 | 0.197 | 4.48× |
| 10 | 5 | 0.183 | 1.264 | 6.90× |
| 10 | 10 | 0.448 | 2.184 | 4.87× |
| 100 | 1 | 0.042 | 0.205 | 4.89× |
| 100 | 5 | 0.207 | 1.341 | 6.46× |
| 100 | 10 | 0.422 | 2.259 | 5.36× |
| 1,000 | 1 | 0.054 | 0.266 | 4.94× |
| 1,000 | 5 | 0.194 | 1.669 | 8.60× |
| 1,000 | 10 | 0.432 | 2.921 | 6.76× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
