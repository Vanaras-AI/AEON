use rusqlite::{Connection, params, Result};
use std::path::{Path, PathBuf};
use std::fs::File;
use std::io::Write;
use chrono::{Local, Utc, Duration, DateTime};
use tokio::sync::{mpsc, oneshot};
use tokio::task;
use std::sync::{Arc, Mutex};
use regex::Regex;
use lazy_static::lazy_static;

const LEDGER_DIR: &str = ".aeon";
const LEDGER_DB: &str = "ledger.db";

#[derive(Clone, Debug)]
pub struct LedgerEntry {
    pub agent_id: String,
    pub operation: String,
    pub target: Option<String>,
    pub status: String,
    pub metadata: Option<String>,
    // timestamp/datetime are set by the actor
}

#[derive(Debug, Clone)]
pub struct LedgerRecord {
    pub id: i64,
    pub timestamp: i64,
    pub datetime: String,
    pub agent_id: String,
    pub operation: String,
    pub target: Option<String>,
    pub status: String,
    pub metadata: Option<String>,
}

impl LedgerRecord {
    pub fn format(&self) -> String {
       let target = self.target.as_deref().unwrap_or("N/A");
       let meta = self.metadata.as_deref().unwrap_or("-");
       let status_icon = if self.status == "SUCCESS" { "✅" } else { "❌" };
       format!(
            "{} [{}] {} | Agent: {} | Op: {} -> {} | Meta: {}", 
            status_icon,
            self.datetime,
            self.id,
            self.agent_id,
            self.operation,
            target,
            meta
        )
    }
}

#[derive(Debug, Clone)]
pub struct AgentStat {
    pub agent_id: String,
    pub count: i64,
}

#[derive(Debug, Clone)]
pub struct OpStat {
    pub operation: String,
    pub count: i64,
}

#[derive(Debug, Clone)]
pub struct TimeStat {
    pub time_bucket: String,
    pub count: i64,
}

#[derive(Debug, Clone)]
pub struct Anomaly {
    pub severity: String,
    pub check: String,
    pub description: String,
    pub timestamp: i64,
}

pub enum DetectionRule {
    All,
    PrivilegeEscalation,
    BurstActivity,
    FailedOpsSpike,
}

enum LedgerMsg {
    Append(LedgerEntry),
    AppendBatch(Vec<LedgerEntry>),
    Query(QueryType, oneshot::Sender<Result<QueryResult>>),
    Archive(u64, oneshot::Sender<Result<usize>>),
    Detect(DetectionRule, oneshot::Sender<Result<Vec<Anomaly>>>),
    Flush(oneshot::Sender<()>),
}

enum QueryType {
    Recent(usize),
    Count,
    TopAgents(usize),
    OpDist,
    SuccessMetrics,
    Timeline,
}

enum QueryResult {
    Records(Vec<LedgerRecord>),
    Count(i64),
    AgentStats(Vec<AgentStat>),
    OpStats(Vec<OpStat>),
    Metrics(i64, i64, f64),
    TimeStats(Vec<TimeStat>),
}

#[derive(Clone)]
pub struct Ledger {
    tx: mpsc::Sender<LedgerMsg>,
}

lazy_static! {
    static ref GLOBAL_LEDGER: Mutex<Option<Ledger>> = Mutex::new(None);
    static ref SECRET_PATTERNS: Vec<Regex> = vec![
        Regex::new(r"(?i)(api_key|token|secret|password|key)['\u0022]?\s*[:=]\s*['\u0022]?([\w\d]+)['\u0022]?").unwrap(),
        Regex::new(r"(sk-[a-zA-Z0-9]{20,})").unwrap(),
    ];
}

fn scrub_secrets(mut entry: LedgerEntry) -> LedgerEntry {
    if let Some(ref meta) = entry.metadata {
        let mut redacted = meta.clone();
        for re in SECRET_PATTERNS.iter() {
            redacted = re.replace_all(&redacted, "$1: [REDACTED]").to_string();
        }
        entry.metadata = Some(redacted);
    }
    entry
}

