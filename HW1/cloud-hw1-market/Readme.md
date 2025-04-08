# 113-1 雲原生應用程式開發 作業一

## 作者資訊
- **學號**：B11605019  
- **姓名**：陳詣斌

## 作業說明

本專案以 Rust 開發，旨在充分發揮 Rust 的安全性、效能與並發特性，以構建一個穩健且可擴展的 CLI 應用程式。  
本專案實作了一個模擬的線上市場交易系統，使用者可以進行以下操作：

- 註冊帳號
- 建立商品列表
- 查詢商品
- 刪除商品

所有操作皆透過標準輸入（STDIN）接收指令，並將結果輸出至標準輸出（STDOUT）。

---

## 執行環境與依賴

- **編譯器**：Rust 1.71.0  
- **套件管理工具**：Cargo  

### 使用的第三方套件

- `rustyline 15.0.0`：提供命令列輸入編輯功能  
- `chrono 0.4`：用於處理時間相關操作  

---

## 系統架構與模組設計

本專案採用模組化設計，以提高可讀性、可維護性與擴展性。目錄結構如下：
'''
src/ 
├── commands/ 
│   ├── mod.rs 
│   ├── register.rs 
│   ├── create\_listing.rs 
│   ├── delete\_listing.rs 
│   ├── get\_listing.rs 
│   ├── get\_category.rs 
│   ├── get\_top\_category.rs 
├── utils/ 
│   ├── mod.rs 
│   └── utils.rs 
├── services/ 
│   ├── mod.rs 
│   ├── listing\_service.rs 
│   └── user\_service.rs 
├── models/ 
│   ├── listing.rs 
│   ├── user.rs 
│   └── utils.rs 
└── main.rs
'''
### 模組間的互動與責任劃分

- `commands/`：負責解析使用者輸入的指令，並呼叫對應的服務邏輯。
- `services/`：實作應用程式的業務邏輯，如使用者與商品的管理。
- `models/`：定義應用程式中的資料結構（如 `User` 與 `Listing`）。
- `utils/`：提供通用的輔助工具函式，例如字串處理與時間處理。

---

## 設計原則與擴展性分析

### 關注點分離（Separation of Concerns）

- `commands` 模組專注於指令解析，不直接處理業務邏輯。
- `services` 模組集中處理商業邏輯，便於維護與測試。
- `models` 模組負責數據結構定義，避免與業務邏輯耦合。

### 可擴展性（Extensibility）

- 採用清楚的指令處理架構，新增新指令時只需：
  - 新增對應的 `.rs` 檔案
  - 更新 `commands/mod.rs` 模組

### 模組化與測試性（Modularity and Testability）

- 模組遵循單一職責原則（SRP），功能分明，易於測試與維護。
- 各個 service 模組可獨立進行單元測試，不需依賴 CLI 主程式。

---

## 結論

本 CLI 市場管理系統透過 Rust 的模組化特性與強型別系統，提供一個安全、高效且可擴展的解決方案。  
清楚的分層架構與良好的職責分離設計，讓後續功能擴充與維護變得更加簡單與直觀。

