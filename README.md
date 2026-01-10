# Gram - A text-to-ASCII-diagram tool

Gram is a CLI tool written in Rust. It converts structured text into clean, beautiful ASCII diagrams.

## Sequence diagram

**Input**
```
Client -> Server: GET /api/data
Server -> Client: JSONResponse
```

**Output**
```
 ┌────────┐      ┌────────┐
 │ Client │      │ Server │
 └───┬────┘      └───┬────┘
     │               │     
     │ GET /api/data │     
     │──────────────>│     
     │               │     
     │ JSONResponse  │     
     │<──────────────│     
     │               │     
 ┌───┴────┐      ┌───┴────┐
 │ Client │      │ Server │
 └────────┘      └────────┘
```