impl Ledger {
    pub fn init() -> Result<Self> {
        let mut global = GLOBAL_LEDGER.lock().unwrap();
        if let Some(ledger) = &*global {
            return Ok(ledger.clone());
        }

        let (tx, mut rx) = mpsc::channel(10000); // Titanium Fix: 100x buffer size for burst absorption
        
        // Spawn background actor
        // We use std::thread::spawn or tokio::spawn? 
        // Since main is tokio, tokio::task::spawn_blocking is best for the SQLite loop.
        // But init might be called before runtime? No, main is async.
        // Warning: if init is called outside async context, spawn_blocking fails.
        // Safety fallback: std::thread if no runtime? 
        // For simplicity in this project, we assume tokio runtime exists.
        
        // Actually, to support "legacy" synchronous calls without runtime, we might need std::thread.
        // But the channel is tokio::mpsc... which requires async recv.
        // So we must be in async context or use std::sync::mpsc (but we want async public API).
        // We stick to tokio.
        
        tokio::task::spawn_blocking(move || {
            let ledger_dir = PathBuf::from(LEDGER_DIR);
            if !ledger_dir.exists() {
                let _ = std::fs::create_dir_all(&ledger_dir);
            }
            let db_path = ledger_dir.join(LEDGER_DB);
            
            // Connect and Init DB
            let mut conn = match Connection::open(&db_path) {
                Ok(c) => c,
                Err(e) => {
                    eprintln!("❌ [LEDGER ACTOR] Failed to open DB: {}", e);
                    return;
                }
            };

            // Schema Init
            let _ = conn.execute(
                "CREATE TABLE IF NOT EXISTS ledger (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    timestamp INTEGER NOT NULL,
                    datetime TEXT NOT NULL,
                    agent_id TEXT NOT NULL,
                    operation TEXT NOT NULL,
                    target TEXT,
                    status TEXT NOT NULL,
                    metadata TEXT
                )", []);
             let _ = conn.execute_batch(
                "CREATE INDEX IF NOT EXISTS idx_timestamp ON ledger(timestamp);
                 CREATE INDEX IF NOT EXISTS idx_agent ON ledger(agent_id);
                 CREATE INDEX IF NOT EXISTS idx_operation ON ledger(operation);
                 PRAGMA journal_mode=WAL;
                 PRAGMA synchronous = NORMAL;
                 PRAGMA temp_store = MEMORY;
                 PRAGMA mmap_size = 30000000000;"
            );

            println!("📊 [LEDGER] Actor started.");
            
            while let Some(msg) = rx.blocking_recv() {
                match msg {
                    LedgerMsg::Append(entry) => {
                         // Redaction is done upstream
                         // [CO-FOUNDER] TITANIUM RULE: If we cannot write, we die. No silent failures.
                         Self::insert_one(&conn, &entry).expect("🔥 [FATAL] LEDGER WRITE FAILURE: Disk Full or Corrupt?");
                    },
                    LedgerMsg::AppendBatch(entries) => {
                        // Redaction is done upstream
                        // [CO-FOUNDER] TITANIUM RULE: Fail-Stop on batch error.
                        Self::insert_batch(&mut conn, &entries).expect("🔥 [FATAL] LEDGER BATCH FAILURE: Transaction Failed");
                    },
                    LedgerMsg::Query(q_type, resp) => {
                        let res = match q_type {
                            QueryType::Recent(limit) => Self::query_recent(&conn, limit).map(QueryResult::Records),
                            QueryType::Count => Self::query_count(&conn).map(QueryResult::Count),
                            QueryType::TopAgents(limit) => Self::query_top_agents(&conn, limit).map(QueryResult::AgentStats),
                            QueryType::OpDist => Self::query_op_dist(&conn).map(QueryResult::OpStats),
                            QueryType::SuccessMetrics => Self::query_metrics(&conn).map(|(s,f,r)| QueryResult::Metrics(s,f,r)),
                            QueryType::Timeline => Self::query_timeline(&conn).map(QueryResult::TimeStats),
                        };
                        let _ = resp.send(res);
                    },
                    LedgerMsg::Archive(days, resp) => {
                        let _ = resp.send(Self::perform_archive(&conn, days));
                    },
                    LedgerMsg::Detect(rule, resp) => {
                        let _ = resp.send(Self::perform_detect(&conn, rule));
                    },
                    LedgerMsg::Flush(resp) => {
                        // Just processing this message means queue is empty up to this point
                        let _ = resp.send(());
                    }
                }
            }
        });

        let ledger = Self { tx };
        *global = Some(ledger.clone());
        Ok(ledger)
    }

    // --- Public Async API ---

    pub async fn append(&self, entry: LedgerEntry) -> Result<()> {
        let safe_entry = scrub_secrets(entry);
        self.tx.send(LedgerMsg::Append(safe_entry)).await
            .map_err(|_| rusqlite::Error::InvalidQuery) // Channel closed
    }
    
