# PivotPoints benchmark (`anchored classic pivot points` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.023 | 42.98M | 0.027 | 37.52M | 0.979 | 42.08× | 36.73× |
| 10,000 | 0.096 | 103.77M | 0.080 | 124.54M | 8.574 | 88.97× | 106.79× |
| 100,000 | 0.898 | 111.38M | 0.709 | 140.99M | 87.699 | 97.68× | 123.65× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.136 | 0.144 | 1.06× |
| 1 | 5 | 0.306 | 0.359 | 1.17× |
| 1 | 10 | 0.449 | 0.684 | 1.53× |
| 10 | 1 | 0.055 | 0.082 | 1.50× |
| 10 | 5 | 0.225 | 0.399 | 1.78× |
| 10 | 10 | 0.473 | 0.822 | 1.74× |
| 100 | 1 | 0.057 | 0.171 | 3.01× |
| 100 | 5 | 0.235 | 0.840 | 3.58× |
| 100 | 10 | 0.522 | 1.775 | 3.40× |
| 1,000 | 1 | 0.065 | 1.003 | 15.52× |
| 1,000 | 5 | 0.319 | 5.079 | 15.95× |
| 1,000 | 10 | 0.612 | 10.648 | 17.41× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
