# 雲原生應用程式開發

學號：B11605019
系級：資管三 
姓名：陳詣斌  

## 程式執行環境
本專案使用 Rust 語言開發，適用於 Linux 環境。建置與執行方式如下：

### 1. 安裝 Rust
請確保系統已安裝 Rust，可以使用以下指令確認版本：
```bash
rustc --version
```
本專案使用 Rust 2021 版。

### 2. 安裝必要工具
本專案使用 `cargo` 進行建置與管理，請確認是否已安裝：
```bash
cargo --version
```

### 3. 建置與執行
- **建置專案**
```bash
./build.sh
```
- **執行專案**
```bash
./run.sh
```

## 專案結構
專案的目錄結構如下：
```
src/
│── main.rs
│── commands/
│   │── mod.rs
│   │── register.rs
│   │── create_listing.rs
│   │── delete_listing.rs
│   │── get_listing.rs
│   │── get_category.rs
│   │── get_top_category.rs
│── utils/
│   │── mod.rs
│   └── utils.rs
│── services/
│   │── mod.rs
│   │── listing_service.rs
│   └── user_service.rs
│── models/
│   │── listing.rs
│   │── user.rs
│   └── utils.rs
└── main.rs

build.sh
run.sh
Cargo.toml
```

## 使用的套件及版本
專案依賴的主要套件如下：
```toml
[dependencies]
rustyline = "15.0.0"
chrono = "0.4"
```
- **rustyline**：用於處理命令行輸入，提供歷史記錄與自動補全功能。
- **chrono**：用於處理日期與時間格式。

## 功能說明
本 CLI 應用程式提供以下功能：
1. **REGISTER `<username>`** - 註冊新使用者。
2. **CREATE_LISTING `<username>` `<title>` `<description>` `<price>` `<category>`** - 創建商品列表。
3. **GET_LISTING `<username>` `<listing_id>`** - 取得指定商品資訊。
4. **DELETE_LISTING `<username>` `<listing_id>`** - 刪除商品。
5. **GET_CATEGORY `<username>` `<category>`** - 獲取該類別下的商品列表。
6. **GET_TOP_CATEGORY `<username>`** - 查詢使用者最常用的商品分類。

## 錯誤處理
- 註冊已存在的使用者會返回錯誤訊息。
- 查詢不存在的商品或分類時，會返回相應的錯誤。
- 指令格式錯誤時，會提示使用正確的指令格式。

## 測試與驗證
請使用 CLI 進行測試，例如：
```bash
./run.sh
REGISTER user1
CREATE_LISTING user1 'Phone model 8' 'Black color, brand new' 1000 'Electronics'
GET_LISTING user1 100001
```

## 其他注意事項
- 所有輸入與輸出皆透過 STDIN 與 STDOUT 處理。
- 不支援 Web 介面，僅提供 CLI 操作。
- 專案已確保模組化設計，便於擴展與測試。