    // Non-blocking append for WASM host functions (avoids tokio panic)
    pub fn append_non_blocking(&self, entry: LedgerEntry) -> Result<()> {
        let safe_entry = scrub_secrets(entry);
        match self.tx.try_send(LedgerMsg::Append(safe_entry)) {
            Ok(_) => Ok(()),
            Err(mpsc::error::TrySendError::Full(_)) => {
                 // [INTEGRITY] Truth in Ledger (Sprint 4.7)
                 // Do not lie. Return error so caller knows we are overloaded.
                 Err(rusqlite::Error::SqliteFailure(
                     rusqlite::ffi::Error { code: rusqlite::ffi::ErrorCode::SystemIoFailure, extended_code: 0 },
                     Some("Ledger Buffer Full (Backpressure)".to_string())
                 ))
            },
            Err(mpsc::error::TrySendError::Closed(_)) => Err(rusqlite::Error::InvalidQuery),
        }
    }

    pub async fn append_batch(&self, entries: Vec<LedgerEntry>) -> Result<()> {
         let safe_entries: Vec<LedgerEntry> = entries.into_iter().map(scrub_secrets).collect();
         self.tx.send(LedgerMsg::AppendBatch(safe_entries)).await
            .map_err(|_| rusqlite::Error::InvalidQuery)
    }
    
    // Titanium Fix: Shutdown flush
    pub async fn flush(&self) {
        let (tx, rx) = oneshot::channel();
        if self.tx.send(LedgerMsg::Flush(tx)).await.is_ok() {
            let _ = rx.await;
        }
    }

    pub async fn recent(&self, limit: usize) -> Result<Vec<LedgerRecord>> {
        let (tx, rx) = oneshot::channel();
        self.tx.send(LedgerMsg::Query(QueryType::Recent(limit), tx)).await.map_err(|_| rusqlite::Error::InvalidQuery)?;
        match rx.await {
            Ok(Ok(QueryResult::Records(r))) => Ok(r),
            Ok(Err(e)) => Err(e),
            _ => Err(rusqlite::Error::InvalidQuery),
        }
    }

    pub async fn count(&self) -> Result<i64> {
        let (tx, rx) = oneshot::channel();
        self.tx.send(LedgerMsg::Query(QueryType::Count, tx)).await.map_err(|_| rusqlite::Error::InvalidQuery)?;
        match rx.await {
            Ok(Ok(QueryResult::Count(c))) => Ok(c),
            Ok(Err(e)) => Err(e),
            _ => Err(rusqlite::Error::InvalidQuery),
        }
    }

    pub async fn top_agents(&self, limit: usize) -> Result<Vec<AgentStat>> {
        let (tx, rx) = oneshot::channel();
        self.tx.send(LedgerMsg::Query(QueryType::TopAgents(limit), tx)).await.map_err(|_| rusqlite::Error::InvalidQuery)?;
        match rx.await {
            Ok(Ok(QueryResult::AgentStats(s))) => Ok(s),
            Ok(Err(e)) => Err(e),
            _ => Err(rusqlite::Error::InvalidQuery),
        }
    }

    pub async fn operation_distribution(&self) -> Result<Vec<OpStat>> {
        let (tx, rx) = oneshot::channel();
        self.tx.send(LedgerMsg::Query(QueryType::OpDist, tx)).await.map_err(|_| rusqlite::Error::InvalidQuery)?;
        match rx.await {
            Ok(Ok(QueryResult::OpStats(s))) => Ok(s),
            Ok(Err(e)) => Err(e),
            _ => Err(rusqlite::Error::InvalidQuery),
        }
    }

     pub async fn success_metrics(&self) -> Result<(i64, i64, f64)> {
        let (tx, rx) = oneshot::channel();
        self.tx.send(LedgerMsg::Query(QueryType::SuccessMetrics, tx)).await.map_err(|_| rusqlite::Error::InvalidQuery)?;
        match rx.await {
            Ok(Ok(QueryResult::Metrics(s,f,r))) => Ok((s,f,r)),
            Ok(Err(e)) => Err(e),
            _ => Err(rusqlite::Error::InvalidQuery),
        }
    }

    pub async fn timeline(&self) -> Result<Vec<TimeStat>> {
        let (tx, rx) = oneshot::channel();
        self.tx.send(LedgerMsg::Query(QueryType::Timeline, tx)).await.map_err(|_| rusqlite::Error::InvalidQuery)?;
        match rx.await {
            Ok(Ok(QueryResult::TimeStats(s))) => Ok(s),
            Ok(Err(e)) => Err(e),
            _ => Err(rusqlite::Error::InvalidQuery),
        }
    }

