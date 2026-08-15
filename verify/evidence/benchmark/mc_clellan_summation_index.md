# McClellanSummationIndex benchmark (`McClellanSummationIndex` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 141.02M | 0.006 | 166.43M | 8.502 | 1199.00× | 1415.06× |
| 10,000 | 0.056 | 179.30M | 0.052 | 193.09M | 84.326 | 1511.93× | 1628.26× |
| 100,000 | 0.500 | 199.83M | 0.489 | 204.33M | 825.244 | 1649.08× | 1686.21× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.162 | 0.258 | 1.59× |
| 1 | 5 | 0.324 | 1.088 | 3.36× |
| 1 | 10 | 0.456 | 2.137 | 4.69× |
| 10 | 1 | 0.043 | 0.301 | 7.05× |
| 10 | 5 | 0.177 | 1.735 | 9.78× |
| 10 | 10 | 0.390 | 2.927 | 7.50× |
| 100 | 1 | 0.044 | 1.051 | 23.93× |
| 100 | 5 | 0.193 | 5.782 | 29.91× |
| 100 | 10 | 0.377 | 11.124 | 29.50× |
| 1,000 | 1 | 0.047 | 8.892 | 187.44× |
| 1,000 | 5 | 0.291 | 51.197 | 176.07× |
| 1,000 | 10 | 0.705 | 89.438 | 126.90× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
