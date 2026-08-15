# InsideBar benchmark (`inside bar relation` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.003 | 348.91M | 0.001 | 706.44M | 0.025 | 8.65× | 17.52× |
| 10,000 | 0.011 | 877.27M | 0.008 | 1.23G | 0.045 | 3.95× | 5.52× |
| 100,000 | 0.103 | 975.45M | 0.073 | 1.37G | 0.253 | 2.47× | 3.45× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.056 | 0.096 | 1.71× |
| 1 | 5 | 0.239 | 0.372 | 1.56× |
| 1 | 10 | 0.396 | 0.816 | 2.06× |
| 10 | 1 | 0.042 | 0.083 | 1.98× |
| 10 | 5 | 0.176 | 0.348 | 1.97× |
| 10 | 10 | 0.385 | 0.776 | 2.02× |
| 100 | 1 | 0.042 | 0.070 | 1.68× |
| 100 | 5 | 0.186 | 0.402 | 2.16× |
| 100 | 10 | 0.412 | 0.739 | 1.80× |
| 1,000 | 1 | 0.047 | 0.082 | 1.76× |
| 1,000 | 5 | 0.193 | 0.508 | 2.64× |
| 1,000 | 10 | 0.399 | 1.277 | 3.20× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