    pub async fn archive_older_than(&self, days: u64) -> Result<usize> {
        let (tx, rx) = oneshot::channel();
         self.tx.send(LedgerMsg::Archive(days, tx)).await.map_err(|_| rusqlite::Error::InvalidQuery)?;
         match rx.await {
            Ok(r) => r,
            Err(_) => Err(rusqlite::Error::InvalidQuery),
        }
    }

    pub async fn detect_anomalies(&self, rule: DetectionRule) -> Result<Vec<Anomaly>> {
        let (tx, rx) = oneshot::channel();
         self.tx.send(LedgerMsg::Detect(rule, tx)).await.map_err(|_| rusqlite::Error::InvalidQuery)?;
         match rx.await {
            Ok(r) => r,
            Err(_) => Err(rusqlite::Error::InvalidQuery),
        }
    }

    // --- Private Sync Helpers (Run inside Actor) ---
    
    fn insert_one(conn: &Connection, entry: &LedgerEntry) -> Result<()> {
        let timestamp = Utc::now().timestamp();
        let datetime = Local::now().to_rfc3339();
        conn.execute(
            "INSERT INTO ledger (timestamp, datetime, agent_id, operation, target, status, metadata)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![timestamp, datetime, entry.agent_id, entry.operation, entry.target, entry.status, entry.metadata],
        )?;
        Ok(())
    }

    fn insert_batch(conn: &mut Connection, entries: &[LedgerEntry]) -> Result<()> {
        let tx = conn.transaction()?;
        {
            let mut stmt = tx.prepare(
                "INSERT INTO ledger (timestamp, datetime, agent_id, operation, target, status, metadata)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)"
            )?;
            let timestamp = Utc::now().timestamp();
            let datetime = Local::now().to_rfc3339();
            for entry in entries {
                stmt.execute(params![timestamp, datetime, entry.agent_id, entry.operation, entry.target, entry.status, entry.metadata])?;
            }
        }
        tx.commit()?;
        Ok(())
    }

    fn query_recent(conn: &Connection, limit: usize) -> Result<Vec<LedgerRecord>> {
        let mut stmt = conn.prepare("SELECT id, timestamp, datetime, agent_id, operation, target, status, metadata FROM ledger ORDER BY timestamp DESC LIMIT ?1")?;
        let rows = stmt.query_map([limit], |row| Ok(LedgerRecord {
            id: row.get(0)?, timestamp: row.get(1)?, datetime: row.get(2)?, agent_id: row.get(3)?, operation: row.get(4)?, target: row.get(5)?, status: row.get(6)?, metadata: row.get(7)?
        }))?;
        rows.collect()
    }

    fn query_count(conn: &Connection) -> Result<i64> {
        conn.query_row("SELECT COUNT(*) FROM ledger", [], |row| row.get(0))
    }

    fn query_top_agents(conn: &Connection, limit: usize) -> Result<Vec<AgentStat>> {
        let mut stmt = conn.prepare("SELECT agent_id, COUNT(*) as c FROM ledger GROUP BY agent_id ORDER BY c DESC LIMIT ?1")?;
        let rows = stmt.query_map([limit], |row| Ok(AgentStat { agent_id: row.get(0)?, count: row.get(1)? }))?;
        let mut stats = Vec::new(); for r in rows { stats.push(r?); } Ok(stats)
    }

    fn query_op_dist(conn: &Connection) -> Result<Vec<OpStat>> {
        let mut stmt = conn.prepare("SELECT operation, COUNT(*) as c FROM ledger GROUP BY operation ORDER BY c DESC")?;
         let rows = stmt.query_map([], |row| Ok(OpStat { operation: row.get(0)?, count: row.get(1)? }))?;
        let mut stats = Vec::new(); for r in rows { stats.push(r?); } Ok(stats)
    }

    fn query_metrics(conn: &Connection) -> Result<(i64, i64, f64)> {
        let total: i64 = conn.query_row("SELECT COUNT(*) FROM ledger", [], |row| row.get(0))?;
        if total == 0 { return Ok((0,0,0.0)); }
        let success: i64 = conn.query_row("SELECT COUNT(*) FROM ledger WHERE status = 'SUCCESS'", [], |row| row.get(0))?;
        Ok((success, total-success, (success as f64 / total as f64) * 100.0))
    }

