# RollSpread benchmark (`rolling Roll spread estimator` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.279 | 3.58M | 0.271 | 3.69M | 0.243 | 0.87× | 0.89× |
| 10,000 | 2.725 | 3.67M | 2.852 | 3.51M | 1.240 | 0.45× | 0.43× |
| 100,000 | 28.807 | 3.47M | 27.448 | 3.64M | 12.258 | 0.43× | 0.45× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.130 | 0.129 | 0.99× |
| 1 | 5 | 0.368 | 0.541 | 1.47× |
| 1 | 10 | 0.567 | 1.302 | 2.30× |
| 10 | 1 | 0.064 | 0.110 | 1.71× |
| 10 | 5 | 0.303 | 0.515 | 1.70× |
| 10 | 10 | 0.603 | 1.037 | 1.72× |
| 100 | 1 | 0.088 | 0.230 | 2.62× |
| 100 | 5 | 0.290 | 1.142 | 3.94× |
| 100 | 10 | 0.604 | 2.319 | 3.84× |
| 1,000 | 1 | 0.353 | 0.335 | 0.95× |
| 1,000 | 5 | 0.517 | 1.443 | 2.79× |
| 1,000 | 10 | 0.952 | 2.842 | 2.99× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
