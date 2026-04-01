# Copilot Instructions: Rust boot start 

## Project Context
This is a revision excercise fo rhe Rust boot start to get familiear with the rust programming language.  This is a focused, 4-day "Rust for Gopher" sprint. Since you are moving from **Go** to **(T0)** (your UDS writer), these exercises bypass generic Rust and focus strictly on **Memory Control** and **Async Messaging**.

---

## 📅 Day 1: Ownership & The Borrow Checker
**Goal:** Understand how to pass data without copying it or crashing the program.

### Exercise 1.1: The Move
* **Task:** Create a function `send_to_writer(data: String)`. Call it twice with the same variable.
* **Lesson:** Observe the compiler error. In Go, this works; in Rust, the first call "consumes" the data.

### Exercise 1.2: The Borrow
* **Task:** Modify the function to take `&String`. Call it twice.
* **Lesson:** Learn how to keep data alive in the "Go Core" while the "Rust Writer" looks at it.

### Exercise 1.3: The Mutable Mutation
* **Task:** Create a `HashMap` of Iceberg properties. Write a function that takes `&mut HashMap` to add a default `olake_version`.
* **Lesson:** Understand why you can only have **one** mutable reference at a time.

---

## 📅 Day 2: Enums, Options, and Results
**Goal:** Learn to handle "Missing Data" and "Errors" without `nil`.

### Exercise 2.1: The CDC Enum
* **Task:** Define an `enum CdcOp { Insert(String), Delete(i64) }`. Use a `match` block to print different messages for each.
* **Lesson:** This is how you will distinguish between Postgres `INSERT` and `DELETE` events.

### Exercise 2.2: Finding Properties
* **Task:** Use `my_hashmap.get("key")`. It returns an `Option`. Use `.unwrap_or()` to provide a default value.
* **Lesson:** Safe handling of optional Iceberg table configurations.

### Exercise 2.3: Error Propagation
* **Task:** Write a function that "connects to S3" (simulated). Return `Result<(), String>`. Use the `?` operator in a caller function.
* **Lesson:** Mastering the "Question Mark" flow to replace `if err != nil`.

---

## 📅 Day 3: Async & Channels (The UDS Prep)
**Goal:** Building the "Pipe" between Go and the Writer.

### Exercise 3.1: The Tokio Spawn
* **Task:** Use `tokio::spawn` to run a loop that prints "Waiting for Go..." every second while the main thread sleeps.
* **Lesson:** Understanding the non-blocking nature of the sidecar.

### Exercise 3.2: MPSC (Multi-Producer, Single-Consumer)
* **Task:** Create a channel. Send 5 "Records" from one task to a "Writer" task.
* **Lesson:** This is exactly how the UDS server will pass data to the Iceberg logic.

### Exercise 3.3: Async Traits
* **Task:** Define a trait `IcebergWriter` with an `async fn write()`. Implement it for a dummy `S3Writer` struct.
* **Lesson:** Preparing for the `iceberg-rust` library's trait-heavy API.

---

## 📅 Day 4: The "Mini-Sidecar" (T0 Mockup)
**Goal:** Putting it all together.

### Exercise 4.1: The Byte Stream
* **Task:** Write a function that takes a `Vec<u8>` (simulating a UDS packet) and converts it into your `CdcOp` enum.
* **Lesson:** Data deserialization at the boundary.

### Exercise 4.2: The Batcher
* **Task:** Create a loop that collects 10 messages in a `Vec` before "flushing" them to a mock Iceberg table.
* **Lesson:** Implementing the "PhonePe scale" batching logic.

---

## 📊 Companion Score Card
Print this out or keep it in a `progress.md` file in your VSCode project.

| Day | Topic | Status (✅/❌) | Key Takeaway |
| :--- | :--- | :--- | :--- |
| **1** | **Ownership** | [ ] | I know when to use `&` vs `&mut`. |
| **1** | **Strings** | [ ] | I understand `String` vs `&str`. |
| **2** | **Enums** | [ ] | I can use `match` for CDC operations. |
| **2** | **Results** | [ ] | I can use `?` to handle S3/IO errors. |
| **3** | **Async** | [ ] | I can use `tokio::spawn`. |
| **3** | **Channels** | [ ] | I can move data between tasks safely. |
| **4** | **Batching** | [ ] | I can collect records into a batch. |
| **4** | **(T0) Mock** | [ ] | I can simulate a basic UDS logic. |

---

### Pro-Tip for your Team
Since there are only two of you, **Pair Program** Day 1 and Day 3. Ownership and Async are the "mental shift" points where having a second set of eyes helps significantly.

**Do you want me to provide the "Solution Code" for Day 1 so you can start your first exercise immediately?**