    fn query_timeline(conn: &Connection) -> Result<Vec<TimeStat>> {
        let mut stmt = conn.prepare("SELECT strftime('%Y-%m-%d %H:00', datetime) as bucket, COUNT(*) as c FROM ledger GROUP BY bucket ORDER BY bucket DESC LIMIT 24")?;
        let rows = stmt.query_map([], |row| Ok(TimeStat { time_bucket: row.get(0)?, count: row.get(1)? }))?;
        let mut stats = Vec::new(); for r in rows { stats.push(r?); }
        stats.reverse(); Ok(stats)
    }

    fn perform_archive(conn: &Connection, days: u64) -> Result<usize> {
        let cutoff = Utc::now() - Duration::days(days as i64);
        let timestamp_cutoff = cutoff.timestamp();
        let ledger_dir = PathBuf::from(LEDGER_DIR);
        let archive_dir = ledger_dir.join("archive");
        if !archive_dir.exists() { let _ = std::fs::create_dir_all(&archive_dir); }
        
        let mut stmt = conn.prepare("SELECT id, timestamp, datetime, agent_id, operation, target, status, metadata FROM ledger WHERE timestamp < ?1 ORDER BY timestamp ASC")?;
        let rows = stmt.query_map([timestamp_cutoff], |row| Ok(LedgerRecord {
             id: row.get(0)?, timestamp: row.get(1)?, datetime: row.get(2)?, agent_id: row.get(3)?, operation: row.get(4)?, target: row.get(5)?, status: row.get(6)?, metadata: row.get(7)?
        }))?;

        let mut to_archive = Vec::new();
        for r in rows { to_archive.push(r?); }
        if to_archive.is_empty() { return Ok(0); }

        let filename = format!("ledger_{}.jsonl", Utc::now().format("%Y%m%d_%H%M%S"));
        let path = archive_dir.join(filename);
        let mut file = File::create(&path).map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;

        for r in &to_archive {
             let json = serde_json::json!({
                "id": r.id, "timestamp": r.timestamp, "datetime": r.datetime, "agent_id": r.agent_id, "operation": r.operation, "target": r.target, "status": r.status, "metadata": r.metadata
            });
            let _ = writeln!(file, "{}", json.to_string());
        }
        
        conn.execute("DELETE FROM ledger WHERE timestamp < ?1", params![timestamp_cutoff])
    }
    
    fn perform_detect(conn: &Connection, rule: DetectionRule) -> Result<Vec<Anomaly>> {
        let mut anomalies = Vec::new();
        let now = Utc::now().timestamp();
        
        if matches!(rule, DetectionRule::All | DetectionRule::PrivilegeEscalation) {
            let mut stmt = conn.prepare("SELECT agent_id, operation, target, timestamp FROM ledger WHERE status LIKE 'BLOCKED%' OR status LIKE 'STOPPED%' ORDER BY timestamp DESC LIMIT 10")?;
            let rows = stmt.query_map([], |row| Ok((row.get::<_,String>(0)?, row.get::<_,String>(1)?, row.get::<_,Option<String>>(2)?, row.get::<_,i64>(3)?)))?;
            for r in rows { let (a,o,t,ts) = r?; anomalies.push(Anomaly { severity: "HIGH".to_string(), check: "Privilege Escalation".to_string(), description: format!("Agent '{}' attempted '{}' on '{:?}'", a,o,t), timestamp: ts }); }
        }
        
        if matches!(rule, DetectionRule::All | DetectionRule::BurstActivity) {
            let mut stmt = conn.prepare("SELECT agent_id, COUNT(*) as c FROM ledger WHERE timestamp > ?1 GROUP BY agent_id HAVING c > 50 ORDER BY c DESC")?;
            let rows = stmt.query_map([now - 60], |row| Ok((row.get::<_,String>(0)?, row.get::<_,i64>(1)?)))?;
            for r in rows { let (a,c) = r?; anomalies.push(Anomaly { severity: "MEDIUM".to_string(), check: "Burst Activity".to_string(), description: format!("Agent '{}' performed {} ops/min", a,c), timestamp: now }); }
        }

        if matches!(rule, DetectionRule::All | DetectionRule::FailedOpsSpike) {
            let mut stmt = conn.prepare("SELECT agent_id, COUNT(*) as c FROM ledger WHERE status = 'FAILURE' GROUP BY agent_id HAVING c > 5 ORDER BY c DESC")?;
            let rows = stmt.query_map([], |row| Ok((row.get::<_,String>(0)?, row.get::<_,i64>(1)?)))?;
             for r in rows { let (a,c) = r?; anomalies.push(Anomaly { severity: "MEDIUM".to_string(), check: "Failure Spike".to_string(), description: format!("Agent '{}' has {} failures", a,c), timestamp: now }); }
        }
        
        Ok(anomalies)
    }
}
