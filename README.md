# 113-1 雲原生應用程式開發 — 作業說明總覽

## 📌 課程作業總覽

本學期課程回家作業包含兩大部分：

- **作業一：CLI 線上市場系統開發（使用 Rust）**
- **作業二：GitHub Workflow 與 CI/CD 操作**

---

## 🧩 作業一：線上市場 CLI 應用程式（Rust 實作）

### 專案簡介

作業一實作了一個模擬的線上市場管理系統，使用 Rust 開發，並具備以下功能：

- 使用者註冊帳號
- 建立商品列表
- 查詢商品資料
- 刪除商品
- 透過 CLI 指令操作，支援即時輸入與輸出

### 技術重點

- 語言：Rust 1.71.0
- 套件管理：Cargo
- 使用套件：
  - `rustyline`：處理 CLI 輸入
  - `chrono`：處理時間資料
- 架構設計：模組化、清晰分層、可擴展與測試友善

📂 完整專案路徑：[hw1 資料夾](./HW1/cloud-hw1-market)

---

## 🛠️ 作業二：GitHub 操作與 CI/CD 工作流程

### ✅ Repo 操作（20 分）

- [x] **Public Repo** 已建立，助教可瀏覽
- [x] **README.md 已自訂修改**（非預設）
- [x] **Branch 數量充足**：共三個 branch（`main`, `hw1-p`, `hw1-f`）

### ✅ Issue 操作（20 分）

- [x] 已創建至少一個 **open 狀態的 Issue**
- [x] 已建立 **Issue Template**，供創建時選擇使用

### ✅ Pull Request 操作（20 分）

- [x] `hw1-p` 對 `main` 的 PR 已建立，含有程式修改
- [x] `hw1-f` 對 `main` 的 PR 已建立，含有程式修改
- [x] 至少一個 PR 內含 **留言互動**，並針對程式碼進行評論

### ✅ GitHub Action 操作（40 分）

- [x] Repo 中已執行過 **至少一次 Action**（成功或失敗）
- [x] Action Workflow 中額外定義 **至少兩個步驟**
- [x] `hw1-p` 的 PR 綁定的 Action 成功執行
- [x] `hw1-f` 的 PR 綁定的 Action 執行結果為失敗（故意造成）

🔗 GitHub Actions Workflow 設定與檔案請參考 `.github/workflows/` 目錄
