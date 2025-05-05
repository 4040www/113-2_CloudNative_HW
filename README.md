# 113-1 雲原生應用程式開發 — 作業說明總覽

## 📌 課程作業總覽

本學期課程回家作業包含以下幾個部分：

- **作業一：CLI 線上市場系統開發（使用 Rust）**
- **作業二：GitHub Workflow 與 CI/CD 操作**
- **作業四：Rust Dockerized Application**

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

---

## 🛠️ 作業四：Rust Dockerized Application

本專案為一個使用 Rust 開發的應用程式，並透過 Docker 建立容器化流程，同時搭配 GitHub Actions 實現自動化建構與推送至 Docker Hub。
相關作業檔案都實做於 docker 以及 broken-docker 分支中。

### 📦 1. 透過 Docker 打包應用程式

請先確認你已安裝 [Docker](https://docs.docker.com/get-docker/) 並進入正確的工作目錄以及分支 docker 中。

```bash
cd HW1/cloud-hw1-market
docker build -t yibchen/2025cloud:latest .
```

這個指令會依照 Dockerfile 設定，進行兩階段建構：
- 第一階段：使用 Rust 官方映像建構 release 可執行檔。
- 第二階段：轉換為更輕量的 Debian 容器，僅保留執行所需的二進位與腳本。

作業要求的 issue 連結：[issue 說明](https://github.com/4040www/113-2_CloudNative_HW/issues/7#issue-3038499081)

---

### ▶️ 2. 透過 Docker 運行 Container Image

你可以選擇以下其中一種方法來運行已構建好的應用程式容器映像檔：

#### 1. **直接從 Docker Hub 拉取並運行映像檔**

如果你不需要本地構建，只需直接從 Docker Hub 上拉取映像檔並運行。使用以下指令：

```bash
docker run -it yibchen/2025cloud:latest
```

這樣會自動拉取最新的映像檔並運行。`--rm` 會在容器停止後自動清除容器，讓環境保持乾淨。

#### 2. **從本地構建並運行容器映像檔**

如果你選擇從源碼構建容器映像檔，可以先執行以下命令來構建映像檔：

```bash
docker build -t yibchen/2025cloud:latest ./HW1/cloud-hw1-market
```

構建完成後，使用以下指令運行本地構建的容器映像檔：

```bash
docker run -it yibchen/2025cloud:latest
```

這樣你就可以使用本地構建的映像檔來運行應用程式。

---

### 🔁 3. 自動化 Container Image 的產生邏輯（CI/CD）

#### GitHub Actions 工作流程

每當符合以下任一條件，將會觸發 GitHub Actions 自動建構與推送：

- 有 commit 推送到 `docker` 分支
- 有 PR（Pull Request）建立至 `docker` 分支
- 手動執行 workflow（workflow_dispatch）

#### Tag 選擇邏輯

目前固定使用 `latest` 作為推送標籤（`yibchen/2025cloud:latest`）。  
若未來需要支援版本標籤，可搭配 Git SHA、版本號等方式擴充。

#### CI/CD 流程圖

```
User pushes code or PR
      ↓
GitHub Action (docker-build.yml)
      ↓
Docker Build (Buildx)
      ↓
Docker Push to yibchen/2025cloud:latest
```

### 🧪 測試失敗 PR 範例

為驗證 CI 能正確偵測錯誤，可參考這個故意破壞 Dockerfile 的 PR：  
👉 [PR 連結範例](https://github.com/4040www/113-2_CloudNative_HW/pull/6)

當中複製不存在的檔案，導致自動化流程失敗。

