# RollingGrangerCausality benchmark (`GrangerCausality` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 1.767 | 566.07K | 1.729 | 578.22K | 7.701 | 4.36× | 4.45× |
| 10,000 | 20.488 | 488.08K | 18.270 | 547.34K | 79.445 | 3.88× | 4.35× |
| 100,000 | 196.210 | 509.66K | 192.065 | 520.66K | 804.462 | 4.10× | 4.19× |
| 1,000,000 | 1856.747 | 538.58K | 1820.224 | 549.38K | 8512.566 | 4.58× | 4.68× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.103 | 0.699 | 6.80× |
| 1 | 5 | 0.256 | 1.377 | 5.39× |
| 1 | 10 | 0.577 | 3.205 | 5.56× |
| 10 | 1 | 0.059 | 0.271 | 4.60× |
| 10 | 5 | 0.249 | 1.450 | 5.83× |
| 10 | 10 | 0.552 | 4.049 | 7.33× |
| 100 | 1 | 0.164 | 0.866 | 5.29× |
| 100 | 5 | 0.576 | 5.193 | 9.01× |
| 100 | 10 | 1.176 | 9.060 | 7.70× |
| 1,000 | 1 | 1.966 | 9.062 | 4.61× |
| 1,000 | 5 | 3.550 | 44.292 | 12.48× |
| 1,000 | 10 | 4.054 | 83.520 | 20.60× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
